use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAD2Config,
};

use super::super::shared::get_debug_loggers_rule;

pub struct DebugLoggersD2Rule;

impl Rule<KEAD2Config> for DebugLoggersD2Rule {
    fn get_name(&self) -> &'static str {
        "LOGGERS::DebugLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::D2
    }
    fn check(&self, config: &KEAD2Config) -> Option<Vec<RuleResult>> {
        if let Some(loggers) = &config.loggers {
            return get_debug_loggers_rule(loggers, RuleConfigs::D2.to_string().as_str());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        common::Rule,
        configs::{KEAD2Config, loggers::KEALoggerSeverityTypes},
    };

    use super::{super::_tests::DEBUG_LOGGERS_D2_RULE_TEMPLATE, DebugLoggersD2Rule};

    #[test]
    fn check_expected_trigger() {
        let data: KEAD2Config = serde_json::from_str(DEBUG_LOGGERS_D2_RULE_TEMPLATE).unwrap();

        let rule = DebugLoggersD2Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(DEBUG_LOGGERS_D2_RULE_TEMPLATE).unwrap();
        json_value["loggers"].as_array_mut().unwrap()[0]["severity"] =
            Value::from(KEALoggerSeverityTypes::INFO.to_string());

        let data: KEAD2Config = serde_json::from_value(json_value).unwrap();

        let rule = DebugLoggersD2Rule;
        assert!(rule.check(&data).is_none());
    }
}
