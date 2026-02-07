use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_bad_hooks_order_rule;

pub struct BadHooksOrderV4Rule;

impl Rule<KEAv4Config> for BadHooksOrderV4Rule {
    fn get_name(&self) -> &'static str {
        "HOOKS::BadHooksOrderRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_bad_hooks_order_rule(&config.hooks_libraries)
    }
}

// The tests are written in a shared directory
