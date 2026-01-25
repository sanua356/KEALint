use std::iter;

use crate::{
    checkers::{common::Problem, tabled_print_problems},
    common::RuleV4,
    configs::v4::KEAv4Config,
    rules::{
        client_classes::EvaluateRequiredAsAdditionalClassesRule,
        hooks::{
            MultithreadingModesNotEqualInConfigAndHARule, UnnecessaryActivatedDatabaseHooksRule,
        },
        interfaces::NoInterfacesInInterfacesConfigRule,
        lease_database::NoEnabledPersistFlagForMemfileLeasesRule,
        subnets::SubnetsPoolsIntersectionRule,
    },
};

pub struct RulesV4 {
    pub interfaces: Vec<Box<dyn RuleV4>>,
    pub lease_database: Vec<Box<dyn RuleV4>>,
    pub hooks: Vec<Box<dyn RuleV4>>,
    pub subnets: Vec<Box<dyn RuleV4>>,
    pub client_classes: Vec<Box<dyn RuleV4>>,
}

impl RulesV4 {
    pub fn default() -> Self {
        RulesV4 {
            interfaces: vec![Box::new(NoInterfacesInInterfacesConfigRule)],
            lease_database: vec![Box::new(NoEnabledPersistFlagForMemfileLeasesRule)],
            hooks: vec![
                Box::new(MultithreadingModesNotEqualInConfigAndHARule),
                Box::new(UnnecessaryActivatedDatabaseHooksRule),
            ],
            subnets: vec![Box::new(SubnetsPoolsIntersectionRule)],
            client_classes: vec![Box::new(EvaluateRequiredAsAdditionalClassesRule)],
        }
    }

    fn values(&self) -> impl Iterator<Item = &Vec<Box<dyn RuleV4>>> {
        let interfaces = iter::once(&self.interfaces);
        let lease_database = iter::once(&self.lease_database);
        let hooks = iter::once(&self.hooks);
        let subnets = iter::once(&self.subnets);
        let client_classes = iter::once(&self.client_classes);

        interfaces
            .chain(lease_database)
            .chain(hooks)
            .chain(subnets)
            .chain(client_classes)
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
