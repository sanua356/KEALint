use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::configs::hooks::KEAHookLibrary;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAD2Config {
    pub ip_address: Option<IpAddr>,
    pub port: Option<u32>,
    pub dns_server_timeout: Option<u32>,
    pub ncr_protocol: Option<String>,
    pub ncr_format: Option<String>,

    pub hooks_libraries: Option<Vec<KEAHookLibrary>>,
}
