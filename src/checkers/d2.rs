use crate::{
    checkers::{find_problems, tabled_print_problems},
    common::{Rule, RuleChecker},
    configs::KEAD2Config,
    rules::{
        ddns_server::NotLocalIPAddressInD2ServerConfigRule, hooks::BadTKeyGSSTSIGHookTimeoutsRule,
        loggers::DebugLoggersD2Rule,
    },
};

pub struct RulesD2 {
    pub global: [Box<dyn Rule<KEAD2Config>>; 1],
    pub hooks: [Box<dyn Rule<KEAD2Config>>; 1],
    pub loggers: [Box<dyn Rule<KEAD2Config>>; 1],
}

impl RuleChecker<KEAD2Config> for RulesD2 {
    fn default() -> Self {
        RulesD2 {
            global: [Box::new(NotLocalIPAddressInD2ServerConfigRule)],
            hooks: [Box::new(BadTKeyGSSTSIGHookTimeoutsRule)],
            loggers: [Box::new(DebugLoggersD2Rule)],
        }
    }

    fn values(&self) -> Vec<&[Box<dyn Rule<KEAD2Config>>]> {
        vec![&self.global, &self.hooks, &self.loggers]
    }

    fn run(&self, config: &KEAD2Config) {
        tabled_print_problems(find_problems(config, self.values()));
    }
}
