use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEAv4Config, allocator::KEAAllocatorTypes},
};

pub struct NotSelectFLQAllocatorInGlobalLevelConfig;

impl Rule<KEAv4Config> for NotSelectFLQAllocatorInGlobalLevelConfig {
    fn get_name(&self) -> &'static str {
        "ALLOCATOR::NotSelectFLQAllocatorInGlobalLevelConfig"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        if let Some(allocator) = &config.allocator
            && allocator == &KEAAllocatorTypes::FLQ
        {
            return Some(vec![RuleResult {
                description: "The 'FLQ' address allocator is not recommended for use at the global configuration level.".to_string(),
                links: Some(&["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#free-lease-queue-allocator"]),
                places: Some(vec!["allocator".to_string()]),
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
        super::_tests::NOT_SELECT_FLQ_ALLOCATOR_IN_GLOBAL_LEVEL_CONFIG_RULE_TEST_TEMPLATE,
        NotSelectFLQAllocatorInGlobalLevelConfig,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            NOT_SELECT_FLQ_ALLOCATOR_IN_GLOBAL_LEVEL_CONFIG_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = NotSelectFLQAllocatorInGlobalLevelConfig;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(
            NOT_SELECT_FLQ_ALLOCATOR_IN_GLOBAL_LEVEL_CONFIG_RULE_TEST_TEMPLATE,
        )
        .unwrap();
        json_value["allocator"] = Value::from("random");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NotSelectFLQAllocatorInGlobalLevelConfig;
        assert!(rule.check(&data).is_none());
    }
}
