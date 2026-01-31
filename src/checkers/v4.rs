use crate::{
    checkers::{find_problems, tabled_print_problems},
    common::{Rule, RuleChecker},
    configs::v4::KEAv4Config,
    rules::{
        client_classes::{
            EvaluateRequiredAsAdditionalClassesRule, NotLifetimeForAdditionalClassesRule,
            NotRecommendedPrefixAFTER_ClassesRule,
        },
        ddns_server::NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule,
        hooks::{
            BadHooksOrderRule, MoreOneObjectConfigHARule,
            MultithreadingModesNotEqualInConfigAndHARule,
            NoActivatedHostCMDsHookForDatabaseBackendRule,
            NoActivatedHostCacheHookForRADIUSHookRule, NoBasicHTTPAuthInHAPeersRule,
            NoMatchClientIdForFlexIDHookRule, UnnecessaryActivatedDatabaseHooksRule,
            UseUsrCheckHookRule,
        },
        interfaces::NoInterfacesInInterfacesConfigRule,
        lease_database::{
            NoEnabledPersistFlagForMemfileLeasesRule, NotChangeStopRetryExitStrategyOnFailRule,
        },
        loggers::{
            DebugLoggersV4Rule, NoLinebreakMessagesLoggersV4, NoPercentMMessagesLoggersV4Rule,
        },
        option_data::{IncompleteOctetsBytesInOptionValuesRule, SpecifiedKEAManagedOptionsRule},
        queue_control::NoEnableQueueAndMultithreadingTogetherRule,
        reservations::{
            AllReservationsOutOfPoolsRule, DisabledInSubnetReservationsWithEnabledOutOfPool,
        },
        shared_networks::{
            InterfaceOrRelaysInsideSubnetsSharedNetworksRule,
            MissingSubnetIdSharedNetworksWithHostDatabases, OneSubnetInSharedNetworksRule,
            SameHostReservationsInDifferentSubnetsSharedNetworksRule,
        },
        subnets::{
            SubnetWithoutPoolsAndReservationsRule, SubnetsOverlappingRule,
            SubnetsPoolsIntersectionRule,
        },
    },
};

pub struct RulesV4 {
    pub interfaces: [Box<dyn Rule<KEAv4Config>>; 1],
    pub lease_database: [Box<dyn Rule<KEAv4Config>>; 2],
    pub hooks: [Box<dyn Rule<KEAv4Config>>; 9],
    pub subnets: [Box<dyn Rule<KEAv4Config>>; 3],
    pub client_classes: [Box<dyn Rule<KEAv4Config>>; 3],
    pub shared_networks: [Box<dyn Rule<KEAv4Config>>; 4],
    pub reservations: [Box<dyn Rule<KEAv4Config>>; 2],
    pub queue_control: [Box<dyn Rule<KEAv4Config>>; 1],
    pub option_data: [Box<dyn Rule<KEAv4Config>>; 2],
    pub dhcp_ddns: [Box<dyn Rule<KEAv4Config>>; 1],
    pub loggers: [Box<dyn Rule<KEAv4Config>>; 3],
}

impl RuleChecker<KEAv4Config> for RulesV4 {
    fn default() -> Self {
        RulesV4 {
            interfaces: [Box::new(NoInterfacesInInterfacesConfigRule)],
            lease_database: [
                Box::new(NoEnabledPersistFlagForMemfileLeasesRule),
                Box::new(NotChangeStopRetryExitStrategyOnFailRule),
            ],
            hooks: [
                Box::new(MultithreadingModesNotEqualInConfigAndHARule),
                Box::new(UnnecessaryActivatedDatabaseHooksRule),
                Box::new(NoActivatedHostCMDsHookForDatabaseBackendRule),
                Box::new(BadHooksOrderRule),
                Box::new(NoBasicHTTPAuthInHAPeersRule),
                Box::new(NoActivatedHostCacheHookForRADIUSHookRule),
                Box::new(UseUsrCheckHookRule),
                Box::new(MoreOneObjectConfigHARule),
                Box::new(NoMatchClientIdForFlexIDHookRule),
            ],
            subnets: [
                Box::new(SubnetsPoolsIntersectionRule),
                Box::new(SubnetsOverlappingRule),
                Box::new(SubnetWithoutPoolsAndReservationsRule),
            ],
            client_classes: [
                Box::new(EvaluateRequiredAsAdditionalClassesRule),
                Box::new(NotLifetimeForAdditionalClassesRule),
                Box::new(NotRecommendedPrefixAFTER_ClassesRule),
            ],
            shared_networks: [
                Box::new(OneSubnetInSharedNetworksRule),
                Box::new(InterfaceOrRelaysInsideSubnetsSharedNetworksRule),
                Box::new(MissingSubnetIdSharedNetworksWithHostDatabases),
                Box::new(SameHostReservationsInDifferentSubnetsSharedNetworksRule),
            ],
            reservations: [
                Box::new(AllReservationsOutOfPoolsRule),
                Box::new(DisabledInSubnetReservationsWithEnabledOutOfPool),
            ],
            queue_control: [Box::new(NoEnableQueueAndMultithreadingTogetherRule)],
            option_data: [
                Box::new(SpecifiedKEAManagedOptionsRule),
                Box::new(IncompleteOctetsBytesInOptionValuesRule),
            ],
            dhcp_ddns: [Box::new(NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule)],
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
            &self.queue_control,
            &self.option_data,
            &self.dhcp_ddns,
            &self.loggers,
        ]
    }

    fn run(&self, config: &KEAv4Config) {
        tabled_print_problems(find_problems(config, self.values()));
    }
}
