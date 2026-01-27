use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::{KEAv4Config, KEAv4Subnet},
    constants::HOST_CMDS_HOOK_LIBRARY,
};

pub struct SubnetWithoutPoolsAndReservationsRule;

fn get_empty_subnets(subnets: &Vec<KEAv4Subnet>) -> Vec<RuleResult> {
    let mut results: Vec<RuleResult> = Vec::new();

    for subnet in subnets {
        if subnet.reservations.as_ref().unwrap_or(&vec![]).is_empty()
            && subnet.pools.as_ref().unwrap_or(&vec![]).is_empty()
        {
            results.push(RuleResult {
                description: format!("There are no pools or address reservations in the subnet '{}'. It can be deleted.", subnet.subnet),
                snapshot: Some(serde_json::to_string(subnet).unwrap()),
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
            .as_ref()?
            .iter()
            .any(|item| item.library.contains(HOST_CMDS_HOOK_LIBRARY));

        // The check is skipped if this hook is present, since reservations may be located in an external database.
        if is_found_host_cmds_hook {
            return None;
        }

        let mut results: Vec<RuleResult> = Vec::new();

        if let Some(subnets) = &config.subnet4 {
            results.extend(get_empty_subnets(subnets));
        }

        if let Some(shared_networks) = &config.shared_networks {
            for shared_network in shared_networks {
                if let Some(subnets) = &shared_network.subnet4 {
                    results.extend(get_empty_subnets(subnets));
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
        common::Rule, configs::v4::KEAv4Config, constants::TEMPLATE_CONFIG_FOR_TESTS_V4,
        rules::subnets::SubnetWithoutPoolsAndReservationsRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();

        let rule = SubnetWithoutPoolsAndReservationsRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();
        json_value["subnet4"].as_array_mut().unwrap().remove(2);
        json_value["shared-networks"]
            .as_array_mut()
            .unwrap()
            .remove(1);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = SubnetWithoutPoolsAndReservationsRule;
        assert!(rule.check(&data).is_none());
    }
}
