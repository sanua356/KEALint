use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_not_lifetime_for_additional_classes_rule;

pub struct NotLifetimeForAdditionalClassesV6Rule;

impl Rule<KEAv6Config> for NotLifetimeForAdditionalClassesV6Rule {
    fn get_name(&self) -> &'static str {
        "CLIENT_CLASSES::NotValidLifetimeForAdditionalClassesRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }
    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        get_not_lifetime_for_additional_classes_rule(&config.client_classes)
    }
}

// The tests are written in a shared directory
