use std::iter;

use crate::{
    checkers::{common::Problem, tabled_print_problems},
    common::Rule,
    configs::v4::KEAv4Config,
    rules::{
        client_classes::EvaluateRequiredAsAdditionalClassesRule,
        hooks::{
            BadHooksOrderRule, MultithreadingModesNotEqualInConfigAndHARule,
            NoActivatedHostCMDsHookForDatabaseBackendRule, UnnecessaryActivatedDatabaseHooksRule,
        },
        interfaces::NoInterfacesInInterfacesConfigRule,
        lease_database::NoEnabledPersistFlagForMemfileLeasesRule,
        loggers::DebugLoggersV4Rule,
        reservations::AllReservationsOutOfPoolsRule,
        shared_networks::OneSubnetInSharedNetworksRule,
        subnets::{SubnetsOverlappingRule, SubnetsPoolsIntersectionRule},
    },
};

pub struct RulesV4 {
    pub interfaces: Vec<Box<dyn Rule<KEAv4Config>>>,
    pub lease_database: Vec<Box<dyn Rule<KEAv4Config>>>,
    pub hooks: Vec<Box<dyn Rule<KEAv4Config>>>,
    pub subnets: Vec<Box<dyn Rule<KEAv4Config>>>,
    pub client_classes: Vec<Box<dyn Rule<KEAv4Config>>>,
    pub shared_networks: Vec<Box<dyn Rule<KEAv4Config>>>,
    pub reservations: Vec<Box<dyn Rule<KEAv4Config>>>,
    pub loggers: Vec<Box<dyn Rule<KEAv4Config>>>,
}

impl RulesV4 {
    pub fn default() -> Self {
        RulesV4 {
            interfaces: vec![Box::new(NoInterfacesInInterfacesConfigRule)],
            lease_database: vec![Box::new(NoEnabledPersistFlagForMemfileLeasesRule)],
            hooks: vec![
                Box::new(MultithreadingModesNotEqualInConfigAndHARule),
                Box::new(UnnecessaryActivatedDatabaseHooksRule),
                Box::new(NoActivatedHostCMDsHookForDatabaseBackendRule),
                Box::new(BadHooksOrderRule),
            ],
            subnets: vec![
                Box::new(SubnetsPoolsIntersectionRule),
                Box::new(SubnetsOverlappingRule),
            ],
            client_classes: vec![Box::new(EvaluateRequiredAsAdditionalClassesRule)],
            shared_networks: vec![Box::new(OneSubnetInSharedNetworksRule)],
            reservations: vec![Box::new(AllReservationsOutOfPoolsRule)],
            loggers: vec![Box::new(DebugLoggersV4Rule)],
        }
    }

    fn values(&self) -> impl Iterator<Item = &Vec<Box<dyn Rule<KEAv4Config>>>> {
        let interfaces = iter::once(&self.interfaces);
        let lease_database = iter::once(&self.lease_database);
        let hooks = iter::once(&self.hooks);
        let subnets = iter::once(&self.subnets);
        let client_classes = iter::once(&self.client_classes);
        let shared_networks = iter::once(&self.shared_networks);
        let reservations = iter::once(&self.reservations);
        let loggers = iter::once(&self.loggers);

        interfaces
            .chain(lease_database)
            .chain(hooks)
            .chain(subnets)
            .chain(client_classes)
            .chain(shared_networks)
            .chain(reservations)
            .chain(loggers)
    }

    pub fn run(&self, config: &KEAv4Config) {
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
