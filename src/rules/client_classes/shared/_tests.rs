#![allow(dead_code)]

pub static NOT_LIFETIME_FOR_ADDITIONAL_CLASSES_RULE_TEST_TEMPLATE: &str = r#"
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
    "client-classes": [
    	{
			"name": "test_not_required_class",
			"test": "",
			"only-in-additional-list": true,
			"valid-lifetime": 4000
		}
	]
}
"#;

#[allow(non_upper_case_globals)]
pub static NOT_RECOMMENDED_PREFIX_AFTER__CLASSES_RULE_TEST_TEMPLATE: &str = r#"
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
    "client-classes": [
    	{
			"name": "AFTER_test_not_required_class",
			"test": ""
		}
	]
}
"#;
