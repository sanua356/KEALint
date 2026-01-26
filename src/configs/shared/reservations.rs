#![allow(dead_code)]

use serde::Deserialize;
use std::net::IpAddr;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAReservation {
    pub circuit_id: Option<String>,
    pub duid: Option<String>,
    pub client_id: Option<String>,
    pub hostname: Option<String>,
    pub hw_address: Option<String>,
    pub ip_address: Option<IpAddr>,
    pub flex_id: Option<String>,
}
