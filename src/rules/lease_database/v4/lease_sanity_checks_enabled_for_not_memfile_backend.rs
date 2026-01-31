use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEALeaseDatabaseTypes, KEAv4Config},
};

pub struct LeaseSanityChecksEnabledForNotMemfileBackend;

impl Rule<KEAv4Config> for LeaseSanityChecksEnabledForNotMemfileBackend {
    fn get_name(&self) -> &'static str {
        "LEASE_DATABASE::LeaseSanityChecksEnabledForNotMemfileBackend"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        if let Some(sanity_checks) = &config.sanity_checks
            && sanity_checks.lease_checks.is_some()
            && config.lease_database.r#type != KEALeaseDatabaseTypes::Memfile
        {
            return Some(vec![RuleResult {
                description: "The Sanity Checks mechanism is not implemented for rent databases other than 'memfile'.".to_string(),
                links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#sanity-checks-in-dhcpv4"]),
                places: Some(vec!["lease-database.type".to_string(), "sanity-checks.lease-checks".to_string()]),
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
        super::_tests::LEASE_SANITY_CHECKS_ENABLED_FOR_NOT_MEMFILE_BACKEND_RULE_TEST_TEMPLATE,
        LeaseSanityChecksEnabledForNotMemfileBackend,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            LEASE_SANITY_CHECKS_ENABLED_FOR_NOT_MEMFILE_BACKEND_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = LeaseSanityChecksEnabledForNotMemfileBackend;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(
            LEASE_SANITY_CHECKS_ENABLED_FOR_NOT_MEMFILE_BACKEND_RULE_TEST_TEMPLATE,
        )
        .unwrap();
        json_value["lease-database"]["type"] = Value::from("memfile");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = LeaseSanityChecksEnabledForNotMemfileBackend;
        assert!(rule.check(&data).is_none());
    }
}
