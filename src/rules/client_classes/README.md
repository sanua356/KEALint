# Rules "Client Classes"

## Implemented For

- ✅ - Implemented for specified config.
- ❌ - NOT implemented for specified config.
- 🚫 - Cannot be implemented for the specified config.

|Rule name|DHCPv4|DDNS|Control Agent|
|--|--|--|--|
|[EvaluateRequiredAsAdditionalClassesRule](#EvaluateRequiredAsAdditionalClassesRule)|✅|🚫|🚫|
|[NotValidLifetimeForAdditionalClassesRule](#NotValidLifetimeForAdditionalClassesRule)|✅|🚫|🚫|
|[NotRecommendedPrefixAFTER_ClassesRule](#NotRecommendedPrefixAFTER_ClassesRule)|✅|🚫|🚫|

## Rules

### EvaluateRequiredAsAdditionalClassesRule

- **Codename** - CLIENT_CLASSES::NotRecommendedPrefixAFTER_ClassesRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#additional-classification

#### Problem

Client classes that do not have the `only-in-additional-list` with value "true" flag, but are specified in a subnet or shared network inside `evaluate-additional-classes` will not work correctly, because they are processed by the server at different stages of packet processing.

```js
{
	"Dhcp4": {
	// DHCPv4 config...
	 	"client-classes": [
			{
				"name": "test_required_class",
				"test": "",
				"only-in-additional-list": false,
			}
		],
    	"subnet4": [{
			"id": 3,
			"subnet": "1.2.3.0/24",
			"evaluate-additional-classes": [
				"test_required_class"
			],
			"pools": [
				{
					"pool": "1.2.3.0/24",
					"evaluate-additional-classes": [
						"test_required_class"
					]
				}
			]
     	}],
		"shared-networks": [{
			"name": "my-secret-lair-level-1",
			"interface": "eth0",
			"evaluate-additional-classes": [
				"test_required_class"
			],
			"subnet4": [
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
		}]
	// DHCPv4 Config...
	}
}
```

#### Solution

Set the `only-in-additional-list` flag to "true" for the required client classes or remove it from the `evaluate-additional-classes` area.

```js
{
	"Dhcp4": {
	// DHCPv4 config...
	 	"client-classes": [
			{
				"name": "test_required_class",
				"test": "",
				"only-in-additional-list": true,
			}
		],
	    "subnet4": [{
			"id": 3,
			"subnet": "1.2.3.0/24",
			"evaluate-additional-classes": [
				"test_required_class" // Or remove from here
			],
			"pools": [
				{
					"pool": "1.2.3.0/24",
					"evaluate-additional-classes": [
						"test_required_class"
					]
				}
			]
		}],
		"shared-networks": [{
			"name": "my-secret-lair-level-1",
			"interface": "eth0",
			"evaluate-additional-classes": [
				"test_required_class" // Or remove from here
			],
			"subnet4": [
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
		}]
	// DHCPv4 Config...
	}
}
```


### NotValidLifetimeForAdditionalClassesRule

- **Codename** - CLIENT_CLASSES::NotValidLifetimeForAdditionalClassesRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/stable/arm/classify.html#class-priority

#### Problem

Client classes with the `only-in-additional-list` parameter set to "true" and having the `min/max/offer/-valid-lifetime` parameters will ignore the lease lifetime settings. Because additional evaluation occurs after lease assignment, parameters that would otherwise impact lease life times.

```js
{
	"Dhcp4": {
	// DHCPv4 Config...
	    "valid-lifetime": 4000,
	    "renew-timer": 1000,
	    "rebind-timer": 2000,
	    "client-classes": [
	    	{
				"name": "test_not_required_class",
				"test": "",
				"only-in-additional-list": true,
				"valid-lifetime": 4000, // Not working while - "only-in-additional-list": true
				"min-valid-lifetime": 5000, // Not working while - "only-in-additional-list": true
				"max-valid-lifetime": 6000, // Not working while - "only-in-additional-list": true
				"offer-lifetime": 7000, // Not working while - "only-in-additional-list": true
			}
		]
	// DHCPv4 Config...
	}
}
```

#### Solution

Set the `only-in-additional-list` flag to "false" for client classes or remove lifetime parameters from the class area.

```js
{
	"Dhcp4": {
	// DHCPv4 Config...
	    "valid-lifetime": 4000,
	    "renew-timer": 1000,
	    "rebind-timer": 2000,
	    "client-classes": [
    	    {
				"name": "test_not_required_class",
				"test": "",
				"only-in-additional-list": false,
				"valid-lifetime": 4000, // Or remove it
				"min-valid-lifetime": 5000, // Or remove it
				"max-valid-lifetime": 6000, // Or remove it
				"offer-lifetime": 7000, // Or remove it
         	}
		]
	// DHCPv4 Config...
	}
}
```


### NotRecommendedPrefixAFTER_ClassesRule

- **Codename** - CLIENT_CLASSES::NotRecommendedPrefixAFTER_ClassesRule.
- **Importance** - INFO.
- **Config type** - DHCPv4.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/classify.html#built-in-client-classes

#### Problem

The prefix "AFTER_" in the names of client classes is intended for a hook that has not yet been written. It can be used, but it is likely that the behavior of the client class may be disrupted when the hook is implemented.

```js
{
	"Dhcp4": {
	// DHCPv4 Config...
	    "client-classes": [
	    	{
				"name": "AFTER_test_not_required_class",
				"test": ""
			}
		]
	// DHCPv4 Config...
	}
}
```

#### Solution

Remove prefix "AFTER_" from name client classes.

```js
{
	"Dhcp4": {
	// DHCPv4 Config...
	    "client-classes": [
	    	{
				"name": "test_not_required_class",
				"test": ""
			}
		]
	// DHCPv4 Config...
	}
}
```
