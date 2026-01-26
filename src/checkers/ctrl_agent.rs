use std::iter;

use crate::{
    checkers::{Problem, tabled_print_problems},
    common::RuleCtrlAgent,
    configs::KEACtrlAgentConfig,
    rules::{
        ctrl_agent::{NoAllControlSocketsSpecifiedRule, NotLocalIPWithoutHTTPSRule},
        loggers::DebugLoggersCtrlAgentRule,
    },
};

pub struct RulesCtrlAgent {
    pub global: Vec<Box<dyn RuleCtrlAgent>>,
    pub loggers: Vec<Box<dyn RuleCtrlAgent>>,
}

impl RulesCtrlAgent {
    pub fn default() -> Self {
        RulesCtrlAgent {
            global: vec![
                Box::new(NotLocalIPWithoutHTTPSRule),
                Box::new(NoAllControlSocketsSpecifiedRule),
            ],
            loggers: vec![Box::new(DebugLoggersCtrlAgentRule)],
        }
    }

    fn values(&self) -> impl Iterator<Item = &Vec<Box<dyn RuleCtrlAgent>>> {
        let global = iter::once(&self.global);
        let loggers = iter::once(&self.loggers);

        global.chain(loggers)
    }

    pub fn run(&self, config: &KEACtrlAgentConfig) {
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
