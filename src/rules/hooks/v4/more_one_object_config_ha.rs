use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
    constants::HIGH_AVAILABILITY_HOOK_LIBRARY,
};

pub struct MoreOneObjectConfigHARule;

impl Rule<KEAv4Config> for MoreOneObjectConfigHARule {
    fn get_name(&self) -> &'static str {
        "HOOKS::MoreOneObjectConfigHARule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let (idx_hook, hook) = config
            .hooks_libraries
            .as_ref()?
            .iter()
            .enumerate()
            .find(|(_, hook)| hook.library.contains(HIGH_AVAILABILITY_HOOK_LIBRARY))?;

        let parameters = hook.parameters.as_ref()?.as_object()?;

        if parameters["high-availability"].as_array()?.len() > 1 {
            return Some(vec![RuleResult {
                description: format!(
                    "For the hook '{}', the 'high-availability' key cannot contain more than one object in the array.",
                    HIGH_AVAILABILITY_HOOK_LIBRARY
                ),
                links: Some(vec![
                    "https://kea.readthedocs.io/en/latest/arm/hooks.html#load-balancing-configuration",
                ]),
                places: Some(vec![format!(
                    "hooks-libraries.{}.parameters.high-availability",
                    idx_hook
                )]),
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
        rules::hooks::{
            MoreOneObjectConfigHARule, v4::_tests::MORE_ONE_OBJECT_CONFIG_HA_RULE_TEST_TEMPLATE,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(MORE_ONE_OBJECT_CONFIG_HA_RULE_TEST_TEMPLATE).unwrap();

        let rule = MoreOneObjectConfigHARule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(MORE_ONE_OBJECT_CONFIG_HA_RULE_TEST_TEMPLATE).unwrap();
        json_value["hooks-libraries"] = serde_json::json!([
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
        }]);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = MoreOneObjectConfigHARule;
        assert!(rule.check(&data).is_none());
    }
}
