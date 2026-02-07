#![allow(non_camel_case_types)]
use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_not_recommended_prefix_AFTER__classes;

pub struct NotRecommendedPrefixAFTER_ClassesV6Rule;

impl Rule<KEAv6Config> for NotRecommendedPrefixAFTER_ClassesV6Rule {
    fn get_name(&self) -> &'static str {
        "CLIENT_CLASSES::NotRecommendedPrefixAFTER_ClassesRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_not_recommended_prefix_AFTER__classes(&config.client_classes)
    }
}

// The tests are written in a shared directory
