use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAD2Config {
    pub ip_address: Option<IpAddr>,
    port: Option<u32>,
    dns_server_timeout: Option<u32>,
    ncr_protocol: Option<String>,
    ncr_format: Option<String>,
}
