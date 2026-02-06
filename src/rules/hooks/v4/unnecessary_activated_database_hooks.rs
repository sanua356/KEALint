use std::collections::HashSet;

use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEAHostsDatabasesTypes, KEALeaseDatabaseTypes, v4::KEAv4Config},
    constants::{MYSQL_HOOK_LIBRARY, PGSQL_HOOK_LIBRARY},
};

pub struct UnnecessaryActivatedDatabaseHooksRule;

impl Rule<KEAv4Config> for UnnecessaryActivatedDatabaseHooksRule {
    fn get_name(&self) -> &'static str {
        "HOOKS::UnnecessaryActivatedDatabaseHooksRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let mysql_hook = config
            .hooks_libraries
            .as_ref()?
            .iter()
            .find(|item| item.library.contains(MYSQL_HOOK_LIBRARY));

        let pgsql_hook = config
            .hooks_libraries
            .as_ref()?
            .iter()
            .find(|item| item.library.contains(PGSQL_HOOK_LIBRARY));

        let mut config_types: HashSet<KEAHostsDatabasesTypes> = HashSet::new();

        match config.lease_database.r#type {
            KEALeaseDatabaseTypes::MySQL => {
                config_types.insert(KEAHostsDatabasesTypes::MySQL);
            }
            KEALeaseDatabaseTypes::PostgreSQL => {
                config_types.insert(KEAHostsDatabasesTypes::PostgreSQL);
            }
            _ => {}
        }

        if let Some(hosts_database) = &config.hosts_database
            && let Some(db_type) = &hosts_database.r#type
        {
            config_types.insert(*db_type);
        }

        if let Some(hosts_databases) = &config.hosts_databases {
            for hosts_db in hosts_databases {
                if let Some(db_type) = &hosts_db.r#type {
                    config_types.insert(*db_type);
                }
            }
        }

        if let Some(config_control) = &config.config_control
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
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{
        super::_tests::UNNECESSARY_ACTIVATED_DATABASE_HOOKS_RULE_TEST_TEMPLATE,
        UnnecessaryActivatedDatabaseHooksRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(UNNECESSARY_ACTIVATED_DATABASE_HOOKS_RULE_TEST_TEMPLATE).unwrap();

        let rule = UnnecessaryActivatedDatabaseHooksRule;
        assert!(rule.check(&data).is_some());
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

        let rule = UnnecessaryActivatedDatabaseHooksRule;
        assert!(rule.check(&data).is_none());
    }
}
