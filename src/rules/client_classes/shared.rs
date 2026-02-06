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

#[allow(non_snake_case)]
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
