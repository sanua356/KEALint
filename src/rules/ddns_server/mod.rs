pub mod d2;
pub mod v4;

pub use d2::not_local_ip_in_ddns_server::NotLocalIPAddressInD2ServerConfigRule;
pub use v4::not_ddns_qualifying_suffix_with_enabled_ddns_updates::NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule;
