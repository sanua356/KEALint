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
