use serde::{Deserialize, Serialize};

use super::{KEAv4HostsDatabasesFailStrategers, KEAv4HostsDatabasesTypes};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAv4ConfigControl {
    pub config_databases: Option<Vec<KEAv4ConfigDatabase>>,
    pub config_fetch_wait_time: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAv4ConfigDatabase {
    pub name: String,
    pub host: Option<String>,
    pub password: Option<String>,
    pub port: Option<u32>,
    pub r#type: Option<KEAv4HostsDatabasesTypes>,
    pub user: Option<String>,
    pub readonly: Option<bool>,
    pub trust_anchor: Option<String>,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
    pub cipher_list: Option<String>,
    pub reconnect_wait_time: Option<u32>,
    pub max_reconnect_tries: Option<u32>,
    pub on_fail: Option<KEAv4HostsDatabasesFailStrategers>,
    pub retry_on_startup: Option<bool>,
    pub connect_timeout: Option<u32>,
    pub read_timeout: Option<u32>,
    pub write_timeout: Option<u32>,
}
