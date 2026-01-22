use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAv4Config {
    pub valid_lifetime: u32,
    pub renew_timer: u32,
    pub rebind_timer: u32,

    pub interfaces_config: KEAv4InterfacesConfig,
    pub lease_database: KEAv4LeaseDatabase,
    pub multi_threading: Option<KEAv4Multithreading>,

    pub hooks_libraries: Option<Vec<KEAv4HookLibrary>>,
    pub client_classes: Option<Vec<KEAv4ClientClass>>,
    pub option_def: Option<Vec<KEAv4OptionDefinition>>,
    pub option_data: Option<Vec<KEAv4OptionData>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KEAv4InterfacesConfig {
    pub interfaces: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KEALeaseDatabaseTypes {
    Memfile,
    MySQL,
    PostgreSQL,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KEAv4LeaseDatabase {
    pub r#type: KEALeaseDatabaseTypes,
    pub persist: Option<bool>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KEAv4HookLibrary {
    pub library: String,
    pub parameters: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAv4Multithreading {
    pub enable_multi_threading: Option<bool>,
    pub thread_pool_size: Option<u32>,
    pub packet_queue_size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAv4ClientClass {
    pub name: String,
    pub test: Option<String>,
    pub template_test: Option<String>,
    pub only_if_required: Option<bool>,
    pub user_context: Value,
    pub next_server: Option<String>,
    pub server_hostname: Option<String>,
    pub boot_file_name: Option<String>,
    pub valid_lifetime: Option<u32>,
    pub renew_timer: Option<u32>,
    pub rebind_timer: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KEAv4OptionData {
    pub code: Option<u8>,
    pub space: Option<String>,
    #[serde(rename = "csv-format")]
    pub csv_format: Option<bool>,
    pub data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum KEAOptionTypes {
    Binary,
    Boolean,
    Empty,
    #[allow(clippy::upper_case_acronyms)]
    FQDN,
    #[serde(rename = "ipv4-address")]
    IPV4Address,
    #[serde(rename = "ipv6-address")]
    IPV6Address,
    #[serde(rename = "ipv6-prefix")]
    IPV6Prefix,
    #[allow(clippy::upper_case_acronyms)]
    PSID,
    Record,
    String,
    Tuple,
    UInt8,
    UInt16,
    UInt32,
    Int8,
    Int16,
    Int32,
    Internal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KEAv4OptionDefinition {
    pub name: Option<String>,
    pub code: Option<u8>,
    pub r#type: KEAOptionTypes,
    pub array: Option<bool>,
    #[serde(rename = "record-types")]
    pub record_types: Option<String>,
    pub space: Option<String>,
    pub encapsulate: Option<String>,
}
