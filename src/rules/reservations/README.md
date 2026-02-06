# Rules "Reservations"

## Implemented For

- ✅ - Implemented for specified config.
- ❌ - NOT implemented for specified config.
- 🚫 - Cannot be implemented for the specified config.

|Rule name|DHCPv4|DHCPv6|DDNS|Control Agent|
|--|--|--|--|--|
|[AllReservationsOutOfPoolsRule](#AllReservationsOutOfPoolsRule)|✅|❌|🚫|🚫|
|[DisabledInSubnetReservationsWithEnabledOutOfPool](#DisabledInSubnetReservationsWithEnabledOutOfPool)|✅|❌|🚫|🚫|
|[GlobalReservationsOccupyDynamicPoolsRule](#GlobalReservationsOccupyDynamicPoolsRule)|✅|❌|🚫|🚫|


## Rules

### AllReservationsOutOfPoolsRule

- **Codename** - RESERVATIONS::AllReservationsOutOfPoolsRule.
- **Importance** - INFO.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#fine-tuning-dhcpv4-host-reservation

#### Problem

If all reservations within a subnet are defined outside the dynamic pools of that subnet, you can set the `reservations-out-of-pool` flag to 'true' to speed up server operation.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"subnet4": [
			{
				"id": 4,
				"subnet": "1.0.0.0/8",
				"pools": [
					{
						"pool": "1.0.0.0-1.5.6.7"
					},
					{
						"pool": "1.8.8.10-1.8.8.20"
					}
				],
				"reservations": [ // All reservations out of pools
					{
						"hostname": "special-snowflake",
						"hw-address": "1a:1b:1c:1d:1e:1f",
						"ip-address": "1.6.0.100"
					},
					{
						"hostname": "out_of_pool_reservation",
						"hw-address": "11:22:33:44:55:66",
						"ip-address": "1.8.8.30"
					}
				]
			}
		],
		"shared-networks": [
			{
				"name": "my-secret-lair-level-1",
				"interface": "eth0",
				"subnet4": [
					{
						"id": 1,
						"subnet": "10.0.0.0/8",
						"pools": [
							{
								"pool": "10.0.0.1 - 10.0.0.99"
							}
						],
						"reservations": [ // All reservations out of pools
							{
								"hostname": "hostname",
								"hw-address": "2a:2b:2c:2d:2e:2f",
								"ip-address": "10.0.0.150"
							}
						]
					}
				]
			}
		]
		// DHCPv4 Config ...
	}
}
```


#### Solution

Define reservations within the established pools or set the `reservations-out-of-pool` flag to 'true'.

> With the 'reservations-out-of-pool' flag set to 'true', the subnet stops checking leases in dynamic pools. Use this flag with caution.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"subnet4": [
			{
				"id": 4,
				"subnet": "1.0.0.0/8",
				"pools": [
					{
						"pool": "1.0.0.0-1.5.6.7"
					},
					{
						"pool": "1.8.8.10-1.8.8.20"
					}
				],
				"reservations-out-of-pool": true, // Can significantly increase performance
				"reservations": [
					{
						"hostname": "special-snowflake",
						"hw-address": "1a:1b:1c:1d:1e:1f",
						"ip-address": "1.6.0.100"
					},
					{
						"hostname": "out_of_pool_reservation",
						"hw-address": "11:22:33:44:55:66",
						"ip-address": "1.8.8.30"
					}
				]
			}
		],
		"shared-networks": [
			{
				"name": "my-secret-lair-level-1",
				"interface": "eth0",
				"subnet4": [
					{
						"id": 1,
						"subnet": "10.0.0.0/8",
						"pools": [
							{
								"pool": "10.0.0.1 - 10.0.0.99"
							}
						],
						"reservations": [ 
							{
								"hostname": "hostname",
								"hw-address": "2a:2b:2c:2d:2e:2f",
								"ip-address": "10.0.0.90" // Moved in pool range
							}
						]
					}
				]
			}
		]
		// DHCPv4 Config ...
	}
}
```


### DisabledInSubnetReservationsWithEnabledOutOfPool

- **Codename** - RESERVATIONS::DisabledInSubnetReservationsWithEnabledOutOfPool.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#fine-tuning-dhcpv4-host-reservation

#### Problem

If the `reservations-in-subnet` flag is set to "false" in the subnet or at configuration levels above, and the `reservations-out-of-pool` flag is set to "false" in the subnet, this may lead to incorrect behavior when issuing leases.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"reservations-in-subnet": false, // Disabled subnet reservations in global layer configuration
		"subnet4": [
			{
				"id": 4,
				"subnet": "1.0.0.0/8",
				"reservations-in-subnet": true,
				"reservations-out-of-pool": true,
				"pools": []
			},
			{
				"id": 5,
				"subnet": "2.0.0.0/8",
				"reservations-out-of-pool": true, // This will not work, the 'reservations-in-subnet' flag is disabled above
				"pools": []
			}
		],
		"shared-networks": [
			{
				"name": "qqq",
				"interface": "eth1",
				"reservations-in-subnet": false,
				"subnet4": [
					{
						"id": 1,
						"subnet": "10.0.0.0/8",
						"pools": [],
						"reservations-in-subnet": true, 
						"reservations-out-of-pool": true
					},
					{
						"id": 2,
						"subnet": "20.0.0.0/8",
						"pools": [],
						"reservations-out-of-pool": true  // This will not work, the 'reservations-in-subnet' flag is disabled above
					}
				]
			}
		]
		// DHCPv4 Config ...
	}
}
```

#### Solution

Set the `reservations-in-subnet` flag to 'true' or set the `reservations-out-of-pool` flag to 'false'.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"reservations-in-subnet": true, // Enable subnet reservations in global layer configuration
		"subnet4": [
			{
				"id": 4,
				"subnet": "1.0.0.0/8",
				"reservations-in-subnet": true,
				"reservations-out-of-pool": true,
				"pools": []
			},
			{
				"id": 5,
				"subnet": "2.0.0.0/8",
				"reservations-out-of-pool": true, // This work, the 'reservations-in-subnet' flag is enabled above
				"pools": []
			}
		],
		"shared-networks": [
			{
				"name": "qqq",
				"interface": "eth1",
				"reservations-in-subnet": true,
				"subnet4": [
					{
						"id": 1,
						"subnet": "10.0.0.0/8",
						"pools": [],
						"reservations-in-subnet": true, 
						"reservations-out-of-pool": true
					},
					{
						"id": 2,
						"subnet": "20.0.0.0/8",
						"pools": [],
						"reservations-out-of-pool": true  // This work, the 'reservations-in-subnet' flag is enabled above
					}
				]
			}
		]
		// DHCPv4 Config ...
	}
}
```


