mod checkers;
mod common;
mod configs;
mod constants;
mod rules;
mod utils;

use std::{env, fs, path::Path};

use crate::{
    checkers::{RuleChecker, RulesCtrlAgent, RulesD2, RulesV4},
    configs::{KEACtrlAgentConfig, KEAD2Config, KEAv4Config},
};

fn main() {
    let current_path = env::current_dir()
        .expect("An error occurred while getting the path to the current startup directory.");

    let content_v4_path = Path::new(current_path.as_os_str()).join("kea-dhcp4.conf");
    let content_v4 = fs::read_to_string(content_v4_path)
        .expect("An error occurred while reading v4 the configuration file.");

    let v4: KEAv4Config = serde_json::from_str(&content_v4).unwrap();
    let checker_v4: RulesV4 = RulesV4::default();
    checker_v4.run(&v4);

    let content_d2_path = Path::new(current_path.as_os_str()).join("kea-dhcp-ddns.conf");
    let content_d2 = fs::read_to_string(content_d2_path)
        .expect("An error occurred while reading d2 the configuration file.");
    let d2: KEAD2Config = serde_json::from_str(&content_d2).unwrap();

    let checker_d2: RulesD2 = RulesD2::default();
    checker_d2.run(&d2);

    let content_ctrl_agent_path = Path::new(current_path.as_os_str()).join("kea-ctrl-agent.conf");
    let content_ctrl_agent = fs::read_to_string(content_ctrl_agent_path)
        .expect("An error occurred while reading ctrl-agent the configuration file.");
    let ctrl_agent: KEACtrlAgentConfig = serde_json::from_str(&content_ctrl_agent).unwrap();

    let checker_ctrl_agent: RulesCtrlAgent = RulesCtrlAgent::default();
    checker_ctrl_agent.run(&ctrl_agent);
}
