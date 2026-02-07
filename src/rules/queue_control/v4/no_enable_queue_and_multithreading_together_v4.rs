use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_no_enable_queue_and_multithreading_together_rule;

pub struct NoEnableQueueAndMultithreadingTogetherV4Rule;

impl Rule<KEAv4Config> for NoEnableQueueAndMultithreadingTogetherV4Rule {
    fn get_name(&self) -> &'static str {
        "QUEUE_CONTROL::NoEnableQueueAndMultithreadingTogetherRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_no_enable_queue_and_multithreading_together_rule(
            &config.multi_threading,
            &config.dhcp_queue_control,
        )
    }
}

// The tests are written in a shared directory
