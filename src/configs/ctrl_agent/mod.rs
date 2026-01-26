use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use super::shared::hooks::KEAHookLibrary;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEACtrlAgentConfig {
    pub http_host: Option<IpAddr>,
    pub http_port: Option<u32>,
    pub trust_anchor: Option<String>,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
    pub cert_required: Option<bool>,

    pub hooks_libraries: Option<Vec<KEAHookLibrary>>,
}
