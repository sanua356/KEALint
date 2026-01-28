use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
    rules::loggers::shared::get_no_linebreak_in_pattern_rule,
};

pub struct NoLinebreakMessagesLoggersV4;

impl Rule<KEAv4Config> for NoLinebreakMessagesLoggersV4 {
    fn get_name(&self) -> &'static str {
        "LOGGERS::NoLinebreakMessagesLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_no_linebreak_in_pattern_rule(config.loggers.as_ref()?, &RuleConfigs::Dhcp4.to_string())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        common::Rule,
        configs::KEAv4Config,
        rules::loggers::{
            NoLinebreakMessagesLoggersV4,
            v4::_tests::NO_LINEBREAK_MESSAGES_LOGGERS_V4_RULE_TEMPLATE,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NO_LINEBREAK_MESSAGES_LOGGERS_V4_RULE_TEMPLATE).unwrap();

        let rule = NoLinebreakMessagesLoggersV4;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_LINEBREAK_MESSAGES_LOGGERS_V4_RULE_TEMPLATE).unwrap();
        json_value["loggers"][0]["output-options"][0]["pattern"] =
            Value::from("%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m \\n");

        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NoLinebreakMessagesLoggersV4;
        assert!(rule.check(&data).is_none());
    }
}
