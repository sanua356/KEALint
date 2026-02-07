use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAD2Config,
};

use super::super::shared::get_no_linebreak_in_pattern_rule;

pub struct NoLinebreakMessagesLoggersD2Rule;

impl Rule<KEAD2Config> for NoLinebreakMessagesLoggersD2Rule {
    fn get_name(&self) -> &'static str {
        "LOGGERS::NoLinebreakMessagesLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::D2
    }
    fn check(&self, config: &KEAD2Config) -> Option<Vec<RuleResult>> {
        get_no_linebreak_in_pattern_rule(&config.loggers)
    }
}

// The tests are written in a shared directory
