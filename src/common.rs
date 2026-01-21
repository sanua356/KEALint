#![allow(dead_code)]

use std::fmt::Display;

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

impl Display for RuleLevels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RuleLevels::Info => write!(f, "Info"),
            RuleLevels::Warning => write!(f, "Warning"),
            RuleLevels::Critical => write!(f, "Critical"),
        }
    }
}

impl Display for RuleConfigs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RuleConfigs::Dhcp4 => write!(f, "Dhcp4"),
            RuleConfigs::Dhcp6 => write!(f, "Dhcp6"),
            RuleConfigs::DhcpDDNS => write!(f, "DhcpDDNS"),
            RuleConfigs::ControlAgent => write!(f, "ControlAgent"),
        }
    }
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
