use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEACtrlAgentConfig,
    tr,
};

pub struct NoAllControlSocketsSpecifiedRule;

fn get_template_rule(socket_type: &str) -> RuleResult {
    RuleResult {
        description: tr!("get_template_rule", socketType = socket_type),
        places: None,
        links: Some(&["https://kea.readthedocs.io/en/stable/arm/agent.html#configuration"]),
    }
}

impl Rule<KEACtrlAgentConfig> for NoAllControlSocketsSpecifiedRule {
    fn get_name(&self) -> &'static str {
        "CTRL_AGENT::NoAllControlSocketsSpecifiedRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::ControlAgent
    }
    fn check(&self, config: &KEACtrlAgentConfig) -> Option<Vec<RuleResult>> {
        let mut results: Vec<RuleResult> = Vec::new();

        if let Some(control_sockets) = &config.control_sockets {
            if control_sockets.dhcp4.is_none() {
                results.push(get_template_rule("dhcp4"));
            }

            if control_sockets.dhcp6.is_none() {
                results.push(get_template_rule("dhcp6"));
            }

            if control_sockets.d2.is_none() {
                results.push(get_template_rule("d2"));
            }
        } else {
            return Some(vec![RuleResult {
                description: tr!("no_all_control_sockets_specified_rule"),
                places: None,
                links: Some(&["https://kea.readthedocs.io/en/stable/arm/agent.html#configuration"]),
            }]);
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

    use crate::{common::Rule, configs::KEACtrlAgentConfig};

    use super::{
        super::_tests::NOT_ALL_CONTROL_SOCKETS_SPECIFIED_RULE_TEST_TEMPLATE,
        NoAllControlSocketsSpecifiedRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEACtrlAgentConfig =
            serde_json::from_str(NOT_ALL_CONTROL_SOCKETS_SPECIFIED_RULE_TEST_TEMPLATE).unwrap();

        let rule = NoAllControlSocketsSpecifiedRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NOT_ALL_CONTROL_SOCKETS_SPECIFIED_RULE_TEST_TEMPLATE).unwrap();
        json_value["control-sockets"]
            .as_object_mut()
            .unwrap()
            .insert(
                "d2".to_string(),
                serde_json::json!({
                    "socket-type": "unix",
                    "socket-name": "kea-ddns-ctrl-socket"
                }),
            );
        let data: KEACtrlAgentConfig = serde_json::from_value(json_value).unwrap();

        let rule = NoAllControlSocketsSpecifiedRule;
        assert!(rule.check(&data).is_none());
    }
}
