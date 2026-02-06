use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_not_ddns_qualifying_suffix_with_enabled_ddns_updates_rule;

pub struct NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesV4Rule;

impl Rule<KEAv4Config> for NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesV4Rule {
    fn get_name(&self) -> &'static str {
        "DDNS_SERVER::NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let is_enabled_ddns = config.ddns_send_updates?;

        let ddns_qualifying_suffix = config.ddns_qualifying_suffix.clone().unwrap_or_default();

        get_not_ddns_qualifying_suffix_with_enabled_ddns_updates_rule(
            is_enabled_ddns,
            ddns_qualifying_suffix,
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{
        super::_tests::NOT_DDNS_QUALIFYING_SUFFIX_WITH_ENABLED_DDNS_UPDATES_RULE_TEST_TEMPLATE,
        NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesV4Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            NOT_DDNS_QUALIFYING_SUFFIX_WITH_ENABLED_DDNS_UPDATES_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesV4Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(
            NOT_DDNS_QUALIFYING_SUFFIX_WITH_ENABLED_DDNS_UPDATES_RULE_TEST_TEMPLATE,
        )
        .unwrap();
        json_value["ddns-qualifying-suffix"] = Value::from("aa.bb.cc");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesV4Rule;
        assert!(rule.check(&data).is_none());
    }
}
