use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_use_user_check_hook_rule;

pub struct UseUsrCheckHookV4Rule;

impl Rule<KEAv4Config> for UseUsrCheckHookV4Rule {
    fn get_name(&self) -> &'static str {
        "HOOKS::UseUsrCheckHookRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_use_user_check_hook_rule(&config.hooks_libraries)
    }
}
