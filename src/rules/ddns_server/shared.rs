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
