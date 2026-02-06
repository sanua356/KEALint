use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KEAInterfacesConfig {
    pub interfaces: Vec<String>,
}
