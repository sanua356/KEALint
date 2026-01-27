#![allow(dead_code)]

pub static EVALUATE_REQUIRED_AS_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE: &str = r#"
{
    "interfaces-config": {
        "interfaces": [ "eth0" ],
        "dhcp-socket-type": "raw"
    },
    "lease-database": {
		"type": "memfile",
		"persist": false,
		"name": "/var/lib/kea/dhcp4.leases"
	},
    "valid-lifetime": 4000,
    "renew-timer": 1000,
    "rebind-timer": 2000,
    "subnet4": [{
		"id": 3,
		"subnet": "1.2.3.0/24",
		"evaluate-additional-classes": [
			"test_required_class"
		],
		"pools": [
			{
				"pool": "1.2.3.0/24",
				"evaluate-additional-classes": [
					"test_required_class"
				]
			}
		]
	}],
	"shared-networks": [{
		"name": "my-secret-lair-level-1",
		"interface": "eth0",
		"evaluate-additional-classes": [
			"test_required_class"
		],
		"subnet4": [
			{
				"id": 1,
				"subnet": "10.0.0.0/8",
				"pools": [
					{
						"pool": "10.0.0.1 - 10.0.0.99"
					}
				]
			}
		]
	}],
    "client-classes": [
		{
			"name": "test_required_class",
			"test": ""
		}
	]
}

"#;
