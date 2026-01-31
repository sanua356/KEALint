use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

pub struct NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule;

impl Rule<KEAv4Config> for NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule {
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

        if is_enabled_ddns && ddns_qualifying_suffix.is_empty() {
            return Some(vec![RuleResult {
                description: "It is recommended to specify the value for the 'ddns-qualifying-suffix' field when enabling DDNS updates.".to_string(),
                links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#kea-dhcp4-name-generation-for-ddns-update-requests"]),
                places: Some(vec!["ddns-send-updates".to_string(), "ddns-qualifying-suffix".to_string()]),
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
        super::_tests::NOT_DDNS_QUALIFYING_SUFFIX_WITH_ENABLED_DDNS_UPDATES_RULE_TEST_TEMPLATE,
        NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            NOT_DDNS_QUALIFYING_SUFFIX_WITH_ENABLED_DDNS_UPDATES_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule;
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

        let rule = NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule;
        assert!(rule.check(&data).is_none());
    }
}
