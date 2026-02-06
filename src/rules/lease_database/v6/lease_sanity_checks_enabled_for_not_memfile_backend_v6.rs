use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_lease_sanity_checks_enabled_for_not_memfile_backend_rule;

pub struct LeaseSanityChecksEnabledForNotMemfileBackendV6Rule;

impl Rule<KEAv6Config> for LeaseSanityChecksEnabledForNotMemfileBackendV6Rule {
    fn get_name(&self) -> &'static str {
        "LEASE_DATABASE::LeaseSanityChecksEnabledForNotMemfileBackend"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_lease_sanity_checks_enabled_for_not_memfile_backend_rule(
            &config.sanity_checks,
            &config.lease_database,
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v6::KEAv6Config};

    use super::{
        super::_tests::LEASE_SANITY_CHECKS_ENABLED_FOR_NOT_MEMFILE_BACKEND_RULE_TEST_TEMPLATE,
        LeaseSanityChecksEnabledForNotMemfileBackendV6Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv6Config = serde_json::from_str(
            LEASE_SANITY_CHECKS_ENABLED_FOR_NOT_MEMFILE_BACKEND_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = LeaseSanityChecksEnabledForNotMemfileBackendV6Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(
            LEASE_SANITY_CHECKS_ENABLED_FOR_NOT_MEMFILE_BACKEND_RULE_TEST_TEMPLATE,
        )
        .unwrap();
        json_value["lease-database"]["type"] = Value::from("memfile");
        let data: KEAv6Config = serde_json::from_value(json_value).unwrap();

        let rule = LeaseSanityChecksEnabledForNotMemfileBackendV6Rule;
        assert!(rule.check(&data).is_none());
    }
}
