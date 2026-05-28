use std::collections::HashSet;

use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEAv4Config, KEAv4Subnet},
};

pub struct EvaluateRequiredAsAdditionalClassesRule;

fn find_additional_classes_from_subnets(subnets: &Vec<KEAv4Subnet>) -> HashSet<String> {
    let mut classes: HashSet<String> = HashSet::new();

    for subnet in subnets {
        match &subnet.evaluate_additional_classes {
            Some(evaluated_classes) => {
                for evaluated in evaluated_classes {
                    classes.insert(evaluated.clone());
                }
            }
            None => {}
        }

        match &subnet.pools {
            Some(pools) => {
                for pool in pools {
                    match &pool.evaluate_additional_classes {
                        Some(evaluated_classes) => {
                            for evaluated in evaluated_classes {
                                classes.insert(evaluated.clone());
                            }
                        }
                        _ => (),
                    }
                }
            }
            None => {}
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
        let client_classes = config.client_classes.as_ref()?;

        let mut evaluate_additional_classes: HashSet<String> = HashSet::new();

        match &config.subnet4 {
            Some(subnets) => {
                evaluate_additional_classes.extend(find_additional_classes_from_subnets(subnets));
            }
            _ => (),
        }

        match &config.shared_networks {
            Some(shared_networks) => {
                for shared_network in shared_networks {
                    match &shared_network.evaluate_additional_classes {
                        Some(evaluated_classes) => {
                            for evaluated in evaluated_classes {
                                evaluate_additional_classes.insert(evaluated.clone());
                            }
                        }
                        _ => (),
                    }

                    match &shared_network.subnet4 {
                        Some(subnets) => {
                            evaluate_additional_classes
                                .extend(find_additional_classes_from_subnets(subnets));
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }

        let mut results: Vec<RuleResult> = Vec::new();

        for (idx, additional_class) in evaluate_additional_classes.into_iter().enumerate() {
            let class = client_classes
                .iter()
                .find(|item| item.name == additional_class);

            match class {
                Some(class_info) if !class_info.only_in_additional_list.unwrap_or_default() => {
                    results.push(RuleResult {
                    description: format!("The client class named '{}' is specified as the value by the 'evaluate-additional-classes' key in the configuration, but does not have the 'only-if-required' or 'only-in-additional-list' flag set to 'true'.", class_info.name),
                    places: Some(vec![format!("client-classes.{}", idx)]),
                    links: Some(&["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#additional-classification"]),
                });
                }
                _ => (),
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
        super::_tests::EVALUATE_REQUIRED_AS_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE,
        EvaluateRequiredAsAdditionalClassesRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(EVALUATE_REQUIRED_AS_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE)
                .unwrap();

        let rule = EvaluateRequiredAsAdditionalClassesRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(EVALUATE_REQUIRED_AS_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE)
                .unwrap();
        json_value["client-classes"].as_array_mut().unwrap()[0]["only-in-additional-list"] =
            Value::from(true);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = EvaluateRequiredAsAdditionalClassesRule;
        assert!(rule.check(&data).is_none());
    }
}
