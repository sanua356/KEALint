use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_more_one_object_config_HA::get_more_one_object_config_HA;

pub struct MoreOneObjectConfigHAV6Rule;

impl Rule<KEAv6Config> for MoreOneObjectConfigHAV6Rule {
    fn get_name(&self) -> &'static str {
        "HOOKS::MoreOneObjectConfigHARule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_more_one_object_config_HA(&config.hooks_libraries)
    }
}

// The tests are written in a shared directory
