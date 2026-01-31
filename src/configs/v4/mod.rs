#![allow(dead_code)]

use serde::Deserialize;

pub use option_data::KEAv4OptionData;
pub use relay::KEAv4Relay;
pub use shared::*;
pub use subnets::{KEAv4PoolVariant, KEAv4Subnet};

use super::shared::{allocator, dhcp_queue_control, hooks, loggers, reservations, sanity_checks};

mod client_classes;
mod config_control;
mod dhcp_ddns;
mod hosts_database;
mod interfaces;
mod lease_database;
mod multithreading;
mod option_data;
mod option_def;
mod relay;
mod shared;
mod shared_networks;
mod subnets;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAv4Config {
    pub server_tag: Option<String>,

    pub allocator: Option<allocator::KEAAllocatorTypes>,
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

    pub reservations_global: Option<bool>,
    pub reservations_in_subnet: Option<bool>,
    pub reservations_out_of_pool: Option<bool>,
    pub reservations: Option<Vec<reservations::KEAReservation>>,

    pub subnet4: Option<Vec<subnets::KEAv4Subnet>>,
    pub shared_networks: Option<Vec<shared_networks::KEAv4SharedNetwork>>,

    pub hosts_databases: Option<Vec<hosts_database::KEAv4HostsDatabase>>,
    pub hosts_database: Option<hosts_database::KEAv4HostsDatabase>,
    pub config_control: Option<config_control::KEAv4ConfigControl>,
    pub dhcp_queue_control: Option<dhcp_queue_control::KEADhcpQueueControl>,
    pub sanity_checks: Option<sanity_checks::KEASanityChecks>,

    pub dhcp_ddns: Option<dhcp_ddns::KEAv4DHCPDDNS>,
    pub ddns_send_updates: Option<bool>,
    pub ddns_override_no_update: Option<bool>,
    pub ddns_override_client_update: Option<bool>,
    pub ddns_replace_client_name: Option<String>,
    pub ddns_generated_prefix: Option<String>,
    pub ddns_qualifying_suffix: Option<String>,
    pub ddns_update_on_renew: Option<bool>,
    pub ddns_conflict_resolution_mode: Option<dhcp_ddns::DDNSConflictResolutionModeTypes>,
    pub hostname_char_set: Option<String>,
    pub hostname_char_replacement: Option<String>,

    pub loggers: Option<Vec<loggers::KEALogger>>,
}
