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
