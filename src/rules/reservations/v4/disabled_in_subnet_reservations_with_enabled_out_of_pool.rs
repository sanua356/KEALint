use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEAv4Config, KEAv4Subnet},
};

pub struct DisabledInSubnetReservationsWithEnabledOutOfPool;

fn get_reservations_out_of_pool_without_in_subnet_flag(
    subnets: &Vec<KEAv4Subnet>,
    is_reservations_in_subnet_global: bool,
    shared_network_id: Option<String>,
) -> Vec<RuleResult> {
    let mut results: Vec<RuleResult> = Vec::new();

    for subnet in subnets {
        let is_reservations_out_of_pool = subnet.reservations_out_of_pool.unwrap_or_default();

        let msg_subtext = match &shared_network_id {
            Some(network_id) => format!(" in shared netowrk '{}'", network_id),
            None => String::new(),
        };

        if subnet.reservations_in_subnet.is_some() {
            if !subnet.reservations_in_subnet.unwrap_or(true) && is_reservations_out_of_pool {
                results.push(RuleResult {
                    description: format!(
                       	"In the subnet '{}'{}, the 'reservations-in-subnet' parameter is set to 'false'. As long as the flag is set to 'false', the 'reservations-out-of-pool' parameter will have no effect.",
                       	subnet.subnet.to_string(),
                        msg_subtext
                    ),
                    snapshot: None,
                    links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#fine-tuning-dhcpv4-host-reservation"]),
                });
            }
        } else {
            if !is_reservations_in_subnet_global && is_reservations_out_of_pool {
                results.push(RuleResult {
                    description: format!(
                       	"At the upper configuration levels, the 'reservations-in-subnet' parameter is set to 'false'. As long as the flag is set to 'false', the 'reservations-out-of-pool' parameter inside the subnet '{}'{} will have no effect.",
                       	subnet.subnet.to_string(),
                        msg_subtext
                    ),
                    snapshot: None,
                    links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#fine-tuning-dhcpv4-host-reservation"]),
                });
            }
        }
    }

    return results;
}

impl Rule<KEAv4Config> for DisabledInSubnetReservationsWithEnabledOutOfPool {
    fn get_name(&self) -> &'static str {
        "RESERVATIONS::DisabledInSubnetReservationsWithEnabledOutOfPool"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let mut results: Vec<RuleResult> = Vec::new();

        let is_reservations_in_subnet_global = config.reservations_in_subnet.unwrap_or(true);

        if let Some(subnets) = &config.subnet4 {
            results.extend(get_reservations_out_of_pool_without_in_subnet_flag(
                subnets,
                is_reservations_in_subnet_global,
                None,
            ));
        }

        if let Some(shared_networks) = &config.shared_networks {
            for shared_network in shared_networks {
                let is_reservations_in_subnet_shared_network =
                    shared_network.reservations_in_subnet.unwrap_or(true);

                if let Some(subnets) = &shared_network.subnet4 {
                    results.extend(get_reservations_out_of_pool_without_in_subnet_flag(
                        subnets,
                        is_reservations_in_subnet_global
                            && is_reservations_in_subnet_shared_network,
                        Some(shared_network.name.clone()),
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
        rules::reservations::{
            DisabledInSubnetReservationsWithEnabledOutOfPool,
            v4::_tests::DISABLED_IN_SUBNET_RESERVATIONS_WITH_ENABLED_OUT_OF_POOL_RULE_TEMPLATE,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            DISABLED_IN_SUBNET_RESERVATIONS_WITH_ENABLED_OUT_OF_POOL_RULE_TEMPLATE,
        )
        .unwrap();

        let rule = DisabledInSubnetReservationsWithEnabledOutOfPool;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(
            DISABLED_IN_SUBNET_RESERVATIONS_WITH_ENABLED_OUT_OF_POOL_RULE_TEMPLATE,
        )
        .unwrap();
        json_value["subnet4"][1]["reservations-in-subnet"] = Value::from(true);
        json_value["shared-networks"][0]["subnet4"][1]["reservations-in-subnet"] =
            Value::from(true);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = DisabledInSubnetReservationsWithEnabledOutOfPool;
        assert!(rule.check(&data).is_none());
    }
}
