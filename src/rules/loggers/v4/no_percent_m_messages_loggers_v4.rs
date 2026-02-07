use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_no_percent_m_in_pattern_rule;

pub struct NoPercentMMessagesLoggersV4Rule;

impl Rule<KEAv4Config> for NoPercentMMessagesLoggersV4Rule {
    fn get_name(&self) -> &'static str {
        "LOGGERS::NoPercentMMessagesLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_no_percent_m_in_pattern_rule(&config.loggers, &self.get_config_type().to_string())
    }
}

// The tests are written in a shared directory
