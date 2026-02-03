#![allow(non_camel_case_types)]
use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

pub struct NotRecommendedPrefixAFTER_ClassesRule;

impl Rule<KEAv4Config> for NotRecommendedPrefixAFTER_ClassesRule {
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
        let mut results: Vec<RuleResult> = Vec::new();

        for (idx, class) in config.client_classes.as_ref()?.iter().enumerate() {
            if class.name.starts_with("AFTER_") {
                results.push(RuleResult {
                    description: format!(
	                    "The client class named '{}' uses the prefix 'AFTER_'. This prefix is reserved by the system for a query processing mechanism that has not yet been written. It is recommended to replace the prefix with an arbitrary one.",
	                    class.name
                    ),
                    places: Some(vec![format!("client-classes.{}", idx)]),
                    links: Some(&["https://kea.readthedocs.io/en/latest/arm/classify.html#built-in-client-classes"]),
                });
            }
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
        super::_tests::NOT_RECOMMENDED_PREFIX_AFTER__CLASSES_RULE_TEST_TEMPLATE,
        NotRecommendedPrefixAFTER_ClassesRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NOT_RECOMMENDED_PREFIX_AFTER__CLASSES_RULE_TEST_TEMPLATE).unwrap();

        let rule = NotRecommendedPrefixAFTER_ClassesRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NOT_RECOMMENDED_PREFIX_AFTER__CLASSES_RULE_TEST_TEMPLATE).unwrap();
        json_value["client-classes"].as_array_mut().unwrap()[0]["name"] = Value::from("test_class");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NotRecommendedPrefixAFTER_ClassesRule;
        assert!(rule.check(&data).is_none());
    }
}
