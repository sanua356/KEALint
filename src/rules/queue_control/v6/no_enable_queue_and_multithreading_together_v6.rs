use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_no_enable_queue_and_multithreading_together_rule;

pub struct NoEnableQueueAndMultithreadingTogetherV6Rule;

impl Rule<KEAv6Config> for NoEnableQueueAndMultithreadingTogetherV6Rule {
    fn get_name(&self) -> &'static str {
        "QUEUE_CONTROL::NoEnableQueueAndMultithreadingTogetherRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_no_enable_queue_and_multithreading_together_rule(
            &config.multi_threading,
            &config.dhcp_queue_control,
        )
    }
}

// The tests are written in a shared directory
