use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_multithread_modes_not_equal::get_multithread_modes_not_equal_rule;

pub struct MultithreadingModesNotEqualInConfigAndHAV4Rule;

impl Rule<KEAv4Config> for MultithreadingModesNotEqualInConfigAndHAV4Rule {
    fn get_name(&self) -> &'static str {
        "HOOKS::MultithreadingModesNotEqualInConfigAndHARule"
    }

    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }

    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }

    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_multithread_modes_not_equal_rule(&config.multi_threading, &config.hooks_libraries)
    }
}

// The tests are written in a shared directory
