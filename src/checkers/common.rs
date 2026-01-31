use std::collections::BTreeMap;

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

    #[tabled(display("display_vec"))]
    pub places: Option<Vec<String>>,

    #[tabled(display("display_vec"))]
    pub links: Option<Vec<&'static str>>,
}

fn display_vec<T: std::fmt::Display>(v: &Option<Vec<T>>) -> String {
    match v {
        Some(vec) => vec
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n\n "),
        None => String::new(),
    }
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

pub fn get_summary_problems(problems: &Vec<Problem>) -> String {
    let mut summary = format!("Found {} problem(s).\n\n", problems.len());

    let mut problem_config_types: BTreeMap<String, u32> = BTreeMap::new();
    let mut problem_importances: BTreeMap<String, u32> = BTreeMap::new();

    for problem in problems {
        *problem_config_types
            .entry(problem.config_type.clone())
            .or_insert(0) += 1;

        *problem_importances
            .entry(problem.importance.clone())
            .or_insert(0) += 1;
    }

    summary = format!("{}\nBy type config:\n", summary);

    for problem_config_type in problem_config_types.iter() {
        summary = format!(
            "{}{} = {} problem(s).\n",
            summary, problem_config_type.0, problem_config_type.1
        );
    }

    summary = format!("{}\nBy importance:\n", summary);

    for problem_importance in problem_importances {
        summary = format!(
            "{}{} = {} problem(s).\n",
            summary, problem_importance.0, problem_importance.1
        );
    }

    summary
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
                        places: item.places,
                        links: item.links,
                    });
                }
            }
        }
    }

    problems
}
