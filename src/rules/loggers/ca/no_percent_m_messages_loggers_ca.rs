use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEACtrlAgentConfig,
};

use super::super::shared::get_no_percent_m_in_pattern_rule;

pub struct NoPercentMMessagesLoggersCtrlAgentRule;

impl Rule<KEACtrlAgentConfig> for NoPercentMMessagesLoggersCtrlAgentRule {
    fn get_name(&self) -> &'static str {
        "LOGGERS::NoPercentMMessagesLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::ControlAgent
    }
    fn check(&self, config: &KEACtrlAgentConfig) -> Option<Vec<RuleResult>> {
        get_no_percent_m_in_pattern_rule(&config.loggers, &self.get_config_type().to_string())
    }
}

// The tests are written in a shared directory
