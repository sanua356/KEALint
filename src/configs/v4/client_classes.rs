use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAv4ClientClass {
    pub name: String,
    pub test: Option<String>,
    pub template_test: Option<String>,
    pub only_if_required: Option<bool>,
    pub user_context: Value,
    pub next_server: Option<String>,
    pub server_hostname: Option<String>,
    pub boot_file_name: Option<String>,
    pub valid_lifetime: Option<u32>,
    pub renew_timer: Option<u32>,
    pub rebind_timer: Option<u32>,
}
