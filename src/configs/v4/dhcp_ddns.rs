use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAv4DHCPDDNS {
    pub enable_updates: Option<bool>,
    pub server_ip: Option<String>,
    pub server_port: Option<u32>,
    pub sender_ip: Option<String>,
    pub sender_port: Option<u32>,
    pub max_queue_size: Option<u32>,
    pub ncr_protocol: Option<String>,
    pub ncr_format: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(clippy::enum_variant_names)]
pub enum DDNSConflictResolutionModeTypes {
    CheckWithDhcid,
    NoCheckWithDhcid,
    CheckExistsWithDhcid,
    NoCheckWithoutDhcid,
}
