
# Rules "Queue Control"

## Implemented For

- ✅ - Implemented for specified config.
- ❌ - NOT implemented for specified config.
- 🚫 - Cannot be implemented for the specified config.

|Rule name|DHCPv4|DHCPv6|DDNS|Control Agent|
|--|--|--|--|--|
|[NoEnableQueueAndMultithreadingTogetherRule](#NoEnableQueueAndMultithreadingTogetherRule)|✅|✅|🚫|🚫|


## Rules

### NoEnableQueueAndMultithreadingTogetherRule

- **Codename** - QUEUE_CONTROL::NoEnableQueueAndMultithreadingTogetherRule.
- **Importance** - INFO.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/congestion-handling.html#configuring-congestion-handling


#### Problem

The mechanism for managing the rent queue using the `dhcp-queue-control' key in the global configuration will not work if the multithreaded mode of the server is enabled.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"multi-threading": {
			"enable-multi-threading": true, // Enabled multithreading mode
			"thread-pool-size": 4,
			"packet-queue-size": 16
		},
		"dhcp-queue-control": {
	    "enable-queue": true, // But queue not working whire multithreading enabled
	    "queue-type": "queue type",
	    "capacity" : 256
	  }
		// DHCPv4 Config ...
	}
}
```


#### Solution

Disable the queue management mechanism or multithreaded server operation.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"multi-threading": {
			"enable-multi-threading": false, // Turn it "false" so that the queue works
			"thread-pool-size": 4,
			"packet-queue-size": 16
		},
		"dhcp-queue-control": {
	    "enable-queue": true, // Or disable it to save consistency
	    "queue-type": "queue type",
	    "capacity" : 256
	  }
		// DHCPv4 Config ...
	}
}
```
