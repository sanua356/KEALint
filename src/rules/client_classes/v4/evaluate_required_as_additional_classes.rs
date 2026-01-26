use std::collections::HashSet;

use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEAv4Config, KEAv4Subnet},
};

pub struct EvaluateRequiredAsAdditionalClassesRule;

fn find_additional_classes_from_subnets(subnets: &Vec<KEAv4Subnet>) -> HashSet<String> {
    let mut classes: HashSet<String> = HashSet::new();

    for subnet in subnets {
        if let Some(evaluated_classes) = &subnet.evaluate_additional_classes {
            for evaluated in evaluated_classes {
                classes.insert(evaluated.clone());
            }
        }

        if let Some(pools) = &subnet.pools {
            for pool in pools {
                if let Some(evaluated_classes) = &pool.evaluate_additional_classes {
                    for evaluated in evaluated_classes {
                        classes.insert(evaluated.clone());
                    }
                }
            }
        }
    }

    classes
}

impl Rule<KEAv4Config> for EvaluateRequiredAsAdditionalClassesRule {
    fn get_name(&self) -> &'static str {
        "CLIENT_CLASSES::EvaluateRequiredAsAdditionalClassesRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        config.client_classes.as_ref()?;

        let client_classes = config.client_classes.as_ref().unwrap();

        let mut evaluate_additional_classes: HashSet<String> = HashSet::new();

        if let Some(subnets) = &config.subnet4 {
            evaluate_additional_classes.extend(find_additional_classes_from_subnets(subnets));
        }

        if let Some(shared_networks) = &config.shared_networks {
            for shared_network in shared_networks {
                if let Some(evaluated_classes) = &shared_network.evaluate_additional_classes {
                    for evaluated in evaluated_classes {
                        evaluate_additional_classes.insert(evaluated.clone());
                    }
                }

                if let Some(subnets) = &shared_network.subnet4 {
                    evaluate_additional_classes
                        .extend(find_additional_classes_from_subnets(subnets));
                }
            }
        }

        let mut results: Vec<RuleResult> = Vec::new();

        for additional_class in evaluate_additional_classes {
            let class = client_classes
                .iter()
                .find(|item| item.name == additional_class);

            if let Some(class_info) = class
                && !class_info.only_if_required.unwrap_or_default()
                && !class_info.only_in_additional_list.unwrap_or_default()
            {
                results.push(RuleResult {
                description: format!("The client class named '{}' is specified as the value by the 'evaluate-additional-classes' key in the configuration, but does not have the 'only-if-required' or 'only-in-additional-list' flag set to 'true'.", class_info.name),
                snapshot: None,
                links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#additional-classification"]),
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

    use crate::{
        common::Rule, configs::v4::KEAv4Config, constants::TEMPLATE_CONFIG_FOR_TESTS_V4,
        rules::client_classes::EvaluateRequiredAsAdditionalClassesRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();

        let rule = EvaluateRequiredAsAdditionalClassesRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();
        json_value["client-classes"].as_array_mut().unwrap()[0]["only-in-additional-list"] =
            Value::from(true);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = EvaluateRequiredAsAdditionalClassesRule;
        assert!(rule.check(&data).is_none());
    }
}
