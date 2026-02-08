use std::collections::HashSet;

use crate::{
    common::RuleResult,
    configs::{
        KEAHostsDatabasesTypes, KEALeaseDatabaseTypes, config_control::KEAConfigControl,
        hooks::KEAHookLibrary, hosts_database::KEAHostsDatabase, lease_database::KEALeaseDatabase,
    },
    constants::{MYSQL_HOOK_LIBRARY, PGSQL_HOOK_LIBRARY},
};

pub fn get_unnecessary_activated_database_hooks_rule(
    hooks_libraries: &Option<Vec<KEAHookLibrary>>,
    lease_database: &KEALeaseDatabase,
    hosts_database: &Option<KEAHostsDatabase>,
    hosts_databases: &Option<Vec<KEAHostsDatabase>>,
    config_control: &Option<KEAConfigControl>,
) -> Option<Vec<RuleResult>> {
    let mysql_hook = hooks_libraries
        .as_ref()?
        .iter()
        .find(|item| item.library.contains(MYSQL_HOOK_LIBRARY));

    let pgsql_hook = hooks_libraries
        .as_ref()?
        .iter()
        .find(|item| item.library.contains(PGSQL_HOOK_LIBRARY));

    let mut config_types: HashSet<KEAHostsDatabasesTypes> = HashSet::new();

    match lease_database.r#type {
        KEALeaseDatabaseTypes::MySQL => {
            config_types.insert(KEAHostsDatabasesTypes::MySQL);
        }
        KEALeaseDatabaseTypes::PostgreSQL => {
            config_types.insert(KEAHostsDatabasesTypes::PostgreSQL);
        }
        _ => {}
    }

    if let Some(hosts_database) = hosts_database
        && let Some(db_type) = &hosts_database.r#type
    {
        config_types.insert(*db_type);
    }

    if let Some(hosts_databases) = hosts_databases {
        for hosts_db in hosts_databases {
            if let Some(db_type) = &hosts_db.r#type {
                config_types.insert(*db_type);
            }
        }
    }

    if let Some(config_control) = config_control
        && let Some(control_databases) = &config_control.config_databases
    {
        for control_db in control_databases {
            if let Some(db_type) = &control_db.r#type {
                config_types.insert(*db_type);
            }
        }
    }

    let mut results: Vec<RuleResult> = Vec::new();

    if mysql_hook.is_some() && !config_types.contains(&KEAHostsDatabasesTypes::MySQL) {
        results.push(
        	RuleResult {
	        	description: "The MySQL support hook is specified in the configuration of the hooks, but it does not serve any functionality.".to_string(),
		        places: None,
		        links: Some(&["https://kea.readthedocs.io/en/latest/arm/hooks.html#libdhcp-mysql-so-database-backend-for-mysql"])
        });
    }

    if pgsql_hook.is_some() && !config_types.contains(&KEAHostsDatabasesTypes::PostgreSQL) {
        results.push(
            RuleResult {
               	description: "The PostgreSQL support hook is specified in the configuration of the hooks, but it does not serve any functionality.".to_string(),
		    	places: None,
			    links: Some(&["https://kea.readthedocs.io/en/latest/arm/hooks.html#libdhcp-pgsql-so-database-backend-for-postgresql"])
		});
    }

    if !results.is_empty() {
        return Some(results);
    }

    None
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::configs::v4::KEAv4Config;

    use super::{
        super::_tests::UNNECESSARY_ACTIVATED_DATABASE_HOOKS_RULE_TEST_TEMPLATE,
        get_unnecessary_activated_database_hooks_rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(UNNECESSARY_ACTIVATED_DATABASE_HOOKS_RULE_TEST_TEMPLATE).unwrap();

        let rule = get_unnecessary_activated_database_hooks_rule(
            &data.hooks_libraries,
            &data.lease_database,
            &data.hosts_database,
            &data.hosts_databases,
            &data.config_control,
        );
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(UNNECESSARY_ACTIVATED_DATABASE_HOOKS_RULE_TEST_TEMPLATE).unwrap();
        json_value["hooks-libraries"]
            .as_array_mut()
            .unwrap()
            .clear();
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = get_unnecessary_activated_database_hooks_rule(
            &data.hooks_libraries,
            &data.lease_database,
            &data.hosts_database,
            &data.hosts_databases,
            &data.config_control,
        );
        assert!(rule.is_none());
    }
}
