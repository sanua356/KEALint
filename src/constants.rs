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
}
