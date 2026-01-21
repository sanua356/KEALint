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
