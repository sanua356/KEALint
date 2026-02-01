# Rules "Shared Networks"

## Implemented For

- ✅ - Implemented for specified config.
- ❌ - NOT implemented for specified config.
- 🚫 - Cannot be implemented for the specified config.

|Rule name|DHCPv4|DDNS|Control Agent|
|--|--|--|--|
|[InterfaceOrRelaysInsideSubnetsSharedNetworksRule](#InterfaceOrRelaysInsideSubnetsSharedNetworksRule)|✅|🚫|🚫|
|[MissingSubnetIdSharedNetworksWithHostDatabases](#MissingSubnetIdSharedNetworksWithHostDatabases)|✅|🚫|🚫|
|[OneSubnetInSharedNetworksRule](#OneSubnetInSharedNetworksRule)|✅|🚫|🚫|
|[SameHostReservationsInDifferentSubnetsSharedNetworksRule](#SameHostReservationsInDifferentSubnetsSharedNetworksRule)|✅|🚫|🚫|

## Rules

### InterfaceOrRelaysInsideSubnetsSharedNetworksRule

- **Codename** - SHARED_NETWORKS::InterfaceOrRelaysInsideSubnetsSharedNetworksRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#local-and-relayed-traffic-in-shared-networks

#### Problem

In shared networks, it is recommended to specify `relay` and `interfaces` at the level of the shared network, instead of specifying at the level of each subnet. Ignoring the recommendation can lead to unexpected behavior.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"shared-networks": [
			{
				"name": "my-secret-lair-level-1",
				"subnet4": [
					{
						"id": 1,
						"subnet": "10.0.0.0/8",
						"interface": "eth0", // Not recommended set in subnet layer configuration
						"pools": [
							{
								"pool": "10.0.0.1 - 10.0.0.99"
							}
						]
					},
					{
						"id": 2,
						"subnet": "20.0.0.0/8",
						"relay": {
							"ip-addresses": ["1.2.3.4"] // Not recommended set in subnet layer configuration
						},
						"pools": [
							{
								"pool": "20.0.0.1 - 20.0.0.99"
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

Move the `relay` and `interfaces` to the shared network layer.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"shared-networks": [
			{
				"name": "my-secret-lair-level-1",
				"interface": "eth0", // Moved to shared network layer configuration
				"relay": {
					"ip-addresses": ["1.2.3.4"] // Moved to shared network layer configuration
				},
				"subnet4": [
					{
						"id": 1,
						"subnet": "10.0.0.0/8",
						 
						"pools": [
							{
								"pool": "10.0.0.1 - 10.0.0.99"
							}
						]
					},
					{
						"id": 2,
						"subnet": "20.0.0.0/8",
						"pools": [
							{
								"pool": "20.0.0.1 - 20.0.0.99"
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


### MissingSubnetIdSharedNetworksWithHostDatabases

- **Codename** - SHARED_NETWORKS::MissingSubnetIdSharedNetworksWithHostDatabases.
- **Importance** - INFO.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#local-and-relayed-traffic-in-shared-networks

#### Problem

If a third-party database is used to store host reservations, it is recommended to specify unique IDs for shared networks and subnets to avoid unexpected behavior.

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
				"user": "keatest"
			}
		],
		"shared-networks": [
			{
				"name": "my-secret-lair-level-1",
				"subnet4": [
					{
						// Not exists "id" key
						"subnet": "10.0.0.0/8",
						"interface": "eth0",
						"pools": [
							{
								"pool": "10.0.0.1 - 10.0.0.99"
							}
						]
					},
					{
						"id": 2,
						"subnet": "20.0.0.0/8",
						"relay": {
							"ip-addresses": ["1.2.3.4"]
						},
						"pools": [
							{
								"pool": "20.0.0.1 - 20.0.0.99"
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

Specify unique IDs for shared networks and subnets.

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
				"user": "keatest"
			}
		],
		"shared-networks": [
			{
				"name": "my-secret-lair-level-1",
				"subnet4": [
					{
						"id": 100, // Setted unique id 
						"subnet": "10.0.0.0/8",
						"interface": "eth0",
						"pools": [
							{
								"pool": "10.0.0.1 - 10.0.0.99"
							}
						]
					},
					{
						"id": 2,
						"subnet": "20.0.0.0/8",
						"relay": {
							"ip-addresses": ["1.2.3.4"]
						},
						"pools": [
							{
								"pool": "20.0.0.1 - 20.0.0.99"
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


### OneSubnetInSharedNetworksRule

- **Codename** - SHARED_NETWORKS::OneSubnetInSharedNetworksRule.
- **Importance** - INFO.
- **Config type** - DHCPv4.
- **Articles** - No article.

#### Problem

If only one subnet is specified in the shared network, it is recommended to move this subnet to the array using the `subnet4` key of the global configuration in order to optimize performance.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"shared-networks": [
			{
				"name": "my-secret-lair-level-1",
				"interface": "eth0",
				"subnet4": [ // Only one subnet
					{
						"id": 1,
						"subnet": "10.0.0.0/8",
						"pools": [
							{
								"pool": "10.0.0.1 - 10.0.0.99"
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

Move subnet to `subnet4` array in global configuration.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"subnet4": [ // Moved to global 'subnet4' array
			{
				"id": 1,
				"subnet": "10.0.0.0/8",
				"pools": [
					{
						"pool": "10.0.0.1 - 10.0.0.99"
					}
				]
			}
		]
		// DHCPv4 Config ...
	}
}
```


### SameHostReservationsInDifferentSubnetsSharedNetworksRule

- **Codename** - SHARED_NETWORKS::SameHostReservationsInDifferentSubnetsSharedNetworksRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#host-reservations-in-shared-networks

#### Problem

If the shared network contains the same reservations in different subnets, it is recommended to leave reservations in only one of them.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"shared-networks": [
			{
				"name": "qqq",
				"subnet4": [
					{
						"id": 11,
						"subnet": "254.254.254.254/32",
						"pools": [],
						"interface": "eth1",
						"reservations": [
							{
								"hostname": "qqq",
								"hw-address": "2a:2b:2c:2d:2e:2f", // Same HW address
								"ip-address": "10.0.0.150"
							}
						]
					},
					{
						"id": 12,
						"subnet": "253.253.253.253/32",
						"reservations": [
							{
								"hostname": "www",
								"hw-address": "2a:2b:2c:2d:2e:2f", // Same HW address
								"ip-address": "10.0.0.160"
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

Delete reservations from all subnets except one.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"shared-networks": [
			{
				"name": "qqq",
				"subnet4": [
					{
						"id": 11,
						"subnet": "254.254.254.254/32",
						"pools": [],
						"interface": "eth1",
						"reservations": [
							{
								"hostname": "qqq",
								"hw-address": "2a:2b:2c:2d:2e:2f", // Unique HW address
								"ip-address": "10.0.0.150"
							}
						]
					},
					{
						"id": 12,
						"subnet": "253.253.253.253/32",
						"reservations": [] // Removed same reservation
					}
				]
			}
		]
		// DHCPv4 Config ...
	}
}
```
