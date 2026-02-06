#![allow(dead_code)]

pub static NO_INTERFACES_IN_INTERFACES_CONFIG_RULE_TEST_TEMPLATE: &str = r#"
{
	"valid-lifetime": 4000,
	"renew-timer": 1000,
	"rebind-timer": 2000,
	"interfaces-config": {
		"interfaces": []
	},
	"lease-database": {
		"type": "memfile",
		"persist": true,
		"name": "/var/lib/kea/dhcp6.leases"
	}
}
"#;
