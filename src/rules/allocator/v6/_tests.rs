#![allow(dead_code)]

pub static NOT_SELECT_FLQ_ALLOCATOR_IN_GLOBAL_LEVEL_CONFIG_RULE_TEST_TEMPLATE: &str = r#"
{
    "interfaces-config": {
        "interfaces": [ "eth0" ],
        "dhcp-socket-type": "raw"
    },
    "lease-database": {
		"type": "memfile",
		"persist": true,
		"name": "/var/lib/kea/dhcp6.leases"
	},
    "valid-lifetime": 4000,
    "renew-timer": 1000,
    "rebind-timer": 2000,
    "allocator": "flq"
}
"#;

pub static NOT_SELECT_ITERATIVE_ALLOCATOR_FOR_SHARED_LEASE_DATABASE_RULE_TEST_TEMPLATE: &str = r#"
{
    "interfaces-config": {
        "interfaces": [ "eth0" ],
        "dhcp-socket-type": "raw"
    },
   	"lease-database": {
		"name": "keatest",
		"host": "localhost",
		"password": "1234",
		"port": 3306,
		"type": "mysql",
		"user": "keatest",
		"on-fail": "serve-retry-continue"
	},
    "valid-lifetime": 4000,
    "renew-timer": 1000,
    "rebind-timer": 2000,
    "allocator": "iterative"
}
"#;
