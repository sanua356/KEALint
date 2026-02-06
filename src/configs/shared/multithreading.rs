use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAMultithreading {
    pub enable_multi_threading: Option<bool>,
    pub thread_pool_size: Option<u32>,
    pub packet_queue_size: Option<u32>,
}
