#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;

pub static HIGH_AVAILABILITY_HOOK_LIBRARY: &str = "libdhcp_ha.so";
pub static MYSQL_HOOK_LIBRARY: &str = "libdhcp_mysql.so";
pub static PGSQL_HOOK_LIBRARY: &str = "libdhcp_pgsql.so";
pub static HOST_CMDS_HOOK_LIBRARY: &str = "libdhcp_host_cmds.so";
pub static FLEX_ID_HOOK_LIBRARY: &str = "libdhcp_flex_id.so";
pub static FORENSIC_LOGGING_HOOK_LIBRARY: &str = "libdhcp_legal_log.so";
pub static LEASE_COMMANDS_HOOK_LIBRARY: &str = "libdhcp_lease_cmds.so";
pub static PING_CHECK_HOOK_LIBRARY: &str = "libdhcp_ping_check.so";
pub static HOST_CACHE_HOOK_LIBRARY: &str = "libdhcp_host_cache.so";
pub static USER_CHK_HOOK_LIBRARY: &str = "libdhcp_user_chk.so";

pub static GSS_TSIG_HOOK_LIBRARY: &str = "libddns_gss_tsig.so";
pub static RADIUS_HOOK_LIBRARY: &str = "libdhcp_radius.so";

lazy_static! {
    // Validate ip range in format: IPV4-IPV4
    pub static ref IPV4_RANGE_REGEXP: Regex = Regex::new(r"^(25[0-5]|2[0-4]\d|1?\d?\d)(?:\.(25[0-5]|2[0-4]\d|1?\d?\d)){3}\s*-\s*(25[0-5]|2[0-4]\d|1?\d?\d)(?:\.(25[0-5]|2[0-4]\d|1?\d?\d)){3}$").unwrap();
    // Validate ip range in format: IPV4-IPV4
    pub static ref CIDR_V4_REGEXP: Regex = Regex::new(r"^(25[0-5]|2[0-4]\d|1?\d?\d)(?:\.(25[0-5]|2[0-4]\d|1?\d?\d)){3}/(3[0-2]|[12]?\d)$").unwrap();
    // Validate bytes string in option-data value
    pub static ref OPTION_DATA_BYTES_REGEXP: Regex = Regex::new(r"^((0x)?([0-9A-Fa-f]{2})+|([0-9A-Fa-f]{2})([ :][0-9A-Fa-f]{2})*)$").unwrap();
}

pub struct KEANoConfigurableOption {
    pub code: u8,
    pub name: &'static str,
}

#[allow(dead_code)]
pub static KEA_NO_CONFIGURABLE_OPTIONS: [KEANoConfigurableOption; 22] = [
    KEANoConfigurableOption {
        code: 1,
        name: "subnet-mask",
    },
    KEANoConfigurableOption {
        code: 12,
        name: "host-name",
    },
    KEANoConfigurableOption {
        code: 50,
        name: "dhcp-requested-address",
    },
    KEANoConfigurableOption {
        code: 51,
        name: "dhcp-lease-time",
    },
    KEANoConfigurableOption {
        code: 53,
        name: "dhcp-message-type",
    },
    KEANoConfigurableOption {
        code: 55,
        name: "dhcp-parameter-required-list",
    },
    KEANoConfigurableOption {
        code: 58,
        name: "dhcp-renewal-time",
    },
    KEANoConfigurableOption {
        code: 59,
        name: "dhcp-rebinding-time",
    },
    KEANoConfigurableOption {
        code: 61,
        name: "dhcp-client-identifier",
    },
    KEANoConfigurableOption {
        code: 81,
        name: "fqdn",
    },
    KEANoConfigurableOption {
        code: 1,
        name: "dhcp-agent-options",
    },
    KEANoConfigurableOption {
        code: 90,
        name: "authenticate",
    },
    KEANoConfigurableOption {
        code: 91,
        name: "client-last-transaction-time",
    },
    KEANoConfigurableOption {
        code: 92,
        name: "associated-ip",
    },
    KEANoConfigurableOption {
        code: 118,
        name: "subnet-selection",
    },
    KEANoConfigurableOption {
        code: 151,
        name: "status-code",
    },
    KEANoConfigurableOption {
        code: 152,
        name: "base-ttme",
    },
    KEANoConfigurableOption {
        code: 153,
        name: "start-time-of-state",
    },
    KEANoConfigurableOption {
        code: 154,
        name: "query-start-time",
    },
    KEANoConfigurableOption {
        code: 155,
        name: "query-end-time",
    },
    KEANoConfigurableOption {
        code: 156,
        name: "dhcp-state",
    },
    KEANoConfigurableOption {
        code: 157,
        name: "data-source",
    },
];
