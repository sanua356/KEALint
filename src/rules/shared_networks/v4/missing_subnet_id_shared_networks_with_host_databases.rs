use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

pub struct MissingSubnetIdSharedNetworksWithHostDatabases;

impl Rule<KEAv4Config> for MissingSubnetIdSharedNetworksWithHostDatabases {
    fn get_name(&self) -> &'static str {
        "SHARED_NETWORKS::MissingSubnetIdSharedNetworksWithHostDatabases"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        if config.hosts_database.is_none()
            && config
                .hosts_databases
                .as_ref()
                .unwrap_or(&vec![])
                .is_empty()
        {
            return None;
        }

        let mut results: Vec<RuleResult> = Vec::new();

        for (idx_shared_network, shared_network) in
            config.shared_networks.as_ref()?.into_iter().enumerate()
        {
            if let Some(subnets) = &shared_network.subnet4 {
                for (idx_subnet, subnet) in subnets.into_iter().enumerate() {
                    if subnet.id.is_none() {
                        results.push(RuleResult {
                            description: format!(
	                            "The shared network '{}' has a subnet '{}' that does not have an 'id' key with a unique value, which is recommended to be specified if your host reservations are served by a database.",
	                            shared_network.name,
	                            subnet.subnet
                            ),
                            places: Some(vec![format!("shared-networks.{}.subnet4.{}", idx_shared_network, idx_subnet)]),
                            links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#host-reservations-in-shared-networks"]),
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
            MissingSubnetIdSharedNetworksWithHostDatabases,
            v4::_tests::MISSING_SUBNET_ID_SHARED_NETWORKS_WITH_HOST_DATABASES_RULE_TEMPLATE,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            MISSING_SUBNET_ID_SHARED_NETWORKS_WITH_HOST_DATABASES_RULE_TEMPLATE,
        )
        .unwrap();

        let rule = MissingSubnetIdSharedNetworksWithHostDatabases;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(
            MISSING_SUBNET_ID_SHARED_NETWORKS_WITH_HOST_DATABASES_RULE_TEMPLATE,
        )
        .unwrap();

        json_value["shared-networks"].as_array_mut().unwrap()[0]["subnet4"][0]
            .as_object_mut()
            .unwrap()
            .insert("id".to_string(), Value::from(15));
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = MissingSubnetIdSharedNetworksWithHostDatabases;
        assert!(rule.check(&data).is_none());
    }
}
