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

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v6::KEAv6Config};

    use super::{
        super::_tests::NOT_SELECT_FLQ_ALLOCATOR_IN_GLOBAL_LEVEL_CONFIG_RULE_TEST_TEMPLATE,
        NotSelectFLQAllocatorInGlobalLevelConfigV6Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv6Config = serde_json::from_str(
            NOT_SELECT_FLQ_ALLOCATOR_IN_GLOBAL_LEVEL_CONFIG_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = NotSelectFLQAllocatorInGlobalLevelConfigV6Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(
            NOT_SELECT_FLQ_ALLOCATOR_IN_GLOBAL_LEVEL_CONFIG_RULE_TEST_TEMPLATE,
        )
        .unwrap();
        json_value["allocator"] = Value::from("random");
        let data: KEAv6Config = serde_json::from_value(json_value).unwrap();

        let rule = NotSelectFLQAllocatorInGlobalLevelConfigV6Rule;
        assert!(rule.check(&data).is_none());
    }
}
