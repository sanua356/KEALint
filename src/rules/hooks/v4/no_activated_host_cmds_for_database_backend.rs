use crate::{
    common::{RuleConfigs, RuleLevels, RuleResult, RuleV4},
    configs::KEAv4Config,
    constants::HOST_CMDS_HOOK_LIBRARY,
};

pub struct NoActivatedHostCMDsHookForDatabaseBackendRule;

impl RuleV4 for NoActivatedHostCMDsHookForDatabaseBackendRule {
    fn get_name(&self) -> &'static str {
        "HOOKS::NoActivatedHostCMDsHookForDatabaseBackendRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let is_found_host_cmds_hook = config
            .hooks_libraries
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .any(|hook| hook.library.contains(HOST_CMDS_HOOK_LIBRARY));

        let is_found_hosts_database = config.hosts_database.is_some();
        let is_found_hosts_databases = match &config.hosts_databases {
            Some(databases) => !databases.is_empty(),
            None => false,
        };

        if !is_found_host_cmds_hook && (is_found_hosts_database || is_found_hosts_databases) {
            return Some(vec![RuleResult {
                description: format!(
                    "When using databases to store host reservations using the 'hosts-database' or 'hosts-databases' key, it is recommended to use the '{}' hook to interact with them in a predictable way.",
                    HOST_CMDS_HOOK_LIBRARY
                ),
                snapshot: None,
                links: Some(vec![
                    "https://kea.readthedocs.io/en/latest/arm/hooks.html#libdhcp-host-cmds-so-host-commands",
                ]),
            }]);
        }

        None
    }
}
