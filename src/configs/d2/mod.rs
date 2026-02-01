use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use super::shared::{hooks, loggers};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAD2Config {
    pub ip_address: Option<IpAddr>,
    pub port: Option<u32>,
    pub dns_server_timeout: Option<u32>,
    pub ncr_protocol: Option<String>,
    pub ncr_format: Option<String>,

    pub hooks_libraries: Option<Vec<hooks::KEAHookLibrary>>,

    pub loggers: Option<Vec<loggers::KEALogger>>,
}

#[derive(Debug, Deserialize)]
pub struct KEAD2ConfigFile {
    #[serde(rename = "DhcpDdns")]
    pub dhcp_ddns: KEAD2Config,
}
