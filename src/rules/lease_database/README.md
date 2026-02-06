
# Rules "Lease Database"

## Implemented For

- ✅ - Implemented for specified config.
- ❌ - NOT implemented for specified config.
- 🚫 - Cannot be implemented for the specified config.

|Rule name|DHCPv4|DHCPv6|DDNS|Control Agent|
|--|--|--|--|--|
|[LeaseSanityChecksEnabledForNotMemfileBackend](#LeaseSanityChecksEnabledForNotMemfileBackend)|✅|✅|🚫|🚫|
|[NoEnabledPersistFlagForMemfileLeases](#NoEnabledPersistFlagForMemfileLeases)|✅|✅|🚫|🚫|
|[NotChangeStopRetryExitStrategyOnFailRule](#NotChangeStopRetryExitStrategyOnFailRule)|✅|✅|🚫|🚫|

## Rules

### LeaseSanityChecksEnabledForNotMemfileBackend

- **Codename** - LEASE_DATABASE::LeaseSanityChecksEnabledForNotMemfileBackend.
- **Importance** - INFO.
- **Config type** - DHCPv4, DHCPv6.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#sanity-checks-in-dhcpv4

#### Problem

Lease Sanity Checks are implemented only for the 'memfile' of the rent storage. If you are using a third-party database of rents, Sanity Checks by key `sanity-checks` will not work correctly.

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
		"sanity-checks": {
			"lease-checks": "warn" // Enabled sanity checks for MySQL backend (unsupported)
		} 
		// DHCPv4 Config ...
	}
}
```

#### Solution

Remove `sanity-checks` from the configuration or replace the lease storage with a 'memfile'.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"lease-database": {
			"type": "memfile",
			"persist": true,
			"name": "/var/lib/kea/dhcp4.leases"
		},
		"sanity-checks": {
			"lease-checks": "warn" // Enabled sanity checks for Memfile backend (supported)
		} 
		// DHCPv4 Config ...
	}
}
```


### NoEnabledPersistFlagForMemfileLeases

- **Codename** - LEASE_DATABASE::NoEnabledPersistFlagForMemfileLeases.
- **Importance** - CRITICAL.
- **Config type** - DHCPv4, DHCPv6.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#memfile-basic-storage-for-leases

#### Problem

The `persist` flag in the `lease-database` configuration determines whether leases will be written to a file on disk. It is recommended to always enable it in order to avoid loss of rental data when the server is turned off.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"lease-database": {
			"type": "memfile",
			"persist": false, // writing addresses leases on disk disabled
			"name": "/var/lib/kea/dhcp4.leases"
		}
		// DHCPv4 Config ...
	}
}
```

#### Solution

Enable `persist` flag in the `lease-database` configuration.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"lease-database": {
			"type": "memfile",
			"persist": true, // writing addresses leases on disk enabled
			"name": "/var/lib/kea/dhcp4.leases"
		}
		// DHCPv4 Config ...
	}
}
```


### NotChangeStopRetryExitStrategyOnFailRule

- **Codename** - LEASE_DATABASE::NotChangeStopRetryExitStrategyOnFailRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4, DHCPv6.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp6-srv.html#lease-database-configuration

#### Problem

When working with third-party rent databases, it is recommended to specify the strategy for dealing with connection errors using the `on-fail` key only in the `stop-retry-exit` mode. Other modes are recommended for informational purposes.

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
			"on-fail": "serve-retry-continue" // A flawed strategy in most cases
		}
		// DHCPv4 Config ...
	}
}
```

#### Solution

Change `on-fail` strategy in `lease-database` configuration to `stop-retry-exit` value.

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
			"on-fail": "stop-retry-exit" // Good strategy in most cases
		}
		// DHCPv4 Config ...
	}
}
```
