use crate::common::{Rule, RuleConfigs, RuleLevels, RuleResult};
use crate::configs::KEAv4Config;

pub struct NoInterfacesInInterfacesConfigRule;

impl Rule<KEAv4Config> for NoInterfacesInInterfacesConfigRule {
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }

    fn get_name(&self) -> &'static str {
        "INTERFACES::NoInterfacesInInterfacesConfigRule"
    }

    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }

    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        if config.interfaces_config.interfaces.iter().len() == 0 {
            return Some(
	            vec![RuleResult {
	                description: "No network interfaces are specified in the server configuration. Addresses will not be serviced.".to_string(),
	                places: Some(vec!["interfaces-config.interfaces".to_string()]),
					links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp6-srv.html#interface-configuration"])
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
        common::Rule,
        configs::v4::KEAv4Config,
        rules::interfaces::v4::{
            _tests::NO_INTERFACES_IN_INTERFACES_CONFIG_RULE_TEST_TEMPLATE,
            no_active_interfaces::NoInterfacesInInterfacesConfigRule,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NO_INTERFACES_IN_INTERFACES_CONFIG_RULE_TEST_TEMPLATE).unwrap();

        let rule = NoInterfacesInInterfacesConfigRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_INTERFACES_IN_INTERFACES_CONFIG_RULE_TEST_TEMPLATE).unwrap();
        json_value["interfaces-config"]
            .as_object_mut()
            .unwrap()
            .insert("interfaces".to_string(), Value::from(["eth0"]));
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NoInterfacesInInterfacesConfigRule;
        assert!(rule.check(&data).is_none());
    }
}
