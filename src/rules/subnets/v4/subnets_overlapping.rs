use crate::{
    common::{Rule, RuleConfigs, RuleLevels, RuleResult},
    configs::KEAv4Config,
    utils::v4_pool_to_start_end_available_ips,
};

pub struct SubnetsOverlappingRule;

impl Rule<KEAv4Config> for SubnetsOverlappingRule {
    fn get_name(&self) -> &'static str {
        "SUBNETS::SubnetsOverlappingRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Critical
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        let mut results: Vec<RuleResult> = Vec::new();

        if let Some(subnets) = &config.subnet4 {
            for i in 0..subnets.len() {
                for j in (i + 1)..subnets.len() {
                    let a = &subnets[i];
                    let b = &subnets[j];

                    let (a_start, a_end) = v4_pool_to_start_end_available_ips(a.subnet);
                    let (b_start, b_end) = v4_pool_to_start_end_available_ips(b.subnet);

                    if a_start <= b_end && a_end >= b_start {
                        results.push(RuleResult {
                        description: format!(
                            "The subnet '{}' overlaps the subnet '{}'. Change the prefix or subnet address.",
                            b.subnet,
                            a.subnet,
                        ),
                        places: None,
                        links: Some(&["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#configuration-of-ipv4-address-pools"]),
                    });
                    }
                }
            }
        }

        if !results.is_empty() {
            return Some(results);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{common::Rule, configs::v4::KEAv4Config};

    use super::{super::_tests::SUBNETS_OVERLAPPING_RULE_TEST_TEMPLATE, SubnetsOverlappingRule};

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config =
            serde_json::from_str(SUBNETS_OVERLAPPING_RULE_TEST_TEMPLATE).unwrap();

        let rule = SubnetsOverlappingRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value =
            serde_json::from_str(SUBNETS_OVERLAPPING_RULE_TEST_TEMPLATE).unwrap();
        json_value["subnet4"].as_array_mut().unwrap().remove(1);
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = SubnetsOverlappingRule;
        assert!(rule.check(&data).is_none());
    }
}
