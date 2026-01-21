use crate::{
    common::{RuleConfigs, RuleLevels, RuleResult, RuleV4},
    configs::v4::KEAv4Config,
    constants::HIGH_AVAILABILITY_HOOK_LIBRARY,
};

pub struct MultithreadingModesNotEqualInConfigAndHA;

impl RuleV4 for MultithreadingModesNotEqualInConfigAndHA {
    fn get_name(&self) -> &'static str {
        "HOOKS::MultithreadingModesNotEqualInConfigAndHA"
    }

    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }

    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }

    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        if config.hooks_libraries.is_none() || config.multi_threading.is_none() {
            return None;
        }
        // Unwrap safety, presence of the value is checked above.
        let is_multithreading_enabled: bool = config
            .multi_threading
            .as_ref()
            .unwrap()
            .enable_multi_threading
            .unwrap_or(true);

        // Unwrap safety, presence of the value is checked above.
        let ha_hook = config
            .hooks_libraries
            .as_ref()
            .unwrap()
            .iter()
            .find(|item| item.library.contains(HIGH_AVAILABILITY_HOOK_LIBRARY));

        match ha_hook {
            Some(hook_info) => {
                let parameters = hook_info.parameters.as_ref().unwrap_or_default();
                let is_hook_multithreading_enabled =
                    parameters["multi-threading"]["enable-multi-threading"]
                        .as_bool()
                        .unwrap_or(true);

                if is_multithreading_enabled != is_hook_multithreading_enabled {
                    return Some(vec![RuleResult {
                        description: "The multithreading control flags in the global server configuration and the high availability hook configuration are not equal.".to_string(),
                        snapshot: None,
                        links: Some(vec![
							"https://kea.readthedocs.io/en/kea-3.0.0/arm/dhcp6-srv.html#multi-threading-settings".to_string(),
							"https://kea.readthedocs.io/en/kea-3.0.0/arm/hooks.html#multi-threaded-configuration-ha-mt".to_string()
						])
                    }]);
                }
            }
            None => {
                return None;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        common::RuleV4, configs::v4::KEAv4Config, constants::TEMPLATE_CONFIG_FOR_TESTS_V4,
        rules::hooks::v4::multithread_modes_not_equal::MultithreadingModesNotEqualInConfigAndHA,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();

        let rule = MultithreadingModesNotEqualInConfigAndHA;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();
        json_value["multi-threading"]
            .as_object_mut()
            .unwrap()
            .insert("enable-multi-threading".to_string(), Value::from(true));
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = MultithreadingModesNotEqualInConfigAndHA;
        assert!(rule.check(&data).is_none());
    }
}
