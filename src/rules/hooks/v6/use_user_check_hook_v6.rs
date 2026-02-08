use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_use_user_check_hook_rule;

pub struct UseUsrCheckHookV6Rule;

impl Rule<KEAv6Config> for UseUsrCheckHookV6Rule {
    fn get_name(&self) -> &'static str {
        "HOOKS::UseUsrCheckHookRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_use_user_check_hook_rule(&config.hooks_libraries)
    }
}
