use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_no_activated_host_cache_hook_for_radius_hook;

pub struct NoActivatedHostCacheHookForRADIUSHookV6Rule;

impl Rule<KEAv6Config> for NoActivatedHostCacheHookForRADIUSHookV6Rule {
    fn get_name(&self) -> &'static str {
        "HOOKS::NoActivatedHostCacheHookForRADIUSHookRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_no_activated_host_cache_hook_for_radius_hook(&config.hooks_libraries)
    }
}

// The tests are written in a shared directory
