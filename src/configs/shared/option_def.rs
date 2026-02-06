use serde::{Deserialize, Serialize};

use super::KEAOptionTypes;

#[derive(Debug, Serialize, Deserialize)]
pub struct KEAOptionDefinition {
    pub name: Option<String>,
    pub code: Option<u8>,
    pub r#type: KEAOptionTypes,
    pub array: Option<bool>,
    #[serde(rename = "record-types")]
    pub record_types: Option<String>,
    pub space: Option<String>,
    pub encapsulate: Option<String>,
}
