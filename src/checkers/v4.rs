use crate::{
    checkers::{Problem, find_problems},
    common::{Rule, RuleChecker},
    configs::v4::KEAv4Config,
    rules::{
        allocator::{
            NotSelectFLQAllocatorInGlobalLevelConfig,
            NotSelectIterativeAllocatorForSharedLeaseDatabase,
        },
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
            LeaseSanityChecksEnabledForNotMemfileBackend, NoEnabledPersistFlagForMemfileLeasesRule,
            NotChangeStopRetryExitStrategyOnFailRule,
        },
        loggers::{
            NoDebugLoggersV4Rule, NoLinebreakMessagesLoggersV4, NoPercentMMessagesLoggersV4Rule,
        },
        option_data::{IncompleteOctetsBytesInOptionValuesRule, SpecifiedKEAManagedOptionsRule},
        queue_control::NoEnableQueueAndMultithreadingTogetherRule,
        reservations::{
            AllReservationsOutOfPoolsRule, DisabledInSubnetReservationsWithEnabledOutOfPool,
            GlobalReservationsOccupyDynamicPoolsRule,
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
    pub allocators: [Box<dyn Rule<KEAv4Config>>; 2],
    pub lease_database: [Box<dyn Rule<KEAv4Config>>; 3],
    pub hooks: [Box<dyn Rule<KEAv4Config>>; 9],
    pub subnets: [Box<dyn Rule<KEAv4Config>>; 3],
    pub client_classes: [Box<dyn Rule<KEAv4Config>>; 3],
    pub shared_networks: [Box<dyn Rule<KEAv4Config>>; 4],
    pub reservations: [Box<dyn Rule<KEAv4Config>>; 3],
    pub queue_control: [Box<dyn Rule<KEAv4Config>>; 1],
    pub option_data: [Box<dyn Rule<KEAv4Config>>; 2],
    pub dhcp_ddns: [Box<dyn Rule<KEAv4Config>>; 1],
    pub loggers: [Box<dyn Rule<KEAv4Config>>; 3],
}

impl RuleChecker<KEAv4Config> for RulesV4 {
    fn default() -> Self {
        RulesV4 {
            interfaces: [Box::new(NoInterfacesInInterfacesConfigRule)],
            allocators: [
                Box::new(NotSelectFLQAllocatorInGlobalLevelConfig),
                Box::new(NotSelectIterativeAllocatorForSharedLeaseDatabase),
            ],
            lease_database: [
                Box::new(NoEnabledPersistFlagForMemfileLeasesRule),
                Box::new(NotChangeStopRetryExitStrategyOnFailRule),
                Box::new(LeaseSanityChecksEnabledForNotMemfileBackend),
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
                Box::new(GlobalReservationsOccupyDynamicPoolsRule),
            ],
            queue_control: [Box::new(NoEnableQueueAndMultithreadingTogetherRule)],
            option_data: [
                Box::new(SpecifiedKEAManagedOptionsRule),
                Box::new(IncompleteOctetsBytesInOptionValuesRule),
            ],
            dhcp_ddns: [Box::new(NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule)],
            loggers: [
                Box::new(NoDebugLoggersV4Rule),
                Box::new(NoLinebreakMessagesLoggersV4),
                Box::new(NoPercentMMessagesLoggersV4Rule),
            ],
        }
    }

    fn values(&self) -> Vec<&[Box<dyn Rule<KEAv4Config>>]> {
        vec![
            &self.interfaces,
            &self.allocators,
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

    fn run(&self, config: &KEAv4Config) -> Vec<Problem> {
        find_problems(config, self.values())
    }
}
