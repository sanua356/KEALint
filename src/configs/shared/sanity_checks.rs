use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEASanityChecks {
    pub lease_checks: Option<String>,
    pub extended_info_checks: Option<String>,
}
