use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_not_select_flq_allocator_in_global_level_config_rule;

pub struct NotSelectFLQAllocatorInGlobalLevelConfigV6Rule;

impl Rule<KEAv6Config> for NotSelectFLQAllocatorInGlobalLevelConfigV6Rule {
    fn get_name(&self) -> &'static str {
        "ALLOCATOR::NotSelectFLQAllocatorInGlobalLevelConfigRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_not_select_flq_allocator_in_global_level_config_rule(&config.allocator)
    }
}

// The tests are written in a shared directory
