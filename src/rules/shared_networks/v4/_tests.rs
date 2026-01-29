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
