#![allow(dead_code)]

pub static NO_BASIC_HTTP_AUTH_IN_HA_PEERS_RULE_TEST_TEMPLATE: &str = r#"
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
	"hooks-libraries": [
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
							},
							{
								"name": "server3",
								"url": "http://192.168.56.99:8005/",
								"basic-auth-user": "foo",
								"basic-auth-password": "1234",
								"role": "backup"
							}
						]
					}
				]
			}
		}
	]
}
"#;

pub static UNNECESSARY_ACTIVATED_DATABASE_HOOKS_RULE_TEST_TEMPLATE: &str = r#"
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
	"hooks-libraries": [
		{
			"library": "libdhcp_pgsql.so",
			"parameters": {}
		},
		{
			"library": "libdhcp_mysql.so",
			"parameters": {}
		}
	]
}
"#;

pub static USE_USER_CHECK_HOOK_RULE_TEST_TEMPLATE: &str = r#"
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
	"hooks-libraries": [
	    {
	        "library": "/usr/local/lib/kea/hooks/libdhcp_user_chk.so"
        }
	]
}
"#;

pub static NO_MATCH_CLIENT_ID_FOR_FLEX_ID_HOOK_RULE_TEST_TEMPLATE: &str = r#"
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
	"hooks-libraries": [
	{
        "library": "libdhcp_flex_id.so",
        "parameters": {
            "identifier-expression": "substring(relay4[0].option[18].hex,0,8)",
            "replace-client-id": true
        }
    }]
}
"#;
