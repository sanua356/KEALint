use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_debug_loggers_rule;

pub struct NoDebugLoggersV6Rule;

impl Rule<KEAv6Config> for NoDebugLoggersV6Rule {
    fn get_name(&self) -> &'static str {
        "LOGGERS::NoDebugLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        if let Some(loggers) = &config.loggers {
            return get_debug_loggers_rule(loggers, &self.get_config_type().to_string());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        common::Rule,
        configs::{loggers::KEALoggerSeverityTypes, v6::KEAv6Config},
    };

    use super::{super::_tests::DEBUG_LOGGERS_V6_RULE_TEMPLATE, NoDebugLoggersV6Rule};

    #[test]
    fn check_expected_trigger() {
        let data: KEAv6Config = serde_json::from_str(DEBUG_LOGGERS_V6_RULE_TEMPLATE).unwrap();

        let rule = NoDebugLoggersV6Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(DEBUG_LOGGERS_V6_RULE_TEMPLATE).unwrap();
        json_value["loggers"].as_array_mut().unwrap()[0]["severity"] =
            Value::from(KEALoggerSeverityTypes::INFO.to_string());

        let data: KEAv6Config = serde_json::from_value(json_value).unwrap();

        let rule = NoDebugLoggersV6Rule;
        assert!(rule.check(&data).is_none());
    }
}
