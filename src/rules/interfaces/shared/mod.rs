use crate::{common::RuleResult, configs::interfaces::KEAInterfacesConfig};

pub fn get_no_active_interfaces(
    interfaces_config: &KEAInterfacesConfig,
) -> Option<Vec<RuleResult>> {
    if interfaces_config.interfaces.iter().len() == 0 {
        return Some(
	      vec![RuleResult {
		        description: "No network interfaces are specified in the server configuration. Addresses will not be serviced.".to_string(),
		        places: Some(vec!["interfaces-config.interfaces".to_string()]),
				links: Some(&["https://kea.readthedocs.io/en/latest/arm/dhcp6-srv.html#interface-configuration"])
			}]
        );
    }
    None
}

#[cfg(test)]
pub mod _tests;

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::configs::v4::KEAv4Config;

    use super::{
        _tests::NO_INTERFACES_IN_INTERFACES_CONFIG_RULE_TEST_TEMPLATE, get_no_active_interfaces,
    };

    #[test]
    fn check_expected_get_no_active_interfaces_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NO_INTERFACES_IN_INTERFACES_CONFIG_RULE_TEST_TEMPLATE).unwrap();

        let rule = get_no_active_interfaces(&data.interfaces_config);
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_get_no_active_interfaces_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_INTERFACES_IN_INTERFACES_CONFIG_RULE_TEST_TEMPLATE).unwrap();
        json_value["interfaces-config"]
            .as_object_mut()
            .unwrap()
            .insert("interfaces".to_string(), Value::from(["eth0"]));
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = get_no_active_interfaces(&data.interfaces_config);
        assert!(rule.is_none());
    }
}
