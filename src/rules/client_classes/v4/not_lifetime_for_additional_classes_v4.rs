use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_not_lifetime_for_additional_classes_rule;

pub struct NotLifetimeForAdditionalClassesV4Rule;

impl Rule<KEAv4Config> for NotLifetimeForAdditionalClassesV4Rule {
    fn get_name(&self) -> &'static str {
        "CLIENT_CLASSES::NotValidLifetimeForAdditionalClassesRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_not_lifetime_for_additional_classes_rule(&config.client_classes)
    }
}

// The tests are written in a shared directory
