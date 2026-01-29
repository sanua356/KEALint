use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAv4Relay {
    pub ip_addresses: Option<Vec<Ipv4Addr>>,
}
