use crate::{
    checkers::{Problem, find_problems},
    common::{Rule, RuleChecker},
    configs::v4::KEAv4Config,
    rules::{
        allocator::{
            NotSelectFLQAllocatorInGlobalLevelConfigV4Rule,
            NotSelectIterativeAllocatorForSharedLeaseDatabaseV4Rule,
        },
        client_classes::{
            EvaluateRequiredAsAdditionalClassesRule, NotLifetimeForAdditionalClassesV4Rule,
            NotRecommendedPrefixAFTER_ClassesV4Rule,
        },
        ddns_server::NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesV4Rule,
        hooks::{
            BadHooksOrderV4Rule, MoreOneObjectConfigHAV4Rule,
            MultithreadingModesNotEqualInConfigAndHAV4Rule,
            NoActivatedHostCMDsHookForDatabaseBackendV4Rule,
            NoActivatedHostCacheHookForRADIUSHookV4Rule, NoBasicHTTPAuthInHAPeersV4Rule,
            NoMatchClientIdForFlexIDHookRule, UnnecessaryActivatedDatabaseHooksV4Rule,
            UseUsrCheckHookRule,
        },
        interfaces::NoInterfacesInInterfacesConfigV4Rule,
        lease_database::{
            LeaseSanityChecksEnabledForNotMemfileBackendV4Rule,
            NoEnabledPersistFlagForMemfileLeasesV4Rule, NotChangeStopRetryExitStrategyOnFailV4Rule,
        },
        loggers::{
            NoDebugLoggersV4Rule, NoLinebreakMessagesLoggersV4Rule, NoPercentMMessagesLoggersV4Rule,
        },
        option_data::{IncompleteOctetsBytesInOptionValuesRule, SpecifiedKEAManagedOptionsRule},
        queue_control::NoEnableQueueAndMultithreadingTogetherV4Rule,
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
            interfaces: [Box::new(NoInterfacesInInterfacesConfigV4Rule)],
            allocators: [
                Box::new(NotSelectFLQAllocatorInGlobalLevelConfigV4Rule),
                Box::new(NotSelectIterativeAllocatorForSharedLeaseDatabaseV4Rule),
            ],
            lease_database: [
                Box::new(NoEnabledPersistFlagForMemfileLeasesV4Rule),
                Box::new(NotChangeStopRetryExitStrategyOnFailV4Rule),
                Box::new(LeaseSanityChecksEnabledForNotMemfileBackendV4Rule),
            ],
            hooks: [
                Box::new(MultithreadingModesNotEqualInConfigAndHAV4Rule),
                Box::new(UnnecessaryActivatedDatabaseHooksV4Rule),
                Box::new(NoActivatedHostCMDsHookForDatabaseBackendV4Rule),
                Box::new(BadHooksOrderV4Rule),
                Box::new(NoBasicHTTPAuthInHAPeersV4Rule),
                Box::new(NoActivatedHostCacheHookForRADIUSHookV4Rule),
                Box::new(UseUsrCheckHookRule),
                Box::new(MoreOneObjectConfigHAV4Rule),
                Box::new(NoMatchClientIdForFlexIDHookRule),
            ],
            subnets: [
                Box::new(SubnetsPoolsIntersectionRule),
                Box::new(SubnetsOverlappingRule),
                Box::new(SubnetWithoutPoolsAndReservationsRule),
            ],
            client_classes: [
                Box::new(EvaluateRequiredAsAdditionalClassesRule),
                Box::new(NotLifetimeForAdditionalClassesV4Rule),
                Box::new(NotRecommendedPrefixAFTER_ClassesV4Rule),
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
            queue_control: [Box::new(NoEnableQueueAndMultithreadingTogetherV4Rule)],
            option_data: [
                Box::new(SpecifiedKEAManagedOptionsRule),
                Box::new(IncompleteOctetsBytesInOptionValuesRule),
            ],
            dhcp_ddns: [Box::new(
                NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesV4Rule,
            )],
            loggers: [
                Box::new(NoDebugLoggersV4Rule),
                Box::new(NoLinebreakMessagesLoggersV4Rule),
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
