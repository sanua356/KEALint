use crate::{
    checkers::{RuleChecker, find_problems, tabled_print_problems},
    common::Rule,
    configs::KEACtrlAgentConfig,
    rules::{
        ctrl_agent::{NoAllControlSocketsSpecifiedRule, NotLocalIPWithoutHTTPSRule},
        loggers::DebugLoggersCtrlAgentRule,
    },
};

pub struct RulesCtrlAgent {
    pub global: [Box<dyn Rule<KEACtrlAgentConfig>>; 2],
    pub loggers: [Box<dyn Rule<KEACtrlAgentConfig>>; 1],
}

impl RuleChecker<KEACtrlAgentConfig> for RulesCtrlAgent {
    fn default() -> Self {
        RulesCtrlAgent {
            global: [
                Box::new(NotLocalIPWithoutHTTPSRule),
                Box::new(NoAllControlSocketsSpecifiedRule),
            ],
            loggers: [Box::new(DebugLoggersCtrlAgentRule)],
        }
    }

    fn values(&self) -> Vec<&[Box<dyn Rule<KEACtrlAgentConfig>>]> {
        vec![&self.global, &self.loggers]
    }

    fn run(&self, config: &KEACtrlAgentConfig) {
        tabled_print_problems(find_problems(config, self.values()));
    }
}
