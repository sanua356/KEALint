use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAD2Config,
    rules::loggers::shared::get_no_linebreak_in_pattern_rule,
};

pub struct NoLinebreakMessagesLoggersD2;

impl Rule<KEAD2Config> for NoLinebreakMessagesLoggersD2 {
    fn get_name(&self) -> &'static str {
        "LOGGERS::NoLinebreakMessagesLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::D2
    }
    fn check(&self, config: &KEAD2Config) -> Option<Vec<RuleResult>> {
        get_no_linebreak_in_pattern_rule(
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
        configs::KEAD2Config,
        rules::loggers::{
            NoLinebreakMessagesLoggersD2,
            d2::_tests::NO_LINEBREAK_MESSAGES_LOGGERS_D2_RULE_TEMPLATE,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAD2Config =
            serde_json::from_str(NO_LINEBREAK_MESSAGES_LOGGERS_D2_RULE_TEMPLATE).unwrap();

        let rule = NoLinebreakMessagesLoggersD2;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_LINEBREAK_MESSAGES_LOGGERS_D2_RULE_TEMPLATE).unwrap();
        json_value["loggers"][0]["output-options"][0]["pattern"] =
            Value::from("%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m \\n");

        let data: KEAD2Config = serde_json::from_value(json_value).unwrap();

        let rule = NoLinebreakMessagesLoggersD2;
        assert!(rule.check(&data).is_none());
    }
}
