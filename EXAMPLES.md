
## KEALint Examples

### Basic example

Standard output, without additional flags.

 **File structure:**

```
	- #️⃣ kealint
	- 📁 kea-configs
			-	📄 kea-dhcp4.conf
			-	📄 kea-dhcp-ddns.conf
			-	📄 kea-ctrl-agent.conf
```

 **Run command:**

 ```
./kealint --dir-path ./kea-configs
 ```
 
  **Result:**
  
 ```
┌──────────────────────┬──────────────┬────────────┬──────────────────────┬──────────────────────┬──────────────────────┐
│ name                 │ config_type  │ importance │ description          │ places               │ links                │
├──────────────────────┼──────────────┼────────────┼──────────────────────┼──────────────────────┼──────────────────────┤
│ LEASE_DATABASE::NoEn │ Dhcp4        │ Warning    │ The 'persist' flag i │ lease-database.persi │ https://kea.readthed │
│ abledPersistFlagForM │              │            │ s not set to 'true'  │ st                   │ ocs.io/en/latest/arm │
│ emfileLeases         │              │            │ for the maintenance  │                      │ /dhcp4-srv.html#memf │
│                      │              │            │ of the arend databas │                      │ ile-basic-storage-fo │
│                      │              │            │ e in the 'memfile'   │                      │ r-leases             │
├──────────────────────┼──────────────┼────────────┼──────────────────────┼──────────────────────┼──────────────────────┤
│ DDNS_SERVER::NotLoca │ D2           │ Critical   │ Loopback addresses m │ ip-address           │ https://kea.readthed │
│ lIPAddressInD2Server │              │            │ ust be used as the s │                      │ ocs.io/en/latest/arm │
│ ConfigRule           │              │            │ erver address to avo │                      │ /ddns.html#global-se │
│                      │              │            │ id attacks with fake │                      │ rver-parameters      │
│                      │              │            │  requests.           │                      │                      │
├──────────────────────┼──────────────┼────────────┼──────────────────────┼──────────────────────┼──────────────────────┤
│ CTRL_AGENT::NotLocal │ ControlAgent │ Warning    │ The configuration sp │ http-host            │ https://kea.readthed │
│ IPWithoutHTTPSRule   │              │            │ ecifies the 'http-po │                      │ ocs.io/en/latest/arm │
│                      │              │            │ rt' key in a value t │                      │ /security.html#tls-h │
│                      │              │            │ hat is not a local I │                      │ ttps-configuration   │
│                      │              │            │ P address, but HTTPS │                      │                      │
│                      │              │            │  support is not enab │                      │                      │
│                      │              │            │ led.                 │                      │                      │
└──────────────────────┴──────────────┴────────────┴──────────────────────┴──────────────────────┴──────────────────────┘
```

### Example with override config path and JSON output

Output in JSON format, its own path to the DHCPv4 configuration.

 **File structure:**

```
	- #️⃣ kealint
	- 📁 kea-configs
			-	📄 kea-dhcp-ddns.conf
			-	📄 kea-ctrl-agent.conf
	- 📄 kea-dhcp4.conf
```


 **Run command:**

 ```
./kealint --dir-path ./kea-configs --v4-filepath ./kea-dhcp4.conf --format json
 ```
 
  **Result:**
  
 ```json
[  
	{  
		"name": "INTERFACES::NoInterfacesInInterfacesConfigRule",  
		"config_type": "Dhcp4",  
		"importance": "Info",  
		"description": "No network interfaces are specified in the server configuration. Addresses will not be serviced.",  
		"places": [  
		"interfaces-config.interfaces"  
		],  
		"links": [  
		"https://kea.readthedocs.io/en/latest/arm/dhcp6-srv.html#interface-configuration"  
		]  
	},
	{  
		"name": "HOOKS::BadTKeyGSSTSIGHookTimeoutsRule",  
		"config_type": "D2",  
		"importance": "Warning",  
		"description": "The value of the 'rekey-interval' parameter in the configuration of the 'GSS-TSIG' hook is recommended  
		to be set in the range of 50-80% of the value of the 'tkey-lifetime' parameter",  
		"places": [  
		"hooks-libraries.0.rekey-interval"  
		],  
		"links": [  
		"https://kea.readthedocs.io/en/latest/arm/integrations.html#using-gss-tsig"  
		]  
	},
	{  
		"name": "CTRL_AGENT::NotLocalIPWithoutHTTPSRule",  
		"config_type": "ControlAgent",  
		"importance": "Warning",  
		"description": "The configuration specifies the 'http-port' key in a value that is not a local IP address, but HTTPS su  
		pport is not enabled.",  
		"places": [  
		"http-host"  
		],  
		"links": [  
		"https://kea.readthedocs.io/en/latest/arm/security.html#tls-https-configuration"  
		]  
	}
]
```


### Example with all custom config path and output in file

Output in tabular format to a file, its own path to all types of configurations.

 **File structure:**

