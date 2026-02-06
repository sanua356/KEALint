use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_not_lifetime_for_additional_classes_rule;

pub struct NotLifetimeForAdditionalClassesV6Rule;

impl Rule<KEAv6Config> for NotLifetimeForAdditionalClassesV6Rule {
    fn get_name(&self) -> &'static str {
        "CLIENT_CLASSES::NotValidLifetimeForAdditionalClassesRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_not_lifetime_for_additional_classes_rule(&config.client_classes)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v6::KEAv6Config};

    use super::{
        super::_tests::NOT_LIFETIME_FOR_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE,
        NotLifetimeForAdditionalClassesV6Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv6Config =
            serde_json::from_str(NOT_LIFETIME_FOR_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE).unwrap();

        let rule = NotLifetimeForAdditionalClassesV6Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NOT_LIFETIME_FOR_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE).unwrap();
        json_value["client-classes"].as_array_mut().unwrap()[0]["only-in-additional-list"] =
            Value::from(false);
        let data: KEAv6Config = serde_json::from_value(json_value).unwrap();

        let rule = NotLifetimeForAdditionalClassesV6Rule;
        assert!(rule.check(&data).is_none());
    }
}
