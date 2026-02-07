use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_no_enabled_persist_flag_rule;

pub struct NoEnabledPersistFlagForMemfileLeasesV6Rule;

impl Rule<KEAv6Config> for NoEnabledPersistFlagForMemfileLeasesV6Rule {
    fn get_name(&self) -> &'static str {
        "LEASE_DATABASE::NoEnabledPersistFlagForMemfileLeases"
    }

    fn get_level(&self) -> RuleLevels {
        RuleLevels::Critical
    }

    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }

    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        let flag = config.lease_database.persist.unwrap_or(false);
        let lease_database = &config.lease_database.r#type;

        get_no_enabled_persist_flag_rule(flag, lease_database)
    }
}

// The tests are written in a shared directory
