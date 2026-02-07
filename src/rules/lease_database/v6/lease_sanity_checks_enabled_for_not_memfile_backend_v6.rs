use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_lease_sanity_checks_enabled_for_not_memfile_backend_rule;

pub struct LeaseSanityChecksEnabledForNotMemfileBackendV6Rule;

impl Rule<KEAv6Config> for LeaseSanityChecksEnabledForNotMemfileBackendV6Rule {
    fn get_name(&self) -> &'static str {
        "LEASE_DATABASE::LeaseSanityChecksEnabledForNotMemfileBackend"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_lease_sanity_checks_enabled_for_not_memfile_backend_rule(
            &config.sanity_checks,
            &config.lease_database,
        )
    }
}

// The tests are written in a shared directory
