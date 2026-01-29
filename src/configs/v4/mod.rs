#![allow(dead_code)]

use serde::Deserialize;

pub use option_data::KEAv4OptionData;
pub use shared::*;
pub use subnets::{KEAv4PoolVariant, KEAv4Subnet};

use super::shared::{dhcp_queue_control, hooks, loggers};

mod client_classes;
mod config_control;
mod hosts_database;
mod interfaces;
mod lease_database;
mod multithreading;
mod option_data;
mod option_def;
mod shared;
mod shared_networks;
mod subnets;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAv4Config {
    pub server_tag: Option<String>,

    pub valid_lifetime: u32,
    pub renew_timer: u32,
    pub rebind_timer: u32,
    pub match_client_id: Option<bool>,

    pub interfaces_config: interfaces::KEAv4InterfacesConfig,
    pub lease_database: lease_database::KEAv4LeaseDatabase,
    pub multi_threading: Option<multithreading::KEAv4Multithreading>,

    pub hooks_libraries: Option<Vec<hooks::KEAHookLibrary>>,
    pub client_classes: Option<Vec<client_classes::KEAv4ClientClass>>,
    pub option_def: Option<Vec<option_def::KEAv4OptionDefinition>>,
    pub option_data: Option<Vec<option_data::KEAv4OptionData>>,

    pub subnet4: Option<Vec<subnets::KEAv4Subnet>>,
    pub shared_networks: Option<Vec<shared_networks::KEAv4SharedNetwork>>,

    pub hosts_databases: Option<Vec<hosts_database::KEAv4HostsDatabase>>,
    pub hosts_database: Option<hosts_database::KEAv4HostsDatabase>,
    pub config_control: Option<config_control::KEAv4ConfigControl>,
    pub dhcp_queue_control: Option<dhcp_queue_control::KEADhcpQueueControl>,

    pub loggers: Option<Vec<loggers::KEALogger>>,
}
