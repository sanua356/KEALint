use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
    constants::{HOST_CACHE_HOOK_LIBRARY, RADIUS_HOOK_LIBRARY},
};

pub struct NoActivatedHostCacheHookForRADIUSHookRule;

impl Rule<KEAv4Config> for NoActivatedHostCacheHookForRADIUSHookRule {
    fn get_name(&self) -> &'static str {
        "HOOKS::NoActivatedHostCacheHookForRADIUSHookRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let host_cache_hook = &config
            .hooks_libraries
            .as_ref()?
            .iter()
            .find(|hook| hook.library.contains(HOST_CACHE_HOOK_LIBRARY));

        let radius_hook = &config
            .hooks_libraries
            .as_ref()?
            .iter()
            .find(|hook| hook.library.contains(RADIUS_HOOK_LIBRARY));

        if radius_hook.is_some() && host_cache_hook.is_none() {
            return Some(vec![RuleResult {
                description: format!(
                    "The '{}' hook requires activation of the '{}' hook to work correctly.",
                    RADIUS_HOOK_LIBRARY, HOST_CACHE_HOOK_LIBRARY
                ),
                snapshot: None,
                links: Some(vec![
                    "https://kea.readthedocs.io/en/stable/arm/integrations.html#radius-hook-library-configuration",
                ]),
            }]);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        common::Rule,
        configs::v4::KEAv4Config,
        rules::hooks::v4::{
            _tests::NO_ACTIVATED_HOST_CACHE_HOOK_FOR_RADIUS_HOOK_RULE_TEST_TEMPLATE,
            no_activated_host_cache_hook_for_radius_hook::NoActivatedHostCacheHookForRADIUSHookRule,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NO_ACTIVATED_HOST_CACHE_HOOK_FOR_RADIUS_HOOK_RULE_TEST_TEMPLATE)
                .unwrap();

        let rule = NoActivatedHostCacheHookForRADIUSHookRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_ACTIVATED_HOST_CACHE_HOOK_FOR_RADIUS_HOOK_RULE_TEST_TEMPLATE)
                .unwrap();

        let hooks = serde_json::json!([{
            "library": "/usr/local/lib/kea/hooks/libdhcp_radius.so",
            "parameters": {

              "dictionary": "/etc/kea/radius/dictionary",

              "bindaddr": "*"
             }
           },
           {
                 "library": "/usr/local/lib/kea/hooks/libdhcp_host_cache.so"
        }]);
        json_value["hooks-libraries"] = hooks;
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NoActivatedHostCacheHookForRADIUSHookRule;
        assert!(rule.check(&data).is_none());
    }
}
