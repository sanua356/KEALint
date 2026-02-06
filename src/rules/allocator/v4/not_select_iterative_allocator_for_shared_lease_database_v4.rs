use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_not_select_iterative_allocator_for_shared_lease_database;

pub struct NotSelectIterativeAllocatorForSharedLeaseDatabaseV4Rule;

impl Rule<KEAv4Config> for NotSelectIterativeAllocatorForSharedLeaseDatabaseV4Rule {
    fn get_name(&self) -> &'static str {
        "ALLOCATOR::NotSelectIterativeAllocatorForSharedLeaseDatabaseRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_not_select_iterative_allocator_for_shared_lease_database(
            &config.allocator,
            &config.lease_database,
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{
        super::_tests::NOT_SELECT_ITERATIVE_ALLOCATOR_FOR_SHARED_LEASE_DATABASE_RULE_TEST_TEMPLATE,
        NotSelectIterativeAllocatorForSharedLeaseDatabaseV4Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            NOT_SELECT_ITERATIVE_ALLOCATOR_FOR_SHARED_LEASE_DATABASE_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = NotSelectIterativeAllocatorForSharedLeaseDatabaseV4Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(
            NOT_SELECT_ITERATIVE_ALLOCATOR_FOR_SHARED_LEASE_DATABASE_RULE_TEST_TEMPLATE,
        )
        .unwrap();
        json_value["allocator"] = Value::from("random");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NotSelectIterativeAllocatorForSharedLeaseDatabaseV4Rule;
        assert!(rule.check(&data).is_none());
    }
}
