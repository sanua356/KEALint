use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_bad_hooks_order::get_bad_hooks_order_rule;

pub struct BadHooksOrderV6Rule;

impl Rule<KEAv6Config> for BadHooksOrderV6Rule {
    fn get_name(&self) -> &'static str {
        "HOOKS::BadHooksOrderRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_bad_hooks_order_rule(&config.hooks_libraries)
    }
}

// The tests are written in a shared directory
