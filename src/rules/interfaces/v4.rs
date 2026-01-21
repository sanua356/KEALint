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
	            }]
            );
        }
        None
    }
}
