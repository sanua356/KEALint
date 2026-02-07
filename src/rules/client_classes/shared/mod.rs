#![allow(non_snake_case)]

use crate::{common::RuleResult, configs::client_classes::KEAClientClass};

pub fn get_not_lifetime_for_additional_classes_rule(
    client_classes: &Option<Vec<KEAClientClass>>,
) -> Option<Vec<RuleResult>> {
    let mut results: Vec<RuleResult> = Vec::new();

    for (idx, class) in client_classes.as_ref()?.iter().enumerate() {
        if class.only_in_additional_list.unwrap_or_default()
            && (class.min_valid_lifetime.is_some()
                || class.valid_lifetime.is_some()
                || class.max_valid_lifetime.is_some()
                || class.offer_lifetime.is_some())
        {
            results.push(RuleResult {
                description: format!(
                   	"For the client class '{}', the 'only-in-additional-list' flag is set to 'true'. The class contains any of the following parameters: 'min-valid-lifetime', 'valid-lifetime', 'max-valid-lifetime' or 'offer-lifetime', but they will have no effect.",
                   	class.name
                ),
                places: Some(vec![format!("client-classes.{}", idx)]),
                links: Some(&["https://kea.readthedocs.io/en/stable/arm/classify.html#class-priority"]),
            });
        }
    }

    if !results.is_empty() {
        return Some(results);
    }

    None
}

pub fn get_not_recommended_prefix_AFTER__classes(
    client_classes: &Option<Vec<KEAClientClass>>,
) -> Option<Vec<RuleResult>> {
    let mut results: Vec<RuleResult> = Vec::new();

    for (idx, class) in client_classes.as_ref()?.iter().enumerate() {
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

#[cfg(test)]
pub mod _tests;

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::configs::v4::KEAv4Config;

    use super::{
        _tests::{
            NOT_LIFETIME_FOR_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE,
            NOT_RECOMMENDED_PREFIX_AFTER__CLASSES_RULE_TEST_TEMPLATE,
        },
        get_not_lifetime_for_additional_classes_rule, get_not_recommended_prefix_AFTER__classes,
    };

    #[test]
    fn check_expected_not_lifetime_for_additional_classes_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NOT_LIFETIME_FOR_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE).unwrap();

        let rule = get_not_lifetime_for_additional_classes_rule(&data.client_classes);
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_not_lifetime_for_additional_classes_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NOT_LIFETIME_FOR_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE).unwrap();
        json_value["client-classes"].as_array_mut().unwrap()[0]["only-in-additional-list"] =
            Value::from(false);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = get_not_lifetime_for_additional_classes_rule(&data.client_classes);
        assert!(rule.is_none());
    }

    #[test]
    fn check_expected_not_recommended_prefix_AFTER__classes_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NOT_RECOMMENDED_PREFIX_AFTER__CLASSES_RULE_TEST_TEMPLATE).unwrap();

        let rule = get_not_recommended_prefix_AFTER__classes(&data.client_classes);
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_not_recommended_prefix_AFTER__classes_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NOT_RECOMMENDED_PREFIX_AFTER__CLASSES_RULE_TEST_TEMPLATE).unwrap();
        json_value["client-classes"].as_array_mut().unwrap()[0]["name"] = Value::from("test_class");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = get_not_recommended_prefix_AFTER__classes(&data.client_classes);
        assert!(rule.is_none());
    }
}
