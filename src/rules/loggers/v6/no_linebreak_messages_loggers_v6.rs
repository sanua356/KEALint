use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_no_linebreak_in_pattern_rule;

pub struct NoLinebreakMessagesLoggersV6Rule;

impl Rule<KEAv6Config> for NoLinebreakMessagesLoggersV6Rule {
    fn get_name(&self) -> &'static str {
        "LOGGERS::NoLinebreakMessagesLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_no_linebreak_in_pattern_rule(&config.loggers, &self.get_config_type().to_string())
    }
}

// The tests are written in a shared directory
