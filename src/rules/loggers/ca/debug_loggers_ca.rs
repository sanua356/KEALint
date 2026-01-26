use crate::{
    common::{RuleConfigs, RuleCtrlAgent, RuleLevels, RuleResult},
    configs::KEACtrlAgentConfig,
};

use super::super::shared::get_debug_loggers_rule;

pub struct DebugLoggersCtrlAgentRule;

impl RuleCtrlAgent for DebugLoggersCtrlAgentRule {
    fn get_name(&self) -> &'static str {
        "LOGGERS::DebugLoggersCtrlAgentRule"
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
        common::RuleCtrlAgent,
        configs::{KEACtrlAgentConfig, loggers::KEALoggerSeverityTypes},
        constants::TEMPLATE_CONFIG_FOR_TESTS_CTRL_AGENT,
        rules::loggers::DebugLoggersCtrlAgentRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEACtrlAgentConfig =
            serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_CTRL_AGENT).unwrap();

        let rule = DebugLoggersCtrlAgentRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_CTRL_AGENT).unwrap();
        json_value["loggers"].as_array_mut().unwrap()[0]["severity"] =
            Value::from(KEALoggerSeverityTypes::INFO.to_string());

        println!("{:?}", json_value["loggers"]);
        let data: KEACtrlAgentConfig = serde_json::from_value(json_value).unwrap();

        let rule = DebugLoggersCtrlAgentRule;
        assert!(rule.check(&data).is_none());
    }
}
