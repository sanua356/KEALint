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
        if let Some(loggers) = &config.loggers {
            return get_debug_loggers_rule(loggers, RuleConfigs::ControlAgent.to_string().as_str());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        common::Rule,
        configs::{KEACtrlAgentConfig, loggers::KEALoggerSeverityTypes},
    };

    use super::{
        super::_tests::DEBUG_LOGGERS_CTRL_AGENT_RULE_TEMPLATE, NoDebugLoggersCtrlAgentRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEACtrlAgentConfig =
            serde_json::from_str(DEBUG_LOGGERS_CTRL_AGENT_RULE_TEMPLATE).unwrap();

        let rule = NoDebugLoggersCtrlAgentRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(DEBUG_LOGGERS_CTRL_AGENT_RULE_TEMPLATE).unwrap();
        json_value["loggers"].as_array_mut().unwrap()[0]["severity"] =
            Value::from(KEALoggerSeverityTypes::INFO.to_string());

        let data: KEACtrlAgentConfig = serde_json::from_value(json_value).unwrap();

        let rule = NoDebugLoggersCtrlAgentRule;
        assert!(rule.check(&data).is_none());
    }
}
