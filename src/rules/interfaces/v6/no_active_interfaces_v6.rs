use crate::common::{Rule, RuleConfigs, RuleLevels, RuleResult};
use crate::configs::KEAv6Config;

use super::super::shared::get_no_active_interfaces;

pub struct NoInterfacesInInterfacesConfigV6Rule;

impl Rule<KEAv6Config> for NoInterfacesInInterfacesConfigV6Rule {
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }

    fn get_name(&self) -> &'static str {
        "INTERFACES::NoInterfacesInInterfacesConfigRule"
    }

    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }

    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_no_active_interfaces(&config.interfaces_config)
    }
}

// The tests are written in a shared directory
