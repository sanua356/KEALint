use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEACtrlAgentConfig,
};

pub struct NotLocalIPWithoutHTTPSRule;

impl Rule<KEACtrlAgentConfig> for NotLocalIPWithoutHTTPSRule {
    fn get_name(&self) -> &'static str {
        "CTRL_AGENT::NotLocalIPWithoutHTTPSRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::ControlAgent
    }
    fn check(&self, config: &KEACtrlAgentConfig) -> Option<Vec<RuleResult>> {
        if let Some(host_ip) = &config.http_host
            && !host_ip.is_loopback()
            && (config.cert_file.is_none()
                && config.key_file.is_none()
                && config.trust_anchor.is_none())
        {
            return Some(vec![RuleResult {
            description: "The configuration specifies the 'http-port' key in a value that is not a local IP address, but HTTPS support is not enabled.".to_string(),
            places: Some(vec!["http-host".to_string()]),
            links: Some(vec![
                "https://kea.readthedocs.io/en/latest/arm/security.html#tls-https-configuration",
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
        configs::KEACtrlAgentConfig,
        rules::ctrl_agent::{
            NotLocalIPWithoutHTTPSRule, ca::_tests::NOT_LOCAL_IP_WITHOUT_HTTPS_RULE_TEST_TEMPLATE,
        },
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEACtrlAgentConfig =
            serde_json::from_str(NOT_LOCAL_IP_WITHOUT_HTTPS_RULE_TEST_TEMPLATE).unwrap();

        let rule = NotLocalIPWithoutHTTPSRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NOT_LOCAL_IP_WITHOUT_HTTPS_RULE_TEST_TEMPLATE).unwrap();
        json_value["http-host"] = Value::from("127.0.0.1");
        let data: KEACtrlAgentConfig = serde_json::from_value(json_value).unwrap();

        let rule = NotLocalIPWithoutHTTPSRule;
        assert!(rule.check(&data).is_none());
    }
}
