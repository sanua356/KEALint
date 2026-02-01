use clap::{Parser, ValueEnum};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::{
    checkers::{
        Problem, RulesCtrlAgent, RulesD2, RulesV4, get_summary_problems, tabled_print_problems,
    },
    common::RuleChecker,
    configs::{KEACtrlAgentConfigFile, KEAD2ConfigFile, KEAv4ConfigFile},
};

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

pub fn run_cli(args: CLIArgs) {
    let mut v4_filepath: PathBuf = PathBuf::new();
    let mut d2_filepath: PathBuf = PathBuf::new();
    let mut ctrl_agent_filepath: PathBuf = PathBuf::new();

    if let Some(dir_path) = args.dir_path {
        let dir = Path::new(&dir_path);
        v4_filepath = dir.join("kea-dhcp4.conf");
        d2_filepath = dir.join("kea-dhcp-ddns.conf");
        ctrl_agent_filepath = dir.join("kea-ctrl-agent.conf");
    }

    if let Some(v4_path_custom) = args.v4_filepath {
        v4_filepath = Path::new(&v4_path_custom).to_path_buf();
    }

    if let Some(d2_path_custom) = args.d2_filepath {
        v4_filepath = Path::new(&d2_path_custom).to_path_buf();
    }

    if let Some(ctrl_agent_path_custom) = args.ctrl_agent_filepath {
        v4_filepath = Path::new(&ctrl_agent_path_custom).to_path_buf();
    }

    let skip_not_exists = args.skip_not_exists;

    let content_v4: Option<KEAv4ConfigFile> = match fs::read_to_string(&v4_filepath) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(config) => Some(config),
            Err(err) => panic!(
                "An error occurred while parsing the DHCPv4 configuration: {}",
                err
            ),
        },
        Err(err) if skip_not_exists => None,
        Err(err) => panic!(
            "An error occurred while reading the DHCPv4 configuration: {}",
            err
        ),
    };

    let content_d2: Option<KEAD2ConfigFile> = match fs::read_to_string(&d2_filepath) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(config) => Some(config),
            Err(err) => panic!(
                "An error occurred while parsing the DHCP DDNS configuration: {}",
                err
            ),
        },
        Err(err) if skip_not_exists => None,
        Err(err) => panic!(
            "An error occurred while reading the DHCP DDNS configuration: {}",
            err
        ),
    };

    let content_ctrl_agent: Option<KEACtrlAgentConfigFile> =
        match fs::read_to_string(&ctrl_agent_filepath) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(config) => Some(config),
                Err(err) => panic!(
                    "An error occurred while parsing the Control Agent configuration: {}",
                    err
                ),
            },
            Err(err) if skip_not_exists => None,
            Err(err) => panic!(
                "An error occurred while reading the Control Agent configuration: {}",
                err
            ),
        };

    let problems: Vec<Problem> = if args.use_threads {
        run_checks_parallel(content_v4, content_d2, content_ctrl_agent)
    } else {
        run_checks(content_v4, content_d2, content_ctrl_agent)
    };

    let summary = if args.with_summary {
        get_summary_problems(&problems)
    } else {
        String::new()
    };

    let printed = match args.format {
        KEALintOutputFormatTypes::table => tabled_print_problems(problems),
        KEALintOutputFormatTypes::json => serde_json::to_string_pretty(&problems).unwrap(),
    };

    if let Some(out_filepath) = &args.output_filepath {
        let path = Path::new(out_filepath).to_path_buf();

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path);

        match file {
            Ok(mut f) => {
                f.write_all(printed.as_bytes())
                    .expect("An error occurred when writing the test results to a file.");
                println!("Check results successfully written in file!");
            }
            Err(err) => panic!(
                "An error occurred while verifying the access rights of the output file: {}",
                err
            ),
        }
    } else {
        println!("{}", printed);

        if !summary.is_empty() {
            println!("{}", summary);
        }
    }
}
fn run_checks(
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

fn run_checks_parallel(
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
