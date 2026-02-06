use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::{
    checkers::{Problem, RulesCtrlAgent, RulesD2, RulesV4},
    common::RuleChecker,
    configs::{KEACtrlAgentConfigFile, KEAD2ConfigFile, KEAv4ConfigFile},
};

pub mod args;
pub mod cli;
pub mod standalone;
pub use args::*;
pub use {cli::run_cli, standalone::run_standalone};

pub fn run_checks(
    config_v4: Option<KEAv4ConfigFile>,
    config_d2: Option<KEAD2ConfigFile>,
    config_ctrl_agent: Option<KEACtrlAgentConfigFile>,
) -> Vec<Problem> {
    let mut results: Vec<Problem> = Vec::new();

    if let Some(config) = config_v4 {
        let checker: RulesV4 = RulesV4::default();
        results.extend(checker.run(&config.dhcp4));
    }

    if let Some(config) = config_d2 {
        let checker: RulesD2 = RulesD2::default();
        results.extend(checker.run(&config.dhcp_ddns));
    }

    if let Some(config) = config_ctrl_agent {
        let checker: RulesCtrlAgent = RulesCtrlAgent::default();
        results.extend(checker.run(&config.ctrl_agent));
    }

    results
}

pub fn get_args(args: CLIArgs) -> CLIArgs {
    if let Some(config_path) = &args.config_filepath {
        let file_args: Option<CLIArgs> = match fs::read_to_string(Path::new(config_path)) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(config) => Some(config),
                Err(err) => panic!("An error occurred while parsing config file: {}", err),
            },
            Err(err) => panic!("An error occurred while reading the config file: {}", err),
        };

        if let Some(f_args) = file_args {
            return f_args;
        }
    }

    args
}

