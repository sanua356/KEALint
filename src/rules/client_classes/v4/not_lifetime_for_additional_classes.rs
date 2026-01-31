use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

pub struct NotLifetimeForAdditionalClassesRule;

impl Rule<KEAv4Config> for NotLifetimeForAdditionalClassesRule {
    fn get_name(&self) -> &'static str {
        "CLIENT_CLASSES::NotValidLifetimeForAdditionalClassesRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let mut results: Vec<RuleResult> = Vec::new();

        for (idx, class) in config.client_classes.as_ref()?.iter().enumerate() {
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
                    links: Some(vec![
                        "https://kea.readthedocs.io/en/stable/arm/classify.html#class-priority",
                    ]),
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
        super::_tests::NOT_LIFETIME_FOR_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE,
        NotLifetimeForAdditionalClassesRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NOT_LIFETIME_FOR_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE).unwrap();

        let rule = NotLifetimeForAdditionalClassesRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NOT_LIFETIME_FOR_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE).unwrap();
        json_value["client-classes"].as_array_mut().unwrap()[0]["only-in-additional-list"] =
            Value::from(false);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NotLifetimeForAdditionalClassesRule;
        assert!(rule.check(&data).is_none());
    }
}
