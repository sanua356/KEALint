use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEACtrlAgentConfig,
    rules::loggers::shared::get_no_percent_m_in_pattern_rule,
};

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
        get_no_percent_m_in_pattern_rule(
            config.loggers.as_ref()?,
            &self.get_config_type().to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        common::Rule,
        configs::KEACtrlAgentConfig,
        rules::loggers::{
            NoPercentMMessagesLoggersCtrlAgentRule,
            ca::_tests::NO_PERCENT_M_MESSAGES_LOGGERS_CTRL_AGENT_RULE_TEMPLATE,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEACtrlAgentConfig =
            serde_json::from_str(NO_PERCENT_M_MESSAGES_LOGGERS_CTRL_AGENT_RULE_TEMPLATE).unwrap();

        let rule = NoPercentMMessagesLoggersCtrlAgentRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_PERCENT_M_MESSAGES_LOGGERS_CTRL_AGENT_RULE_TEMPLATE).unwrap();
        json_value["loggers"][0]["output-options"][0]["pattern"] =
            Value::from("%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m \\n");

        let data: KEACtrlAgentConfig = serde_json::from_value(json_value).unwrap();

        let rule = NoPercentMMessagesLoggersCtrlAgentRule;
        assert!(rule.check(&data).is_none());
    }
}
