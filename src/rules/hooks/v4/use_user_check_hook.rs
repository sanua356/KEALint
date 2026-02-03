use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
    constants::USER_CHK_HOOK_LIBRARY,
};

pub struct UseUsrCheckHookRule;

impl Rule<KEAv4Config> for UseUsrCheckHookRule {
    fn get_name(&self) -> &'static str {
        "HOOKS::UseUsrCheckHookRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let (idx_hook, _) = config
            .hooks_libraries
            .as_ref()?
            .iter()
            .enumerate()
            .find(|(_, hook)| hook.library.contains(USER_CHK_HOOK_LIBRARY))?;

        Some(vec![RuleResult {
            description: format!(
                "The '{}' hook is outdated. It is recommended to use it only for educational purposes. Use the hosts reservations mechanisms of the global configuration instead of the hook.",
                USER_CHK_HOOK_LIBRARY
            ),
            places: Some(vec![format!("hooks-libraries.{}", idx_hook)]),
            links: Some(&[
                "https://kea.readthedocs.io/en/latest/arm/hooks.html#libdhcp-user-chk-so-user-check",
            ]),
        }])
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{super::_tests::USE_USER_CHECK_HOOK_RULE_TEST_TEMPLATE, UseUsrCheckHookRule};

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(USE_USER_CHECK_HOOK_RULE_TEST_TEMPLATE).unwrap();

        let rule = UseUsrCheckHookRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(USE_USER_CHECK_HOOK_RULE_TEST_TEMPLATE).unwrap();
        json_value["hooks-libraries"]
            .as_array_mut()
            .unwrap()
            .clear();
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = UseUsrCheckHookRule;
        assert!(rule.check(&data).is_none());
    }
}
