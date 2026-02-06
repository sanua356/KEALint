use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_no_percent_m_in_pattern_rule;

pub struct NoPercentMMessagesLoggersV6Rule;

impl Rule<KEAv6Config> for NoPercentMMessagesLoggersV6Rule {
    fn get_name(&self) -> &'static str {
        "LOGGERS::NoPercentMMessagesLoggersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_no_percent_m_in_pattern_rule(
            config.loggers.as_ref()?,
            &self.get_config_type().to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::KEAv6Config};

    use super::{
        super::_tests::NO_PERCENT_M_MESSAGES_LOGGERS_V6_RULE_TEMPLATE,
        NoPercentMMessagesLoggersV6Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv6Config =
            serde_json::from_str(NO_PERCENT_M_MESSAGES_LOGGERS_V6_RULE_TEMPLATE).unwrap();

        let rule = NoPercentMMessagesLoggersV6Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_PERCENT_M_MESSAGES_LOGGERS_V6_RULE_TEMPLATE).unwrap();
        json_value["loggers"][0]["output-options"][0]["pattern"] =
            Value::from("%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m \\n");

        let data: KEAv6Config = serde_json::from_value(json_value).unwrap();

        let rule = NoPercentMMessagesLoggersV6Rule;
        assert!(rule.check(&data).is_none());
    }
}
