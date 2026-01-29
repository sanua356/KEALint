use crate::{
    checkers::{find_problems, tabled_print_problems},
    common::{Rule, RuleChecker},
    configs::v4::KEAv4Config,
    rules::{
        client_classes::{
            EvaluateRequiredAsAdditionalClassesRule, NotLifetimeForAdditionalClassesRule,
        },
        hooks::{
            BadHooksOrderRule, MoreOneObjectConfigHARule,
            MultithreadingModesNotEqualInConfigAndHARule,
            NoActivatedHostCMDsHookForDatabaseBackendRule,
            NoActivatedHostCacheHookForRADIUSHookRule, NoBasicHTTPAuthInHAPeersRule,
            UnnecessaryActivatedDatabaseHooksRule, UseUsrCheckHookRule,
        },
        interfaces::NoInterfacesInInterfacesConfigRule,
        lease_database::NoEnabledPersistFlagForMemfileLeasesRule,
        loggers::{
            DebugLoggersV4Rule, NoLinebreakMessagesLoggersV4, NoPercentMMessagesLoggersV4Rule,
        },
        reservations::AllReservationsOutOfPoolsRule,
        shared_networks::OneSubnetInSharedNetworksRule,
        subnets::{
            SubnetWithoutPoolsAndReservationsRule, SubnetsOverlappingRule,
            SubnetsPoolsIntersectionRule,
        },
    },
};

pub struct RulesV4 {
    pub interfaces: [Box<dyn Rule<KEAv4Config>>; 1],
    pub lease_database: [Box<dyn Rule<KEAv4Config>>; 1],
    pub hooks: [Box<dyn Rule<KEAv4Config>>; 8],
    pub subnets: [Box<dyn Rule<KEAv4Config>>; 3],
    pub client_classes: [Box<dyn Rule<KEAv4Config>>; 2],
    pub shared_networks: [Box<dyn Rule<KEAv4Config>>; 1],
    pub reservations: [Box<dyn Rule<KEAv4Config>>; 1],
    pub loggers: [Box<dyn Rule<KEAv4Config>>; 3],
}

impl RuleChecker<KEAv4Config> for RulesV4 {
    fn default() -> Self {
        RulesV4 {
            interfaces: [Box::new(NoInterfacesInInterfacesConfigRule)],
            lease_database: [Box::new(NoEnabledPersistFlagForMemfileLeasesRule)],
            hooks: [
                Box::new(MultithreadingModesNotEqualInConfigAndHARule),
                Box::new(UnnecessaryActivatedDatabaseHooksRule),
                Box::new(NoActivatedHostCMDsHookForDatabaseBackendRule),
                Box::new(BadHooksOrderRule),
                Box::new(NoBasicHTTPAuthInHAPeersRule),
                Box::new(NoActivatedHostCacheHookForRADIUSHookRule),
                Box::new(UseUsrCheckHookRule),
                Box::new(MoreOneObjectConfigHARule),
            ],
            subnets: [
                Box::new(SubnetsPoolsIntersectionRule),
                Box::new(SubnetsOverlappingRule),
                Box::new(SubnetWithoutPoolsAndReservationsRule),
            ],
            client_classes: [
                Box::new(EvaluateRequiredAsAdditionalClassesRule),
                Box::new(NotLifetimeForAdditionalClassesRule),
            ],
            shared_networks: [Box::new(OneSubnetInSharedNetworksRule)],
            reservations: [Box::new(AllReservationsOutOfPoolsRule)],
            loggers: [
                Box::new(DebugLoggersV4Rule),
                Box::new(NoLinebreakMessagesLoggersV4),
                Box::new(NoPercentMMessagesLoggersV4Rule),
            ],
        }
    }

    fn values(&self) -> Vec<&[Box<dyn Rule<KEAv4Config>>]> {
        vec![
            &self.interfaces,
            &self.lease_database,
            &self.hooks,
            &self.subnets,
            &self.client_classes,
            &self.shared_networks,
            &self.reservations,
            &self.loggers,
        ]
    }

    fn run(&self, config: &KEAv4Config) {
        tabled_print_problems(find_problems(config, self.values()));
    }
}
