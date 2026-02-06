use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_lease_sanity_checks_enabled_for_not_memfile_backend_rule;

pub struct LeaseSanityChecksEnabledForNotMemfileBackendV4Rule;

impl Rule<KEAv4Config> for LeaseSanityChecksEnabledForNotMemfileBackendV4Rule {
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
        get_lease_sanity_checks_enabled_for_not_memfile_backend_rule(
            &config.sanity_checks,
            &config.lease_database,
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{
        super::_tests::LEASE_SANITY_CHECKS_ENABLED_FOR_NOT_MEMFILE_BACKEND_RULE_TEST_TEMPLATE,
        LeaseSanityChecksEnabledForNotMemfileBackendV4Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            LEASE_SANITY_CHECKS_ENABLED_FOR_NOT_MEMFILE_BACKEND_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = LeaseSanityChecksEnabledForNotMemfileBackendV4Rule;
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

        let rule = LeaseSanityChecksEnabledForNotMemfileBackendV4Rule;
        assert!(rule.check(&data).is_none());
    }
}
