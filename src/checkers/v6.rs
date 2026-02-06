use crate::{
    checkers::{Problem, find_problems},
    common::{Rule, RuleChecker},
    configs::v6::KEAv6Config,
    rules::loggers::{
        NoDebugLoggersV6Rule, NoLinebreakMessagesLoggersV6Rule, NoPercentMMessagesLoggersV6Rule,
    },
};

pub struct RulesV6 {
    pub loggers: [Box<dyn Rule<KEAv6Config>>; 3],
}

impl RuleChecker<KEAv6Config> for RulesV6 {
    fn default() -> Self {
        RulesV6 {
            loggers: [
                Box::new(NoDebugLoggersV6Rule),
                Box::new(NoLinebreakMessagesLoggersV6Rule),
                Box::new(NoPercentMMessagesLoggersV6Rule),
            ],
        }
    }

    fn values(&self) -> Vec<&[Box<dyn Rule<KEAv6Config>>]> {
        vec![&self.loggers]
    }

    fn run(&self, config: &KEAv6Config) -> Vec<Problem> {
        find_problems(config, self.values())
    }
}
