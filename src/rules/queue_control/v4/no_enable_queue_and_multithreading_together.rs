use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

pub struct NoEnableQueueAndMultithreadingTogetherRule;

impl Rule<KEAv4Config> for NoEnableQueueAndMultithreadingTogetherRule {
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
        let is_enabled_multithreading = config.multi_threading.as_ref()?.enable_multi_threading?;
        let is_enabled_queue_control = config.dhcp_queue_control.as_ref()?.enable_queue?;

        if is_enabled_multithreading && is_enabled_queue_control {
            return Some(vec![RuleResult {
                description: "The 'dhcp-queue-control' parameters will not work while multithreading mode is activated for KEA in the global configuration using the 'multi-threading' key.".to_string(),
                places: Some(vec!["dhcp-queue-control.enable-queue".to_string(), "multi-threading.enable-multi-threading".to_string()]),
                links: Some(&[
                    "https://kea.readthedocs.io/en/latest/arm/congestion-handling.html#configuring-congestion-handling",
                ]),
            }]);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{
        super::_tests::NO_ENABLE_QUEUE_AND_MULTITHREADING_TOGETER_RULE_TEMPLATE,
        NoEnableQueueAndMultithreadingTogetherRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NO_ENABLE_QUEUE_AND_MULTITHREADING_TOGETER_RULE_TEMPLATE).unwrap();

        let rule = NoEnableQueueAndMultithreadingTogetherRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_ENABLE_QUEUE_AND_MULTITHREADING_TOGETER_RULE_TEMPLATE).unwrap();
        json_value["multi-threading"]["enable-multi-threading"] = Value::from(false);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NoEnableQueueAndMultithreadingTogetherRule;
        assert!(rule.check(&data).is_none());
    }
}
