use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KEAv4OptionData {
    pub code: Option<u8>,
    pub space: Option<String>,
    #[serde(rename = "csv-format")]
    pub csv_format: Option<bool>,
    pub data: Option<String>,
}
