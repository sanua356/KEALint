#![allow(dead_code)]

pub static BAD_TKEY_GSS_TSIG_HOOK_TIMEOUTS_RULE_TEST_TEMPLATE: &str = r#"
{
	"ip-address": "1.2.3.4",
	"port": 53001,
	"dns-server-timeout": 500,
	"ncr-protocol": "UDP",
	"ncr-format": "JSON",
	"loggers": [
		{
			"name": "kea-dhcp-ddns",
			"output-options": [
				{
					"output": "kea-dhcp-ddns.log",
					"maxsize": 52428800,
					"maxver": 100,
					"pattern": "%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m\n"
				}
			],
			"severity": "DEBUG",
			"debuglevel": 0
		}
	],
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
