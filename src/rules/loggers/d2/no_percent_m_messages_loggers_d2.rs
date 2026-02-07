use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAD2Config,
};

use super::super::shared::get_no_percent_m_in_pattern_rule;

pub struct NoPercentMMessagesLoggersD2Rule;

impl Rule<KEAD2Config> for NoPercentMMessagesLoggersD2Rule {
    fn get_name(&self) -> &'static str {
        "LOGGERS::NoPercentMMessagesLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::D2
    }
    fn check(&self, config: &KEAD2Config) -> Option<Vec<RuleResult>> {
        get_no_percent_m_in_pattern_rule(&config.loggers, &self.get_config_type().to_string())
    }
}

// The tests are written in a shared directory
