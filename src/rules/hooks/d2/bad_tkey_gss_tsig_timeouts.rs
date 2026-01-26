use serde_json::Value;

use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAD2Config,
    constants::GSS_TSIG_HOOK_LIBRARY,
};

pub struct BadTKeyGSSTSIGHookTimeoutsRule;

fn check_rekey_percent(
    tkey_lifetime: Option<i64>,
    rekey_interval: Option<i64>,
    parameters: &Value,
) -> Option<RuleResult> {
    if let (Some(tkey), Some(rekey)) = (tkey_lifetime, rekey_interval) {
        let rekey_percent = rekey * 100 / tkey;

        if !(50..=80).contains(&rekey_percent) {
            return Some(RuleResult {
                description: "The value of the 'rekey-interval' parameter in the configuration of the 'GSS-TSIG' hook is recommended to be set in the range of 50-80% of the value of the 'tkey-lifetime' parameter".to_string(),
                snapshot: Some(serde_json::to_string(parameters).unwrap()),
                links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/integrations.html#using-gss-tsig"]),
            });
        }
    }

    None
}

fn check_retry_interval(
    tkey_lifetime: Option<i64>,
    rekey_interval: Option<i64>,
    retry_interval: Option<i64>,
    parameters: &Value,
) -> Option<RuleResult> {
    if let (Some(tkey), Some(rekey), Some(retry)) = (tkey_lifetime, rekey_interval, retry_interval)
        && retry > ((tkey - rekey) / 3)
    {
        return Some(RuleResult {
		       description: "The value of the 'retry-interval' parameter in the configuration of the 'GSS-TSIG' hook is recommended to be set no more than 1/3 of the difference between the values of the 'tkey-lifetime' and 'rekey-interval' parameters.".to_string(),
		       snapshot: Some(serde_json::to_string(parameters).unwrap()),
		       links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/integrations.html#using-gss-tsig"]),
		   });
    }

    None
}

impl Rule<KEAD2Config> for BadTKeyGSSTSIGHookTimeoutsRule {
    fn get_name(&self) -> &'static str {
        "HOOKS::BadTKeyGSSTSIGHookTimeoutsRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::D2
    }
    fn check(&self, config: &KEAD2Config) -> Option<Vec<RuleResult>> {
        config.hooks_libraries.as_ref()?;

        let gss_tsig_hook = config
            .hooks_libraries
            .as_ref()
            .unwrap()
            .iter()
            .find(|hook| hook.library.contains(GSS_TSIG_HOOK_LIBRARY));

        let mut results: Vec<RuleResult> = Vec::new();

        if let Some(hook) = gss_tsig_hook {
            let parameters = hook.parameters.as_ref().unwrap_or_default();

            let tkey_lifetime = parameters["tkey-lifetime"].as_i64();
            let rekey_interval = parameters["rekey-interval"].as_i64();
            let retry_interval = parameters["retry-interval"].as_i64();

            if let Some(rule) = check_rekey_percent(tkey_lifetime, rekey_interval, parameters) {
                results.push(rule);
            }

            if let Some(rule) =
                check_retry_interval(tkey_lifetime, rekey_interval, retry_interval, parameters)
            {
                results.push(rule);
            }

            if let Some(servers) = parameters["servers"].as_array() {
                for server in servers {
                    let server_tkey_lifetime = server["tkey-lifetime"].as_i64();
                    let server_rekey_interval = server["rekey-interval"].as_i64();
                    let server_retry_interval = server["retry-interval"].as_i64();

                    if let Some(rule) =
                        check_rekey_percent(server_tkey_lifetime, server_rekey_interval, server)
                    {
                        results.push(rule);
                    }

                    if let Some(rule) = check_retry_interval(
                        server_tkey_lifetime,
                        server_rekey_interval,
                        server_retry_interval,
                        server,
                    ) {
                        results.push(rule);
                    }
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

    use crate::{
        common::Rule, configs::KEAD2Config, constants::TEMPLATE_CONFIG_FOR_TESTS_D2,
        rules::hooks::BadTKeyGSSTSIGHookTimeoutsRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAD2Config = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_D2).unwrap();

        let rule = BadTKeyGSSTSIGHookTimeoutsRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_D2).unwrap();
        let x = json_value["hooks-libraries"][0]["parameters"]
            .as_object_mut()
            .unwrap();

        x["tkey-lifetime"] = Value::from(3600);
        x["rekey-interval"] = Value::from(2700);
        x["retry-interval"] = Value::from(120);

        let server = x["servers"][0].as_object_mut().unwrap();

        server["tkey-lifetime"] = Value::from(7200);
        server["rekey-interval"] = Value::from(5400);
        server["retry-interval"] = Value::from(240);

        let data: KEAD2Config = serde_json::from_value(json_value).unwrap();

        let rule = BadTKeyGSSTSIGHookTimeoutsRule;
        assert!(rule.check(&data).is_none());
    }
}
