use crate::{common::RuleResult, configs::hooks::KEAHookLibrary, constants::USER_CHK_HOOK_LIBRARY};

pub fn get_use_user_check_hook_rule(
    hooks_libraries: &Option<Vec<KEAHookLibrary>>,
) -> Option<Vec<RuleResult>> {
    let (idx_hook, _) = hooks_libraries
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

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::configs::v4::KEAv4Config;

    use super::{
        super::_tests::USE_USER_CHECK_HOOK_RULE_TEST_TEMPLATE, get_use_user_check_hook_rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(USE_USER_CHECK_HOOK_RULE_TEST_TEMPLATE).unwrap();

        let rule = get_use_user_check_hook_rule(&data.hooks_libraries);
        assert!(rule.is_some());
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

        let rule = get_use_user_check_hook_rule(&data.hooks_libraries);
        assert!(rule.is_none());
    }
}
