#![allow(non_camel_case_types)]
use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_not_recommended_prefix_AFTER__classes;

pub struct NotRecommendedPrefixAFTER_ClassesV4Rule;

impl Rule<KEAv4Config> for NotRecommendedPrefixAFTER_ClassesV4Rule {
    fn get_name(&self) -> &'static str {
        "CLIENT_CLASSES::NotRecommendedPrefixAFTER_ClassesRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Info
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_not_recommended_prefix_AFTER__classes(&config.client_classes)
    }
}

// The tests are written in a shared directory
