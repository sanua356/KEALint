#![allow(dead_code)]

pub static BAD_HOOKS_ORDER_RULE_TEST_TEMPLATE: &str = r#"
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
			"library": "libdhcp_legal_log.so",
			"parameters": {
				"path": "/var/lib/kea/log",
				"base-name": "kea-forensic4"
			}
		},
		{
			"library": "libdhcp_pgsql.so",
			"parameters": {}
		},
	    {
	        "library": "/usr/local/lib/kea/hooks/libdhcp_radius.so",
	        "parameters": {

	          "dictionary": "/etc/kea/radius/dictionary",

	          "bindaddr": "*"
	         }
        },
        {
          	"library": "/usr/local/lib/kea/hooks/libdhcp_host_cache.so"
	    },
		{
			"library": "libdhcp_mysql.so",
			"parameters": {}
		},
		{
			"library": "libdhcp_flex_id.so",
			"parameters": {
				"identifier-expression": "substring(relay4[0].option[18].hex,0,8)"
			}
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
		},
		{
			"library": "libdhcp_lease_cmds.so",
			"parameters": {}
		},
		{
			"library": "libdhcp_ping_check.so",
			"parameters": {
				"enable-ping-check": true,
				"min-ping-requests": 1,
				"reply-timeout": 100,
				"ping-cltt-secs": 60,
				"ping-channel-threads": 0
			}
		}
	]
}
"#;

pub static MORE_ONE_OBJECT_CONFIG_HA_RULE_TEST_TEMPLATE: &str = r#"
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
				},
				{
					"this-server-name": "server1_1",
					"mode": "hot-standby",
					"multi-threading": {
						"enable-multi-threading": true,
						"http-dedicated-listener": true,
						"http-listener-threads": 4,
						"http-client-threads": 4
					},
					"peers": [
						{
							"name": "server1_1",
							"url": "http://1.2.3.4:8005/",
							"role": "primary"
						},
						{
							"name": "server2_2",
							"url": "http://5.6.7.8:8005/",
							"role": "backup"
						}
					]
				}
			]
		}
	}]
}
"#;