### GlobalReservationsOccupyDynamicPoolsRule

- **Codename** - RESERVATIONS::GlobalReservationsOccupyDynamicPoolsRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#conflicts-in-dhcpv4-reservations

#### Problem

Reservations specified at the global level that overlap with dynamic subnet pools may cause conflicts when issuing leases.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"reservations-global": true,
		"reservations": [
			{
				"hostname": "hostname",
				"hw-address": "2a:2b:2c:2d:2e:2f",
				"ip-address": "1.0.0.50" // Intersects with the subnet pool
			},
			{
				"hostname": "qqq",
				"hw-address": "3a:3b:3c:3d:3e:3f",
				"ip-address": "10.0.0.150" // Intersects with the shared network pool
			}
		],
		"subnet4": [
			{
				"id": 4,
				"subnet": "1.0.0.0/8",
				"pools": [{"pool": "1.0.0.1-1.0.0.80"}]
			}
		],
		"shared-networks": [
			{
				"name": "qqq",
				"interface": "eth1",
				"subnet4": [
					{
						"id": 1,
						"subnet": "10.0.0.0/8",
						"pools": [{"pool": "10.0.0.100-10.0.0.200"}]
					}
				]
			}
		]
		// DHCPv4 Config ...
	}
}
```

#### Solution

Change the reservation addresses or subnet pool ranges so that there is no overlap between global reservations and pools.
Or may be disable global reservations on key `reservations-global`.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"reservations-global": true,
		"reservations": [
			{
				"hostname": "hostname",
				"hw-address": "2a:2b:2c:2d:2e:2f",
				"ip-address": "1.0.0.100" // Address changed
			},
			{
				"hostname": "qqq",
				"hw-address": "3a:3b:3c:3d:3e:3f",
				"ip-address": "10.0.0.150"
			}
		],
		"subnet4": [
			{
				"id": 4,
				"subnet": "1.0.0.0/8",
				"pools": [{"pool": "1.0.0.1-1.0.0.80"}]
			}
		],
		"shared-networks": [
			{
				"name": "qqq",
				"interface": "eth1",
				"subnet4": [
					{
						"id": 1,
						"subnet": "10.0.0.0/8",
						"pools": [{"pool": "10.0.0.100-10.0.0.140"}] // Pool range changed
					}
				]
			}
		]
		// DHCPv4 Config ...
	}
}
```
