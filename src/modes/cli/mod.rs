use serde::de::DeserializeOwned;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    checkers::{Problem, get_summary_problems, tabled_print_problems},
    configs::{KEACtrlAgentConfigFile, KEAD2ConfigFile, KEAv4ConfigFile, KEAv6ConfigFile},
};

use super::{CLIArgs, KEALintOutputFormatTypes, run_checks, run_checks_parallel};

fn process_config_file<T>(
    path: PathBuf,
    skip_not_exists: bool,
    config_type: &'static str,
) -> Option<T>
where
    T: DeserializeOwned,
{
    match fs::read_to_string(&path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(config) => Some(config),
            Err(err) => panic!(
                "An error occurred while parsing the {} configuration: {}",
                config_type, err
            ),
        },
        Err(err) if skip_not_exists => None,
        Err(err) => panic!(
            "An error occurred while reading the {} configuration: {}",
            config_type, err
        ),
    }
}

pub fn run_cli(args: CLIArgs) {
    let mut v4_filepath: PathBuf = PathBuf::new();
    let mut v6_filepath: PathBuf = PathBuf::new();
    let mut d2_filepath: PathBuf = PathBuf::new();
    let mut ctrl_agent_filepath: PathBuf = PathBuf::new();

    if let Some(dir_path) = args.dir_path {
        let dir = Path::new(&dir_path);
        v4_filepath = dir.join("kea-dhcp4.conf");
        v6_filepath = dir.join("kea-dhcp6.conf");
        d2_filepath = dir.join("kea-dhcp-ddns.conf");
        ctrl_agent_filepath = dir.join("kea-ctrl-agent.conf");
    }

    if let Some(v4_path_custom) = args.v4_filepath {
        v4_filepath = Path::new(&v4_path_custom).to_path_buf();
    }

    if let Some(v6_path_custom) = args.v6_filepath {
        v6_filepath = Path::new(&v6_path_custom).to_path_buf();
    }

    if let Some(d2_path_custom) = args.d2_filepath {
        d2_filepath = Path::new(&d2_path_custom).to_path_buf();
    }

    if let Some(ctrl_agent_path_custom) = args.ctrl_agent_filepath {
        ctrl_agent_filepath = Path::new(&ctrl_agent_path_custom).to_path_buf();
    }

    let skip_not_exists = args.skip_not_exists;

    let content_v4: Option<KEAv4ConfigFile> =
        process_config_file(v4_filepath, skip_not_exists, "DHCPv4");

    let content_v6: Option<KEAv6ConfigFile> =
        process_config_file(v6_filepath, skip_not_exists, "DHCPv6");

    let content_d2: Option<KEAD2ConfigFile> =
        process_config_file(d2_filepath, skip_not_exists, "DHCP DDNS");

    let content_ctrl_agent: Option<KEACtrlAgentConfigFile> =
        process_config_file(ctrl_agent_filepath, skip_not_exists, "Control Agent");

    let problems: Vec<Problem> = if args.use_threads {
        run_checks_parallel(content_v4, content_v6, content_d2, content_ctrl_agent)
    } else {
        run_checks(content_v4, content_v6, content_d2, content_ctrl_agent)
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

mod _tests;

#[cfg(test)]
mod test {
    use std::io::Write;
    use std::path::Path;
    use tempfile::NamedTempFile;

    use crate::configs::{KEACtrlAgentConfigFile, KEAD2ConfigFile, KEAv4ConfigFile};

    use super::_tests::{
        PROCESS_CONFIG_FILE_CA_TEMPLATE, PROCESS_CONFIG_FILE_D2_TEMPLATE,
        PROCESS_CONFIG_FILE_V4_TEMPLATE,
    };
    use super::process_config_file;

    #[test]
    #[should_panic]
    fn process_config_test() {
        let mut pseudofile = NamedTempFile::new().unwrap();

        pseudofile
            .write_all(PROCESS_CONFIG_FILE_V4_TEMPLATE.as_bytes())
            .unwrap();
        let v4: Option<KEAv4ConfigFile> =
            process_config_file(pseudofile.path().to_path_buf(), false, "DHCPv4");
        assert!(v4.is_some());

        pseudofile = NamedTempFile::new().unwrap();
        pseudofile
            .write_all(PROCESS_CONFIG_FILE_D2_TEMPLATE.as_bytes())
            .unwrap();
        let d2: Option<KEAD2ConfigFile> =
            process_config_file(pseudofile.path().to_path_buf(), false, "DHCP DDNS");
        assert!(d2.is_some());

        pseudofile = NamedTempFile::new().unwrap();
        pseudofile
            .write_all(PROCESS_CONFIG_FILE_CA_TEMPLATE.as_bytes())
            .unwrap();
        let ca: Option<KEACtrlAgentConfigFile> =
            process_config_file(pseudofile.path().to_path_buf(), false, "Control Agent");
        assert!(ca.is_some());

        pseudofile = NamedTempFile::new().unwrap();
        pseudofile.write_all("BAD DATA".as_bytes()).unwrap();

        // none
        assert!(
            process_config_file::<KEAv4ConfigFile>(
                Path::new("BAD PATH").to_path_buf(),
                true,
                "DHCPv4",
            )
            .is_none()
        );

        // panic
        process_config_file::<KEAv4ConfigFile>(
            pseudofile.path().to_path_buf(),
            false,
            "Control Agent",
        );
    }
}
