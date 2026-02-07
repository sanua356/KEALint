# Rules "Subnets"

## Implemented For

- ✅ - Implemented for specified config.
- ❌ - NOT implemented for specified config.
- 🚫 - Cannot be implemented for the specified config.

|Rule name|DHCPv4|DHCPv6|DDNS|Control Agent|
|--|--|--|--|--|
|[SubnetsOverlappingRule](#SubnetsOverlappingRule)|✅|❌|🚫|🚫|
|[SubnetsPoolsIntersectionRule](#SubnetsPoolsIntersectionRule)|✅|❌|🚫|🚫|
|[SubnetWithoutPoolsAndReservationsRule](#SubnetWithoutPoolsAndReservationsRule)|✅|❌|🚫|🚫|

## Rules

### SubnetsOverlappingRule

- **Codename** - SUBNETS::SubnetsOverlappingRule.
- **Importance** - CRITICAL.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#configuration-of-ipv4-address-pools

#### Problem

Subnet prefixes may overlap each other. It is recommended to avoid such behavior in order to avoid conflicts.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"subnet4": [
			{
				"id": 3,
				"subnet": "1.2.3.0/24",
				"pools": [
					{
						"pool": "1.2.3.0/24"
					}
				]
			},
			{
				"id": 4,
				"subnet": "1.0.0.0/8", // Overlap subnet with id: 3
				"pools": [
					{
						"pool": "1.0.0.0-1.5.6.7"
					}
				]
			}
		]
		// DHCPv4 Config ...
	}
}
```

#### Solution

Change the CIDR of the subnets so that they do not overlap.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"subnet4": [
			{
				"id": 3,
				"subnet": "1.2.3.0/24",
				"pools": [
					{
						"pool": "1.2.3.0/24"
					}
				]
			},
			{
				"id": 4,
				"subnet": "1.2.4.0/24", // No overlap any
				"pools": [
				{
						"pool": "1.2.4.0/24"
				}
				]
			}
		]
		// DHCPv4 Config ...
	}
}
```


### SubnetsPoolsIntersectionRule

- **Codename** - SUBNETS::SubnetsPoolsIntersectionRule.
- **Importance** - CRITICAL.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#configuration-of-ipv4-address-pools

#### Problem

Subnets pools may intersection each other. It is recommended to avoid such behavior in order to avoid conflicts.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"subnet4": [
			{
				"id": 3,
				"subnet": "1.2.3.0/24",
				"pools": [
					{
						"pool": "1.2.3.0/24"
					}
				]
			},
			{
				"id": 4,
				"subnet": "1.0.0.0/8",
				"pools": [
					{
						"pool": "1.0.0.0-1.5.6.7" // Intersection with pool: 1.2.3.0/24
					},
					{
						"pool": "1.8.8.10-1.8.8.20"
					}
				]
			}
		]
		// DHCPv4 Config ...
	}
}
```

#### Solution

Change the subnet pools so that they do not intersect.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"subnet4": [
			{
				"id": 3,
				"subnet": "1.2.3.0/24",
				"pools": [
					{
						"pool": "1.2.3.0/24"
					}
				]
			},
			{
				"id": 4,
				"subnet": "1.4.2.0/24",
				"pools": [
					{
						"pool": "1.4.2.10-1.4.2.20" // No intersection any
					},
					{
						"pool": "1.8.8.10-1.8.8.20"
					}
				]
			}
		]
		// DHCPv4 Config ...
	}
}
```

### SubnetWithoutPoolsAndReservationsRule

- **Codename** - SUBNETS::SubnetWithoutPoolsAndReservationsRule.
- **Importance** - INFO.
- **Config type** - DHCPv4.
- **Articles** - No article.

#### Problem

Subnets without pools and reservations do not perform useful work and can be deleted without losing server functionality.

> Check is not performed if the Host CMDs hook is activated, because subnet reservations may be stored in a third-party database.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"subnet4": [
			{
				"id": 3,
				"subnet": "1.2.3.0/24",
				"pools": [] // Empty subnet
			},
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
						"subnet": "10.0.0.0/8" // Empty subnet
					}
				]
			},
			{
				"name": "qqq",
				"interface": "eth0",
				"subnet4": [
					{
						"id": 11,
						"subnet": "11.0.0.0/8",
						"pools": [
							{
								"pool": "11.0.0.0-1.5.6.7"
							},
							{
								"pool": "11.8.8.10-1.8.8.20"
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

Remove subnets without pools and reservations from configuration.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"subnet4": [ // Removed empty subnet
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
				]
			}
		],
		"shared-networks": [ // Removed shared network with empty subnet
			{
				"name": "qqq",
				"interface": "eth0",
				"subnet4": [
					{
						"id": 11,
						"subnet": "11.0.0.0/8",
						"pools": [
							{
								"pool": "11.0.0.0-1.5.6.7"
							},
							{
								"pool": "11.8.8.10-1.8.8.20"
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
