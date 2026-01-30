#![allow(dead_code)]

pub static NOT_DDNS_QUALIFYING_SUFFIX_WITH_ENABLED_DDNS_UPDATES_RULE_TEST_TEMPLATE: &str = r#"
{
    "interfaces-config": {
        "interfaces": [ "eth0" ],
        "dhcp-socket-type": "raw"
    },
    "lease-database": {
		"type": "memfile",
		"persist": false,
		"name": "/var/lib/kea/dhcp4.leases"
	},
    "valid-lifetime": 4000,
    "renew-timer": 1000,
    "rebind-timer": 2000,
    "dhcp-ddns": {
		"enable-updates": false,
		"server-ip": "127.0.0.1",
		"server-port": 53001,
		"sender-ip": "",
		"sender-port": 0,
		"max-queue-size": 1024,
		"ncr-protocol": "UDP",
		"ncr-format": "JSON"
	},
	"ddns-send-updates": true,
	"ddns-override-no-update": false,
	"ddns-override-client-update": false,
	"ddns-replace-client-name": "never",
	"ddns-generated-prefix": "myhost",
	"ddns-qualifying-suffix": "",
	"ddns-update-on-renew": false,
	"ddns-conflict-resolution-mode": "check-with-dhcid",
	"hostname-char-set": "",
	"hostname-char-replacement": ""
}
"#;
