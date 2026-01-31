use serde::Serialize;
use tabled::{
    Table, Tabled,
    settings::{Modify, Style, Width, object::Columns},
};

use crate::common::Rule;

#[derive(Debug, Serialize, Tabled)]
#[tabled(display(Option, "tabled::derive::display::option", ""))]
pub struct Problem {
    pub name: String,
    pub config_type: String,
    pub importance: String,
    pub description: String,
    pub places: Option<String>,
    pub links: Option<String>,
}

pub fn tabled_print_problems(problems: Vec<Problem>) -> String {
    let mut table = Table::new(problems);
    table.with(Style::modern());
    table.with(Modify::new(Columns::one(0)).with(Width::wrap(20)));
    table.with(Modify::new(Columns::one(3)).with(Width::wrap(20)));
    table.with(Modify::new(Columns::one(4)).with(Width::wrap(20)));
    table.with(Modify::new(Columns::one(5)).with(Width::wrap(20)));

    table.to_string()
}

pub fn find_problems<T>(config: &T, values: Vec<&[Box<dyn Rule<T>>]>) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();

    for rules_item in values {
        for rule in rules_item {
            let checks = rule.check(config);
            if let Some(check_items) = checks {
                for item in check_items {
                    problems.push(Problem {
                        name: rule.get_name().to_string(),
                        importance: rule.get_level().to_string(),
                        config_type: rule.get_config_type().to_string(),
                        description: item.description,
                        places: Some(item.places.unwrap_or_default().join("\n\n")),
                        links: Some(item.links.unwrap_or_default().join("\n\n")),
                    });
                }
            }
        }
    }

    problems
}
