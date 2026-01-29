use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

pub struct InterfaceOrRelaysInsideSubnetsSharedNetworksRule;

impl Rule<KEAv4Config> for InterfaceOrRelaysInsideSubnetsSharedNetworksRule {
    fn get_name(&self) -> &'static str {
        "SHARED_NETWORKS::InterfaceOrRelaysInsideSubnetsSharedNetworksRule"
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
            if let Some(subnets) = &shared_network.subnet4 {
                for subnet in subnets {
                    if let Some(interface) = &subnet.interface
                        && !interface.is_empty()
                    {
                        results.push(RuleResult {
                            description: format!(
	                            "The shared network named '{}' has a subnet named '{}', which has its own network interface named '{}'. It is recommended to create a common network interface for all subnets within the same shared network.",
	                            shared_network.name,
	                            subnet.subnet,
	                            interface
                            ),
                            snapshot: Some(serde_json::to_string(subnet).unwrap()),
                            links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#local-and-relayed-traffic-in-shared-networks"]),
                        });
                    }

                    if let Some(relay) = &subnet.relay
                        && !relay.ip_addresses.as_ref().unwrap_or(&vec![]).is_empty()
                    {
                        results.push(RuleResult {
                            description: format!(
	                            "The shared network named '{}' has a subnet named '{}', which has its own relay addresses. It is recommended to make common relay addresses for all subnets within the same shared network.",
	                            shared_network.name,
	                            subnet.subnet
                            ),
                            snapshot: Some(serde_json::to_string(subnet).unwrap()),
                            links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#local-and-relayed-traffic-in-shared-networks"]),
                        });
                    }
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
            InterfaceOrRelaysInsideSubnetsSharedNetworksRule,
            v4::_tests::INTERFACE_OR_RELAYS_INSIDE_SUBNETS_SHARED_NETWORKS_RULE_TEMPLATE,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(INTERFACE_OR_RELAYS_INSIDE_SUBNETS_SHARED_NETWORKS_RULE_TEMPLATE)
                .unwrap();

        let rule = InterfaceOrRelaysInsideSubnetsSharedNetworksRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(INTERFACE_OR_RELAYS_INSIDE_SUBNETS_SHARED_NETWORKS_RULE_TEMPLATE)
                .unwrap();

        json_value["shared-networks"].as_array_mut().unwrap()[0]["subnet4"][0]
            .as_object_mut()
            .unwrap()
            .remove("interface");
        json_value["shared-networks"].as_array_mut().unwrap()[0]["subnet4"][1]
            .as_object_mut()
            .unwrap()
            .remove("relay");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = InterfaceOrRelaysInsideSubnetsSharedNetworksRule;
        assert!(rule.check(&data).is_none());
    }
}
