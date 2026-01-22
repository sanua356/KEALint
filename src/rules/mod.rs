use std::iter;

use tabled::{
    Table, Tabled,
    settings::{Modify, Style, Width, object::Columns},
};

use crate::{
    common::RuleV4,
    configs::v4::KEAv4Config,
    rules::{
        hooks::{
            MultithreadingModesNotEqualInConfigAndHARule, UnnecessaryActivatedDatabaseHooksRule,
        },
        interfaces::NoInterfacesInInterfacesConfigRule,
        lease_database::NoEnabledPersistFlagForMemfileLeasesRule,
    },
};

pub mod hooks;
pub mod interfaces;
pub mod lease_database;

pub struct RulesV4 {
    pub interfaces: Vec<Box<dyn RuleV4>>,
    pub lease_database: Vec<Box<dyn RuleV4>>,
    pub hooks: Vec<Box<dyn RuleV4>>,
}

#[derive(Tabled)]
#[tabled(display(Option, "tabled::derive::display::option", ""))]
struct Problem {
    name: String,
    config_type: String,
    importance: String,
    description: String,
    snapshot: Option<String>,
    links: Option<String>,
}

impl RulesV4 {
    pub fn default() -> Self {
        RulesV4 {
            interfaces: vec![Box::new(NoInterfacesInInterfacesConfigRule)],
            lease_database: vec![Box::new(NoEnabledPersistFlagForMemfileLeasesRule)],
            hooks: vec![
                Box::new(MultithreadingModesNotEqualInConfigAndHARule),
                Box::new(UnnecessaryActivatedDatabaseHooksRule),
            ],
        }
    }

    fn values(&self) -> impl Iterator<Item = &Vec<Box<dyn RuleV4>>> {
        let interfaces = iter::once(&self.interfaces);
        let lease_database = iter::once(&self.lease_database);
        let hooks = iter::once(&self.hooks);

        interfaces.chain(lease_database).chain(hooks)
    }

    pub fn run(&self, config: &KEAv4Config) {
        let mut problems: Vec<Problem> = Vec::new();

        for rules_item in self.values() {
            for rule in rules_item {
                let checks = rule.check(config);
                if let Some(check_items) = checks {
                    for item in check_items {
                        problems.push(Problem {
                            name: rule.get_name().to_string(),
                            importance: rule.get_level().to_string(),
                            config_type: rule.get_config_type().to_string(),
                            description: item.description,
                            snapshot: item.snapshot,
                            links: Some(item.links.unwrap_or_default().join("\n\n")),
                        });
                    }
                }
            }
        }

        let mut table = Table::new(problems);
        table.with(Style::modern());
        table.with(Modify::new(Columns::one(0)).with(Width::wrap(20)));
        table.with(Modify::new(Columns::one(3)).with(Width::wrap(20)));
        table.with(Modify::new(Columns::one(4)).with(Width::wrap(20)));
        table.with(Modify::new(Columns::one(5)).with(Width::wrap(20)));
        println!("{}", table);

        println!("{} problem(s) found.", &table.count_rows() - 1);
    }
}
