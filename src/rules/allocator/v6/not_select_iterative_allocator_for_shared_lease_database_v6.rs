use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_not_select_iterative_allocator_for_shared_lease_database;

pub struct NotSelectIterativeAllocatorForSharedLeaseDatabaseV6Rule;

impl Rule<KEAv6Config> for NotSelectIterativeAllocatorForSharedLeaseDatabaseV6Rule {
    fn get_name(&self) -> &'static str {
        "ALLOCATOR::NotSelectIterativeAllocatorForSharedLeaseDatabaseRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_not_select_iterative_allocator_for_shared_lease_database(
            &config.allocator,
            &config.lease_database,
        )
    }
}

// The tests are written in a shared directory
