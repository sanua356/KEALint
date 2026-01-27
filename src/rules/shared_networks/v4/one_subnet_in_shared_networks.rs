use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

pub struct OneSubnetInSharedNetworksRule;

impl Rule<KEAv4Config> for OneSubnetInSharedNetworksRule {
    fn get_name(&self) -> &'static str {
        "SHARED_NETWORKS::OneSubnetInSharedNetworksRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let mut results: Vec<RuleResult> = Vec::new();

        for shared_network in config.shared_networks.as_ref()? {
            if let Some(subnets) = &shared_network.subnet4
                && subnets.len() == 1
            {
                results.push(RuleResult {
                    description: format!("There is one subnet in the network using the 'shared-networks' key named '{}'. It can be moved to the 'subnet4' global configuration.", shared_network.name),
                    snapshot: None,
                    links: None,
                });
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
            OneSubnetInSharedNetworksRule, v4::_tests::ONE_SUBNET_IN_SHARED_NETWORKS_RULE_TEMPLATE,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(ONE_SUBNET_IN_SHARED_NETWORKS_RULE_TEMPLATE).unwrap();

        let rule = OneSubnetInSharedNetworksRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(ONE_SUBNET_IN_SHARED_NETWORKS_RULE_TEMPLATE).unwrap();
        let subnets: Value = serde_json::json!(
            [{
                "id": 1,
                "subnet": "10.0.0.0/8",
                "pools": [
                    {
                        "pool": "10.0.0.1 - 10.0.0.99"
                    }
                ]
            },
            {
                "id": 2,
                "subnet": "10.0.0.0/16",
                "pools": [
                    {
                        "pool": "10.0.0.100 - 10.0.0.150"
                    }
                ]
            }]
        );
        json_value["shared-networks"].as_array_mut().unwrap()[0]["subnet4"] = subnets;
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = OneSubnetInSharedNetworksRule;
        assert!(rule.check(&data).is_none());
    }
}
