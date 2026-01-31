use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEAv4Config, KEAv4Subnet},
    constants::HOST_CMDS_HOOK_LIBRARY,
};

pub struct SubnetWithoutPoolsAndReservationsRule;

fn get_empty_subnets(subnets: &[KEAv4Subnet], placement: String) -> Vec<RuleResult> {
    let mut results: Vec<RuleResult> = Vec::new();

    for (idx_subnet, subnet) in subnets.iter().enumerate() {
        if subnet.reservations.as_ref().unwrap_or(&vec![]).is_empty()
            && subnet.pools.as_ref().unwrap_or(&vec![]).is_empty()
        {
            results.push(RuleResult {
                description: format!("There are no pools or address reservations in the subnet '{}'. It can be deleted.", subnet.subnet),
                places: Some(vec![format!("{}.{}", placement, idx_subnet)]),
                links: None,
            });
        }
    }

    results
}

impl Rule<KEAv4Config> for SubnetWithoutPoolsAndReservationsRule {
    fn get_name(&self) -> &'static str {
        "SUBNETS::SubnetWithoutPoolsAndReservationsRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let is_found_host_cmds_hook = config
            .hooks_libraries
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .any(|item| item.library.contains(HOST_CMDS_HOOK_LIBRARY));

        // The check is skipped if this hook is present, since reservations may be located in an external database.
        if is_found_host_cmds_hook {
            return None;
        }

        let mut results: Vec<RuleResult> = Vec::new();

        if let Some(subnets) = &config.subnet4 {
            results.extend(get_empty_subnets(subnets, "subnet4".to_string()));
        }

        if let Some(shared_networks) = &config.shared_networks {
            for (idx_shared_network, shared_network) in shared_networks.iter().enumerate() {
                if let Some(shared_subnets) = &shared_network.subnet4 {
                    results.extend(get_empty_subnets(
                        shared_subnets,
                        format!("shared-networks.{}", idx_shared_network),
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
        super::_tests::SUBNETS_WITHOUT_POOLS_AND_RESERVATIONS_TEST_TEMPLATE,
        SubnetWithoutPoolsAndReservationsRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(SUBNETS_WITHOUT_POOLS_AND_RESERVATIONS_TEST_TEMPLATE).unwrap();

        let rule = SubnetWithoutPoolsAndReservationsRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(SUBNETS_WITHOUT_POOLS_AND_RESERVATIONS_TEST_TEMPLATE).unwrap();
        json_value["subnet4"].as_array_mut().unwrap().remove(0);
        json_value["shared-networks"]
            .as_array_mut()
            .unwrap()
            .remove(0);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = SubnetWithoutPoolsAndReservationsRule;
        assert!(rule.check(&data).is_none());
    }
}
