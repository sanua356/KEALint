#![allow(dead_code)]

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
