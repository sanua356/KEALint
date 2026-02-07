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

#[cfg(test)]
pub mod _tests;

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::configs::v4::KEAv4Config;

    use super::{
        _tests::{
            LEASE_SANITY_CHECKS_ENABLED_FOR_NOT_MEMFILE_BACKEND_RULE_TEST_TEMPLATE,
            NO_ENABLED_PERSIST_FLAG_FOR_MEMFILE_LEASES_RULE_TEST_TEMPLATE,
            NOT_CHANGE_STOP_RETRY_EXIT_STRATEGY_ON_FAIL_RULE_TEST_TEMPLATE,
        },
        get_lease_sanity_checks_enabled_for_not_memfile_backend_rule,
        get_no_enabled_persist_flag_rule, get_not_change_stop_retry_exit_strategy_on_fail_rule,
    };

    #[test]
    fn check_expected_lease_sanity_checks_enabled_for_not_memfile_backend_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            LEASE_SANITY_CHECKS_ENABLED_FOR_NOT_MEMFILE_BACKEND_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = get_lease_sanity_checks_enabled_for_not_memfile_backend_rule(
            &data.sanity_checks,
            &data.lease_database,
        );
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_lease_sanity_checks_enabled_for_not_memfile_backend_trigger() {
        let mut json_value: Value = serde_json::from_str(
            LEASE_SANITY_CHECKS_ENABLED_FOR_NOT_MEMFILE_BACKEND_RULE_TEST_TEMPLATE,
        )
        .unwrap();
        json_value["lease-database"]["type"] = Value::from("memfile");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = get_lease_sanity_checks_enabled_for_not_memfile_backend_rule(
            &data.sanity_checks,
            &data.lease_database,
        );
        assert!(rule.is_none());
    }

    #[test]
    fn check_expected_no_enabled_persist_flag_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NO_ENABLED_PERSIST_FLAG_FOR_MEMFILE_LEASES_RULE_TEST_TEMPLATE)
                .unwrap();

        let flag = data.lease_database.persist.unwrap_or(false);
        let lease_database = &data.lease_database.r#type;

        let rule = get_no_enabled_persist_flag_rule(flag, lease_database);
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_no_enabled_persist_flag_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_ENABLED_PERSIST_FLAG_FOR_MEMFILE_LEASES_RULE_TEST_TEMPLATE)
                .unwrap();
        json_value["lease-database"]
            .as_object_mut()
            .unwrap()
            .insert("persist".to_string(), Value::from(true));
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let flag = data.lease_database.persist.unwrap_or(false);
        let lease_database = &data.lease_database.r#type;

        let rule = get_no_enabled_persist_flag_rule(flag, lease_database);
        assert!(rule.is_none());
    }

    #[test]
    fn check_expected_not_change_stop_retry_exit_strategy_on_fail_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NOT_CHANGE_STOP_RETRY_EXIT_STRATEGY_ON_FAIL_RULE_TEST_TEMPLATE)
                .unwrap();

        let rule = get_not_change_stop_retry_exit_strategy_on_fail_rule(&data.lease_database);
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_not_change_stop_retry_exit_strategy_on_fail_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NOT_CHANGE_STOP_RETRY_EXIT_STRATEGY_ON_FAIL_RULE_TEST_TEMPLATE)
                .unwrap();
        json_value["lease-database"]["on-fail"] = Value::from("stop-retry-exit");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = get_not_change_stop_retry_exit_strategy_on_fail_rule(&data.lease_database);
        assert!(rule.is_none());
    }
}
