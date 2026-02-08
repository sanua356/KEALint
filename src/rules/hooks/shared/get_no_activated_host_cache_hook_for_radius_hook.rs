use crate::{
    common::RuleResult,
    configs::hooks::KEAHookLibrary,
    constants::{HOST_CACHE_HOOK_LIBRARY, RADIUS_HOOK_LIBRARY},
};

pub fn get_no_activated_host_cache_hook_for_radius_hook(
    hooks_libraries: &Option<Vec<KEAHookLibrary>>,
) -> Option<Vec<RuleResult>> {
    let host_cache_hook = hooks_libraries
        .as_ref()?
        .iter()
        .find(|hook| hook.library.contains(HOST_CACHE_HOOK_LIBRARY));

    let radius_hook = hooks_libraries
        .as_ref()?
        .iter()
        .find(|hook| hook.library.contains(RADIUS_HOOK_LIBRARY));

    if radius_hook.is_some() && host_cache_hook.is_none() {
        return Some(vec![RuleResult {
            description: format!(
                "The '{}' hook requires activation of the '{}' hook to work correctly.",
                RADIUS_HOOK_LIBRARY, HOST_CACHE_HOOK_LIBRARY
            ),
            places: None,
            links: Some(&[
                "https://kea.readthedocs.io/en/stable/arm/integrations.html#radius-hook-library-configuration",
            ]),
        }]);
    }

    None
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::configs::v4::KEAv4Config;

    use super::{
        super::_tests::NO_ACTIVATED_HOST_CACHE_HOOK_FOR_RADIUS_HOOK_RULE_TEST_TEMPLATE,
        get_no_activated_host_cache_hook_for_radius_hook,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NO_ACTIVATED_HOST_CACHE_HOOK_FOR_RADIUS_HOOK_RULE_TEST_TEMPLATE)
                .unwrap();

        let rule = get_no_activated_host_cache_hook_for_radius_hook(&data.hooks_libraries);
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_ACTIVATED_HOST_CACHE_HOOK_FOR_RADIUS_HOOK_RULE_TEST_TEMPLATE)
                .unwrap();

        let hooks = serde_json::json!([{
            "library": "libdhcp_radius.so",
            "parameters": {
              "dictionary": "/etc/kea/radius/dictionary",
              "bindaddr": "*"
             }
           },
           {
                "library": "libdhcp_host_cache.so"
           }
        ]);
        json_value["hooks-libraries"] = hooks;
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = get_no_activated_host_cache_hook_for_radius_hook(&data.hooks_libraries);
        assert!(rule.is_none());
    }
}
