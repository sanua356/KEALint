#![allow(dead_code)]

pub static SUBNETS_OVERLAPPING_RULE_TEST_TEMPLATE: &str = r#"
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
	"subnet4": [
		{
			"id": 3,
			"subnet": "1.2.3.0/24",
			"pools": [
				{
					"pool": "1.2.3.0/24"
				}
			]
		},
		{
			"id": 4,
			"subnet": "1.0.0.0/8",
			"pools": [
				{
					"pool": "1.0.0.0-1.5.6.7"
				}
			]
		}
	]
}
"#;

pub static SUBNETS_POOLS_INTERSECTION_TEST_TEMPLATE: &str = r#"
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
	"subnet4": [
		{
			"id": 3,
			"subnet": "1.2.3.0/24",
			"pools": [
				{
					"pool": "1.2.3.0/24"
				}
			]
		},
		{
			"id": 4,
			"subnet": "1.0.0.0/8",
			"pools": [
				{
					"pool": "1.0.0.0-1.5.6.7"
				},
				{
					"pool": "1.8.8.10-1.8.8.20"
				}
			]
		}
	]
}
"#;

pub static SUBNETS_WITHOUT_POOLS_AND_RESERVATIONS_TEST_TEMPLATE: &str = r#"
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
	"subnet4": [
		{
			"id": 3,
			"subnet": "1.2.3.0/24",
			"pools": []
		},
		{
			"id": 4,
			"subnet": "1.0.0.0/8",
			"pools": [
				{
					"pool": "1.0.0.0-1.5.6.7"
				},
				{
					"pool": "1.8.8.10-1.8.8.20"
				}
			]
		}
	],
	"shared-networks": [
		{
			"name": "my-secret-lair-level-1",
			"interface": "eth0",
			"subnet4": [
				{
					"id": 1,
					"subnet": "10.0.0.0/8"
				}
			]
		},
		{
			"name": "qqq",
			"interface": "eth0",
			"subnet4": [
				{
					"id": 11,
					"subnet": "11.0.0.0/8",
					"pools": [
						{
							"pool": "11.0.0.0-1.5.6.7"
						},
						{
							"pool": "11.8.8.10-1.8.8.20"
						}
					]
				}
			]
		}
	]
}
"#;
