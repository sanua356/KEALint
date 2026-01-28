#![allow(dead_code)]

pub static DEBUG_LOGGERS_D2_RULE_TEMPLATE: &str = r#"
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
	]
}
"#;

pub static NO_LINEBREAK_MESSAGES_LOGGERS_D2_RULE_TEMPLATE: &str = r#"
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
					"pattern": "%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m"
				}
			],
			"severity": "INFO",
			"debuglevel": 0
		}
	]
}
"#;
