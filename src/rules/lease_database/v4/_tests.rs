#![allow(dead_code)]

pub static NO_ENABLED_PERSIST_FLAG_FOR_MEMFILE_LEASES_RULE_TEST_TEMPLATE: &str = r#"
{
	"valid-lifetime": 4000,
	"renew-timer": 1000,
	"rebind-timer": 2000,
	"interfaces-config": {
		"interfaces": []
	},
	"lease-database": {
		"type": "memfile",
		"persist": false,
		"name": "/var/lib/kea/dhcp4.leases"
	}
}
"#;

pub static NOT_CHANGE_STOP_RETRY_EXIT_STRATEGY_ON_FAIL_RULE_TEST_TEMPLATE: &str = r#"
{
	"valid-lifetime": 4000,
	"renew-timer": 1000,
	"rebind-timer": 2000,
	"interfaces-config": {
		"interfaces": []
	},
	"lease-database": {
		"name": "keatest",
		"host": "localhost",
		"password": "1234",
		"port": 3306,
		"type": "mysql",
		"user": "keatest",
		"on-fail": "serve-retry-continue"
	}
}
"#;