pub fn run_checks_parallel(
    config_v4: Option<KEAv4ConfigFile>,
    config_d2: Option<KEAD2ConfigFile>,
    config_ctrl_agent: Option<KEACtrlAgentConfigFile>,
) -> Vec<Problem> {
    let results: Arc<Mutex<Vec<Problem>>> = Arc::new(Mutex::new(Vec::new()));

    let mut join_handlers: Vec<JoinHandle<()>> = Vec::new();

    if let Some(config) = config_v4 {
        let cloned_results = results.clone();

        let handle = thread::spawn(move || {
            let checker: RulesV4 = RulesV4::default();
            let check_results = checker.run(&config.dhcp4);

            let mut res = cloned_results.lock().expect("It was not possible to block the stream for recording the results of the configuration check.");
            res.extend(check_results);
        });

        join_handlers.push(handle);
    }

    if let Some(config) = config_d2 {
        let cloned_results = results.clone();

        let handle = thread::spawn(move || {
            let checker: RulesD2 = RulesD2::default();
            let check_results = checker.run(&config.dhcp_ddns);

            let mut res = cloned_results.lock().expect("It was not possible to block the stream for recording the results of the configuration check.");
            res.extend(check_results);
        });

        join_handlers.push(handle);
    }

    if let Some(config) = config_ctrl_agent {
        let cloned_results = results.clone();

        let handle = thread::spawn(move || {
            let checker: RulesCtrlAgent = RulesCtrlAgent::default();
            let check_results = checker.run(&config.ctrl_agent);

            let mut res = cloned_results.lock().expect("It was not possible to block the stream for recording the results of the configuration check.");
            res.extend(check_results);
        });

        join_handlers.push(handle);
    }

    for handle in join_handlers {
        handle.join().unwrap();
    }

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

mod _tests;

#[cfg(test)]
mod test {
    use std::io::Write;

    use serde_json::Value;
    use tempfile::NamedTempFile;

    use crate::configs::{KEACtrlAgentConfigFile, KEAD2ConfigFile, KEAv4ConfigFile};
    use crate::modes::CLIArgs;

    use super::_tests::{
        GET_FILE_ARGS_TEMPLATE, RUN_CHECKS_CA_TEMPLATE, RUN_CHECKS_D2_TEMPLATE,
        RUN_CHECKS_V4_TEMPLATE,
    };
    use super::{get_args, run_checks, run_checks_parallel};

    fn prepare_configs() -> (KEAv4ConfigFile, KEAD2ConfigFile, KEACtrlAgentConfigFile) {
        let v4: KEAv4ConfigFile = serde_json::from_str(RUN_CHECKS_V4_TEMPLATE).unwrap();
        let d2: KEAD2ConfigFile = serde_json::from_str(RUN_CHECKS_D2_TEMPLATE).unwrap();
        let ca: KEACtrlAgentConfigFile = serde_json::from_str(RUN_CHECKS_CA_TEMPLATE).unwrap();

        (v4, d2, ca)
    }

    fn prepare_configs_with_problems() -> (KEAv4ConfigFile, KEAD2ConfigFile, KEACtrlAgentConfigFile)
    {
        let mut v4_value: Value = serde_json::from_str(RUN_CHECKS_V4_TEMPLATE).unwrap();
        v4_value["Dhcp4"]["loggers"][0]["severity"] = Value::from("DEBUG");
        let v4: KEAv4ConfigFile = serde_json::from_value(v4_value).unwrap();

        let mut d2_value: Value = serde_json::from_str(RUN_CHECKS_D2_TEMPLATE).unwrap();
        d2_value["DhcpDdns"]["loggers"][0]["severity"] = Value::from("DEBUG");
        let d2: KEAD2ConfigFile = serde_json::from_value(d2_value).unwrap();

        let mut ca_value: Value = serde_json::from_str(RUN_CHECKS_CA_TEMPLATE).unwrap();
        ca_value["Control-agent"]["loggers"][0]["severity"] = Value::from("DEBUG");
        let ca: KEACtrlAgentConfigFile = serde_json::from_value(ca_value).unwrap();

        (v4, d2, ca)
    }

    #[test]
    fn test_checks() {
        // No problems
        let mut mock_configs = prepare_configs();
        assert_eq!(
            run_checks(
                Some(mock_configs.0),
                Some(mock_configs.1),
                Some(mock_configs.2)
            )
            .len(),
            0
        );

        mock_configs = prepare_configs();
        assert_eq!(
            run_checks(
                Some(mock_configs.0),
                Some(mock_configs.1),
                Some(mock_configs.2)
            )
            .len(),
            0
        );

        let mut mock_broken_configs = prepare_configs_with_problems();
        assert_eq!(
            run_checks(
                Some(mock_broken_configs.0),
                Some(mock_broken_configs.1),
                Some(mock_broken_configs.2)
            )
            .len(),
            3
        );

        mock_broken_configs = prepare_configs_with_problems();
        assert_eq!(
            run_checks_parallel(
                Some(mock_broken_configs.0),
                Some(mock_broken_configs.1),
                Some(mock_broken_configs.2)
            )
            .len(),
            3
        );
    }

    #[test]
    fn get_args_from_cli_test() {
        let mock_args = CLIArgs {
            config_filepath: None,
            skip_not_exists: true,
            use_threads: false,
            with_summary: false,
            ctrl_agent_filepath: None,
            d2_filepath: None,
            database_filepath: None,
            database_type: super::KEALintDatabaseTypes::sqlite,
            dir_path: Some("./qqq-path".to_string()),
            format: super::KEALintOutputFormatTypes::json,
            mode: super::KEALintModeTypes::cli,
            output_filepath: None,
            unix_socket_filepath: None,
            v4_filepath: Some("./test-path".to_string()),
        };

        let args = get_args(mock_args);

        assert_eq!(args.with_summary, false);
        assert_eq!(args.dir_path, Some("./qqq-path".to_string()));
        assert_eq!(args.use_threads, false);
        assert_eq!(args.v4_filepath, Some("./test-path".to_string()));
        // Not exists params
        assert_eq!(args.ctrl_agent_filepath, None);
        assert_eq!(args.output_filepath, None);
    }

    #[test]
    fn get_args_from_file_test() {
        let mut tempfile = NamedTempFile::new().unwrap();
        tempfile
            .write_all(GET_FILE_ARGS_TEMPLATE.as_bytes())
            .unwrap();

        let mock_args = CLIArgs {
            config_filepath: Some(String::from(tempfile.path().to_str().unwrap_or_default())),
            skip_not_exists: true,
            use_threads: false,
            with_summary: false,
            ctrl_agent_filepath: None,
            d2_filepath: None,
            database_filepath: None,
            database_type: super::KEALintDatabaseTypes::sqlite,
            dir_path: None,
            format: super::KEALintOutputFormatTypes::json,
            mode: super::KEALintModeTypes::cli,
            output_filepath: None,
            unix_socket_filepath: None,
            v4_filepath: Some("BAD PATH".to_string()),
        };

        let args = get_args(mock_args);

        assert_eq!(args.with_summary, true);
        assert_eq!(args.dir_path, Some("./".to_string()));
        assert_eq!(args.use_threads, true);
        // Not exists params
        assert_eq!(args.v4_filepath, None);
        assert_eq!(args.skip_not_exists, false);
    }
}
