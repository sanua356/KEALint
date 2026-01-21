use crate::common::{RuleLevels, RuleResult, RuleV4};
use crate::configs::v4::KEAv4Config;

pub struct NoInterfacesInInterfacesConfigRule;

impl RuleV4 for NoInterfacesInInterfacesConfigRule {
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }

    fn get_name(&self) -> &'static str {
        "INTERFACES::NoInterfacesInInterfacesConfigRule"
    }

    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        if config.interfaces_config.interfaces.iter().len() == 0 {
            return Some(
	            vec![RuleResult {
	                description: "No network interfaces are specified in the server configuration. Addresses will not be serviced.".to_string(),
	                snapshot: Some(serde_json::to_string(&config.interfaces_config).unwrap()),
					links: Some(vec!["https://kea.readthedocs.io/en/kea-3.0.0/arm/dhcp6-srv.html#interface-configuration".to_string()])
	            }]
            );
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        common::RuleV4, configs::v4::KEAv4Config, constants::TEMPLATE_CONFIG_FOR_TESTS_V4,
        rules::interfaces::v4::no_active_interfaces::NoInterfacesInInterfacesConfigRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();

        let rule = NoInterfacesInInterfacesConfigRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();
        json_value["interfaces-config"]
            .as_object_mut()
            .unwrap()
            .insert("interfaces".to_string(), Value::from(["eth0"]));
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NoInterfacesInInterfacesConfigRule;
        assert!(rule.check(&data).is_none());
    }
}
