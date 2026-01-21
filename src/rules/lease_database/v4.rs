use crate::{
    common::{RuleLevels, RuleResult, RuleV4},
    configs::v4::{KEALeaseDatabaseTypes, KEAv4Config},
};

pub struct NoEnabledPersistFlagForMemfileLeases;

impl RuleV4 for NoEnabledPersistFlagForMemfileLeases {
    fn get_name(&self) -> &'static str {
        "LEASE_DATABASE::NoEnabledPersistFlagForMemfileLeases"
    }

    fn get_level(&self) -> RuleLevels {
        RuleLevels::Warning
    }

    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let flag = config.lease_database.persist.unwrap_or(false);
        let lease_database = &config.lease_database.r#type;

        if flag == false && lease_database == &KEALeaseDatabaseTypes::Memfile {
            return Some(vec![
                RuleResult {
                    description: "The 'persist' flag is not set to 'true' for the maintenance of the arend database in the 'memfile'".to_string(),
                    snapshot: Some(serde_json::to_string(&config.lease_database).unwrap()),
                    links: Some(vec!["https://kea.readthedocs.io/en/kea-3.0.0/arm/dhcp4-srv.html#memfile-basic-storage-for-leases".to_string()]),
                }
            ]);
        }

        None
    }
}
