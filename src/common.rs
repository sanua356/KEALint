#![allow(dead_code)]

use super::configs::v4::KEAv4Config;

#[derive(Debug)]
pub enum RuleConfigs {
    Dhcp4,
    Dhcp6,
    DhcpDDNS,
    ControlAgent,
}

#[derive(Debug)]
pub enum RuleLevels {
    Info,
    Warning,
    Critical,
}

#[derive(Debug)]
pub struct RuleResult {
    pub description: String,
    pub snapshot: Option<String>,
}

pub trait RuleV4 {
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>>;

    fn get_name(&self) -> &'static str;

    fn get_level(&self) -> RuleLevels;

    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
}
