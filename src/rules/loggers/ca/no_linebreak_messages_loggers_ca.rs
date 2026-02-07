use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEACtrlAgentConfig,
};

use super::super::shared::get_no_linebreak_in_pattern_rule;

pub struct NoLinebreakMessagesLoggersCtrlAgent;

impl Rule<KEACtrlAgentConfig> for NoLinebreakMessagesLoggersCtrlAgent {
    fn get_name(&self) -> &'static str {
        "LOGGERS::NoLinebreakMessagesLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::ControlAgent
    }
    fn check(&self, config: &KEACtrlAgentConfig) -> Option<Vec<RuleResult>> {
        get_no_linebreak_in_pattern_rule(&config.loggers)
    }
}

// The tests are written in a shared directory
