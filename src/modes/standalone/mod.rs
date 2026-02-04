use chrono;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{
    fs::{self},
    io::{BufReader, Read},
    os::unix::net::{UnixListener, UnixStream},
    sync::Arc,
    thread,
};

use crate::configs::{KEACtrlAgentConfigFile, KEAD2ConfigFile, KEAv4ConfigFile};

mod loaders;
mod migrations;
use super::{CLIArgs, run_checks};
use loaders::load_problems;
use migrations::run_migrations;

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
    let d2_config: Option<KEAD2ConfigFile> = try_configuration(&config, "DhcpDdns");
    let ctrl_agent_config: Option<KEACtrlAgentConfigFile> =
        try_configuration(&config, "Control-agent");

    let problems = run_checks(v4_config, d2_config, ctrl_agent_config);

    match load_problems(config, problems, database_path) {
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
    if args.unix_socket_path.is_none() {
        panic!("The path for the UNIX listening socket is not specified!");
    }

    let unix_socket_path = args.unix_socket_path.unwrap();

    if args.database_path.is_none() {
        panic!("The path for the database is not specified!");
    }

    let database_path = args.database_path.unwrap();

    match run_migrations(database_path.clone()) {
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
