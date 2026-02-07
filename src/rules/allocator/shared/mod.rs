use crate::{
    common::RuleResult,
    configs::{
        KEALeaseDatabaseTypes, allocator::KEAAllocatorTypes, lease_database::KEALeaseDatabase,
    },
};

pub fn get_not_select_flq_allocator_in_global_level_config_rule(
    allocator: &Option<KEAAllocatorTypes>,
) -> Option<Vec<RuleResult>> {
    if let Some(allocator) = &allocator
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

pub fn get_not_select_iterative_allocator_for_shared_lease_database(
    allocator: &Option<KEAAllocatorTypes>,
    lease_database: &KEALeaseDatabase,
) -> Option<Vec<RuleResult>> {
    if let Some(allocator) = allocator
        && allocator == &KEAAllocatorTypes::Iterative
        && lease_database.r#type != KEALeaseDatabaseTypes::Memfile
    {
        return Some(vec![RuleResult {
            description: "The 'iterative' address allocator is not recommended for use with a shared database of rents on several servers.".to_string(),
            links: Some(&["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#iterative-allocator"]),
            places: Some(vec!["allocator".to_string()]),
        }]);
    }

    None
}

#[cfg(test)]
pub mod _tests;

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::configs::v4::KEAv4Config;

    use super::{
        _tests::{
            NOT_SELECT_FLQ_ALLOCATOR_IN_GLOBAL_LEVEL_CONFIG_RULE_TEST_TEMPLATE,
            NOT_SELECT_ITERATIVE_ALLOCATOR_FOR_SHARED_LEASE_DATABASE_RULE_TEST_TEMPLATE,
        },
        get_not_select_flq_allocator_in_global_level_config_rule,
        get_not_select_iterative_allocator_for_shared_lease_database,
    };

    #[test]
    fn check_expected_not_select_flq_allocator_in_global_level_config_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            NOT_SELECT_FLQ_ALLOCATOR_IN_GLOBAL_LEVEL_CONFIG_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = get_not_select_flq_allocator_in_global_level_config_rule(&data.allocator);
        assert!(rule.is_some());
    }

    #[test]
    fn check_absence_not_select_flq_allocator_in_global_level_config_trigger() {
        let mut json_value: Value = serde_json::from_str(
            NOT_SELECT_FLQ_ALLOCATOR_IN_GLOBAL_LEVEL_CONFIG_RULE_TEST_TEMPLATE,
        )
        .unwrap();
        json_value["allocator"] = Value::from("random");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = get_not_select_flq_allocator_in_global_level_config_rule(&data.allocator);
        assert!(rule.is_none());
    }

    #[test]
    fn check_expected_not_select_iterative_allocator_for_shared_lease_database_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            NOT_SELECT_ITERATIVE_ALLOCATOR_FOR_SHARED_LEASE_DATABASE_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = get_not_select_iterative_allocator_for_shared_lease_database(
            &data.allocator,
            &data.lease_database,
        );
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_not_select_iterative_allocator_for_shared_lease_database_trigger() {
        let mut json_value: Value = serde_json::from_str(
            NOT_SELECT_ITERATIVE_ALLOCATOR_FOR_SHARED_LEASE_DATABASE_RULE_TEST_TEMPLATE,
        )
        .unwrap();
        json_value["allocator"] = Value::from("random");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = get_not_select_iterative_allocator_for_shared_lease_database(
            &data.allocator,
            &data.lease_database,
        );
        assert!(rule.is_none());
    }
}
