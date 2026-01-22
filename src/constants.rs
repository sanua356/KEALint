#![allow(dead_code)]

pub static HIGH_AVAILABILITY_HOOK_LIBRARY: &str = "libdhcp_ha.so";
pub static MYSQL_HOOK_LIBRARY: &str = "libdhcp_mysql.so";
pub static PGSQL_HOOK_LIBRARY: &str = "libdhcp_pgsql.so";

pub static GSS_TSIG_HOOK_LIBRARY: &str = "libddns_gss_tsig.so";

// Not production. Only for tests.
// This file should cause all rules in tests to fire correctly.
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
	"hosts-databases": [
		{
			"name": "keatest",
			"host": "localhost",
			"password": "1234",
			"port": 3306,
			"type": "mysql",
			"user": "keatest",
			"readonly": false,
			"trust-anchor": "my-ca",
			"cert-file": "my-cert",
			"key-file": "my-key",
			"cipher-list": "AES",
			"reconnect-wait-time": 3000,
			"max-reconnect-tries": 3,
			"on-fail": "stop-retry-exit",
			"retry-on-startup": false,
			"connect-timeout": 100,
			"read-timeout": 120,
			"write-timeout": 180
		}
	],
	"hooks-libraries": [
		{
			"library": "libdhcp_lease_cmds.so",
			"parameters": {}
		},
		{
			"library": "libdhcp_pgsql.so",
			"parameters": {}
		},
		{
			"library": "libdhcp_mysql.so",
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

// Not production. Only for tests.
// This file should cause all rules in tests to fire correctly.
pub static TEMPLATE_CONFIG_FOR_TESTS_D2: &str = r#"
{
	"ip-address": "1.2.3.4",
	"port": 53001,
	"dns-server-timeout": 500,
	"ncr-protocol": "UDP",
	"ncr-format": "JSON",
	"tsig-keys": [],
	"forward-ddns": {
		"ddns-domains": []
	},
	"reverse-ddns": {
		"ddns-domains": []
	}
}
"#;
