use crate::{
    common::RuleResult, configs::hooks::KEAHookLibrary, constants::HIGH_AVAILABILITY_HOOK_LIBRARY,
};

pub fn get_no_basic_http_auth_in_ha_peers_rule(
    hooks_libraries: &Option<Vec<KEAHookLibrary>>,
) -> Option<Vec<RuleResult>> {
    let mut results: Vec<RuleResult> = Vec::new();

    if let Some(hooks) = hooks_libraries {
        let (idx_hook, ha_hook) = hooks
            .iter()
            .enumerate()
            .find(|(_, hook)| hook.library.contains(HIGH_AVAILABILITY_HOOK_LIBRARY))?;

        if let Some(parameters) = &ha_hook.parameters
            && let Some(builds) = parameters["high-availability"].as_array()
        {
            for (idx_ha, ha) in builds.iter().enumerate() {
                if let Some(peers) = ha["peers"].as_array() {
                    for (idx_peer, peer) in peers.iter().enumerate() {
                        if peer.get("basic-auth-user").is_none()
                            && peer.get("basic-auth-password").is_none()
                        {
                            results.push(RuleResult {
                                description: format!("The peer named '{}' of the high availability hook lacks basic HTTP authentication.", peer["name"].as_str().unwrap()),
                                places: Some(vec![format!("hooks-libraries.{}.parameters.high-availability.{}.peers.{}", idx_hook, idx_ha, idx_peer)]),
                                links: Some(&["https://kea.readthedocs.io/en/latest/arm/hooks.html#hot-standby-configuration"]),
                            });
                        }
                    }
                }
            }
        }
    }

    if !results.is_empty() {
        return Some(results);
    }

    None
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::configs::v4::KEAv4Config;

    use super::{
        super::_tests::NO_BASIC_HTTP_AUTH_IN_HA_PEERS_RULE_TEST_TEMPLATE,
        get_no_basic_http_auth_in_ha_peers_rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NO_BASIC_HTTP_AUTH_IN_HA_PEERS_RULE_TEST_TEMPLATE).unwrap();

        let rule = get_no_basic_http_auth_in_ha_peers_rule(&data.hooks_libraries);
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_BASIC_HTTP_AUTH_IN_HA_PEERS_RULE_TEST_TEMPLATE).unwrap();
        json_value["hooks-libraries"][0]["parameters"]["high-availability"][0]["peers"] = serde_json::json!([
        {
            "name": "server2",
            "url": "http://1.2.3.4:8005/",
            "basic-auth-user": "qqq",
            "basic-auth-password": "eee",
            "role": "backup"
        },
        {
            "name": "server3",
            "url": "http://192.168.56.99:8005/",
            "basic-auth-user": "foo",
            "basic-auth-password": "1234",
            "role": "backup"
        }
        ]);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = get_no_basic_http_auth_in_ha_peers_rule(&data.hooks_libraries);
        assert!(rule.is_none());
    }
}
