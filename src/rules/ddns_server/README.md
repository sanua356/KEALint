
# Rules "DDNS Server"

## Implemented For

- ✅ - Implemented for specified config.
- ❌ - NOT implemented for specified config.
- 🚫 - Cannot be implemented for the specified config.

|Rule name|DHCPv4|DHCPv6|DDNS|Control Agent|
|--|--|--|--|--|
|[NotLocalIPAddressInD2ServerConfigRule](#NotLocalIPAddressInD2ServerConfigRule)|🚫|🚫|✅|🚫|
|[NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule](#NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule)|✅|✅|🚫|🚫|

## Rules

### NotLocalIPAddressInD2ServerConfigRule

- **Codename** - DDNS_SERVER::NotLocalIPAddressInD2ServerConfigRule.
- **Importance** - CRITICAL.
- **Config type** - D2.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/ddns.html#global-server-parameters

#### Problem

It is possible for a malicious attacker to send bogus NameChangeRequests to the DHCP-DDNS server. Addresses in key `ip-address` other than the IPv4 or IPv6 loopback addresses (127.0.0.1 or ::1) should only be used for testing purposes; note that local users may still communicate with the DHCP-DDNS server.

```js
{
	"DhcpDdns": {
		// DDNS Config ...
		"ip-address": "1.2.3.4", // Not local IPv4 address
		"port": 53001
		// DDNS Config ...
	}
}
```

#### Solution

Set local IP-address in `ip-address` key.


```js
{
	"DhcpDdns": {
		// DDNS Config ...
		"ip-address": "127.0.0.1", // Local IPv4 address
		"port": 53001
		// DDNS Config ...
	}
}
```


### NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule

- **Codename** - DDNS_SERVER::NotDDNSQualifyingSuffixWithEnabledDDNSUpdatesRule.
- **Importance** - WARNING.
- **Config type** - DHCPv4, DHCPv6.
- **Articles** - https://kea.readthedocs.io/en/latest/arm/dhcp4-srv.html#kea-dhcp4-name-generation-for-ddns-update-requests

#### Problem

When DDNS updates are enabled, there are situations where the client cannot be uniquely identified in a particular domain. It is recommended to specify a suffix for automatic host generation using the `ddns-qualifying-suffix` key in order to identify clients by a specific attribute.

```js
{
	"Dhcp4": {
	    "interfaces-config": {
	        "interfaces": [ "eth0" ],
	        "dhcp-socket-type": "raw"
	    },
	    "lease-database": {
			"type": "memfile",
			"persist": false,
			"name": "/var/lib/kea/dhcp4.leases"
		},
	    "valid-lifetime": 4000,
	    "renew-timer": 1000,
	    "rebind-timer": 2000,
	    "dhcp-ddns": {
			"enable-updates": false,
			"server-ip": "127.0.0.1",
			"server-port": 53001,
			"sender-ip": "",
			"sender-port": 0,
			"max-queue-size": 1024,
			"ncr-protocol": "UDP",
			"ncr-format": "JSON"
		},
		"ddns-send-updates": true, // Enabled ddns updates
		"ddns-override-no-update": false,
		"ddns-override-client-update": false,
		"ddns-replace-client-name": "never",
		"ddns-generated-prefix": "myhost",
		"ddns-qualifying-suffix": "", // Not setted DDNS suffix
		"ddns-update-on-renew": false,
		"ddns-conflict-resolution-mode": "check-with-dhcid",
		"hostname-char-set": "",
		"hostname-char-replacement": ""
	}
}
```

#### Problem

Specify a suffix for automatic host generation using the `ddns-qualifying-suffix` key in order to identify clients by a specific attribute.

```js
{
	"Dhcp4": {
	    "interfaces-config": {
	        "interfaces": [ "eth0" ],
	        "dhcp-socket-type": "raw"
	    },
	    "lease-database": {
			"type": "memfile",
			"persist": false,
			"name": "/var/lib/kea/dhcp4.leases"
		},
	    "valid-lifetime": 4000,
	    "renew-timer": 1000,
	    "rebind-timer": 2000,
	    "dhcp-ddns": {
			"enable-updates": false,
			"server-ip": "127.0.0.1",
			"server-port": 53001,
			"sender-ip": "",
			"sender-port": 0,
			"max-queue-size": 1024,
			"ncr-protocol": "UDP",
			"ncr-format": "JSON"
		},
		"ddns-send-updates": true, // Enabled ddns updates
		"ddns-override-no-update": false,
		"ddns-override-client-update": false,
		"ddns-replace-client-name": "never",
		"ddns-generated-prefix": "myhost",
		"ddns-qualifying-suffix": "foo.example.org", // Now setted DDNS suffix
		"ddns-update-on-renew": false,
		"ddns-conflict-resolution-mode": "check-with-dhcid",
		"hostname-char-set": "",
		"hostname-char-replacement": ""
	}
}
```
