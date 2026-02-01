# KEALint

## Introduction

KEALint - ⚡ blazingly fast ⚡ static configuration analyzer ISC KEA DHCP for version 3.x.x, **written in Rust!**

**More than 40 validation rules** for DHCPv4, D2, and Control Agent configurations.

Implements a flexible **CLI interface** for interaction and output of verification results.
 
## Example of work

 **Run command:**
 ```
kealint --dir-path ./kea-configs
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

## CLI Parameters

Command line interface 'KEALint' implements the following interaction parameters:

`--dir-path` - Optional. Specifies the path to the directory where the KEA configuration files are stored. If this parameter is specified, files named 'kea-dhcp4.conf', 'kea-dhcp-ddns.conf' and 'kea-ctrl-agent.conf' are searched inside the specified directory.

`--format` - Optional. Defines the format for the output of the verification result. You can specify the value 'table' or 'json'.

`--v4-filepath` - Optional. Specifies the path to the KEA DHCPv4 configuration file. If specified together the `dir-path` parameter, the current parameter overrides the path to the v4 configuration file.

`--d2-filepath` - Optional. Specifies the path to the KEA DHCP DDNS configuration file. If specified together the `dir-path` parameter, the current parameter overrides the path to the DDNS configuration file.

`--ctrl-agent-filepath` - Optional. Specifies the path to the KEA Control Agent configuration file. If specified together the `dir-path` parameter, the current parameter overrides the path to the Control Agent configuration file.

`--output_filepath` - Optional. Specifies the path to the file to which the verification result will be uploaded. If the file does not exist, it will be created.

`--skip-not-exists` - Optional. If specified, the check will run even if not all configuration files exist. By default, the utility waits for all configuration files to run (v4, DDNS, and Control Agent).

`--use-threads` - Optional. If enabled, processing is performed in multithreaded mode.

`--with-summary` - Optional. Adds additional information when displaying the result as a table.

## Rules

KEALint implements a set of rules for configurations from the following set:
|Config part|Link|Rules count|
|--|--|--|
|Allocators|[See more](https://github.com/sanua356/KEALint/tree/master/src/rules/allocator)|2|
|Client Classes|[See more](https://github.com/sanua356/KEALint/tree/master/src/rules/client_classes)|3|
|Control Agent|[See more](https://github.com/sanua356/KEALint/tree/master/src/rules/ctrl_agent)|2|
|DDNS Server|[See more](https://github.com/sanua356/KEALint/tree/master/src/rules/ddns_server)|2|
|Hooks|[See more](https://github.com/sanua356/KEALint/tree/master/src/rules/hooks)|11|
|Interfaces|[See more](https://github.com/sanua356/KEALint/tree/master/src/rules/interfaces)|1|
|Lease Database|[See more](https://github.com/sanua356/KEALint/tree/master/src/rules/lease_database)|3|
|Loggers|[See more](https://github.com/sanua356/KEALint/tree/master/src/rules/loggers)|3+|
|Option Data|[See more](https://github.com/sanua356/KEALint/tree/master/src/rules/option_data)|2|
|Queue Control|[See more](https://github.com/sanua356/KEALint/tree/master/src/rules/queue_control)|1|
|Reservations|[See more](https://github.com/sanua356/KEALint/tree/master/src/rules/reservations)|3|
|Shared Networks|[See more](https://github.com/sanua356/KEALint/tree/master/src/rules/shared_networks)|4|
|Subnets|[See more](https://github.com/sanua356/KEALint/tree/master/src/rules/subnets)|3|

## Build

1. Install Rust Compiler from [offical site.](https://rust-lang.org/)
2. Clone repository: `git clone https://github.com/sanua356/KEALint.git`.
3. Go to cloned repository folder: `cd ./KEALint`.
4. Run build command: `cargo build --release`.
5. Find builded binary file `kealint` by path: `./target/release`.  

## License

GPLv3
