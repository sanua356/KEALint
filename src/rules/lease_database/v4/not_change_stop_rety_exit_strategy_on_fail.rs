use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEAHostsDatabasesFailStrategers, KEAv4Config},
};

pub struct NotChangeStopRetryExitStrategyOnFailRule;

impl Rule<KEAv4Config> for NotChangeStopRetryExitStrategyOnFailRule {
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
        let lease_database_strategy = config.lease_database.on_fail.as_ref()?;

        if lease_database_strategy != &KEAHostsDatabasesFailStrategers::StopRetryExit {
            return Some(vec![RuleResult {
                description: "It is recommended to set the 'on-fail' parameter in the 'lease-database' configuration to 'stop-retry-exit' for the correct processing of leases in the production environment.".to_string(),
                links: Some(&["https://kea.readthedocs.io/en/latest/arm/dhcp6-srv.html#lease-database-configuration"]),
                places: Some(vec!["lease-database.on-fail".to_string()]),
            }]);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{
        super::_tests::NOT_CHANGE_STOP_RETRY_EXIT_STRATEGY_ON_FAIL_RULE_TEST_TEMPLATE,
        NotChangeStopRetryExitStrategyOnFailRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NOT_CHANGE_STOP_RETRY_EXIT_STRATEGY_ON_FAIL_RULE_TEST_TEMPLATE)
                .unwrap();

        let rule = NotChangeStopRetryExitStrategyOnFailRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NOT_CHANGE_STOP_RETRY_EXIT_STRATEGY_ON_FAIL_RULE_TEST_TEMPLATE)
                .unwrap();
        json_value["lease-database"]["on-fail"] = Value::from("stop-retry-exit");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NotChangeStopRetryExitStrategyOnFailRule;
        assert!(rule.check(&data).is_none());
    }
}
