use crate::{
    common::{RuleConfigs, RuleLevels, RuleResult, RuleV4},
    configs::KEAv4Config,
};

use super::super::shared::get_debug_loggers_rule;

pub struct DebugLoggersV4Rule;

impl RuleV4 for DebugLoggersV4Rule {
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
        common::RuleV4,
        configs::{loggers::KEALoggerSeverityTypes, v4::KEAv4Config},
        constants::TEMPLATE_CONFIG_FOR_TESTS_V4,
        rules::loggers::DebugLoggersV4Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();

        let rule = DebugLoggersV4Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();
        json_value["loggers"].as_array_mut().unwrap()[0]["severity"] =
            Value::from(KEALoggerSeverityTypes::INFO.to_string());

        println!("{:?}", json_value["loggers"]);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = DebugLoggersV4Rule;
        assert!(rule.check(&data).is_none());
    }
}
