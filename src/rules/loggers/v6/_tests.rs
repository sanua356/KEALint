#![allow(dead_code)]

pub static DEBUG_LOGGERS_V6_RULE_TEMPLATE: &str = r#"
{
	"valid-lifetime": 4000,
	"renew-timer": 1000,
	"rebind-timer": 2000,
	"interfaces-config": {
		"interfaces": []
	},
	"lease-database": {
		"type": "memfile",
		"persist": true,
		"name": "/var/lib/kea/dhcp6.leases"
	},
	"loggers": [
		{
			"name": "kea-dhcp6",
			"output-options": [
				{
					"output": "kea-dhcp6.log",
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

pub static NO_LINEBREAK_MESSAGES_LOGGERS_V6_RULE_TEMPLATE: &str = r#"
{
	"valid-lifetime": 4000,
	"renew-timer": 1000,
	"rebind-timer": 2000,
	"interfaces-config": {
		"interfaces": []
	},
	"lease-database": {
		"type": "memfile",
		"persist": true,
		"name": "/var/lib/kea/dhcp6.leases"
	},
	"loggers": [
		{
			"name": "kea-dhcp6",
			"output-options": [
				{
					"output": "kea-dhcp6.log",
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

pub static NO_PERCENT_M_MESSAGES_LOGGERS_V6_RULE_TEMPLATE: &str = r#"
{
	"valid-lifetime": 4000,
	"renew-timer": 1000,
	"rebind-timer": 2000,
	"interfaces-config": {
		"interfaces": []
	},
	"lease-database": {
		"type": "memfile",
		"persist": true,
		"name": "/var/lib/kea/dhcp6.leases"
	},
	"loggers": [
		{
			"name": "kea-dhcp6",
			"output-options": [
				{
					"output": "kea-dhcp6.log",
					"maxsize": 52428800,
					"maxver": 100,
					"pattern": "%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] \n"
				}
			],
			"severity": "INFO",
			"debuglevel": 0
		}
	]
}
"#;
