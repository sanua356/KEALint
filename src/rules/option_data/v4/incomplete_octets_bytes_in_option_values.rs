use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEAv4Config, KEAv4OptionData, KEAv4Subnet},
    constants::OPTION_DATA_BYTES_REGEXP,
};

pub struct IncompleteOctetsBytesInOptionValuesRule;

fn get_kea_incomplete_bytes_options(
    options: &[KEAv4OptionData],
    placement: String,
) -> Vec<RuleResult> {
    let mut results: Vec<RuleResult> = Vec::new();

    for (idx_option, option) in options.iter().enumerate() {
        let option_value = &option.data.clone().unwrap_or(String::from("0x00"));

        if !option.csv_format.unwrap_or(true) && !OPTION_DATA_BYTES_REGEXP.is_match(option_value) {
            let option_id = if option.name.is_some() {
                format!("name '{}'", option.name.clone().unwrap())
            } else {
                format!("code '{}'", option.code.unwrap())
            };
            results.push(RuleResult {
                description: format!(
                "The option with {} has the parameter 'csv-format' set to 'false', but the key 'data' has incomplete octets in value '{}'. It is recommended to always specify octets in pairs (for example: FF, A1, 22).",
				option_id,
				option_value
                ),
                places: Some(vec![format!("{}.{}", placement, idx_option)]),
                links: Some(&["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#standard-dhcpv4-options"]),
            });
        }
    }

    results
}

fn get_kea_incomplete_bytes_options_in_subnets(
    subnets: &[KEAv4Subnet],
    placement: String,
) -> Vec<RuleResult> {
    let mut results: Vec<RuleResult> = Vec::new();

    for (idx_subnet, subnet) in subnets.iter().enumerate() {
        if let Some(option_data) = &subnet.option_data {
            results.extend(get_kea_incomplete_bytes_options(
                option_data,
                format!("{}.{}.option-data", placement, idx_subnet),
            ));
        }

        for (idx_pool, pool) in subnet.pools.as_ref().unwrap_or(&vec![]).iter().enumerate() {
            if pool.option_data.is_some() {
                results.extend(get_kea_incomplete_bytes_options(
                    pool.option_data.as_ref().unwrap_or(&vec![]),
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

impl Rule<KEAv4Config> for IncompleteOctetsBytesInOptionValuesRule {
    fn get_name(&self) -> &'static str {
        "OPTION_DATA::IncompleteOctetsBytesInOptionValuesRule"
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
            results.extend(get_kea_incomplete_bytes_options(
                options,
                "option-data".to_string(),
            ));
        }
        if let Some(classes) = &config.client_classes {
            for (idx_class, class) in classes.iter().enumerate() {
                results.extend(get_kea_incomplete_bytes_options(
                    class.option_data.as_ref().unwrap_or(&vec![]),
                    format!("client-classes.{}.option-data", idx_class),
                ));
            }
        }
        if let Some(subnets) = &config.subnet4 {
            results.extend(get_kea_incomplete_bytes_options_in_subnets(
                subnets,
                "subnet4".to_string(),
            ));
        }

        if let Some(shared_networks) = &config.shared_networks {
            for (idx_shared_network, shared_network) in shared_networks.iter().enumerate() {
                if shared_network.option_data.is_some() {
                    results.extend(get_kea_incomplete_bytes_options(
                        shared_network.option_data.as_ref().unwrap_or(&vec![]),
                        format!("shared-networks.{}.option-data", idx_shared_network),
                    ));
                }
                if shared_network.subnet4.is_some() {
                    results.extend(get_kea_incomplete_bytes_options_in_subnets(
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

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{
        super::_tests::INCOMPLETE_OCTETS_BYTES_IN_OPTION_VALUES_RULE_TEST_TEMPLATE,
        IncompleteOctetsBytesInOptionValuesRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(INCOMPLETE_OCTETS_BYTES_IN_OPTION_VALUES_RULE_TEST_TEMPLATE)
                .unwrap();

        let rule = IncompleteOctetsBytesInOptionValuesRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(INCOMPLETE_OCTETS_BYTES_IN_OPTION_VALUES_RULE_TEST_TEMPLATE)
                .unwrap();

        json_value["option-data"][0]["data"] = Value::from("1AB2");
        json_value["subnet4"][0]["option-data"][0]["data"] = Value::from("AABBCC");
        json_value["subnet4"][1]["pools"][0]["option-data"][0]["data"] = Value::from("0xee12");
        json_value["shared-networks"][0]["option-data"][0]["data"] = Value::from("0x12");
        json_value["shared-networks"][0]["subnet4"][0]["option-data"][0]["data"] =
            Value::from("11 22");
        json_value["shared-networks"][0]["subnet4"][0]["pools"][0]["option-data"][0]["data"] =
            Value::from("AA:22:33");

        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = IncompleteOctetsBytesInOptionValuesRule;
        assert!(rule.check(&data).is_none());
    }
}
