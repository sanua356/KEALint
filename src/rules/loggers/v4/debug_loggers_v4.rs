use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_debug_loggers_rule;

pub struct DebugLoggersV4Rule;

impl Rule<KEAv4Config> for DebugLoggersV4Rule {
    fn get_name(&self) -> &'static str {
        "LOGGERS::DebugLoggersV4Rule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        if let Some(loggers) = &config.loggers {
            return get_debug_loggers_rule(loggers, RuleConfigs::Dhcp4.to_string().as_str());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        common::Rule,
        configs::{loggers::KEALoggerSeverityTypes, v4::KEAv4Config},
        rules::loggers::{DebugLoggersV4Rule, v4::_tests::DEBUG_LOGGERS_V4_RULE_TEMPLATE},
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(DEBUG_LOGGERS_V4_RULE_TEMPLATE).unwrap();

        let rule = DebugLoggersV4Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(DEBUG_LOGGERS_V4_RULE_TEMPLATE).unwrap();
        json_value["loggers"].as_array_mut().unwrap()[0]["severity"] =
            Value::from(KEALoggerSeverityTypes::INFO.to_string());

        println!("{:?}", json_value["loggers"]);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = DebugLoggersV4Rule;
        assert!(rule.check(&data).is_none());
    }
}
