use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct KEAv4HookLibrary {
    pub library: String,
    pub parameters: Option<Value>,
}
