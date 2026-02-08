use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_no_basic_http_auth_in_ha_peers_rule;

pub struct NoBasicHTTPAuthInHAPeersV4Rule;

impl Rule<KEAv4Config> for NoBasicHTTPAuthInHAPeersV4Rule {
    fn get_name(&self) -> &'static str {
        "HOOKS::NoBasicHTTPAuthInHAPeersRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        get_no_basic_http_auth_in_ha_peers_rule(&config.hooks_libraries)
    }
}

// The tests are written in a shared directory
