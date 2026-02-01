# Rules "Hooks"

## Implemented For

- ✅ - Implemented for specified config.
- ❌ - NOT implemented for specified config.
- 🚫 - Cannot be implemented for the specified config.

|Rule name|DHCPv4|DDNS|Control Agent|
|--|--|--|--|
|[BadTKeyGSSTSIGHookTimeoutsRule](#BadTKeyGSSTSIGHookTimeoutsRule)|🚫|✅|🚫|
|[NoCredentialsCacheAndKeytabTogetherInGSSTSIGRule](#NoCredentialsCacheAndKeytabTogetherInGSSTSIGRule)|🚫|✅|🚫|
|[BadHooksOrderRule](#BadHooksOrderRule)|✅|🚫|🚫|
|[MoreOneObjectConfigHARule](#MoreOneObjectConfigHARule)|✅|🚫|🚫|
|[MultithreadingModesNotEqualInConfigAndHARule](#MultithreadingModesNotEqualInConfigAndHARule)|✅|🚫|🚫|
|[NoActivatedHostCacheHookForRADIUSHookRule](#NoActivatedHostCacheHookForRADIUSHookRule)|✅|🚫|🚫|
|[NoActivatedHostCMDsHookForDatabaseBackendRule](#NoActivatedHostCMDsHookForDatabaseBackendRule)|✅|🚫|🚫|
|[NoBasicHTTPAuthInHAPeersRule](#NoBasicHTTPAuthInHAPeersRule)|✅|🚫|🚫|
|[NoMatchClientIdForFlexIDHookRule](#NoMatchClientIdForFlexIDHookRule)|✅|🚫|🚫|
|[UnnecessaryActivatedDatabaseHooksRule](#UnnecessaryActivatedDatabaseHooksRule)|✅|🚫|🚫|
|[UseUsrCheckHookRule](#UseUsrCheckHookRule)|✅|🚫|🚫|


## Rules

### BadTKeyGSSTSIGHookTimeoutsRule

- **Codename** - HOOKS::BadTKeyGSSTSIGHookTimeoutsRule.
- **Importance** - WARNING.
- **Config type** - D2.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/integrations.html#using-gss-tsig

#### Problem

When configuring the GSS-TSIG hook, which performs the work of updating DDNS records, specify the parameters of the TKEY key update intervals. Incorrect configuration of these parameters can lead to errors in updating DNS records or issuing TKEY keys.

```js
{
	"DhcpDdns": {
		// DDNS Config ...
		"hooks-libraries": [
			{
				"library": "/opt/lib/libddns_gss_tsig.so",
				"parameters": {
					"server-principal": "DNS/server.example.org@EXAMPLE.ORG",
					"client-principal": "DHCP/admin.example.org@EXAMPLE.ORG",
					"client-keytab": "FILE:/etc/kea/kea.keytab",
					"credentials-cache": "",
					"gss-replay-flag": true,
					"gss-sequence-flag": false,
					"tkey-lifetime": 36000, // An exaggerated example for clarity
					"rekey-interval": 270, // An exaggerated example for clarity
					"retry-interval": 12000, // An exaggerated example for clarity
					"tkey-protocol": "TCP",
					"fallback": false,
					"servers": [
						{
							"id": "server1",
							"domain-names": [],
							"ip-address": "192.0.2.1",
							"port": 53,
							"server-principal": "DNS/server1.example.org@EXAMPLE.ORG",
							"client-principal": "DHCP/admin1.example.org@EXAMPLE.ORG",
							"gss-replay-flag": false,
							"gss-sequence-flag": false,
							"tkey-lifetime": 7200000, // An exaggerated example for clarity
							"rekey-interval": 54, // An exaggerated example for clarity
							"retry-interval": 24000, // An exaggerated example for clarity
							"tkey-protocol": "TCP",
							"fallback": true
						}
					]
				}
			}
		]
		// DDNS Config ...
	}
}
```

#### Solution

It is recommended to specify the parameters of timeouts according to the following principles:

- The `rekey-interval` value must be smaller than the `tkey-lifetime` value (it is recommended to be set between 50% and 80% of the `tkey-lifetime` value). It is expressed in seconds and defaults to 2700 (45 minutes, or 75% of one hour).
- The `retry-interval` value must be smaller than the `rekey-interval` value, and should be at most 1/3 of the difference between `tkey-lifetime` and `rekey-interval`. It is expressed in seconds and defaults to 120 (2 minutes).

```js
{
	"DhcpDdns": {
		// DDNS Config ...
		"hooks-libraries": [
			{
				"library": "/opt/lib/libddns_gss_tsig.so",
				"parameters": {
					"server-principal": "DNS/server.example.org@EXAMPLE.ORG",
					"client-principal": "DHCP/admin.example.org@EXAMPLE.ORG",
					"client-keytab": "FILE:/etc/kea/kea.keytab",
					"credentials-cache": "",
					"gss-replay-flag": true,
					"gss-sequence-flag": false,
					"tkey-lifetime": 3600, // Correct value
					"rekey-interval": 2700, // Correct value
					"retry-interval": 120, // Correct value
					"tkey-protocol": "TCP",
					"fallback": false,
					"servers": [
						{
							"id": "server1",
							"domain-names": [],
							"ip-address": "192.0.2.1",
							"port": 53,
							"server-principal": "DNS/server1.example.org@EXAMPLE.ORG",
							"client-principal": "DHCP/admin1.example.org@EXAMPLE.ORG",
							"gss-replay-flag": false,
							"gss-sequence-flag": false,
							"tkey-lifetime": 7200, // Correct value
							"rekey-interval": 5400, // Correct value
							"retry-interval": 240, // Correct value
							"tkey-protocol": "TCP",
							"fallback": true
						}
					]
				}
			}
		]
		// DDNS Config ...
	}
}
```


### NoCredentialsCacheAndKeytabTogetherInGSSTSIGRule

- **Codename** - HOOKS::NoCredentialsCacheAndKeytabTogetherInGSSTSIGRule.
- **Importance** - WARNING.
- **Config type** - D2.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/integrations.html#using-gss-tsig

#### Problem

If you specify the `credentials-cache` and `client-keytab` parameters at the same time, the GSS-TSIG hook may be disrupted. It is recommended to specify only one of the two parameters.

```js
{
	"DhcpDdns": {
		// DDNS Config ...
		"hooks-libraries": [
			{
				"library": "/opt/lib/libddns_gss_tsig.so",
				"parameters": {
					"server-principal": "DNS/server.example.org@EXAMPLE.ORG",
					"client-principal": "DHCP/admin.example.org@EXAMPLE.ORG",
					"client-keytab": "FILE:/etc/kea/kea.keytab", // Specified "client-keytab" and "credentials-cache" together
					"credentials-cache": "FILE:/etc/ccache", // Specified "client-keytab" and "credentials-cache" together
					"gss-replay-flag": true,
					"gss-sequence-flag": false,
					"tkey-lifetime": 3600,
					"rekey-interval": 2700,
					"retry-interval": 120,
					"tkey-protocol": "TCP",
					"fallback": false,
					"servers": []
				}
			}
		]
		// DDNS Config ...
	}
}
```

#### Solution

You must specify either the `credentials-cache` or `client-keytab` parameter.

```js
{
	"DhcpDdns": {
		// DDNS Config ...
		"hooks-libraries": [
			{
				"library": "/opt/lib/libddns_gss_tsig.so",
				"parameters": {
					"server-principal": "DNS/server.example.org@EXAMPLE.ORG",
					"client-principal": "DHCP/admin.example.org@EXAMPLE.ORG",
					"client-keytab": "FILE:/etc/kea/kea.keytab", // Specified only "client-keytab"
					"credentials-cache": "",
					"gss-replay-flag": true,
					"gss-sequence-flag": false,
					"tkey-lifetime": 3600,
					"rekey-interval": 2700,
					"retry-interval": 120,
					"tkey-protocol": "TCP",
					"fallback": false,
					"servers": []
				}
			}
		]
		// DDNS Config ...
	}
}
```

### BadHooksOrderRule

- **Codename** - HOOKS::BadHooksOrderRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - [
https://kea.readthedocs.io/en/latest/arm/hooks.html#order-of-configuration
https://kea.readthedocs.io/en/latest/arm/hooks.html#load-balancing-configuration
https://kea.readthedocs.io/en/latest/arm/hooks.html#binding-variables
https://kea.readthedocs.io/en/latest/arm/integrations.html#radius-hook-library-configuration 
]

#### Problem

Incorrect order of specifying hooks in `hooks-libraries` can lead to incorrect behavior or disrupt the operation of KEA.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"hooks-libraries": [
		{
			"library": "libdhcp_legal_log.so", // It is recommended to place the last in the list.
			"parameters": {
				"path": "/var/lib/kea/log",
				"base-name": "kea-forensic4"
			}
		},
	    {
	        "library": "libdhcp_radius.so",
	        "parameters": {
	          "dictionary": "/etc/kea/radius/dictionary",
	          "bindaddr": "*"
	         }
        },
        {
          	"library": "libdhcp_host_cache.so" // Must be positioned before the RADIUS hook
	    },
		{
			"library": "libdhcp_flex_id.so", // It is recommended to place the first in the list.
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
			"library": "libdhcp_lease_cmds.so", // Must be positioned before the HA hook
			"parameters": {}
		},
		{
			"library": "libdhcp_ping_check.so", // Must be positioned before the Lease Commands hook
			"parameters": {
				"enable-ping-check": true,
				"min-ping-requests": 1,
				"reply-timeout": 100,
				"ping-cltt-secs": 60,
				"ping-channel-threads": 0
			}
		}]
		// DHCPv4 Config ...
	}
}
```

#### Solution

It is recommended to arrange the hooks in the order of the following principles:

- Put the Flex ID hook at the top of the list (clause 16.3.1).
- Put Forensic Logging last in the list (clause 16.3.1).
- Lease Commands must be in front of High Availability (clause 16.12.6).
- Ping Check must be placed before Lease Commands (clause 16.15.12)
- Host Cache must be placed before RADIUS (clause 16.15.12)

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"hooks-libraries": [
        {
            "library": "libdhcp_flex_id.so",
            "parameters": {
                "identifier-expression": "substring(relay4[0].option[18].hex,0,8)"
            }
        },
        {
            "library": "host_cache.so",
            "parameters": {}
        },
        {
            "library": "libdhcp_radius.so",
            "parameters": {
                "dictionary": "/etc/kea/radius/dictionary",
                "bindaddr": "*"
            }
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
        },
        {
            "library": "libdhcp_lease_cmds.so",
            "parameters": {}
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
	        "library": "libdhcp_legal_log.so",
	        "parameters": {
	            "path": "/var/lib/kea/log",
	            "base-name": "kea-forensic4"
	        }
	    }
	]
	// DHCPv4 Config ...
	}
}
```

### MoreOneObjectConfigHARule

- **Codename** - HOOKS::MoreOneObjectConfigHARule.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/hooks.html#load-balancing-configuration

#### Problem

Currently, the high availability hook in KEA does not support more than one element in the array of configuration parameters for the `high-availability` key.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"hooks-libraries": [
		{
			"library": "libdhcp_ha.so",
			"parameters": {
				"high-availability": [ // Two items in array
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
		// DHCPv4 Config ...
	}
}
```

#### Solution

Specifying only one element in the array of configuration parameters using the `high-availability` key.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"hooks-libraries": [
		{
			"library": "libdhcp_ha.so",
			"parameters": {
				"high-availability": [ // One item in array
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
		}]
		// DHCPv4 Config ...
	}
}
```

### MultithreadingModesNotEqualInConfigAndHARule

- **Codename** - HOOKS::MultithreadingModesNotEqualInConfigAndHARule.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - [
https://kea.readthedocs.io/en/latest/arm/dhcp6-srv.html#multi-threading-settings
https://kea.readthedocs.io/en/latest/arm/hooks.html#multi-threaded-configuration-ha-mt
]

#### Problem

It may turn out that the high availability hook is configured to work in multithreaded mode, while the KEA server is in single-threaded mode and vice versa. This configuration is not recommended for configuration, it may disrupt the operation of the high-availability address synchronization mechanism.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"multi-threading": { // Multithreading disabled in global server configuration level
			"enable-multi-threading": false,
			"thread-pool-size": 4,
			"packet-queue-size": 16
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
								"enable-multi-threading": true,  // But multithreading enabled in hook configuration
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
		// DHCPv4 Config ...
	}
}
```

#### Solution

Synchronize the activation parameters of multithreading modes in the global and high availability hook configurations.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"multi-threading": { // Multithreading enabled in global server configuration level
			"enable-multi-threading": true,
			"thread-pool-size": 4,
			"packet-queue-size": 16
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
								"enable-multi-threading": true,  // And multithreading enabled in hook configuration
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
		// DHCPv4 Config ...
	}
}
```


### NoActivatedHostCacheHookForRADIUSHookRule

- **Codename** - HOOKS::NoActivatedHostCacheHookForRADIUSHookRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/stable/arm/integrations.html#radius-hook-library-configuration

#### Problem

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
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
		        "library": "/usr/local/lib/kea/hooks/libdhcp_radius.so",
		        "parameters": {
		          "dictionary": "/etc/kea/radius/dictionary",
		          "bindaddr": "*"
		         }
	        }
		]
		// DHCPv4 Config ...
	}
}
```

For the RADIUS support hook to work correctly, the activation of the Host Cache hook is also required.

#### Solution

Activate the Host Cache hook along with the RADIUS support hook.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"hooks-libraries": [
	    {
         	"library": "libdhcp_host_cache.so"
       	},
        {
	        "library": "libdhcp_radius.so",
	        "parameters": {
	          	"dictionary": "/etc/kea/radius/dictionary",
		        "bindaddr": "*"
	        }
       	}
    ]
	// DHCPv4 Config ...
	}
}
```

### NoActivatedHostCMDsHookForDatabaseBackendRule

- **Codename** - HOOKS::NoActivatedHostCMDsHookForDatabaseBackendRule.
- **Importance** - INFO.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/hooks.html#libdhcp-host-cmds-so-host-commands

#### Problem

If host reservations are served by a third-party database, it is recommended to use the Host Commands hook to manipulate it, since direct editing of reservations in configuration files can cause conflicts and disrupt the operation of the KEA server.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"hosts-databases": [
			{
				"name": "keatest",
				"host": "localhost",
				"password": "1234",
				"port": 3306,
				"type": "mysql",
				"user": "keatest",
				"readonly": false,
				"trust-anchor": "my-ca",
				"cert-file": "my-cert",
				"key-file": "my-key",
				"cipher-list": "AES",
				"reconnect-wait-time": 3000,
				"max-reconnect-tries": 3,
				"on-fail": "stop-retry-exit",
				"retry-on-startup": false,
				"connect-timeout": 100,
				"read-timeout": 120,
				"write-timeout": 180
			}
		], // Hosts reservations managed by MySQL database
		"hooks-libraries": [] // Not activated Host Commands hook
		// DHCPv4 Config ...
	}
}
```

#### Solution

Activate the Host Commands hook and interact with reservations via the API.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"hosts-databases": [
			{
				"name": "keatest",
				"host": "localhost",
				"password": "1234",
				"port": 3306,
				"type": "mysql",
				"user": "keatest",
				"readonly": false,
				"trust-anchor": "my-ca",
				"cert-file": "my-cert",
				"key-file": "my-key",
				"cipher-list": "AES",
				"reconnect-wait-time": 3000,
				"max-reconnect-tries": 3,
				"on-fail": "stop-retry-exit",
				"retry-on-startup": false,
				"connect-timeout": 100,
				"read-timeout": 120,
				"write-timeout": 180
			}
		], // Hosts reservations managed by MySQL database
		"hooks-libraries": [
		{
            "library": "libdhcp_host_cmds.so", // And activated Host Commands hook
            "parameters": {}
    	}
	]
	// DHCPv4 Config ...
	}
}
```


### NoBasicHTTPAuthInHAPeersRule

- **Codename** - HOOKS::NoBasicHTTPAuthInHAPeersRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/hooks.html#hot-standby-configuration

#### Problem

When using a high availability hook, peers in which do not have basic HTTP authorization may experience security problems and attempts to undesirably connect the high availability server to malicious servers. It is recommended to always specify the basic HTTP authorization in the parameters of the hook.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
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
									"role": "primary" // No basic HTTP auth
								},
								{
									"name": "server2",
									"url": "http://192.168.56.66:8005/", // No basic HTTP auth
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
		// DHCPv4 Config ...
	}
}
```

#### Solution

Specify the basic HTTP authorization in the parameters of the hook.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
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
									"role": "primary",
									"basic-auth-user": "qqq", // Enabled basic HTTP auth
									"basic-auth-password": "www",
								},
								{
									"name": "server2",
									"url": "http://192.168.56.66:8005/",
									"role": "secondary",
									"basic-auth-user": "eee", // Enabled basic HTTP auth
									"basic-auth-password": "fff",
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
		// DHCPv4 Config ...
	}
}
```


### NoMatchClientIdForFlexIDHookRule

- **Codename** - HOOKS::NoMatchClientIdForFlexIDHookRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/hooks.html#the-replace-client-id-flag

#### Problem

If the `replace-client-id` key is set to "true" in the Flex ID hook parameters, the `match-client-id` parameter must also be set to "true" in the global configuration, otherwise the hook will not work correctly.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"match-client-id": false, // It should have been "true"
		"hooks-libraries": [
			{
        "library": "libdhcp_flex_id.so",
        "parameters": {
            "identifier-expression": "substring(relay4[0].option[18].hex,0,8)",
            "replace-client-id": true // Expects the "match-client-id" parameter to be set to "true"
        	}
    	}]
		// DHCPv4 Config ...
	}
}
```

#### Solution

Activate the `match-client-id` parameters in the global configuration and the `replace-client-id` parameters in the hook configuration at the same time, or vice versa, disable them.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"match-client-id": true, // Enabled
		"hooks-libraries": [{
        "library": "libdhcp_flex_id.so",
        "parameters": {
            "identifier-expression": "substring(relay4[0].option[18].hex,0,8)",
            "replace-client-id": true // And it is also enabled
        	}
    	}]
		// DHCPv4 Config ...
	}
}
```


