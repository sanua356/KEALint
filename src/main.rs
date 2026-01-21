mod common;
mod configs;
mod rules;

use configs::KEAConfig;
use std::{env, fs, path::Path};

use crate::rules::RulesV4;

fn main() {
    let current_path = env::current_dir()
        .expect("An error occurred while getting the path to the current startup directory.");

    let content_v4_path = Path::new(current_path.as_os_str()).join("kea-dhcp4.conf");
    let content_v4 = fs::read_to_string(content_v4_path)
        .expect("An error occurred while reading the configuration file.");

    let a: KEAConfig = serde_json::from_str(&content_v4).unwrap();

    let checker_v4: RulesV4 = RulesV4::default();
    checker_v4.run(&a.dhcp4);
}
