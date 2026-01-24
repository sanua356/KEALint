use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KEAv4InterfacesConfig {
    pub interfaces: Vec<String>,
}
