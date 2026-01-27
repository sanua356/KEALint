#![allow(dead_code)]

pub static NOT_ALL_CONTROL_SOCKETS_SPECIFIED_RULE_TEST_TEMPLATE: &str = r#"
{
	"http-host": "0.0.0.0",
	"http-port": 8002,
	"cert-required": true,
	"control-sockets": {
		"dhcp4": {
			"socket-type": "unix",
			"socket-name": "kea4-ctrl-socket"
		},
		"dhcp6": {
			"socket-type": "unix",
			"socket-name": "kea6-ctrl-socket"
		}
	}
}
"#;

pub static NOT_LOCAL_IP_WITHOUT_HTTPS_RULE_TEST_TEMPLATE: &str = r#"
{
	"http-host": "0.0.0.0",
	"http-port": 8002,
	"cert-required": true,
	"control-sockets": {},
	"hooks-libraries": []
}
"#;
