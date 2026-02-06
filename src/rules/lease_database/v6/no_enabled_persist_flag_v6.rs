use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv6Config,
};

use super::super::shared::get_no_enabled_persist_flag_rule;

pub struct NoEnabledPersistFlagForMemfileLeasesV6Rule;

impl Rule<KEAv6Config> for NoEnabledPersistFlagForMemfileLeasesV6Rule {
    fn get_name(&self) -> &'static str {
        "LEASE_DATABASE::NoEnabledPersistFlagForMemfileLeases"
    }

    fn get_level(&self) -> RuleLevels {
        RuleLevels::Critical
    }

    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp6
    }

    fn check(&self, config: &KEAv6Config) -> Option<Vec<RuleResult>> {
        let flag = config.lease_database.persist.unwrap_or(false);
        let lease_database = &config.lease_database.r#type;

        get_no_enabled_persist_flag_rule(flag, lease_database)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v6::KEAv6Config};

    use super::{
        super::_tests::NO_ENABLED_PERSIST_FLAG_FOR_MEMFILE_LEASES_RULE_TEST_TEMPLATE,
        NoEnabledPersistFlagForMemfileLeasesV6Rule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv6Config =
            serde_json::from_str(NO_ENABLED_PERSIST_FLAG_FOR_MEMFILE_LEASES_RULE_TEST_TEMPLATE)
                .unwrap();

        let rule = NoEnabledPersistFlagForMemfileLeasesV6Rule;
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
        let data: KEAv6Config = serde_json::from_value(json_value).unwrap();

        let rule = NoEnabledPersistFlagForMemfileLeasesV6Rule;
        assert!(rule.check(&data).is_none());
    }
}