```
	- #️⃣ kealint
	- 📁 a
			-	📄 kea-dhcp-ddns.conf
	- 📁 b
			-	📄 kea-ctrl-agent.conf
	- 📁 c
			-	📄 kea-dhcp4.conf
```


 **Run command:**

 ```
./kealint --d2-filepath ./a/kea-dhcp-ddns.conf --ctrl-agent-filepath ./b/kea-ctrl-a  
gent.conf --v4-filepath ./c/kea-dhcp4.conf --format table --output-filepath ./output.txt
 ```
 
 **Result:**

Results written in file "output.txt".


### Example with multithread mode and view summary

Tabular output, viewing the final check, multithreading, and skipping missing configurations.

 **File structure:**

```
	- #️⃣ kealint
	- 📁 kea-configs
			-	📄 kea-dhcp4.conf
			-	📄 kea-ctrl-agent.conf
```


 **Run command:**

 ```
./kealint --dir-path ./kea-configs --with-summary --use-threads --skip-not-exists
 ```
 
  **Result:**

 ```
┌──────────────────────┬──────────────┬────────────┬──────────────────────┬──────────────────────┬──────────────────────┐
│ name                 │ config_type  │ importance │ description          │ places               │ links                │
├──────────────────────┼──────────────┼────────────┼──────────────────────┼──────────────────────┼──────────────────────┤
│ INTERFACES::NoInterf │ Dhcp4        │ Info       │ No network interface │ interfaces-config.in │ https://kea.readthed │
│ acesInInterfacesConf │              │            │ s are specified in t │ terfaces             │ ocs.io/en/latest/arm │
│ igRule               │              │            │ he server configurat │                      │ /dhcp6-srv.html#inte │
│                      │              │            │ ion. Addresses will  │                      │ rface-configuration  │
│                      │              │            │ not be serviced.     │                      │                      │
├──────────────────────┼──────────────┼────────────┼──────────────────────┼──────────────────────┼──────────────────────┤
│ ALLOCATOR::NotSelect │ Dhcp4        │ Info       │ The 'iterative' addr │ allocator            │ https://kea.readthed │
│ IterativeAllocatorFo │              │            │ ess allocator is not │                      │ ocs.io/en/latest/arm │
│ rSharedLeaseDatabase │              │            │  recommended for use │                      │ /dhcp4-srv.html#iter │
│                      │              │            │  with a shared datab │                      │ ative-allocator      │
│                      │              │            │ ase of rents on seve │                      │                      │
│                      │              │            │ ral servers.         │                      │                      │
├──────────────────────┼──────────────┼────────────┼──────────────────────┼──────────────────────┼──────────────────────┤
│ LEASE_DATABASE::NotC │ Dhcp4        │ Warning    │ It is recommended to │ lease-database.on-fa │ https://kea.readthed │
│ hangeStopRetryExitSt │              │            │  set the 'on-fail' p │ il                   │ ocs.io/en/latest/arm │
│ rategyOnFailRule     │              │            │ arameter in the 'lea │                      │ /dhcp6-srv.html#leas │
│                      │              │            │ se-database' configu │                      │ e-database-configura │
│                      │              │            │ ration to 'stop-retr │                      │ tion                 │
│                      │              │            │ y-exit' for the corr │                      │                      │
│                      │              │            │ ect processing of le │                      │                      │
│                      │              │            │ ases in the producti │                      │                      │
│                      │              │            │ on environment.      │                      │                      │
├──────────────────────┼──────────────┼────────────┼──────────────────────┼──────────────────────┼──────────────────────┤
│ LEASE_DATABASE::Leas │ Dhcp4        │ Info       │ The Sanity Checks me │ lease-database.type  │ https://kea.readthed │
│ eSanityChecksEnabled │              │            │ chanism is not imple │                      │ ocs.io/en/latest/arm │
│ ForNotMemfileBackend │              │            │ mented for rent data │  sanity-checks.lease │ /dhcp4-srv.html#sani │
│                      │              │            │ bases other than 'me │ -checks              │ ty-checks-in-dhcpv4  │
│                      │              │            │ mfile'.              │                      │                      │
├──────────────────────┼──────────────┼────────────┼──────────────────────┼──────────────────────┼──────────────────────┤
... more rules

Found 54 problem(s).  
  
  
By type config:  
ControlAgent = 5 problem(s).  
Dhcp4 = 49 problem(s).  
  
By importance:  
Critical = 2 problem(s).  
Info = 23 problem(s).  
Warning = 29 problem(s).

```


### Example with standalone mode

Writing checks to the SQLite3 database when receiving them from a UNIX socket.

 **File structure:**

```
	- #️⃣ kealint
	- 💾 database.sq3
```


 **Run command:**

 ```
./kealint --mode standalone --unix-socket-path /tmp/kealint_unix.sock --database-path testdb.sq3
 ```
 
  **Result:**

 ```
Database migrations applied successfully!
Server runned in standalone mode!
 ```
