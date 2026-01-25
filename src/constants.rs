#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;

pub static HIGH_AVAILABILITY_HOOK_LIBRARY: &str = "libdhcp_ha.so";
pub static MYSQL_HOOK_LIBRARY: &str = "libdhcp_mysql.so";
pub static PGSQL_HOOK_LIBRARY: &str = "libdhcp_pgsql.so";
pub static HOST_CMDS_HOOK_LIBRARY: &str = "libdhcp_host_cmds.so";

pub static GSS_TSIG_HOOK_LIBRARY: &str = "libddns_gss_tsig.so";

lazy_static! {
    // Validate ip range in format: IPV4-IPV4
    pub static ref IPV4_RANGE_REGEXP: Regex = Regex::new(r"^(25[0-5]|2[0-4]\d|1?\d?\d)(?:\.(25[0-5]|2[0-4]\d|1?\d?\d)){3}\s*-\s*(25[0-5]|2[0-4]\d|1?\d?\d)(?:\.(25[0-5]|2[0-4]\d|1?\d?\d)){3}$").unwrap();
    // Validate ip range in format: IPV4-IPV4
    pub static ref CIDR_V4_REGEXP: Regex = Regex::new(r"^(25[0-5]|2[0-4]\d|1?\d?\d)(?:\.(25[0-5]|2[0-4]\d|1?\d?\d)){3}/(3[0-2]|[12]?\d)$").unwrap();
}

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
	"multi-threading": {
		"enable-multi-threading": false,
		"thread-pool-size": 4,
		"packet-queue-size": 16
	},
	"client-classes": [
		{
			"name": "test_required_class",
			"test": ""
		}
	],
	"shared-networks": [
		{
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
		}
	],
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
	},
	"hooks-libraries": [
		{
			"library": "/opt/lib/libddns_gss_tsig.so",
			"parameters": {
				"server-principal": "DNS/server.example.org@EXAMPLE.ORG",
				"client-principal": "DHCP/admin.example.org@EXAMPLE.ORG",
				"credentials-cache": "FILE:/etc/ccache",
				"gss-replay-flag": true,
				"gss-sequence-flag": false,
				"tkey-lifetime": 36000,
				"rekey-interval": 270,
				"retry-interval": 12000,
				"tkey-protocol": "TCP",
				"fallback": false,
				"servers": [
					{
						"id": "server1",
						"domain-names": [],
						"ip-address": "192.0.2.1",
						"port": 53,
						"server-principal": "DNS/server1.example.org@EXAMPLE.ORG",
						"client-principal": "DHCP/admin1.example.org@EXAMPLE.ORG",
						"gss-replay-flag": false,
						"gss-sequence-flag": false,
						"tkey-lifetime": 7200000,
						"rekey-interval": 54,
						"retry-interval": 24000,
						"tkey-protocol": "TCP",
						"fallback": true
					},
					{
						"id": "server2",
						"ip-address": "192.0.2.2",
						"port": 5300
					}
				]
			}
		}
	]
}
"#;
