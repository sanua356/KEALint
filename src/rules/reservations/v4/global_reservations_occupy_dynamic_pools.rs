use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEAv4Config, KEAv4Subnet, reservations::KEAReservation},
    utils::is_address_in_pool,
};

pub struct GlobalReservationsOccupyDynamicPoolsRule;

fn get_global_resrvations_occupy_dynamic_pool_in_subnets(
    global_reservations: &[KEAReservation],
    subnets: &[KEAv4Subnet],
) -> Vec<RuleResult> {
    let mut results: Vec<RuleResult> = Vec::new();

    for subnet in subnets {
        match &subnet.pools {
            Some(pools) => {
                for pool in pools {
                    let global_reservation = global_reservations.iter().enumerate().find(|item| {
                        match item.1.ip_address {
                            Some(ip_address) => return is_address_in_pool(ip_address, &pool.pool),
                            _ => (),
                        }
                        false
                    });

                    match global_reservation {
                        Some((reservation_idx, reservation)) => {
                            results.push(RuleResult {
                                description: format!(
                	                                    "The global reservation with the IP address '{}' belongs to the dynamic pool '{}'. It is recommended to change the reservation address or pool size.",
                	                                    reservation.ip_address.unwrap(),
                 	                                    pool.pool,
                                ),
                                places: Some(vec![format!("reservation.{}", reservation_idx)]),
                                links: Some(&["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#conflicts-in-dhcpv4-reservations"]),
                            });
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    results
}

impl Rule<KEAv4Config> for GlobalReservationsOccupyDynamicPoolsRule {
    fn get_name(&self) -> &'static str {
        "RESERVATIONS::GlobalReservationsOccupyDynamicPoolsRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let mut results: Vec<RuleResult> = Vec::new();

        if !config.reservations_global.unwrap_or_default() {
            return None;
        }

        match &config.subnet4 {
            Some(subnets) => {
                results.extend(get_global_resrvations_occupy_dynamic_pool_in_subnets(
                    config.reservations.as_ref().unwrap_or(&vec![]),
                    subnets,
                ));
            }
            _ => (),
        }

        match &config.shared_networks {
            Some(shared_networks) => {
                for shared_network in shared_networks {
                    match &shared_network.subnet4 {
                        Some(subnets) => {
                            results.extend(get_global_resrvations_occupy_dynamic_pool_in_subnets(
                                config.reservations.as_ref().unwrap_or(&vec![]),
                                subnets,
                            ));
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
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
        super::_tests::GLOBAL_RESERVATIONS_OCCUPY_DYNAMIC_POOLS_RULE_TEMPLATE,
        GlobalReservationsOccupyDynamicPoolsRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(GLOBAL_RESERVATIONS_OCCUPY_DYNAMIC_POOLS_RULE_TEMPLATE).unwrap();

        let rule = GlobalReservationsOccupyDynamicPoolsRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(GLOBAL_RESERVATIONS_OCCUPY_DYNAMIC_POOLS_RULE_TEMPLATE).unwrap();
        json_value["reservations"][0]["ip-address"] = Value::from("1.2.3.4");
        json_value["reservations"][1]["ip-address"] = Value::from("5.6.7.8");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = GlobalReservationsOccupyDynamicPoolsRule;
        assert!(rule.check(&data).is_none());
    }
}
