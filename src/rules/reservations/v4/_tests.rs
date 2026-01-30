#![allow(dead_code)]

pub static ALL_RESERVATIONS_OUT_OF_POOLS_RULE_TEMPLATE: &str = r#"
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
			"id": 4,
			"subnet": "1.0.0.0/8",
			"pools": [
				{
					"pool": "1.0.0.0-1.5.6.7"
				},
				{
					"pool": "1.8.8.10-1.8.8.20"
				}
			],
			"reservations": [
				{
					"hostname": "special-snowflake",
					"hw-address": "1a:1b:1c:1d:1e:1f",
					"ip-address": "1.0.0.100"
				},
				{
					"hostname": "out_of_pool_reservation",
					"hw-address": "11:22:33:44:55:66",
					"ip-address": "1.8.8.30"
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
					"subnet": "10.0.0.0/8",
					"pools": [
						{
							"pool": "10.0.0.1 - 10.0.0.99"
						}
					],
					"reservations": [
						{
							"hostname": "hostname",
							"hw-address": "2a:2b:2c:2d:2e:2f",
							"ip-address": "10.0.0.150"
						}
					]
				}
			]
		}
	]
}
"#;

pub static DISABLED_IN_SUBNET_RESERVATIONS_WITH_ENABLED_OUT_OF_POOL_RULE_TEMPLATE: &str = r#"
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
	"reservations-in-subnet": false,
	"subnet4": [
		{
			"id": 4,
			"subnet": "1.0.0.0/8",
			"reservations-in-subnet": true,
			"reservations-out-of-pool": true,
			"pools": []
		},
		{
			"id": 5,
			"subnet": "2.0.0.0/8",
			"reservations-out-of-pool": true,
			"pools": []
		}
	],
	"shared-networks": [
		{
			"name": "qqq",
			"interface": "eth1",
			"reservations-in-subnet": false,
			"subnet4": [
				{
					"id": 1,
					"subnet": "10.0.0.0/8",
					"pools": [],
					"reservations-in-subnet": true,
					"reservations-out-of-pool": true
				},
				{
					"id": 2,
					"subnet": "20.0.0.0/8",
					"pools": [],
					"reservations-out-of-pool": true
				}
			]
		}
	]
}
"#;
