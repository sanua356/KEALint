use std::iter;

use crate::{
    common::RuleV4,
    configs::v4::KEAv4Config,
    rules::{
        interfaces::v4::NoInterfacesInInterfacesConfigRule,
        lease_database::v4::NoEnabledPersistFlagForMemfileLeases,
    },
};

pub mod interfaces;
pub mod lease_database;

pub struct RulesV4 {
    pub interfaces: Vec<Box<dyn RuleV4>>,
    pub lease_database: Vec<Box<dyn RuleV4>>,
}

impl RulesV4 {
    pub fn default() -> Self {
        RulesV4 {
            interfaces: vec![Box::new(NoInterfacesInInterfacesConfigRule)],
            lease_database: vec![Box::new(NoEnabledPersistFlagForMemfileLeases)],
        }
    }

    fn values(&self) -> impl Iterator<Item = &Vec<Box<dyn RuleV4>>> {
        let interfaces = iter::once(&self.interfaces);
        let lease_database = iter::once(&self.lease_database);

        interfaces.chain(lease_database)
    }

    pub fn run(&self, config: &KEAv4Config) {
        for rules_item in self.values() {
            for rule in rules_item {
                let checks = rule.check(&config);
                match checks {
                    Some(check_items) => {
                        for item in check_items {
                            println!(
                                "Rule: {}; Config: {:?}; Type: {:?}; Description: {}; Snapshot: {};",
                                rule.get_name(),
                                rule.get_config_type(),
                                rule.get_level(),
                                item.description,
                                item.snapshot.unwrap_or_default()
                            )
                        }
                    }
                    None => {}
                }
            }
        }
    }
}