### UnnecessaryActivatedDatabaseHooksRule

- **Codename** - HOOKS::UnnecessaryActivatedDatabaseHooksRule.
- **Importance** - INFO.
- **Config type** - DHCPv4.
- **Articles** - [
https://kea.readthedocs.io/en/latest/arm/hooks.html#libdhcp-mysql-so-database-backend-for-mysql
https://kea.readthedocs.io/en/latest/arm/hooks.html#libdhcp-pgsql-so-database-backend-for-postgresql
]

#### Problem

Activating database maintenance hooks unnecessarily (they do not service configuration, leases, and reservations) can lead to a decrease in server performance.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		
		// Not specified "lease-database", "hosts-database", "hosts-databases" or "config-control" 
		"hooks-libraries": [
			{
				"library": "libdhcp_pgsql.so", // But activated database hook 
				"parameters": {}
			},
			{
				"library": "libdhcp_mysql.so", // But activated database hook 
				"parameters": {}
			}
		]
		// DHCPv4 Config ...
	}
}
```

#### Solution

Specify any database maintenance area unnecessarily (configuration, lease, or reservation storage) or disable database hooks.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"lease-database": {
			"name": "keatest",
			"host": "localhost",
			"password": "1234",
			"port": 3306,
			"type": "mysql",
			"user": "keatest",
			"on-fail": "serve-retry-continue"
		},
		"hosts-databases": [
			{
				"name": "keatest",
				"host": "localhost",
				"password": "1234",
				"port": 3306,
				"type": "mysql",
				"user": "keatest",
				"readonly": false,
				"trust-anchor": "my-ca",
				"cert-file": "my-cert",
				"key-file": "my-key",
				"cipher-list": "AES",
				"reconnect-wait-time": 3000,
				"max-reconnect-tries": 3,
				"on-fail": "stop-retry-exit",
				"retry-on-startup": false,
				"connect-timeout": 100,
				"read-timeout": 120,
				"write-timeout": 180
			}
		],
		"hooks-libraries": [
		{
			"library": "libdhcp_pgsql.so", // It serves host reservations
			"parameters": {}
		},
		{
			"library": "libdhcp_mysql.so", // It serves leases
			"parameters": {}
		}
		]
		// DHCPv4 Config ...
	}
}
```


### UseUsrCheckHookRule

- **Codename** - HOOKS::UseUsrCheckHookRule.
- **Importance** - INFO.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/hooks.html#libdhcp-user-chk-so-user-check

#### Problem

Activation of the User Check hook is recommended for training purposes. It is recommended to use the host reservation functionality to work in production.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"hooks-libraries": [
	    {
	        "library": "/usr/local/lib/kea/hooks/libdhcp_user_chk.so"
      	}]
		// DHCPv4 Config ...
	}
}
```

#### Solution

Deactivate hook and use host reservations functionality.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"hooks-libraries": [], // User Check hook deactivated
		"reservations": [{
				"hostname": "hostname",
				"hw-address": "2a:2b:2c:2d:2e:2f",
				"ip-address": "10.0.0.50"
			},
			{
				"hostname": "qqq",
				"hw-address": "3a:3b:3c:3d:3e:3f",
				"ip-address": "192.0.2.100"
		}]
		// DHCPv4 Config ...
	}
}
```
