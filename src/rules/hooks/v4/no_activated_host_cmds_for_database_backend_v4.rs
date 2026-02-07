use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_no_activated_host_cmds_for_database_backend_rule;

pub struct NoActivatedHostCMDsHookForDatabaseBackendV4Rule;

impl Rule<KEAv4Config> for NoActivatedHostCMDsHookForDatabaseBackendV4Rule {
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
        get_no_activated_host_cmds_for_database_backend_rule(
            &config.hooks_libraries,
            &config.hosts_database,
            &config.hosts_databases,
        )
    }
}

// The tests are written in a shared directory
