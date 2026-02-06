#![allow(non_camel_case_types)]
use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_not_recommended_prefix_AFTER__classes;

pub struct NotRecommendedPrefixAFTER_ClassesV4Rule;

impl Rule<KEAv4Config> for NotRecommendedPrefixAFTER_ClassesV4Rule {
    fn get_name(&self) -> &'static str {
        "CLIENT_CLASSES::NotRecommendedPrefixAFTER_ClassesRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_not_recommended_prefix_AFTER__classes(&config.client_classes)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{
        super::_tests::NOT_RECOMMENDED_PREFIX_AFTER__CLASSES_RULE_TEST_TEMPLATE,
        NotRecommendedPrefixAFTER_ClassesV4Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NOT_RECOMMENDED_PREFIX_AFTER__CLASSES_RULE_TEST_TEMPLATE).unwrap();

        let rule = NotRecommendedPrefixAFTER_ClassesV4Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NOT_RECOMMENDED_PREFIX_AFTER__CLASSES_RULE_TEST_TEMPLATE).unwrap();
        json_value["client-classes"].as_array_mut().unwrap()[0]["name"] = Value::from("test_class");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NotRecommendedPrefixAFTER_ClassesV4Rule;
        assert!(rule.check(&data).is_none());
    }
}
