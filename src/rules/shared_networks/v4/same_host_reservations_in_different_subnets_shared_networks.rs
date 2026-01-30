use std::collections::HashMap;

use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

pub struct SameHostReservationsInDifferentSubnetsSharedNetworksRule;

impl Rule<KEAv4Config> for SameHostReservationsInDifferentSubnetsSharedNetworksRule {
    fn get_name(&self) -> &'static str {
        "SHARED_NETWORKS::SameHostReservationsInDifferentSubnetsSharedNetworksRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let mut results: Vec<RuleResult> = Vec::new();

        for shared_network in config.shared_networks.as_ref()? {
            let mut hw_address_reservations: HashMap<String, Vec<String>> = HashMap::new();

            if let Some(subnets) = &shared_network.subnet4 {
                for subnet in subnets {
                    if let Some(reservations) = &subnet.reservations {
                        for reservation in reservations {
                            let hw = reservation.hw_address.clone().unwrap_or("".to_string());
                            if !hw.is_empty() {
                                hw_address_reservations
                                    .entry(hw.clone())
                                    .or_default()
                                    .push(subnet.subnet.to_string());
                            }
                        }
                    }
                }
            }

            for hw_reservation in hw_address_reservations {
                if hw_reservation.1.len() > 1 {
                    results.push(RuleResult {
                        description: format!(
	                        "The reservation of a host with a MAC address '{}' in the shared network '{}' is specified simultaneously in subnets: '{}'. It is recommended to specify a reservation in only one of the subnets.",
	                        hw_reservation.0,
	                        shared_network.name,
	                        hw_reservation.1.join(", ")
                        ),
                        places: None,
                        links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#host-reservations-in-shared-networks"]),
                    });
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
        rules::shared_networks::{
            SameHostReservationsInDifferentSubnetsSharedNetworksRule,
            v4::_tests::SAME_HOST_RESERVATIONS_IN_DIFFERENT_SUBNETS_SHARED_NETWORKS_RULE_TEMPLATE,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            SAME_HOST_RESERVATIONS_IN_DIFFERENT_SUBNETS_SHARED_NETWORKS_RULE_TEMPLATE,
        )
        .unwrap();

        let rule = SameHostReservationsInDifferentSubnetsSharedNetworksRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(
            SAME_HOST_RESERVATIONS_IN_DIFFERENT_SUBNETS_SHARED_NETWORKS_RULE_TEMPLATE,
        )
        .unwrap();

        json_value["shared-networks"].as_array_mut().unwrap()[0]["subnet4"][0]["reservations"]
            .as_array_mut()
            .unwrap()[0]["hw-address"] = Value::from("11:22:33:44:55:FF");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = SameHostReservationsInDifferentSubnetsSharedNetworksRule;
        assert!(rule.check(&data).is_none());
    }
}
