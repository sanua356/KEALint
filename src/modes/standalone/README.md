# KEALint

## Introduction

KEALint implements a mechanism for communicating with KEA services when the configuration is changed. It is implemented using the proprietary [kealint-unix-hook](https://github.com/sanua356/kealint-unix-hook), which sends data to a UNIX socket, and KEALint accepts data from it, runs the check and puts the results in the SQLite3 database.

The general scheme is as follows.
```
🖥 KEA DHCPv4/D2/CA ---> CONFIGURATION CHANGED! ---> ⚓ KEALint UNIX Hook ---> SEND TO UNIX SOCKET ---> #️⃣ KEALint ---> RUN CHECKS ---> 💾 SQLite Database ---> ✅ Done
```

## Database table structure

**Current database schema version: ** 1.

The database consists of three tables: `schema_version`, `snapshot` and `problem`.

### Table "schema_version"

A service table required for the correct application of migrations.

**Fields:**
`version` - Integer type, not null. Defines the current version of the migration scheme. There should always be only one row in this table.

### Table "shapshot"

A table for storing snapshots of changed server configurations.

**Fields:**
`id` - Integer type, primary key. The unique ID of each configuration snapshot.

`config_type` - Text type, not null. The type of configuration that is saved in the snapshot. Possible values: 'Dhcp4, 'D2', 'ControlAgent'.

`data` - Text type, not null. A snapshot of the configuration in the form of text encoded in JSON.

`created_at` - Datetime type, default: datetime('now', 'localtime'). The date when the configuration snapshot was created. It can also be interpreted as the checks date.

### Table "problem"

A table for storing problems found in the configuration snapshot.

`id` - Integer type, primary key. The unique ID of each problem.

`snapshot_id` - Integer type, foregin key to 'snapshot.id'. A foreign key to the ID of the snapshot in which the problem was found.

`name` - Text type, not null. The name of the rule that failed the correctness check in the snapshot.

`importance` - Text type, not null. The importance of a rule that has not passed the correctness check in the snapshot. Possible values: 'Info', 'Warning', 'Critical'.

`description` - Text type, not null. Description of the rule that failed the correctness check in the snapshot. It contains recommendations for correction.

`places` - Text type. A JSON array in text format indicating the path to the specific configuration areas where the problem occurred.

`links` - A JSON array in text format indicating the path of documentation articles describing the problem and possibly solutions.
