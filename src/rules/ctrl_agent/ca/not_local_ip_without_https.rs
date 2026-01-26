use crate::{
    common::{RuleConfigs, RuleCtrlAgent, RuleLevels, RuleResult},
    configs::KEACtrlAgentConfig,
};

pub struct NotLocalIPWithoutHTTPSRule;

impl RuleCtrlAgent for NotLocalIPWithoutHTTPSRule {
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
            snapshot: None,
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
        common::RuleCtrlAgent, configs::KEACtrlAgentConfig,
        constants::TEMPLATE_CONFIG_FOR_TESTS_CTRL_AGENT,
        rules::ctrl_agent::NotLocalIPWithoutHTTPSRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEACtrlAgentConfig =
            serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_CTRL_AGENT).unwrap();

        let rule = NotLocalIPWithoutHTTPSRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_CTRL_AGENT).unwrap();
        json_value["http-host"] = Value::from("127.0.0.1");
        let data: KEACtrlAgentConfig = serde_json::from_value(json_value).unwrap();

        let rule = NotLocalIPWithoutHTTPSRule;
        assert!(rule.check(&data).is_none());
    }
}
