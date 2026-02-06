use crate::{
    common::RuleResult,
    configs::{
        KEAHostsDatabasesFailStrategers, KEALeaseDatabaseTypes, lease_database::KEALeaseDatabase,
        sanity_checks::KEASanityChecks,
    },
};

pub fn get_lease_sanity_checks_enabled_for_not_memfile_backend_rule(
    sanity_checks: &Option<KEASanityChecks>,
    lease_database: &KEALeaseDatabase,
) -> Option<Vec<RuleResult>> {
    if let Some(sanity_checks) = sanity_checks
        && sanity_checks.lease_checks.is_some()
        && lease_database.r#type != KEALeaseDatabaseTypes::Memfile
    {
        return Some(vec![RuleResult {
            description: "The Sanity Checks mechanism is not implemented for rent databases other than 'memfile'.".to_string(),
            places: Some(vec!["lease-database.type".to_string(), "sanity-checks.lease-checks".to_string()]),
            links: Some(&["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#sanity-checks-in-dhcpv4"]),
        }]);
    }

    None
}

pub fn get_no_enabled_persist_flag_rule(
    is_persist: bool,
    lease_database_type: &KEALeaseDatabaseTypes,
) -> Option<Vec<RuleResult>> {
    if !is_persist && lease_database_type == &KEALeaseDatabaseTypes::Memfile {
        return Some(vec![
            RuleResult {
                description: "The 'persist' flag is not set to 'true' for the maintenance of the arend database in the 'memfile'".to_string(),
                places: Some(vec!["lease-database.persist".to_string()]),
                links: Some(&["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#memfile-basic-storage-for-leases"]),
            }
        ]);
    }

    None
}

pub fn get_not_change_stop_retry_exit_strategy_on_fail_rule(
    lease_database: &KEALeaseDatabase,
) -> Option<Vec<RuleResult>> {
    let lease_database_strategy = lease_database.on_fail.as_ref()?;

    if lease_database_strategy != &KEAHostsDatabasesFailStrategers::StopRetryExit {
        return Some(vec![RuleResult {
            description: "It is recommended to set the 'on-fail' parameter in the 'lease-database' configuration to 'stop-retry-exit' for the correct processing of leases in the production environment.".to_string(),
            links: Some(&["https://kea.readthedocs.io/en/latest/arm/dhcp6-srv.html#lease-database-configuration"]),
            places: Some(vec!["lease-database.on-fail".to_string()]),
        }]);
    }

    None
}
