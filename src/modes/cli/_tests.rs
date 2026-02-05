#![allow(dead_code)]

pub static PROCESS_CONFIG_FILE_CA_TEMPLATE: &str = r#"
{
	"Control-agent": {
		"http-host": "0.0.0.0",
		"http-port": 8002,
		"cert-required": true,
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

pub static PROCESS_CONFIG_FILE_D2_TEMPLATE: &str = r#"
{
	"DhcpDdns": {
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
				"severity": "INFO",
				"debuglevel": 0
			}
		]
	}
}
"#;

pub static PROCESS_CONFIG_FILE_V4_TEMPLATE: &str = r#"
{
	"Dhcp4": {
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
