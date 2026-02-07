use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_not_select_iterative_allocator_for_shared_lease_database;

pub struct NotSelectIterativeAllocatorForSharedLeaseDatabaseV4Rule;

impl Rule<KEAv4Config> for NotSelectIterativeAllocatorForSharedLeaseDatabaseV4Rule {
    fn get_name(&self) -> &'static str {
        "ALLOCATOR::NotSelectIterativeAllocatorForSharedLeaseDatabaseRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_not_select_iterative_allocator_for_shared_lease_database(
            &config.allocator,
            &config.lease_database,
        )
    }
}

// The tests are written in a shared directory
