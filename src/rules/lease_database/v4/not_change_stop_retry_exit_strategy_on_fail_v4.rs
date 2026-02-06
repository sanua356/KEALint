use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_not_change_stop_retry_exit_strategy_on_fail_rule;

pub struct NotChangeStopRetryExitStrategyOnFailV4Rule;

impl Rule<KEAv4Config> for NotChangeStopRetryExitStrategyOnFailV4Rule {
    fn get_name(&self) -> &'static str {
        "LEASE_DATABASE::NotChangeStopRetryExitStrategyOnFailRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_not_change_stop_retry_exit_strategy_on_fail_rule(&config.lease_database)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{
        super::_tests::NOT_CHANGE_STOP_RETRY_EXIT_STRATEGY_ON_FAIL_RULE_TEST_TEMPLATE,
        NotChangeStopRetryExitStrategyOnFailV4Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NOT_CHANGE_STOP_RETRY_EXIT_STRATEGY_ON_FAIL_RULE_TEST_TEMPLATE)
                .unwrap();

        let rule = NotChangeStopRetryExitStrategyOnFailV4Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NOT_CHANGE_STOP_RETRY_EXIT_STRATEGY_ON_FAIL_RULE_TEST_TEMPLATE)
                .unwrap();
        json_value["lease-database"]["on-fail"] = Value::from("stop-retry-exit");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NotChangeStopRetryExitStrategyOnFailV4Rule;
        assert!(rule.check(&data).is_none());
    }
}
