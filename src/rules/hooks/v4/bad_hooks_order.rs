use std::vec;

use crate::{
    common::{RuleConfigs, RuleLevels, RuleResult, RuleV4},
    configs::KEAv4Config,
    constants::{
        FLEX_ID_HOOK_LIBRARY, FORENSIC_LOGGING_HOOK_LIBRARY, HIGH_AVAILABILITY_HOOK_LIBRARY,
        LEASE_COMMANDS_HOOK_LIBRARY, PING_CHECK_HOOK_LIBRARY,
    },
};

pub struct BadHooksOrderRule;

impl RuleV4 for BadHooksOrderRule {
    fn get_name(&self) -> &'static str {
        "HOOKS::BadHooksOrderRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let mut results: Vec<RuleResult> = Vec::new();

        if let Some(hooks) = &config.hooks_libraries {
            let flex_id_hook_index = hooks
                .iter()
                .position(|hook| hook.library.contains(FLEX_ID_HOOK_LIBRARY));

            if let Some(index) = flex_id_hook_index {
                //It is recommended to put the Flex ID hook at the top of the list (clause 16.3.1).
                if index != 0 {
                    results.push(RuleResult {
	                    description: format!("It is recommended to move the '{}' hook first in the list, otherwise the correct operation of the '{}' hook may be disrupted.", FLEX_ID_HOOK_LIBRARY, HIGH_AVAILABILITY_HOOK_LIBRARY),
	                    snapshot: Some(serde_json::to_string(&hooks[index]).unwrap()),
	                    links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/hooks.html#order-of-configuration"]),
                    });
                }
            }

            let forensic_logging_hook_index = hooks
                .iter()
                .position(|hook| hook.library.contains(FORENSIC_LOGGING_HOOK_LIBRARY));

            if let Some(index) = forensic_logging_hook_index {
                // We recommend putting Forensic Logging last in the list (clause 16.3.1).
                if index != hooks.len() - 1 {
                    results.push(RuleResult {
			            description: format!("It is recommended to move the '{}' hook last in the list to ensure that all higher-level hooks have already contributed to packet processing.", FORENSIC_LOGGING_HOOK_LIBRARY),
			            snapshot: Some(serde_json::to_string(&hooks[index]).unwrap()),
			            links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/hooks.html#order-of-configuration"]),
		            });
                }
            }

            let lease_commands_hook_index = hooks
                .iter()
                .position(|hook| hook.library.contains(LEASE_COMMANDS_HOOK_LIBRARY));

            let high_availability_hook_index = hooks
                .iter()
                .position(|hook| hook.library.contains(HIGH_AVAILABILITY_HOOK_LIBRARY));

            if let (Some(lease_index), Some(ha_index)) =
                (lease_commands_hook_index, high_availability_hook_index)
            {
                // Lease Commands must be in front of High Availability (clause 16.12.6).
                if lease_index > ha_index {
                    results.push(RuleResult {
			            description: format!("For correct operation, the '{}' hook must be specified before the '{}' hook.", LEASE_COMMANDS_HOOK_LIBRARY, HIGH_AVAILABILITY_HOOK_LIBRARY),
			            snapshot: Some(serde_json::to_string(&(&hooks[lease_index], &hooks[ha_index])).unwrap()),
			            links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/hooks.html#load-balancing-configuration"]),
		            });
                }
            }

            let ping_check_hook_index = hooks
                .iter()
                .position(|hook| hook.library.contains(PING_CHECK_HOOK_LIBRARY));

            if let (Some(lease_index), Some(ping_check_index)) =
                (lease_commands_hook_index, ping_check_hook_index)
            {
                // Ping Check must be placed before Lease Commands (clause 16.15.12)
                if ping_check_index > lease_index {
                    results.push(RuleResult {
			            description: format!("For correct operation, the '{}' hook must be specified before the '{}' hook.", PING_CHECK_HOOK_LIBRARY, LEASE_COMMANDS_HOOK_LIBRARY),
			            snapshot: Some(serde_json::to_string(&(&hooks[lease_index], &hooks[ping_check_index])).unwrap()),
			            links: Some(vec!["https://kea.readthedocs.io/en/kea-2.7.7/arm/hooks.html#binding-variables"]),
		            });
                }
            }
        }

        if !results.is_empty() {
            return Some(results);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        common::RuleV4, configs::v4::KEAv4Config, constants::TEMPLATE_CONFIG_FOR_TESTS_V4,
        rules::hooks::BadHooksOrderRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();

        let rule = BadHooksOrderRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();
        json_value["hooks-libraries"]
            .as_array_mut()
            .unwrap()
            .clear();
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = BadHooksOrderRule;
        assert!(rule.check(&data).is_none());
    }
}
