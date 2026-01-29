use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEADhcpQueueControl {
    pub enable_queue: Option<bool>,
    pub queue_type: Option<String>,
    pub capacity: Option<u32>,
}
