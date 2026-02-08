use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_unnecessary_activated_database_hooks_rule;

pub struct UnnecessaryActivatedDatabaseHooksV4Rule;

impl Rule<KEAv4Config> for UnnecessaryActivatedDatabaseHooksV4Rule {
    fn get_name(&self) -> &'static str {
        "HOOKS::UnnecessaryActivatedDatabaseHooksRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_unnecessary_activated_database_hooks_rule(
            &config.hooks_libraries,
            &config.lease_database,
            &config.hosts_database,
            &config.hosts_databases,
            &config.config_control,
        )
    }
}

// The tests are written in a shared directory
