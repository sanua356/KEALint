use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEAv4Config, KEAv4HostsDatabasesFailStrategers},
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

        if lease_database_strategy != &KEAv4HostsDatabasesFailStrategers::StopRetryExit {
            return Some(vec![RuleResult {
                description: "It is recommended to set the 'on-fail' parameter in the 'lease-database' configuration to 'stop-retry-exit' for the correct processing of leases in the production environment.".to_string(),
                links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp6-srv.html#lease-database-configuration"]),
                snapshot: Some(serde_json::to_string(&config.lease_database).unwrap()),
            }]);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        common::Rule,
        configs::v4::KEAv4Config,
        rules::lease_database::v4::{
            _tests::NOT_CHANGE_STOP_RETRY_EXIT_STRATEGY_ON_FAIL_RULE_TEST_TEMPLATE,
            not_change_stop_rety_exit_strategy_on_fail::NotChangeStopRetryExitStrategyOnFailRule,
        },
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
