#![allow(dead_code)]

pub static RUN_CHECKS_CA_TEMPLATE: &str = r#"
{
	"Control-agent": {
		"http-host": "127.0.0.1",
		"http-port": 8002,
		"cert-required": true,
		"control-sockets": {
			"dhcp4": {
				"socket-type": "unix",
				"socket-name": "kea4-ctrl-socket"
			},
			"d2": {
				"socket-type": "unix",
				"socket-name": "kea-dhcp-ddns-socket"
			},
			"dhcp6": {
				"socket-type": "unix",
				"socket-name": "kea6-ctrl-socket"
			}
		},
		"loggers": [
			{
				"name": "kea-ctrl-agent",
				"output-options": [
					{
						"output": "kea-ctrl-agent.log",
						"maxsize": 52428800,
						"maxver": 100,
						"pattern": "%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m\n"
					}
				],
				"severity": "INFO",
				"debuglevel": 0
			}
		]
	}
}
"#;

pub static RUN_CHECKS_D2_TEMPLATE: &str = r#"
{
	"DhcpDdns": {
		"ip-address": "127.0.0.1",
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
				"severity": "INFO",
				"debuglevel": 0
			}
		]
	}
}
"#;

pub static RUN_CHECKS_V4_TEMPLATE: &str = r#"
{
	"Dhcp4": {
		"valid-lifetime": 4000,
		"renew-timer": 1000,
		"rebind-timer": 2000,
		"interfaces-config": {
			"interfaces": ["eth0"]
		},
		"lease-database": {
			"type": "memfile",
			"persist": true,
			"name": "/var/lib/kea/dhcp4.leases"
		},
		"loggers": [
			{
				"name": "kea-dhcp4",
				"output-options": [
					{
						"output": "kea-dhcp4.log",
						"maxsize": 52428800,
						"maxver": 100,
						"pattern": "%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m\n"
					}
				],
				"severity": "INFO",
				"debuglevel": 0
			}
		]
	}
}
"#;

pub static GET_FILE_ARGS_TEMPLATE: &str = r#"
{
	"with-summary": true,
	"dir-path": "./",
	"use-threads": true
}
"#;
