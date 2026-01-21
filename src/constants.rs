#![allow(dead_code)]

pub static HIGH_AVAILABILITY_HOOK_LIBRARY: &str = "libdhcp_ha.so";

// Not production. Only for tests.
pub static TEMPLATE_CONFIG_FOR_TESTS_V4: &str = r#"
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
			"id": 1,
			"subnet": "192.0.2.0/24",
			"pools": [
				{
					"pool": "192.0.2.1 - 192.0.2.200"
				}
			]
		}
	],
	"multi-threading": {
		"enable-multi-threading": false,
		"thread-pool-size": 4,
		"packet-queue-size": 16
	},
	"hooks-libraries": [
		{
			"library": "libdhcp_lease_cmds.so",
			"parameters": {}
		},
		{
			"library": "libdhcp_ha.so",
			"parameters": {
				"high-availability": [
					{
						"this-server-name": "server1",
						"mode": "load-balancing",
						"multi-threading": {
							"enable-multi-threading": true,
							"http-dedicated-listener": true,
							"http-listener-threads": 4,
							"http-client-threads": 4
						},
						"peers": [
							{
								"name": "server1",
								"url": "http://192.168.56.33:8005/",
								"role": "primary"
							},
							{
								"name": "server2",
								"url": "http://192.168.56.66:8005/",
								"role": "secondary"
							}
						]
					}
				]
			}
		}
	]
}
"#;
