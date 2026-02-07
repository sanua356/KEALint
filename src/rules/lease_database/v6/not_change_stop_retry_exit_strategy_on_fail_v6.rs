use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_not_change_stop_retry_exit_strategy_on_fail_rule;

pub struct NotChangeStopRetryExitStrategyOnFailV6Rule;

impl Rule<KEAv6Config> for NotChangeStopRetryExitStrategyOnFailV6Rule {
    fn get_name(&self) -> &'static str {
        "LEASE_DATABASE::NotChangeStopRetryExitStrategyOnFailRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_not_change_stop_retry_exit_strategy_on_fail_rule(&config.lease_database)
    }
}

// The tests are written in a shared directory
