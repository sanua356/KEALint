use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
};

use super::super::shared::get_no_enabled_persist_flag_rule;

pub struct NoEnabledPersistFlagForMemfileLeasesV4Rule;

impl Rule<KEAv4Config> for NoEnabledPersistFlagForMemfileLeasesV4Rule {
    fn get_name(&self) -> &'static str {
        "LEASE_DATABASE::NoEnabledPersistFlagForMemfileLeases"
    }

    fn get_level(&self) -> RuleLevels {
        RuleLevels::Critical
    }

    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }

    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let flag = config.lease_database.persist.unwrap_or(false);
        let lease_database = &config.lease_database.r#type;

        get_no_enabled_persist_flag_rule(flag, lease_database)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{
        super::_tests::NO_ENABLED_PERSIST_FLAG_FOR_MEMFILE_LEASES_RULE_TEST_TEMPLATE,
        NoEnabledPersistFlagForMemfileLeasesV4Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(NO_ENABLED_PERSIST_FLAG_FOR_MEMFILE_LEASES_RULE_TEST_TEMPLATE)
                .unwrap();

        let rule = NoEnabledPersistFlagForMemfileLeasesV4Rule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_ENABLED_PERSIST_FLAG_FOR_MEMFILE_LEASES_RULE_TEST_TEMPLATE)
                .unwrap();
        json_value["lease-database"]
            .as_object_mut()
            .unwrap()
            .insert("persist".to_string(), Value::from(true));
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = NoEnabledPersistFlagForMemfileLeasesV4Rule;
        assert!(rule.check(&data).is_none());
    }
}
