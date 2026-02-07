use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_no_linebreak_in_pattern_rule;

pub struct NoLinebreakMessagesLoggersV4Rule;

impl Rule<KEAv4Config> for NoLinebreakMessagesLoggersV4Rule {
    fn get_name(&self) -> &'static str {
        "LOGGERS::NoLinebreakMessagesLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_no_linebreak_in_pattern_rule(&config.loggers)
    }
}

// The tests are written in a shared directory
