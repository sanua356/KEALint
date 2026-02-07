use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_multithread_modes_not_equal::get_multithread_modes_not_equal_rule;

pub struct MultithreadingModesNotEqualInConfigAndHAV6Rule;

impl Rule<KEAv6Config> for MultithreadingModesNotEqualInConfigAndHAV6Rule {
    fn get_name(&self) -> &'static str {
        "HOOKS::MultithreadingModesNotEqualInConfigAndHARule"
    }

    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }

    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }

    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_multithread_modes_not_equal_rule(&config.multi_threading, &config.hooks_libraries)
    }
}

// The tests are written in a shared directory
