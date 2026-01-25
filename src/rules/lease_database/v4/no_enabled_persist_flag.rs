use crate::{
    common::{RuleConfigs, RuleLevels, RuleResult, RuleV4},
    configs::{KEALeaseDatabaseTypes, KEAv4Config},
};

pub struct NoEnabledPersistFlagForMemfileLeasesRule;

impl RuleV4 for NoEnabledPersistFlagForMemfileLeasesRule {
    fn get_name(&self) -> &'static str {
        "LEASE_DATABASE::NoEnabledPersistFlagForMemfileLeases"
    }

    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }

    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }

    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let flag = config.lease_database.persist.unwrap_or(false);
        let lease_database = &config.lease_database.r#type;

        if !flag && lease_database == &KEALeaseDatabaseTypes::Memfile {
            return Some(vec![
                RuleResult {
                    description: "The 'persist' flag is not set to 'true' for the maintenance of the arend database in the 'memfile'".to_string(),
                    snapshot: Some(serde_json::to_string(&config.lease_database).unwrap()),
                    links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#memfile-basic-storage-for-leases"]),
                }
            ]);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        common::RuleV4, configs::v4::KEAv4Config, constants::TEMPLATE_CONFIG_FOR_TESTS_V4,
        rules::lease_database::v4::no_enabled_persist_flag::NoEnabledPersistFlagForMemfileLeasesRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();

        let rule = NoEnabledPersistFlagForMemfileLeasesRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();
        json_value["lease-database"]
            .as_object_mut()
            .unwrap()
            .insert("persist".to_string(), Value::from(true));
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NoEnabledPersistFlagForMemfileLeasesRule;
        assert!(rule.check(&data).is_none());
    }
}
