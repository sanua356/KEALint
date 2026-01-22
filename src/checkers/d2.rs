use std::iter;

use crate::{
    checkers::{Problem, tabled_print_problems},
    common::RuleD2,
    configs::KEAD2Config,
    rules::ddns_server::NotLocalIPAddressInD2ServerConfigRule,
};

pub struct RulesD2 {
    pub global: Vec<Box<dyn RuleD2>>,
}

impl RulesD2 {
    pub fn default() -> Self {
        RulesD2 {
            global: vec![Box::new(NotLocalIPAddressInD2ServerConfigRule)],
        }
    }

    fn values(&self) -> impl Iterator<Item = &Vec<Box<dyn RuleD2>>> {
        iter::once(&self.global)
    }

    pub fn run(&self, config: &KEAD2Config) {
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

        tabled_print_problems(problems);
    }
}
