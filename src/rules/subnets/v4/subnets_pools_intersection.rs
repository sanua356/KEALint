use crate::{
    common::{RuleConfigs, RuleLevels, RuleResult, RuleV4},
    configs::{KEAv4Config, KEAv4PoolVariant},
    utils::v4_pool_to_start_end_available_ips,
};

pub struct SubnetsPoolsIntersectionRule;

#[derive(Debug, PartialEq)]
struct SubnetPool {
    subnet: String,
    pool: KEAv4PoolVariant,
}

impl RuleV4 for SubnetsPoolsIntersectionRule {
    fn get_name(&self) -> &'static str {
        "SUBNETS::SubnetsPoolsIntersectionRule"
    }
    fn get_level(&self) -> RuleLevels {
        RuleLevels::Critical
    }
    fn get_config_type(&self) -> RuleConfigs {
        RuleConfigs::Dhcp4
    }
    fn check(&self, config: &KEAv4Config) -> Option<Vec<RuleResult>> {
        config.subnet4.as_ref()?;

        let mut all_subnets_pools: Vec<SubnetPool> = Vec::new();

        for subnet in config.subnet4.as_ref().unwrap() {
            if let Some(pools) = &subnet.pools {
                for pool in pools {
                    all_subnets_pools.push(SubnetPool {
                        subnet: subnet.subnet.to_string().clone(),
                        pool: pool.pool,
                    });
                }
            }
        }

        let mut results: Vec<RuleResult> = Vec::new();

        for i in 0..all_subnets_pools.len() {
            for j in (i + 1)..all_subnets_pools.len() {
                let a = &all_subnets_pools[i];
                let b = &all_subnets_pools[j];

                let (a_start, a_end) = v4_pool_to_start_end_available_ips(a.pool);
                let (b_start, b_end) = v4_pool_to_start_end_available_ips(b.pool);

                if a_start <= b_end && a_end >= b_start {
                    results.push(RuleResult {
                        description: format!(
                            "Pool '{}' in subnet '{}' intersection with pool '{}' in subnet '{}'.",
                            a.pool,
                            a.subnet,
                            b.pool,
                            b.subnet
                        ),
                        snapshot: None,
                        links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#configuration-of-ipv4-address-pools"]),
                    });
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

    use crate::{
        common::RuleV4, configs::v4::KEAv4Config, constants::TEMPLATE_CONFIG_FOR_TESTS_V4,
        rules::subnets::SubnetsPoolsIntersectionRule,
    };

    #[test]
    fn check_expected_trigger() {
        let data: KEAv4Config = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();

        let rule = SubnetsPoolsIntersectionRule;
        assert!(rule.check(&data).is_some());
    }

    #[test]
    fn check_absense_trigger() {
        let mut json_value: Value = serde_json::from_str(TEMPLATE_CONFIG_FOR_TESTS_V4).unwrap();
        json_value["subnet4"].as_array_mut().unwrap()[1]["pools"][0]["pool"] =
            Value::from("1.0.0.1-1.0.0.10");
        let data: KEAv4Config = serde_json::from_value(json_value).unwrap();

        let rule = SubnetsPoolsIntersectionRule;
        assert!(rule.check(&data).is_none());
    }
}
