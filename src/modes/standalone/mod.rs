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

use crate::configs::{KEACtrlAgentConfigFile, KEAD2ConfigFile, KEAv4ConfigFile, KEAv6ConfigFile};

mod databases;

use super::{CLIArgs, run_checks};
use databases::{DatabaseResults, DatabaseResultsChecker};

fn try_configuration<T>(config: &str, config_key: &'static str) -> Option<T>
where
    T: DeserializeOwned,
{
    let value: Value = serde_json::from_str(config)
        .expect("An error occurred when converting the configuration to a structure");
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
        .expect("An error occurred when converting the socket buffer to a string.");

    let v4_config: Option<KEAv4ConfigFile> = try_configuration(&config, "Dhcp4");
    let v6_config: Option<KEAv6ConfigFile> = try_configuration(&config, "Dhcp6");
    let d2_config: Option<KEAD2ConfigFile> = try_configuration(&config, "DhcpDdns");
    let ctrl_agent_config: Option<KEACtrlAgentConfigFile> =
        try_configuration(&config, "Control-agent");

    let problems = run_checks(v4_config, v6_config, d2_config, ctrl_agent_config);

    let connection = Connection::open(database_path)
        .expect("An error occurred when connecting to an SQLite database.");
    let database: DatabaseResults<Connection> = DatabaseResults { connection };

    match database.load_results(config, problems) {
        Ok(_) => {
            println!(
                "Check results saved to database successfully! {}",
                chrono::offset::Local::now()
            );
        }
        Err(err) => {
            panic!(
                "An error occurred when saving check results in the database. {}",
                err
            )
        }
    }
}

pub fn run_standalone(args: CLIArgs) {
    if args.unix_socket_filepath.is_none() {
        panic!("The path for the UNIX listening socket is not specified!");
    }

    let unix_socket_path = args.unix_socket_filepath.unwrap();

    if args.database_filepath.is_none() {
        panic!("The path for the database is not specified!");
    }

    let database_path = args.database_filepath.unwrap();
    let connection = Connection::open(database_path.clone())
        .expect("An error occurred when connecting to an SQLite database.");
    let database: DatabaseResults<Connection> = DatabaseResults { connection };

    match database.run_migrations() {
        Ok(_) => {
            println!("Database migrations applied successfully!");
        }
        Err(err) => {
            panic!(
                "An error occurred when applying migrations in the database. {}",
                err
            )
        }
    }

    if fs::exists(&unix_socket_path).unwrap() {
        fs::remove_file(&unix_socket_path)
            .expect("The previous UNIX socket file could not be deleted.");
    }

    let listener = UnixListener::bind(unix_socket_path)
        .expect("An error occurred when connecting to a UNIX socket.");

    println!("Server runned in standalone mode!");

    let database_path_arc = Arc::new(database_path);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db_path = Arc::clone(&database_path_arc);
                thread::spawn(move || handle_query(stream, (*db_path).clone()));
            }
            Err(err) => {
                println!("Error in time listening UNIX socket: {}", err);
                break;
            }
        }
    }
}
