use crate::{
    checkers::{Problem, find_problems},
    common::{Rule, RuleChecker},
    configs::KEAD2Config,
    rules::{
        ddns_server::NotLocalIPAddressInD2ServerConfigRule,
        hooks::{
            BadTKeyGSSTSIGHookTimeoutsRule, NoCredentialsCacheAndClientKeytabTogetherInGSSTSIGRule,
        },
        loggers::{
            DebugLoggersD2Rule, NoLinebreakMessagesLoggersD2, NoPercentMMessagesLoggersD2Rule,
        },
    },
};

pub struct RulesD2 {
    pub global: [Box<dyn Rule<KEAD2Config>>; 1],
    pub hooks: [Box<dyn Rule<KEAD2Config>>; 2],
    pub loggers: [Box<dyn Rule<KEAD2Config>>; 3],
}

impl RuleChecker<KEAD2Config> for RulesD2 {
    fn default() -> Self {
        RulesD2 {
            global: [Box::new(NotLocalIPAddressInD2ServerConfigRule)],
            hooks: [
                Box::new(BadTKeyGSSTSIGHookTimeoutsRule),
                Box::new(NoCredentialsCacheAndClientKeytabTogetherInGSSTSIGRule),
            ],
            loggers: [
                Box::new(DebugLoggersD2Rule),
                Box::new(NoLinebreakMessagesLoggersD2),
                Box::new(NoPercentMMessagesLoggersD2Rule),
            ],
        }
    }

    fn values(&self) -> Vec<&[Box<dyn Rule<KEAD2Config>>]> {
        vec![&self.global, &self.hooks, &self.loggers]
    }

    fn run(&self, config: &KEAD2Config) -> Vec<Problem> {
        find_problems(config, self.values())
    }
}
