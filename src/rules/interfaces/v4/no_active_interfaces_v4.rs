use crate::common::{Rule, RuleConfigs, RuleLevels, RuleResult};
use crate::configs::KEAv4Config;

use super::super::shared::get_no_active_interfaces;

pub struct NoInterfacesInInterfacesConfigV4Rule;

impl Rule<KEAv4Config> for NoInterfacesInInterfacesConfigV4Rule {
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
        get_no_active_interfaces(&config.interfaces_config)
    }
}

// The tests are written in a shared directory
