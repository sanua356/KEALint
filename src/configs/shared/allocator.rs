use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum KEAAllocatorTypes {
    Iterative,
    Random,
    #[serde(rename = "flq")]
    FLQ,
}
