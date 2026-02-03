use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAD2Config,
};

pub struct NotLocalIPAddressInD2ServerConfigRule;

impl Rule<KEAD2Config> for NotLocalIPAddressInD2ServerConfigRule {
    fn get_name(&self) -> &'static str {
        "DDNS_SERVER::NotLocalIPAddressInD2ServerConfigRule"
    }

    fn get_level(&self) -> RuleLevels {
        RuleLevels::Critical
    }

    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::D2
    }

    fn check(&self, config: &KEAD2Config) -> Option<Vec<RuleResult>> {
        if !config.ip_address.as_ref()?.is_loopback() {
            return Some(vec![RuleResult {
                description: "Loopback addresses must be used as the server address to avoid attacks with fake requests.".to_string(),
                places: Some(vec!["ip-address".to_string()]),
                links: Some(&["https://kea.readthedocs.io/en/latest/arm/ddns.html#global-server-parameters"]),
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
        super::_tests::NOT_LOCAL_IP_ADDRESS_IN_D2_SERVER_CONFIG_RULE_TEST_TEMPLATE,
        NotLocalIPAddressInD2ServerConfigRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAD2Config =
            serde_json::from_str(NOT_LOCAL_IP_ADDRESS_IN_D2_SERVER_CONFIG_RULE_TEST_TEMPLATE)
                .unwrap();

        let rule = NotLocalIPAddressInD2ServerConfigRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NOT_LOCAL_IP_ADDRESS_IN_D2_SERVER_CONFIG_RULE_TEST_TEMPLATE)
                .unwrap();
        json_value["ip-address"] = Value::from("127.0.0.1");
        let data: KEAD2Config = serde_json::from_value(json_value).unwrap();

        let rule = NotLocalIPAddressInD2ServerConfigRule;
        assert!(rule.check(&data).is_none());
    }
}
