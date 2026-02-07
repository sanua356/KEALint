# Rules "Option Data"

## Implemented For

- ✅ - Implemented for specified config.
- ❌ - NOT implemented for specified config.
- 🚫 - Cannot be implemented for the specified config.

|Rule name|DHCPv4|DHCPv6|DDNS|Control Agent|
|--|--|--|--|--|
|[IncompleteOctetsBytesInOptionValuesRule](#IncompleteOctetsBytesInOptionValuesRule)|✅|❌|🚫|🚫|
|[SpecifiedKEAManagedOptionsRule](#SpecifiedKEAManagedOptionsRule)|✅|❌|🚫|🚫|


## Rules

### IncompleteOctetsBytesInOptionValuesRule

- **Codename** - OPTION_DATA::IncompleteOctetsBytesInOptionValuesRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#standard-dhcpv4-options

> Examples will be shown for `option-data` at the global configuration level, but they are also checked at the subnet, pool, reservation, and shared network levels.

#### Problem

Options with the `csv-format` flag set to `false` and the `data` key data having a non-paired byte string ("A A A" instead of "0A 0A 0A") they may be misinterpreted. It is recommended to always set byte values in pairs.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"option-data": [
			{
				"name": "dhcp-client-identifier",
				"csv-format": false, // Enabled bytes data format
				"data": "11 2" // No pairs bytes
			},
			{
				"name": "dhcp-lease-time",
				"csv-format": false, // Enabled bytes data format
				"data": "0xee1" // No pairs bytes
			}
		]
		// DHCPv4 Config ...
	}
}
```


#### Solution

Set byte values in pairs for all option values.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"option-data": [
			{
				"name": "dhcp-client-identifier",
				"csv-format": false, // Enabled bytes data format
				"data": "11 22" // Pairs bytes
			},
			{
				"name": "dhcp-lease-time",
				"csv-format": false, // Enabled bytes data format
				"data": "0xee12" // Pairs bytes
			}
		]
		// DHCPv4 Config ...
	}
}
```


### SpecifiedKEAManagedOptionsRule

- **Codename** - OPTION_DATA::SpecifiedKEAManagedOptionsRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#id6

#### Problem

There are options that are controlled by the KEA engine and it is not recommended to override them manually.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"option-data": [
			{
				"code": 1, // Option with code 1 managed by KEA engine
				"data": "hostname"
			}
		]
		// DHCPv4 Config ...
	}
}
```

#### Problem

Do not specify or redefine the option numbers controlled by the KEA engine.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"option-data": [
			{
				"code": 6, // Option with code 6 managed by user
				"data": "hostname"
			}
		]
		// DHCPv4 Config ...
	}
}
```
