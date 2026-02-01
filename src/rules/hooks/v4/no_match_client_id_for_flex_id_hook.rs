use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
    constants::FLEX_ID_HOOK_LIBRARY,
};

pub struct NoMatchClientIdForFlexIDHookRule;

impl Rule<KEAv4Config> for NoMatchClientIdForFlexIDHookRule {
    fn get_name(&self) -> &'static str {
        "HOOKS::NoMatchClientIdForFlexIDHookRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let (idx_hook, flex_id_hook) = &config
            .hooks_libraries
            .as_ref()?
            .iter()
            .enumerate()
            .find(|(_, hook)| hook.library.contains(FLEX_ID_HOOK_LIBRARY))?;

        let match_client_id = &config.match_client_id.unwrap_or_default();

        let parameters = flex_id_hook.parameters.as_ref()?.as_object()?;
        let replace_client_id = parameters["replace-client-id"]
            .as_bool()
            .unwrap_or_default();

        if replace_client_id && !match_client_id {
            return Some(vec![RuleResult {
                description: format!(
                    "The 'replace-client-id' parameter is set to 'true' inside the '{}' hook. For it to work, the 'match-client-id' parameter must be set to 'true' in the global configuration.",
                    FLEX_ID_HOOK_LIBRARY
                ),
                places: Some(vec![
                    "match-client-id".to_string(),
                    format!("hooks-libraries.{}.parameters.replace-client-id", idx_hook),
                ]),
                links: Some(vec![
                    "https://kea.readthedocs.io/en/latest/arm/hooks.html#the-replace-client-id-flag",
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
        super::_tests::NO_MATCH_CLIENT_ID_FOR_FLEX_ID_HOOK_RULE_TEST_TEMPLATE,
        NoMatchClientIdForFlexIDHookRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NO_MATCH_CLIENT_ID_FOR_FLEX_ID_HOOK_RULE_TEST_TEMPLATE).unwrap();

        let rule = NoMatchClientIdForFlexIDHookRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_MATCH_CLIENT_ID_FOR_FLEX_ID_HOOK_RULE_TEST_TEMPLATE).unwrap();
        json_value["match-client-id"] = Value::from(true);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NoMatchClientIdForFlexIDHookRule;
        assert!(rule.check(&data).is_none());
    }
}
