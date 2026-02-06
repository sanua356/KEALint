use crate::common::{Rule, RuleConfigs, RuleLevels, RuleResult};
use crate::configs::KEAv6Config;

use super::super::shared::get_no_active_interfaces;

pub struct NoInterfacesInInterfacesConfigV6Rule;

impl Rule<KEAv6Config> for NoInterfacesInInterfacesConfigV6Rule {
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }

    fn get_name(&self) -> &'static str {
        "INTERFACES::NoInterfacesInInterfacesConfigRule"
    }

    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }

    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_no_active_interfaces(&config.interfaces_config)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v6::KEAv6Config};

    use super::{
        super::_tests::NO_INTERFACES_IN_INTERFACES_CONFIG_RULE_TEST_TEMPLATE,
        NoInterfacesInInterfacesConfigV6Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv6Config =
            serde_json::from_str(NO_INTERFACES_IN_INTERFACES_CONFIG_RULE_TEST_TEMPLATE).unwrap();

        let rule = NoInterfacesInInterfacesConfigV6Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_INTERFACES_IN_INTERFACES_CONFIG_RULE_TEST_TEMPLATE).unwrap();
        json_value["interfaces-config"]
            .as_object_mut()
            .unwrap()
            .insert("interfaces".to_string(), Value::from(["eth0"]));
        let data: KEAv6Config = serde_json::from_value(json_value).unwrap();

        let rule = NoInterfacesInInterfacesConfigV6Rule;
        assert!(rule.check(&data).is_none());
    }
}
