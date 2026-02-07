use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_no_activated_host_cache_hook_for_radius_hook;

pub struct NoActivatedHostCacheHookForRADIUSHookV4Rule;

impl Rule<KEAv4Config> for NoActivatedHostCacheHookForRADIUSHookV4Rule {
    fn get_name(&self) -> &'static str {
        "HOOKS::NoActivatedHostCacheHookForRADIUSHookRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_no_activated_host_cache_hook_for_radius_hook(&config.hooks_libraries)
    }
}

// The tests are written in a shared directory
