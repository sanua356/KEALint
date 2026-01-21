use serde::{Deserialize, Serialize};

use super::configs::v4::KEAv4Config;

pub mod v4;

#[derive(Debug, Serialize, Deserialize)]
pub struct KEAConfig {
    #[serde(rename = "Dhcp4")]
    pub dhcp4: KEAv4Config,
}
