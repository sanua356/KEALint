
# Rules "Interfaces"

## Implemented For

- ✅ - Implemented for specified config.
- ❌ - NOT implemented for specified config.
- 🚫 - Cannot be implemented for the specified config.

|Rule name|DHCPv4|DHCPv6|DDNS|Control Agent|
|--|--|--|--|--|
|[NoInterfacesInInterfacesConfigRule](#NoInterfacesInInterfacesConfigRule)|✅|✅|🚫|🚫|


## Rules

### NoInterfacesInInterfacesConfigRule

- **Codename** - INTERFACES::NoInterfacesInInterfacesConfigRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4, DHCPv6.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp6-srv.html#interface-configuration

#### Problem

If no network interfaces in key `interfaces` is specified, the KEA server will not serve clients.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"interfaces-config": {
			"interfaces": []
		}
		// DHCPv4 Config ...
	}
}
```

#### Solution

Specify network interfaces using the `interfaces` key.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"interfaces-config": {
			"interfaces": ["eth0", "enp1s0"] // Specified network interfaces
		}
		// DHCPv4 Config ...
	}
}
```
