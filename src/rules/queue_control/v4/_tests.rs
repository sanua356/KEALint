#![allow(dead_code)]

pub static NO_ENABLE_QUEUE_AND_MULTITHREADING_TOGETER_RULE_TEMPLATE: &str = r#"
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
	},
	"multi-threading": {
		"enable-multi-threading": true,
		"thread-pool-size": 4,
		"packet-queue-size": 16
	},
	"dhcp-queue-control": {
	    "enable-queue": true,
	    "queue-type": "queue type",
	    "capacity" : 256
    }
}
"#;
