# Rules "Ctrl Agent"

## Implemented For

- ✅ - Implemented for specified config.
- ❌ - NOT implemented for specified config.
- 🚫 - Cannot be implemented for the specified config.

|Rule name|DHCPv4|DHCPv6|DDNS|Control Agent|
|--|--|--|--|--|
|[NoAllControlSocketsSpecifiedRule](#NoAllControlSocketsSpecifiedRule)|🚫|🚫|🚫|✅|
|[NotLocalIPWithoutHTTPSRule](#NotLocalIPWithoutHTTPSRule)|🚫|🚫|🚫|✅|

## Rules

### NoAllControlSocketsSpecifiedRule

- **Codename** - CTRL_AGENT::NoAllControlSocketsSpecifiedRule.
- **Importance** - WARNING.
- **Config type** - Control Agent.
- **Articles** - https://kea.readthedocs.io/en/stable/arm/agent.html#configuration

#### Problem

The absence of KEA service sockets in the configuration using the `control-sockets` key may lead to the inability to work with services via the API.

```js
{
	"Control-agent": {
		// Control Agent Config ...
		"control-sockets": {
			"dhcp4": {
				"socket-type": "unix",
				"socket-name": "kea4-ctrl-socket"
			},
			"dhcp6": {
				"socket-type": "unix",
				"socket-name": "kea6-ctrl-socket"
			}
			// Not found DDNS control socket
		}
		// Control Agent Config ...
	}
}
```

#### Solution

Specify all control sockets (v4, v6, and D2) in the configuration using the `control-sockets` key.


```js
{
	"Control-agent": {
		// Control Agent Config ...
		"control-sockets": {
			"dhcp4": {
				"socket-type": "unix",
				"socket-name": "kea4-ctrl-socket"
			},
			"dhcp6": {
				"socket-type": "unix",
				"socket-name": "kea6-ctrl-socket"
			},
			// Added new control socket
			"d2": {
		        "socket-type": "unix",
		        "socket-name": "kea-ddns-ctrl-socket"
		    }
		}
		// Control Agent Config ...
	}
}
```


### NotLocalIPWithoutHTTPSRule

- **Codename** - CTRL_AGENT::NotLocalIPWithoutHTTPSRule.
- **Importance** - WARNING.
- **Config type** - Control Agent.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/security.html#tls-https-configuration

#### Problem

Specifying the 'http-host` parameter that does not point to a local network address without activated TLS (HTTPS) support can lead to security problems and unwanted unintended API access.

```js
{
	"Control-agent": {
		// Control Agent Config ...
		{
			"http-host": "88.8.8.8", // Not local IP-address
			"http-port": 8002,
			"cert-required": true,
			"control-sockets": {}
			// Not exists TLS parameters
		}
		// Control Agent Config ...
	}
}
```

#### Solution

Specify the `http-host` parameter pointing to the local network address or activate TLS support.

```js
{
	"Control-agent": {
		// Control Agent Config ...
		{
			"http-host": "127.0.0.1", // Local IP-address
			"http-port": 8002,
			"cert-required": true,
			"control-sockets": {},
			// Or set TLS parameters
			"trust-anchor": "/home/abc/certs/ca.pem",
			"cert-file": "/home/abc/certs/cert.pem",
			"key-file": "/home/abc/certs/key.pem",
			"cert-required": true
		}
		// Control Agent Config ...
	}
}
```
