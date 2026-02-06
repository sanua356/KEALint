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
