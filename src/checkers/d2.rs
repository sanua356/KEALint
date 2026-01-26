use std::iter;

use crate::{
    checkers::{Problem, tabled_print_problems},
    common::Rule,
    configs::KEAD2Config,
    rules::{
        ddns_server::NotLocalIPAddressInD2ServerConfigRule, hooks::BadTKeyGSSTSIGHookTimeoutsRule,
        loggers::DebugLoggersD2Rule,
    },
};

pub struct RulesD2 {
    pub global: Vec<Box<dyn Rule<KEAD2Config>>>,
    pub hooks: Vec<Box<dyn Rule<KEAD2Config>>>,
    pub loggers: Vec<Box<dyn Rule<KEAD2Config>>>,
}

impl RulesD2 {
    pub fn default() -> Self {
        RulesD2 {
            global: vec![Box::new(NotLocalIPAddressInD2ServerConfigRule)],
            hooks: vec![Box::new(BadTKeyGSSTSIGHookTimeoutsRule)],
            loggers: vec![Box::new(DebugLoggersD2Rule)],
        }
    }

    fn values(&self) -> impl Iterator<Item = &Vec<Box<dyn Rule<KEAD2Config>>>> {
        let global = iter::once(&self.global);
        let hooks = iter::once(&self.hooks);
        let loggers = iter::once(&self.loggers);

        global.chain(hooks).chain(loggers)
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
