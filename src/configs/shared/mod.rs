pub mod allocator;
pub mod client_classes;
pub mod config_control;
pub mod dhcp_ddns;
pub mod dhcp_queue_control;
pub mod hooks;
pub mod hosts_database;
pub mod interfaces;
pub mod lease_database;
pub mod loggers;
pub mod multithreading;
pub mod option_data;
pub mod option_def;
pub mod reservations;
pub mod sanity_checks;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum KEAHostsDatabasesFailStrategers {
    #[serde(rename = "stop-retry-exit")]
    StopRetryExit,
    #[serde(rename = "serve-retry-exit")]
    ServeRetryExit,
    #[serde(rename = "serve-retry-continue")]
    ServeRetryContinue,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KEALeaseDatabaseTypes {
    Memfile,
    MySQL,
    PostgreSQL,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KEAHostsDatabasesTypes {
    MySQL,
    PostgreSQL,
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
