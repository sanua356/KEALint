use serde::{Deserialize, Serialize};
use std::net::IpAddr;

use super::shared::{hooks, loggers};

mod sockets;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEACtrlAgentConfig {
    pub http_host: Option<IpAddr>,
    pub http_port: Option<u32>,
    pub trust_anchor: Option<String>,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
    pub cert_required: Option<bool>,

    pub control_sockets: Option<sockets::KEACtrlAgentControlSockets>,

    pub hooks_libraries: Option<Vec<hooks::KEAHookLibrary>>,

    pub loggers: Option<Vec<loggers::KEALogger>>,
}

#[derive(Debug, Deserialize)]
pub struct KEACtrlAgentConfigFile {
    #[serde(rename = "Control-agent")]
    pub ctrl_agent: KEACtrlAgentConfig,
}
