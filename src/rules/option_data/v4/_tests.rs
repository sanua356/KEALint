#![allow(dead_code)]

pub static SPECIFIED_KEA_MANAGED_OPTIONS_RULE_TEST_TEMPLATE: &str = r#"
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
	"option-data": [
		{
			"code": 1,
			"data": "hostname"
		}
	],
	"shared-networks": [
		{
			"name": "my-secret-lair-level-1",
			"interface": "eth0",
			"option-data": [
				{
					"name": "dhcp-parameter-required-list",
					"data": "111"
				}
			],
			"subnet4": [
				{
					"option-data": [
						{
							"name": "dhcp-client-identifier",
							"data": "vvv"
						}
					],
					"subnet": "10.0.0.0/8",
					"pools": [
						{
							"pool": "10.0.0.1 - 10.0.0.99",
							"option-data": [
								{
									"name": "fqdn",
									"data": "aa.bb.cc"
								}
							]
						}
					]
				}
			]
		}
	],
	"subnet4": [
		{
			"id": 3,
			"subnet": "1.2.3.0/24",
			"pools": [
				{
					"pool": "1.2.3.0/24"
				}
			],
			"option-data": [
				{
					"code": 12,
					"data": "qqq"
				}
			]
		},
		{
			"id": 4,
			"subnet": "1.0.0.0/8",
			"pools": [
				{
					"pool": "1.0.0.0-1.5.6.7",
					"option-data": [
						{
							"name": "dhcp-lease-time",
							"data": "eee"
						}
					]
				}
			]
		}
	]
}
"#;

pub static INCOMPLETE_OCTETS_BYTES_IN_OPTION_VALUES_RULE_TEST_TEMPLATE: &str = r#"
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
	"option-data": [
		{
			"code": 2,
			"csv-format": false,
			"data": "1AB"
		}
	],
	"shared-networks": [
		{
			"name": "my-secret-lair-level-1",
			"interface": "eth0",
			"option-data": [
				{
					"name": "dhcp-parameter-required-list",
					"csv-format": false,
					"data": "0x1"
				}
			],
			"subnet4": [
				{
					"option-data": [
						{
							"name": "dhcp-client-identifier",
							"csv-format": false,
							"data": "11 2"
						}
					],
					"subnet": "10.0.0.0/8",
					"pools": [
						{
							"pool": "10.0.0.1 - 10.0.0.99",
							"option-data": [
								{
									"name": "fqdn",
									"csv-format": false,
									"data": "AA:22:3"
								}
							]
						}
					]
				}
			]
		}
	],
	"subnet4": [
		{
			"id": 3,
			"subnet": "1.2.3.0/24",
			"pools": [
				{
					"pool": "1.2.3.0/24"
				}
			],
			"option-data": [
				{
					"code": 12,
					"csv-format": false,
					"data": "AABBC"
				}
			]
		},
		{
			"id": 4,
			"subnet": "1.0.0.0/8",
			"pools": [
				{
					"pool": "1.0.0.0-1.5.6.7",
					"option-data": [
						{
							"name": "dhcp-lease-time",
							"csv-format": false,
							"data": "0xee1"
						}
					]
				}
			]
		}
	]
}
"#;
