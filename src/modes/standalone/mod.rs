use chrono;
use rusqlite::Connection;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{
    fs::{self},
    io::{BufReader, Read},
    os::unix::net::{UnixListener, UnixStream},
    sync::Arc,
    thread,
};

use crate::{
    configs::{KEACtrlAgentConfigFile, KEAD2ConfigFile, KEAv4ConfigFile, KEAv6ConfigFile},
    tr,
};

mod databases;

use super::{CLIArgs, run_checks};
use databases::{DatabaseResults, DatabaseResultsChecker};

fn try_configuration<T>(config: &str, config_key: &'static str) -> Option<T>
where
    T: DeserializeOwned,
{
    let value: Value =
        serde_json::from_str(config).expect(tr!("convert-configuration-to-struct-error").as_str());
    let obj = value.as_object().unwrap();

    if obj.contains_key(config_key) {
        serde_json::from_value(value).unwrap()
    } else {
        None
    }
}

fn handle_query(stream: UnixStream, database_path: String) {
    let mut buffer = BufReader::new(stream);
    let mut config = String::new();

    buffer
        .read_to_string(&mut config)
        .expect(tr!("convert-unix-socket-buffer-error").as_str());

    let v4_config: Option<KEAv4ConfigFile> = try_configuration(&config, "Dhcp4");
    let v6_config: Option<KEAv6ConfigFile> = try_configuration(&config, "Dhcp6");
    let d2_config: Option<KEAD2ConfigFile> = try_configuration(&config, "DhcpDdns");
    let ctrl_agent_config: Option<KEACtrlAgentConfigFile> =
        try_configuration(&config, "Control-agent");

    let problems = run_checks(v4_config, v6_config, d2_config, ctrl_agent_config);

    let connection =
        Connection::open(database_path).expect(tr!("sqlite-connection-error").as_str());
    let database: DatabaseResults<Connection> = DatabaseResults { connection };

    match database.load_results(config, problems) {
        Ok(_) => {
            println!(
                "{}",
                tr!(
                    "save-results-to-database",
                    datetime = chrono::offset::Local::now().to_string()
                )
            );
        }
        Err(err) => {
            panic!(
                "{}",
                tr!("save-results-to-database-error", err = err.to_string())
            )
        }
    }
}

pub fn run_standalone(args: CLIArgs) {
    if args.unix_socket_filepath.is_none() {
        panic!("{}", tr!("unix-socket-path-not-exists-error").as_str());
    }

    let unix_socket_path = args.unix_socket_filepath.unwrap();

    if args.database_filepath.is_none() {
        panic!("{}", tr!("path-to-database-not-exists-error").as_str());
    }

    let database_path = args.database_filepath.unwrap();
    let connection =
        Connection::open(database_path.clone()).expect(tr!("sqlite-connection-error").as_str());
    let database: DatabaseResults<Connection> = DatabaseResults { connection };

    match database.run_migrations() {
        Ok(_) => {
            println!(
                "{}",
                tr!("database-migrations-applied-successfully").as_str()
            );
        }
        Err(err) => {
            panic!(
                "{}",
                tr!("database-migrations-applying-error", err = err.to_string())
            )
        }
    }

    if fs::exists(&unix_socket_path).unwrap() {
        fs::remove_file(&unix_socket_path)
            .expect(tr!("remove-previous-unix-socket-file-error").as_str());
    }

    let listener =
        UnixListener::bind(unix_socket_path).expect(tr!("unix-socket-connection-error").as_str());

    println!("{}", tr!("standalone-mode-running-successfully").as_str());

    let database_path_arc = Arc::new(database_path);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db_path = Arc::clone(&database_path_arc);
                thread::spawn(move || handle_query(stream, (*db_path).clone()));
            }
            Err(err) => {
                panic!(
                    "{}",
                    tr!("unix-socket-listening-error", err = err.to_string())
                );
            }
        }
    }
}
