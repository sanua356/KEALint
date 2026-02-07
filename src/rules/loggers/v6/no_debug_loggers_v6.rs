use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_debug_loggers_rule;

pub struct NoDebugLoggersV6Rule;

impl Rule<KEAv6Config> for NoDebugLoggersV6Rule {
    fn get_name(&self) -> &'static str {
        "LOGGERS::NoDebugLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_debug_loggers_rule(&config.loggers, &self.get_config_type().to_string())
    }
}

// The tests are written in a shared directory
