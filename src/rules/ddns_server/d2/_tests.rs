#![allow(dead_code)]

pub static NOT_LOCAL_IP_ADDRESS_IN_D2_SERVER_CONFIG_RULE_TEST_TEMPLATE: &str = r#"
{
	"ip-address": "1.2.3.4",
	"port": 53001,
	"dns-server-timeout": 500,
	"ncr-protocol": "UDP",
	"ncr-format": "JSON"
}
"#;
