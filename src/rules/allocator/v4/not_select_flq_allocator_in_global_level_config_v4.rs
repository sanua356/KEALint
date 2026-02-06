use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_not_select_flq_allocator_in_global_level_config_rule;

pub struct NotSelectFLQAllocatorInGlobalLevelConfigV4Rule;

impl Rule<KEAv4Config> for NotSelectFLQAllocatorInGlobalLevelConfigV4Rule {
    fn get_name(&self) -> &'static str {
        "ALLOCATOR::NotSelectFLQAllocatorInGlobalLevelConfigRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_not_select_flq_allocator_in_global_level_config_rule(&config.allocator)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{
        super::_tests::NOT_SELECT_FLQ_ALLOCATOR_IN_GLOBAL_LEVEL_CONFIG_RULE_TEST_TEMPLATE,
        NotSelectFLQAllocatorInGlobalLevelConfigV4Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            NOT_SELECT_FLQ_ALLOCATOR_IN_GLOBAL_LEVEL_CONFIG_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = NotSelectFLQAllocatorInGlobalLevelConfigV4Rule;
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

        let rule = NotSelectFLQAllocatorInGlobalLevelConfigV4Rule;
        assert!(rule.check(&data).is_none());
    }
}
