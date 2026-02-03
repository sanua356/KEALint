use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAD2Config,
    constants::GSS_TSIG_HOOK_LIBRARY,
};

pub struct NoCredentialsCacheAndClientKeytabTogetherInGSSTSIGRule;

impl Rule<KEAD2Config> for NoCredentialsCacheAndClientKeytabTogetherInGSSTSIGRule {
    fn get_name(&self) -> &'static str {
        "HOOKS::NoCredentialsCacheAndKeytabTogetherInGSSTSIGRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::D2
    }
    fn check(&self, config: &KEAD2Config) -> Option<Vec<RuleResult>> {
        let (idx_hook, gss_tsig) = config
            .hooks_libraries
            .as_ref()?
            .iter()
            .enumerate()
            .find(|(_, hook)| hook.library.contains(GSS_TSIG_HOOK_LIBRARY))?;

        let parameters = gss_tsig.parameters.as_ref().unwrap_or_default();

        if let (Some(credentials_cache), Some(client_keytab)) = (
            parameters.get("credentials-cache"),
            parameters.get("client-keytab"),
        ) && (credentials_cache.as_str().unwrap_or("").chars().count() > 0
            && client_keytab.as_str().unwrap_or("").chars().count() > 0)
        {
            return Some(vec![RuleResult {
                description: format!(
                    "It is not recommended to specify both the 'credentials-cache' and 'client-keytab' parameters in the configuration of the '{}' hook. Use one of the two above.",
                    GSS_TSIG_HOOK_LIBRARY
                ),
                places: Some(vec![format!("hooks-libraries.{}", idx_hook)]),
                links: Some(&[
                    "https://kea.readthedocs.io/en/latest/arm/integrations.html#using-gss-tsig",
                ]),
            }]);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::KEAD2Config};

    use super::{
        super::_tests::NO_CREDENTIALS_CACHE_AND_CLIENT_KEYTAB_TOGETHER_IN_GSS_TSIG_RULE_TEST_TEMPLATE,
        NoCredentialsCacheAndClientKeytabTogetherInGSSTSIGRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAD2Config = serde_json::from_str(
            NO_CREDENTIALS_CACHE_AND_CLIENT_KEYTAB_TOGETHER_IN_GSS_TSIG_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let rule = NoCredentialsCacheAndClientKeytabTogetherInGSSTSIGRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(
            NO_CREDENTIALS_CACHE_AND_CLIENT_KEYTAB_TOGETHER_IN_GSS_TSIG_RULE_TEST_TEMPLATE,
        )
        .unwrap();
        let x = json_value["hooks-libraries"][0]["parameters"]
            .as_object_mut()
            .unwrap();

        x.remove("credentials-cache");

        let data: KEAD2Config = serde_json::from_value(json_value).unwrap();

        let rule = NoCredentialsCacheAndClientKeytabTogetherInGSSTSIGRule;
        assert!(rule.check(&data).is_none());
    }
}
