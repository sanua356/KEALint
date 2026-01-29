#![allow(dead_code)]

pub static ONE_SUBNET_IN_SHARED_NETWORKS_RULE_TEMPLATE: &str = r#"
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
					]
				}
			]
		}
	]
}
"#;

pub static INTERFACE_OR_RELAYS_INSIDE_SUBNETS_SHARED_NETWORKS_RULE_TEMPLATE: &str = r#"
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
	"shared-networks": [
		{
			"name": "my-secret-lair-level-1",
			"subnet4": [
				{
					"id": 1,
					"subnet": "10.0.0.0/8",
					"interface": "eth0",
					"pools": [
						{
							"pool": "10.0.0.1 - 10.0.0.99"
						}
					]
				},
				{
					"id": 2,
					"subnet": "20.0.0.0/8",
					"relay": {
						"ip-addresses": ["1.2.3.4"]
					},
					"pools": [
						{
							"pool": "20.0.0.1 - 20.0.0.99"
						}
					]
				}
			]
		}
	]
}
"#;

pub static MISSING_SUBNET_ID_SHARED_NETWORKS_WITH_HOST_DATABASES_RULE_TEMPLATE: &str = r#"
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
	"hosts-databases": [
		{
			"name": "keatest",
			"host": "localhost",
			"password": "1234",
			"port": 3306,
			"type": "mysql",
			"user": "keatest"
		}
	],
	"shared-networks": [
		{
			"name": "my-secret-lair-level-1",
			"subnet4": [
				{
					"subnet": "10.0.0.0/8",
					"interface": "eth0",
					"pools": [
						{
							"pool": "10.0.0.1 - 10.0.0.99"
						}
					]
				},
				{
					"id": 2,
					"subnet": "20.0.0.0/8",
					"relay": {
						"ip-addresses": ["1.2.3.4"]
					},
					"pools": [
						{
							"pool": "20.0.0.1 - 20.0.0.99"
						}
					]
				}
			]
		}
	]
}
"#;

pub static SAME_HOST_RESERVATIONS_IN_DIFFERENT_SUBNETS_SHARED_NETWORKS_RULE_TEMPLATE: &str = r#"
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
	"shared-networks": [
		{
			"name": "qqq",
			"subnet4": [
				{
					"id": 11,
					"subnet": "254.254.254.254/32",
					"pools": [],
					"interface": "eth1",
					"reservations": [
						{
							"hostname": "qqq",
							"hw-address": "2a:2b:2c:2d:2e:2f",
							"ip-address": "10.0.0.150"
						}
					]
				},
				{
					"id": 12,
					"subnet": "253.253.253.253/32",
					"reservations": [
						{
							"hostname": "www",
							"hw-address": "2a:2b:2c:2d:2e:2f",
							"ip-address": "10.0.0.160"
						}
					]
				}
			]
		}
	]
}
"#;
