use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_not_ddns_qualifying_suffix_with_enabled_ddns_updates_rule;

pub struct NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesV4Rule;

impl Rule<KEAv4Config> for NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesV4Rule {
    fn get_name(&self) -> &'static str {
        "DDNS_SERVER::NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let is_enabled_ddns = config.ddns_send_updates?;
        let ddns_qualifying_suffix = config.ddns_qualifying_suffix.clone().unwrap_or_default();

        get_not_ddns_qualifying_suffix_with_enabled_ddns_updates_rule(
            is_enabled_ddns,
            ddns_qualifying_suffix,
        )
    }
}

// The tests are written in a shared directory
