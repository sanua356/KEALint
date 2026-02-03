use std::vec;

use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
    constants::{
        FLEX_ID_HOOK_LIBRARY, FORENSIC_LOGGING_HOOK_LIBRARY, HIGH_AVAILABILITY_HOOK_LIBRARY,
        HOST_CACHE_HOOK_LIBRARY, LEASE_COMMANDS_HOOK_LIBRARY, PING_CHECK_HOOK_LIBRARY,
        RADIUS_HOOK_LIBRARY,
    },
};

pub struct BadHooksOrderRule;

impl Rule<KEAv4Config> for BadHooksOrderRule {
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
	                    places: Some(vec![format!("hooks-libraries.{}", index)]),
	                    links: Some(&["https://kea.readthedocs.io/en/latest/arm/hooks.html#order-of-configuration"]),
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
			            places: Some(vec![format!("hooks-libraries.{}", index)]),
			            links: Some(&["https://kea.readthedocs.io/en/latest/arm/hooks.html#order-of-configuration"]),
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
			            places: Some(vec![format!("hooks-libraries.{}", lease_index), format!("hooks-libraries.{}", ha_index)]),
			            links: Some(&["https://kea.readthedocs.io/en/latest/arm/hooks.html#load-balancing-configuration"]),
		            });
                }
            }

            let ping_check_hook_index = hooks
                .iter()
                .position(|hook| hook.library.contains(PING_CHECK_HOOK_LIBRARY));

            if let (Some(lease_index), Some(ping_check_index)) =
                (lease_commands_hook_index, ping_check_hook_index)
            {
                // Ping Check must be placed before Lease Commands (clause 16.15.12).
                if ping_check_index > lease_index {
                    results.push(RuleResult {
			            description: format!("For correct operation, the '{}' hook must be specified before the '{}' hook.", PING_CHECK_HOOK_LIBRARY, LEASE_COMMANDS_HOOK_LIBRARY),
			            places: Some(vec![format!("hooks-libraries.{}", lease_index), format!("hooks-libraries.{}", ping_check_index)]),
			            links: Some(&["https://kea.readthedocs.io/en/latest/arm/hooks.html#binding-variables"]),
		            });
                }
            }

            let host_cache_hook_index = hooks
                .iter()
                .position(|hook| hook.library.contains(HOST_CACHE_HOOK_LIBRARY));

            let radius_hook_index = hooks
                .iter()
                .position(|hook| hook.library.contains(RADIUS_HOOK_LIBRARY));

            if let (Some(host_cache_index), Some(radius_index)) =
                (host_cache_hook_index, radius_hook_index)
            {
                // Host Cache must be placed before RADIUS (clause 16.15.12).
                if host_cache_index > radius_index {
                    results.push(RuleResult {
				            description: format!("For correct operation, the '{}' hook must be specified before the '{}' hook.", HOST_CACHE_HOOK_LIBRARY, RADIUS_HOOK_LIBRARY),
				            places: Some(vec![format!("hooks-libraries.{}", host_cache_index), format!("hooks-libraries.{}", radius_index)]),
				            links: Some(&["https://kea.readthedocs.io/en/latest/arm/integrations.html#radius-hook-library-configuration"]),
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

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{super::_tests::BAD_HOOKS_ORDER_RULE_TEST_TEMPLATE, BadHooksOrderRule};

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(BAD_HOOKS_ORDER_RULE_TEST_TEMPLATE).unwrap();

        let rule = BadHooksOrderRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(BAD_HOOKS_ORDER_RULE_TEST_TEMPLATE).unwrap();
        json_value["hooks-libraries"] = serde_json::json!([
        {
            "library": "libdhcp_flex_id.so",
            "parameters": {
                "identifier-expression": "substring(relay4[0].option[18].hex,0,8)"
            }
        },
        {
            "library": "host_cache.so",
            "parameters": {}
        },
        {
            "library": "/usr/local/lib/kea/hooks/libdhcp_radius.so",
            "parameters": {
                "dictionary": "/etc/kea/radius/dictionary",
                "bindaddr": "*"
            }
        },
        {
            "library": "libdhcp_ping_check.so",
            "parameters": {
                "enable-ping-check": true,
                "min-ping-requests": 1,
                "reply-timeout": 100,
                "ping-cltt-secs": 60,
                "ping-channel-threads": 0
            }
        },
        {
            "library": "libdhcp_lease_cmds.so",
            "parameters": {}
        },
        {
            "library": "libdhcp_ha.so",
            "parameters": {
                "high-availability": [
                    {
                        "this-server-name": "server1",
                        "mode": "load-balancing",
                        "multi-threading": {
                            "enable-multi-threading": true,
                            "http-dedicated-listener": true,
                            "http-listener-threads": 4,
                            "http-client-threads": 4
                        },
                        "peers": [
                            {
                                "name": "server1",
                                "url": "http://192.168.56.33:8005/",
                                "role": "primary"
                            },
                            {
                                "name": "server2",
                                "url": "http://192.168.56.66:8005/",
                                "role": "secondary"
                            },
                            {
                                "name": "server3",
                                "url": "http://192.168.56.99:8005/",
                                "basic-auth-user": "foo",
                                "basic-auth-password": "1234",
                                "role": "backup"
                            }
                        ]
                    }
                ]
            }
        },
        {
            "library": "libdhcp_legal_log.so",
            "parameters": {
                "path": "/var/lib/kea/log",
                "base-name": "kea-forensic4"
            }
        }
        ]);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = BadHooksOrderRule;
        assert!(rule.check(&data).is_none());
    }
}
