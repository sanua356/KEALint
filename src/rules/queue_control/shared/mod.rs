use crate::{
    common::RuleResult,
    configs::{dhcp_queue_control::KEADhcpQueueControl, multithreading::KEAMultithreading},
};

pub fn get_no_enable_queue_and_multithreading_together_rule(
    multi_threading: &Option<KEAMultithreading>,
    dhcp_queue_control: &Option<KEADhcpQueueControl>,
) -> Option<Vec<RuleResult>> {
    let is_enabled_multithreading = multi_threading.as_ref()?.enable_multi_threading?;
    let is_enabled_queue_control = dhcp_queue_control.as_ref()?.enable_queue?;

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

#[cfg(test)]
pub mod _tests;

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::configs::v4::KEAv4Config;

    use super::{
        _tests::NO_ENABLE_QUEUE_AND_MULTITHREADING_TOGETER_RULE_TEMPLATE,
        get_no_enable_queue_and_multithreading_together_rule,
    };

    #[test]
    fn check_expected_no_enable_queue_and_multithreading_together_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NO_ENABLE_QUEUE_AND_MULTITHREADING_TOGETER_RULE_TEMPLATE).unwrap();

        let rule = get_no_enable_queue_and_multithreading_together_rule(
            &data.multi_threading,
            &data.dhcp_queue_control,
        );
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_no_enable_queue_and_multithreading_together_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_ENABLE_QUEUE_AND_MULTITHREADING_TOGETER_RULE_TEMPLATE).unwrap();
        json_value["multi-threading"]["enable-multi-threading"] = Value::from(false);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = get_no_enable_queue_and_multithreading_together_rule(
            &data.multi_threading,
            &data.dhcp_queue_control,
        );
        assert!(rule.is_none());
    }
}
