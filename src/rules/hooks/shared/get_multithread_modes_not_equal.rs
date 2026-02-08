use crate::{
    common::RuleResult,
    configs::{hooks::KEAHookLibrary, multithreading::KEAMultithreading},
    constants::HIGH_AVAILABILITY_HOOK_LIBRARY,
};

pub fn get_multithread_modes_not_equal_rule(
    multi_threading: &Option<KEAMultithreading>,
    hooks_libraries: &Option<Vec<KEAHookLibrary>>,
) -> Option<Vec<RuleResult>> {
    let is_multithreading_enabled: bool = multi_threading
        .as_ref()?
        .enable_multi_threading
        .unwrap_or(true);

    let (idx_hook, ha_hook) = hooks_libraries
        .as_ref()?
        .iter()
        .enumerate()
        .find(|(_, hook)| hook.library.contains(HIGH_AVAILABILITY_HOOK_LIBRARY))?;

    let parameters = ha_hook.parameters.as_ref().unwrap_or_default();
    let is_hook_multithreading_enabled =
        parameters["high-availability"][0]["multi-threading"]["enable-multi-threading"]
            .as_bool()
            .unwrap_or(true);

    if is_multithreading_enabled != is_hook_multithreading_enabled {
        return Some(vec![RuleResult {
                description: "The multithreading control flags in the global server configuration and the high availability hook configuration are not equal.".to_string(),
                places: Some(vec!["multi-threading".to_string(), format!("hooks-libraries.{}.parameters.high-availability.0.multi-threading", idx_hook)]),
                links: Some(&[
					"https://kea.readthedocs.io/en/latest/arm/dhcp6-srv.html#multi-threading-settings",
					"https://kea.readthedocs.io/en/latest/arm/hooks.html#multi-threaded-configuration-ha-mt"
				])
            }]
        );
    }

    None
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::configs::v4::KEAv4Config;

    use super::{
        super::_tests::MULTITHREADING_MODES_NOT_EQUAL_IN_CONFIG_AND_HA_RULE_TEST_TEMPLATE,
        get_multithread_modes_not_equal_rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            MULTITHREADING_MODES_NOT_EQUAL_IN_CONFIG_AND_HA_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule =
            get_multithread_modes_not_equal_rule(&data.multi_threading, &data.hooks_libraries);
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(
            MULTITHREADING_MODES_NOT_EQUAL_IN_CONFIG_AND_HA_RULE_TEST_TEMPLATE,
        )
        .unwrap();
        json_value["multi-threading"]
            .as_object_mut()
            .unwrap()
            .insert("enable-multi-threading".to_string(), Value::from(true));
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule =
            get_multithread_modes_not_equal_rule(&data.multi_threading, &data.hooks_libraries);
        assert!(rule.is_none());
    }
}
