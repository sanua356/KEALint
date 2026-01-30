use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEAv4Config, KEAv4OptionData, KEAv4Subnet},
    constants::KEA_NO_CONFIGURABLE_OPTIONS,
};

pub struct SpecifiedKEAManagedOptionsRule;

fn get_kea_managed_options(options: &Vec<KEAv4OptionData>, placement: String) -> Vec<RuleResult> {
    let mut results: Vec<RuleResult> = Vec::new();

    for (idx_option, option) in options.into_iter().enumerate() {
        let finded_kea_managed_option =
            KEA_NO_CONFIGURABLE_OPTIONS
                .iter()
                .find(|kea_managed_option| {
                    (kea_managed_option.code == option.code.unwrap_or(0))
                        || (kea_managed_option.name
                            == option.name.as_ref().unwrap_or(&"".to_string()))
                });

        if finded_kea_managed_option.is_some() {
            let managed_option = finded_kea_managed_option.unwrap();
            results.push(RuleResult {
                description: format!(
                "The option with the name '{}' and the code '{}' is controlled by the KEA engine and is not recommended for manual setting.",
				managed_option.name,
				managed_option.code
                ),
                places: Some(vec![format!("{}.{}", placement, idx_option)]),
                links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#id6"]),
            });
        }
    }

    results
}

fn get_kea_managed_options_in_subnets(
    subnets: &Vec<KEAv4Subnet>,
    placement: String,
) -> Vec<RuleResult> {
    let mut results: Vec<RuleResult> = Vec::new();

    for (idx_subnet, subnet) in subnets.into_iter().enumerate() {
        if subnet.option_data.is_some() {
            results.extend(get_kea_managed_options(
                &subnet.option_data.as_ref().unwrap_or(&vec![]),
                format!("{}.{}.option-data", placement, idx_subnet),
            ));
        }

        for (idx_pool, pool) in subnet
            .pools
            .as_ref()
            .unwrap_or(&vec![])
            .into_iter()
            .enumerate()
        {
            if pool.option_data.is_some() {
                results.extend(get_kea_managed_options(
                    &pool.option_data.as_ref().unwrap_or(&vec![]),
                    format!(
                        "{}.{}.pools.{}.option-data",
                        placement, idx_subnet, idx_pool
                    ),
                ));
            }
        }
    }

    results
}

impl Rule<KEAv4Config> for SpecifiedKEAManagedOptionsRule {
    fn get_name(&self) -> &'static str {
        "OPTION_DATA::SpecifiedKEAManagedOptionsRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let mut results: Vec<RuleResult> = Vec::new();

        if let Some(options) = &config.option_data {
            results.extend(get_kea_managed_options(options, "option-data".to_string()));
        }
        if let Some(classes) = &config.client_classes {
            for (idx_class, class) in classes.into_iter().enumerate() {
                results.extend(get_kea_managed_options(
                    &class.option_data.as_ref().unwrap_or(&vec![]),
                    format!("client-classes.{}.option-data", idx_class),
                ));
            }
        }
        if let Some(subnets) = &config.subnet4 {
            results.extend(get_kea_managed_options_in_subnets(
                subnets,
                "subnet4".to_string(),
            ));
        }

        if let Some(shared_networks) = &config.shared_networks {
            for (idx_shared_network, shared_network) in shared_networks.into_iter().enumerate() {
                if shared_network.option_data.is_some() {
                    results.extend(get_kea_managed_options(
                        shared_network.option_data.as_ref().unwrap_or(&vec![]),
                        format!("shared-networks.{}.option-data", idx_shared_network),
                    ));
                }
                if shared_network.subnet4.is_some() {
                    results.extend(get_kea_managed_options_in_subnets(
                        shared_network.subnet4.as_ref().unwrap_or(&vec![]),
                        format!("shared-networks.{}.subnet4", idx_shared_network),
                    ));
                }
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
        common::Rule,
        configs::v4::KEAv4Config,
        rules::option_data::{
            SpecifiedKEAManagedOptionsRule,
            v4::_tests::SPECIFIED_KEA_MANAGED_OPTIONS_RULE_TEST_TEMPLATE,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(SPECIFIED_KEA_MANAGED_OPTIONS_RULE_TEST_TEMPLATE).unwrap();

        let rule = SpecifiedKEAManagedOptionsRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(SPECIFIED_KEA_MANAGED_OPTIONS_RULE_TEST_TEMPLATE).unwrap();

        json_value["option-data"][0]["code"] = Value::from(77);
        json_value["subnet4"][0]["option-data"][0]["code"] = Value::from(77);
        json_value["subnet4"][1]["pools"][0]["option-data"][0]["name"] = Value::from("user-class");
        json_value["shared-networks"][0]["option-data"][0]["name"] = Value::from("user-class");
        json_value["shared-networks"][0]["subnet4"][0]["option-data"][0]["name"] =
            Value::from("user-class");
        json_value["shared-networks"][0]["subnet4"][0]["pools"][0]["option-data"][0]["name"] =
            Value::from("user-class");

        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = SpecifiedKEAManagedOptionsRule;
        assert!(rule.check(&data).is_none());
    }
}
