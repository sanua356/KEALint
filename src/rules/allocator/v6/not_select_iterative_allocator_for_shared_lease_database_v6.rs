use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_not_select_iterative_allocator_for_shared_lease_database;

pub struct NotSelectIterativeAllocatorForSharedLeaseDatabaseV6Rule;

impl Rule<KEAv6Config> for NotSelectIterativeAllocatorForSharedLeaseDatabaseV6Rule {
    fn get_name(&self) -> &'static str {
        "ALLOCATOR::NotSelectIterativeAllocatorForSharedLeaseDatabaseRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_not_select_iterative_allocator_for_shared_lease_database(
            &config.allocator,
            &config.lease_database,
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v6::KEAv6Config};

    use super::{
        super::_tests::NOT_SELECT_ITERATIVE_ALLOCATOR_FOR_SHARED_LEASE_DATABASE_RULE_TEST_TEMPLATE,
        NotSelectIterativeAllocatorForSharedLeaseDatabaseV6Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv6Config = serde_json::from_str(
            NOT_SELECT_ITERATIVE_ALLOCATOR_FOR_SHARED_LEASE_DATABASE_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = NotSelectIterativeAllocatorForSharedLeaseDatabaseV6Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(
            NOT_SELECT_ITERATIVE_ALLOCATOR_FOR_SHARED_LEASE_DATABASE_RULE_TEST_TEMPLATE,
        )
        .unwrap();
        json_value["allocator"] = Value::from("random");
        let data: KEAv6Config = serde_json::from_value(json_value).unwrap();

        let rule = NotSelectIterativeAllocatorForSharedLeaseDatabaseV6Rule;
        assert!(rule.check(&data).is_none());
    }
}
