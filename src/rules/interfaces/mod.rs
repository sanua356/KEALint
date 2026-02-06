pub mod shared;
pub mod v4;
pub mod v6;

pub use v4::no_active_interfaces_v4::NoInterfacesInInterfacesConfigV4Rule;

pub use v6::no_active_interfaces_v6::NoInterfacesInInterfacesConfigV6Rule;
