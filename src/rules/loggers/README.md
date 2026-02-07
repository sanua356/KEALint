
# Rules "Loggers"

## Implemented For

- ✅ - Implemented for specified config.
- ❌ - NOT implemented for specified config.
- 🚫 - Cannot be implemented for the specified config.

|Rule name|DHCPv4|DHCPv6|DDNS|Control Agent|
|--|--|--|--|--|
|[NoDebugLoggersRule](#NoDebugLoggersRule)|✅|✅|✅|✅|
|[NoLinebreakMessagesLoggersRule](#NoLinebreakMessagesLoggersRule)|✅|✅|✅|✅|
|[NoPercentMMessagesLoggersCtrlAgentRule](#NoPercentMMessagesLoggersCtrlAgentRule)|✅|✅|✅|✅|

> The examples in the rules below are for the "Dhcp4" configuration, but they are also relevant for "D2" and "Control Agent".

## Rules

### NoDebugLoggersRule

- **Codename** - LOGGERS::NoDebugLoggersRule.
- **Importance** - INFO.
- **Config type** - DHCPv4, DHCPv6, D2, Control Agent.
- **Articles** - No article.

#### Problem

Logging at the 'DEBUG' level can be very detailed and slow down the server. It is recommended to disable logging of this level in production.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"loggers": [
			{
				"name": "kea-dhcp4",
				"output-options": [
					{
						"output": "kea-dhcp4.log",
						"maxsize": 52428800,
						"maxver": 100,
						"pattern": "%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m\n"
					}
				],
				"severity": "DEBUG", // DEBUG level logging (may be slow)
				"debuglevel": 10
			}
		]
		// DHCPv4 Config ...
	}
}
```


#### Solution

Change the logging level (for example, to "INFO").

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"loggers": [
			{
				"name": "kea-dhcp4",
				"output-options": [
					{
						"output": "kea-dhcp4.log",
						"maxsize": 52428800,
						"maxver": 100,
						"pattern": "%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m\n"
					}
				],
				"severity": "INFO", // INFO level logging (may be fast)
				"debuglevel": 10
			}
		]
		// DHCPv4 Config ...
	}
}
```


### NoLinebreakMessagesLoggersRule

- **Codename** - LOGGERS::NoLinebreakMessagesLoggersRule.
- **Importance** - INFO.
- **Config type** - DHCPv4, DHCPv6, D2, Control Agent.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/logging.html#logging-message-format

#### Problem

In the value of the `pattern` key of the loggers configuration, it is recommended to put a '\n' at the end of the line, otherwise the lines of the log files can be composed in one long line.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"loggers": [
			{
				"name": "kea-dhcp4",
				"output-options": [
					{
						"output": "kea-dhcp4.log",
						"maxsize": 52428800,
						"maxver": 100,
						"pattern": "%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m" // \n not exist in end of line
					}
				],
				"severity": "INFO",
				"debuglevel": 10
			}
		]
		// DHCPv4 Config ...
	}
}
```

#### Solution

Add '\n' to the end of the string values for the `pattern` key of the loggers configuration.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"loggers": [
			{
				"name": "kea-dhcp4",
				"output-options": [
					{
						"output": "kea-dhcp4.log",
						"maxsize": 52428800,
						"maxver": 100,
						"pattern": "%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m\n" // \n placed in end of line
					}
				],
				"severity": "INFO",
				"debuglevel": 10
			}
		]
		// DHCPv4 Config ...
	}
}
```


### NoPercentMMessagesLoggersCtrlAgentRule

- **Codename** - LOGGERS::NoPercentMMessagesLoggersCtrlAgentRule.
- **Importance** - INFO.
- **Config type** - DHCPv4, DHCPv6, D2, Control Agent.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/logging.html#logging-message-format

#### Problem

In the value of the `pattern` key of the loggers configuration, it is recommended to put a '%m' at the line, otherwise there will be no messages describing the event, and the log will essentially become of little use.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"loggers": [
			{
				"name": "kea-dhcp4",
				"output-options": [
					{
						"output": "kea-dhcp4.log",
						"maxsize": 52428800,
						"maxver": 100,
						"pattern": "%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] \n" // %m not exist in line
					}
				],
				"severity": "INFO",
				"debuglevel": 10
			}
		]
		// DHCPv4 Config ...
	}
}
```

#### Solution

Add '%m,' to the the string values for the `pattern` key of the loggers configuration.

```js
{
	"Dhcp4": {
		// DHCPv4 Config ...
		"loggers": [
			{
				"name": "kea-dhcp4",
				"output-options": [
					{
						"output": "kea-dhcp4.log",
						"maxsize": 52428800,
						"maxver": 100,
						"pattern": "%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m\n" // %n placed in line
					}
				],
				"severity": "INFO",
				"debuglevel": 10
			}
		]
		// DHCPv4 Config ...
	}
}
```
