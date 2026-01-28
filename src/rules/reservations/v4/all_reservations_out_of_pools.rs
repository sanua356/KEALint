use std::net::IpAddr;

use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEAv4Config, KEAv4PoolVariant, KEAv4Subnet},
    utils::v4_pool_to_start_end_available_ips,
};

pub struct AllReservationsOutOfPoolsRule;

fn is_address_in_pool(address: IpAddr, pool: &KEAv4PoolVariant) -> bool {
    match address {
        IpAddr::V4(addr_v4) => {
            let (start_ip, end_ip) = v4_pool_to_start_end_available_ips(*pool);

            addr_v4 >= start_ip && addr_v4 <= end_ip
        }
        IpAddr::V6(_) => false,
    }
}

fn get_reservations_out_of_pool_in_subnet(subnets: &Vec<KEAv4Subnet>) -> Vec<RuleResult> {
    let mut out_of_pool: Vec<RuleResult> = Vec::new();

    for subnet in subnets {
        if subnet.reservations_out_of_pool.unwrap_or_default() {
            continue;
        }

        if subnet.reservations.as_ref().unwrap_or(&vec![]).is_empty() {
            continue;
        }

        let mut is_any_reservation_in_pool = false;

        if let (Some(pools), Some(reservations)) = (&subnet.pools, &subnet.reservations) {
            for reservation in reservations {
                if let Some(address) = reservation.ip_address {
                    for pool in pools {
                        if is_address_in_pool(address, &pool.pool) {
                            is_any_reservation_in_pool = true;
                        }
                    }
                }
            }
        }

        if !is_any_reservation_in_pool {
            out_of_pool.push(RuleResult {
                description: format!("In the subnet '{}', all reservations are registered outside of dynamic pools. It is possible to set the 'reservations-out-of-pool' flag to 'true' to improve performance.", subnet.subnet),
                snapshot: None,
                links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#fine-tuning-dhcpv4-host-reservation"]),
            });
        }
    }

    out_of_pool
}

impl Rule<KEAv4Config> for AllReservationsOutOfPoolsRule {
    fn get_name(&self) -> &'static str {
        "RESERVATIONS::AllReservationsOutOfPoolsRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let mut results: Vec<RuleResult> = Vec::new();

        if let Some(subnets) = &config.subnet4 {
            results.extend(get_reservations_out_of_pool_in_subnet(subnets));
        }

        if let Some(shared_networks) = &config.shared_networks {
            for shared_network in shared_networks {
                if let Some(subnets) = &shared_network.subnet4 {
                    results.extend(get_reservations_out_of_pool_in_subnet(subnets));
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
            AllReservationsOutOfPoolsRule, v4::_tests::ALL_RESERVATIONS_OUT_OF_POOLS_RULE_TEMPLATE,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(ALL_RESERVATIONS_OUT_OF_POOLS_RULE_TEMPLATE).unwrap();

        let rule = AllReservationsOutOfPoolsRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(ALL_RESERVATIONS_OUT_OF_POOLS_RULE_TEMPLATE).unwrap();
        json_value["subnet4"][0]["reservations"] = serde_json::json!([
            {
                "hostname": "in_pool_reservation1",
                "hw-address": "1a:1b:1c:1d:1e:1f",
                "ip-address": "1.8.8.11"
            },
            {
                "hostname": "in_pool_reservation2",
                "hw-address": "11:22:33:44:55:66",
                "ip-address": "1.8.8.12"
            }
        ]);
        json_value["shared-networks"][0]["subnet4"][0]["reservations"] = serde_json::json!([
            {
                "hostname": "qqq",
                "hw-address": "2a:2b:2c:2d:2e:2f",
                "ip-address": "10.0.0.80"
            },
            {
                "hostname": "eee",
                "hw-address": "aa:bb:cc:dd:ee:ff",
                "ip-address": "10.0.0.90"
            }
        ]);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = AllReservationsOutOfPoolsRule;
        assert!(rule.check(&data).is_none());
    }
}
