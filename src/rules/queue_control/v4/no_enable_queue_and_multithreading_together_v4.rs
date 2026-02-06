use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_no_enable_queue_and_multithreading_together_rule;

pub struct NoEnableQueueAndMultithreadingTogetherV4Rule;

impl Rule<KEAv4Config> for NoEnableQueueAndMultithreadingTogetherV4Rule {
    fn get_name(&self) -> &'static str {
        "QUEUE_CONTROL::NoEnableQueueAndMultithreadingTogetherRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_no_enable_queue_and_multithreading_together_rule(
            &config.multi_threading,
            &config.dhcp_queue_control,
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{
        super::_tests::NO_ENABLE_QUEUE_AND_MULTITHREADING_TOGETER_RULE_TEMPLATE,
        NoEnableQueueAndMultithreadingTogetherV4Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NO_ENABLE_QUEUE_AND_MULTITHREADING_TOGETER_RULE_TEMPLATE).unwrap();

        let rule = NoEnableQueueAndMultithreadingTogetherV4Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_ENABLE_QUEUE_AND_MULTITHREADING_TOGETER_RULE_TEMPLATE).unwrap();
        json_value["multi-threading"]["enable-multi-threading"] = Value::from(false);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NoEnableQueueAndMultithreadingTogetherV4Rule;
        assert!(rule.check(&data).is_none());
    }
}
