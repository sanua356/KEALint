# Rules "Allocator"

## Implemented For

- ✅ - Implemented for specified config.
- ❌- NOT implemented for specified config.
- 🚫 - Cannot be implemented for the specified config.

|Rule name|DHCPv4|DDNS|Control Agent|
|--|--|--|--|
|[NotSelectFLQAllocatorInGlobalLevelConfig](#NotSelectFLQAllocatorInGlobalLevelConfig)|✅|🚫|🚫|
|[NotSelectIterativeAllocatorForSharedLeaseDatabase](#NotSelectIterativeAllocatorForSharedLeaseDatabase)|✅|🚫|🚫|

## Rules

### NotSelectFLQAllocatorInGlobalLevelConfig

- **Codename** - ALLOCATOR::NotSelectFLQAllocatorInGlobalLevelConfig.
- **Importance** - INFO.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#free-lease-queue-allocator

#### Problem

For example, using it for a subnet with a `/8` pool may delay the server's startup by 15 seconds or more. On the other hand, the startup delay and the memory consumption increase should be acceptable for subnets with a `/16` pool or smaller. We also recommend specifying another allocator type in the global configuration settings and overriding this selection at the subnet or shared-network level, to use the FLQ allocator only for selected subnets. That way, when a new subnet is added without an allocator specification, the global setting is used, thus avoiding unnecessary impact on the server's startup time.

```json
{
	"Dhcp4": {
    // DHCPv4 Config ...
    "allocator": "flq",
    // DHCPv4 Config ...
	}
}
```

#### Solution

Specify a "random" or "iteratve" address allocator in global level configuration.

```json
{
	"Dhcp4": {
	    // DHCPv4 Config ...
    "allocator": "random", // Or "iterative"
    // DHCPv4 Config ...
	}
}
```


### NotSelectIterativeAllocatorForSharedLeaseDatabase

- **Codename** - ALLOCATOR::NotSelectIterativeAllocatorForSharedLeaseDatabase.
- **Importance** - INFO.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#iterative-allocator

#### Problem 

The iterative allocation underperforms when multiple DHCP servers share a lease database or are connected to a cluster. The servers tend to offer and allocate the same blocks of addresses to different clients independently, which causes many allocation conflicts between the servers and retransmissions by clients. A random allocation addresses this issue by dispersing the allocation order.

```json
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
    "allocator": "iterative",
    // DHCPv4 Config ...
	}
}
```

#### Solution

Specify a "random" address allocator if a shared database of rents is used for several servers.

```json
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
    "allocator": "random",
    // DHCPv4 Config ...
	}
}
```

> The utility cannot explicitly check whether multiple KEA servers are actually being used inside the same database, so "importance" is set as "INFO", not "WARNING".
