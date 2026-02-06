pub mod d2;
pub mod shared;
pub mod v4;
pub mod v6;

pub use d2::not_local_ip_in_ddns_server::NotLocalIPAddressInD2ServerConfigRule;

pub use v4::not_ddns_qualifying_suffix_with_enabled_ddns_updates_v4::NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesV4Rule;

pub use v6::not_ddns_qualifying_suffix_with_enabled_ddns_updates_v6::NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesV6Rule;
