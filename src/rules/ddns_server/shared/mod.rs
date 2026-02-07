use crate::common::RuleResult;

pub fn get_not_ddns_qualifying_suffix_with_enabled_ddns_updates_rule(
    is_enabled_ddns: bool,
    ddns_qualifying_suffix: String,
) -> Option<Vec<RuleResult>> {
    if is_enabled_ddns && ddns_qualifying_suffix.is_empty() {
        return Some(vec![RuleResult {
            description: "It is recommended to specify the value for the 'ddns-qualifying-suffix' field when enabling DDNS updates.".to_string(),
            links: Some(&["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#kea-dhcp4-name-generation-for-ddns-update-requests"]),
            places: Some(vec!["ddns-send-updates".to_string(), "ddns-qualifying-suffix".to_string()]),
        }]);
    }

    None
}

#[cfg(test)]
pub mod _tests;

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::configs::v4::KEAv4Config;

    use super::{
        _tests::NOT_DDNS_QUALIFYING_SUFFIX_WITH_ENABLED_DDNS_UPDATES_RULE_TEST_TEMPLATE,
        get_not_ddns_qualifying_suffix_with_enabled_ddns_updates_rule,
    };

    #[test]
    fn check_expected_not_ddns_qualifying_suffix_with_enabled_ddns_updates_trigger() {
        let data: KEAv4Config = serde_json::from_str(
            NOT_DDNS_QUALIFYING_SUFFIX_WITH_ENABLED_DDNS_UPDATES_RULE_TEST_TEMPLATE,
        )
        .unwrap();

        let is_enabled_ddns = data.ddns_send_updates.unwrap_or_default();
        let ddns_qualifying_suffix = data.ddns_qualifying_suffix.clone().unwrap_or_default();

        let rule = get_not_ddns_qualifying_suffix_with_enabled_ddns_updates_rule(
            is_enabled_ddns,
            ddns_qualifying_suffix,
        );
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_not_ddns_qualifying_suffix_with_enabled_ddns_updates_trigger() {
        let mut json_value: Value = serde_json::from_str(
            NOT_DDNS_QUALIFYING_SUFFIX_WITH_ENABLED_DDNS_UPDATES_RULE_TEST_TEMPLATE,
        )
        .unwrap();
        json_value["ddns-qualifying-suffix"] = Value::from("aa.bb.cc");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let is_enabled_ddns = data.ddns_send_updates.unwrap_or_default();
        let ddns_qualifying_suffix = data.ddns_qualifying_suffix.clone().unwrap_or_default();

        let rule = get_not_ddns_qualifying_suffix_with_enabled_ddns_updates_rule(
            is_enabled_ddns,
            ddns_qualifying_suffix,
        );
        assert!(rule.is_none());
    }
}
