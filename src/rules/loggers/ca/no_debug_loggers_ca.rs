use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEACtrlAgentConfig,
};

use super::super::shared::get_debug_loggers_rule;

pub struct NoDebugLoggersCtrlAgentRule;

impl Rule<KEACtrlAgentConfig> for NoDebugLoggersCtrlAgentRule {
    fn get_name(&self) -> &'static str {
        "LOGGERS::NoDebugLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::ControlAgent
    }
    fn check(&self, config: &KEACtrlAgentConfig) -> Option<Vec<RuleResult>> {
        get_debug_loggers_rule(
            &config.loggers,
            RuleConfigs::ControlAgent.to_string().as_str(),
        )
    }
}

// The tests are written in a shared directory
