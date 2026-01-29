#![allow(dead_code)]

pub static DEBUG_LOGGERS_CTRL_AGENT_RULE_TEMPLATE: &str = r#"
{
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
			"severity": "DEBUG",
			"debuglevel": 0
		}
	]
}
"#;

pub static NO_LINEBREAK_MESSAGES_LOGGERS_CTRL_AGENT_RULE_TEMPLATE: &str = r#"
{
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
					"pattern": "%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m"
				}
			],
			"severity": "INFO",
			"debuglevel": 0
		}
	]
}
"#;

pub static NO_PERCENT_M_MESSAGES_LOGGERS_CTRL_AGENT_RULE_TEMPLATE: &str = r#"
{
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
					"pattern": "%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] \n"
				}
			],
			"severity": "INFO",
			"debuglevel": 0
		}
	]
}
"#;
