use crate::{
    checkers::{Problem, find_problems},
    common::{Rule, RuleChecker},
    configs::v6::KEAv6Config,
    rules::{
        allocator::{
            NotSelectFLQAllocatorInGlobalLevelConfigV6Rule,
            NotSelectIterativeAllocatorForSharedLeaseDatabaseV6Rule,
        },
        client_classes::{
            NotLifetimeForAdditionalClassesV6Rule, NotRecommendedPrefixAFTER_ClassesV6Rule,
        },
        ddns_server::NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesV6Rule,
        loggers::{
            NoDebugLoggersV6Rule, NoLinebreakMessagesLoggersV6Rule, NoPercentMMessagesLoggersV6Rule,
        },
    },
};

pub struct RulesV6 {
    pub loggers: [Box<dyn Rule<KEAv6Config>>; 3],
    pub allocators: [Box<dyn Rule<KEAv6Config>>; 2],
    pub client_classes: [Box<dyn Rule<KEAv6Config>>; 2],
    pub dhcp_ddns: [Box<dyn Rule<KEAv6Config>>; 1],
}

impl RuleChecker<KEAv6Config> for RulesV6 {
    fn default() -> Self {
        RulesV6 {
            loggers: [
                Box::new(NoDebugLoggersV6Rule),
                Box::new(NoLinebreakMessagesLoggersV6Rule),
                Box::new(NoPercentMMessagesLoggersV6Rule),
            ],
            allocators: [
                Box::new(NotSelectFLQAllocatorInGlobalLevelConfigV6Rule),
                Box::new(NotSelectIterativeAllocatorForSharedLeaseDatabaseV6Rule),
            ],
            client_classes: [
                Box::new(NotLifetimeForAdditionalClassesV6Rule),
                Box::new(NotRecommendedPrefixAFTER_ClassesV6Rule),
            ],
            dhcp_ddns: [Box::new(
                NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesV6Rule,
            )],
        }
    }

    fn values(&self) -> Vec<&[Box<dyn Rule<KEAv6Config>>]> {
        vec![
            &self.loggers,
            &self.allocators,
            &self.client_classes,
            &self.dhcp_ddns,
        ]
    }

    fn run(&self, config: &KEAv6Config) -> Vec<Problem> {
        find_problems(config, self.values())
    }
}
