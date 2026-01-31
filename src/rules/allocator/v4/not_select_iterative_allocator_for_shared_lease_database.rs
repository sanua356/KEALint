use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEALeaseDatabaseTypes, KEAv4Config, allocator::KEAAllocatorTypes},
};

pub struct NotSelectIterativeAllocatorForSharedLeaseDatabase;

impl Rule<KEAv4Config> for NotSelectIterativeAllocatorForSharedLeaseDatabase {
    fn get_name(&self) -> &'static str {
        "ALLOCATOR::NotSelectIterativeAllocatorForSharedLeaseDatabase"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        if let Some(allocator) = &config.allocator
            && allocator == &KEAAllocatorTypes::Iterative
            && config.lease_database.r#type != KEALeaseDatabaseTypes::Memfile
        {
            return Some(vec![RuleResult {
                description: "The 'iterative' address allocator is not recommended for use with a shared database of rents on several servers.".to_string(),
                links: Some(vec![
                    "https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#iterative-allocator",
                ]),
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
        super::_tests::NOT_SELECT_ITERATIVE_ALLOCATOR_FOR_SHARED_LEASE_DATABASE_RULE_TEST_TEMPLATE,
        NotSelectIterativeAllocatorForSharedLeaseDatabase,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            NOT_SELECT_ITERATIVE_ALLOCATOR_FOR_SHARED_LEASE_DATABASE_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = NotSelectIterativeAllocatorForSharedLeaseDatabase;
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

        let rule = NotSelectIterativeAllocatorForSharedLeaseDatabase;
        assert!(rule.check(&data).is_none());
    }
}
