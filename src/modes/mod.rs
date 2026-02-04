use clap::{Parser, ValueEnum};
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::{
    checkers::{Problem, RulesCtrlAgent, RulesD2, RulesV4},
    common::RuleChecker,
    configs::{KEACtrlAgentConfigFile, KEAD2ConfigFile, KEAv4ConfigFile},
};

pub mod cli;
pub mod standalone;
pub use {cli::run_cli, standalone::run_standalone};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum KEALintModeTypes {
    #[allow(non_camel_case_types)]
    cli,
    #[allow(non_camel_case_types)]
    standalone,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum KEALintDatabaseTypes {
    #[allow(non_camel_case_types)]
    sqlite,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum KEALintOutputFormatTypes {
    #[allow(non_camel_case_types)]
    json,
    #[allow(non_camel_case_types)]
    table,
}

#[derive(Debug, Parser)]
#[command(
    version,
    about = "A command-line utility for linting configuration of ISC KEA DHCP 3.0.0 or higher."
)]
pub struct CLIArgs {
    #[arg(
        long,
        help = "Optional. Defines the mode of operation of the utility. If not specified, the default value is 'cli'. If 'standalone' is specified, it instructs the server to operate in UNIX socket listener mode and write checks to the database.",
        value_enum,
        default_value_t = KEALintModeTypes::cli
    )]
    pub mode: KEALintModeTypes,

    #[arg(
        long,
        help = "Optional. Defines the format for the output of the verification result. You can specify the value 'table' or 'json'.",
        value_enum,
        default_value_t = KEALintOutputFormatTypes::table
    )]
    format: KEALintOutputFormatTypes,

    #[arg(
        long,
        help = "Optional. Specifies the path to the directory where the KEA configuration files are stored."
    )]
    dir_path: Option<String>,

    #[arg(
        long,
        help = "Optional. Specifies the path to the KEA DHCPv4 configuration file."
    )]
    v4_filepath: Option<String>,

    #[arg(
        long,
        help = "Optional. Specifies the path to the KEA DHCP DDNS configuration file."
    )]
    d2_filepath: Option<String>,

    #[arg(
        long,
        help = "Optional. Specifies the path to the KEA Control Agent configuration file."
    )]
    ctrl_agent_filepath: Option<String>,

    #[arg(
        long,
        help = "Optional. Specifies the path to the file to which the verification result will be uploaded. If the file does not exist, it will be created."
    )]
    output_filepath: Option<String>,

    #[arg(
        long,
        help = "Optional. (Only in the 'standalone' mode). Defines the path to the UNIX socket that needs to be listened to in order to receive configurations."
    )]
    unix_socket_path: Option<String>,

    #[arg(
        long,
        help = "Optional. (Only in the 'standalone' mode). Defines the type of database to connect to. So far, only one value of 'sqlite' is supported.",
        value_enum,
        default_value_t = KEALintDatabaseTypes::sqlite
    )]
    database_type: KEALintDatabaseTypes,

    #[arg(
        long,
        help = "Optional. (Only in the 'standalone' mode). Defines the path to the database to which the results of the checks will need to be recorded."
    )]
    database_path: Option<String>,

    #[arg(
        long,
        help = "Optional. If specified, the check will run even if not all configuration files exist."
    )]
    skip_not_exists: bool,

    #[arg(
        long,
        help = "Optional. If enabled, processing is performed in multithreaded mode."
    )]
    use_threads: bool,

    #[arg(
        long,
        help = "Optional. Adds additional information when displaying the result as a table."
    )]
    with_summary: bool,
}

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
