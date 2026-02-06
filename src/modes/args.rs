use clap::{Parser, ValueEnum};
use serde::Deserialize;

#[derive(Debug, Default, Clone, Copy, ValueEnum, Deserialize)]
pub enum KEALintModeTypes {
    #[allow(non_camel_case_types)]
    #[default]
    cli,
    #[allow(non_camel_case_types)]
    standalone,
}

#[derive(Debug, Default, Clone, Copy, ValueEnum, Deserialize)]
pub enum KEALintDatabaseTypes {
    #[allow(non_camel_case_types)]
    #[default]
    sqlite,
}

#[derive(Debug, Default, Clone, Copy, ValueEnum, Deserialize)]
pub enum KEALintOutputFormatTypes {
    #[allow(non_camel_case_types)]
    json,
    #[allow(non_camel_case_types)]
    #[default]
    table,
}

#[derive(Debug, Parser, Deserialize)]
#[command(
    version,
    about = "A command-line utility for linting configuration of ISC KEA DHCP 3.0.0 or higher."
)]
pub struct CLIArgs {
    #[arg(
        long,
        help = "Optional. Defines the mode of operation of the utility. If 'standalone' is specified, it instructs the server to operate in UNIX socket listener mode and write checks to the database.",
        value_enum,
        default_value_t = KEALintModeTypes::cli
    )]
    #[serde(default)]
    pub mode: KEALintModeTypes,

    #[arg(
        long,
        help = "Optional. Defines the format for the output of the verification result.",
        value_enum,
        default_value_t = KEALintOutputFormatTypes::table
    )]
    #[serde(default)]
    pub format: KEALintOutputFormatTypes,

    #[arg(
        long,
        help = "Optional. Defines the path to the configuration file. If the path is specified, the values will be loaded from the file instead of the flags."
    )]
    #[serde(rename = "config-filepath")]
    pub config_filepath: Option<String>,

    #[arg(
        long,
        help = "Optional. Specifies the path to the directory where the KEA configuration files are stored."
    )]
    #[serde(rename = "dir-path")]
    pub dir_path: Option<String>,

    #[arg(
        long,
        help = "Optional. Specifies the path to the KEA DHCPv4 configuration file."
    )]
    #[serde(rename = "v4-filepath")]
    pub v4_filepath: Option<String>,

    #[arg(
        long,
        help = "Optional. Specifies the path to the KEA DHCP DDNS configuration file."
    )]
    #[serde(rename = "d2-filepath")]
    pub d2_filepath: Option<String>,

    #[arg(
        long,
        help = "Optional. Specifies the path to the KEA Control Agent configuration file."
    )]
    #[serde(rename = "ctrl-agent-filepath")]
    pub ctrl_agent_filepath: Option<String>,

    #[arg(
        long,
        help = "Optional. Specifies the path to the file to which the verification result will be uploaded. If the file does not exist, it will be created."
    )]
    #[serde(rename = "output-filepath")]
    pub output_filepath: Option<String>,

    #[arg(
        long,
        help = "Optional. (Only in the 'standalone' mode). Defines the path to the UNIX socket that needs to be listened to in order to receive configurations."
    )]
    #[serde(rename = "unix-socket-filepath")]
    pub unix_socket_filepath: Option<String>,

    #[arg(
        long,
        help = "Optional. (Only in the 'standalone' mode). Defines the type of database to connect to.",
        value_enum,
        default_value_t = KEALintDatabaseTypes::sqlite
    )]
    #[serde(default)]
    #[serde(rename = "database-type")]
    pub database_type: KEALintDatabaseTypes,

    #[arg(
        long,
        help = "Optional. (Only in the 'standalone' mode). Defines the path to the database to which the results of the checks will need to be recorded."
    )]
    #[serde(rename = "database-filepath")]
    pub database_filepath: Option<String>,

    #[arg(
        long,
        help = "Optional. If specified, the check will run even if not all configuration files exist."
    )]
    #[serde(default)]
    #[serde(rename = "skip-not-exists")]
    pub skip_not_exists: bool,

    #[arg(
        long,
        help = "Optional. If enabled, processing is performed in multithreaded mode."
    )]
    #[serde(default)]
    #[serde(rename = "use-threads")]
    pub use_threads: bool,

    #[arg(
        long,
        help = "Optional. Adds additional information when displaying the result as a table."
    )]
    #[serde(default)]
    #[serde(rename = "with-summary")]
    pub with_summary: bool,
}
