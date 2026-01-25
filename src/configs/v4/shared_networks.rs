use serde::Deserialize;

use super::{KEAv4OptionData, KEAv4Subnet};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAv4SharedNetwork {
    pub name: String,

    pub valid_lifetime: Option<u32>,
    pub renew_timer: Option<u32>,
    pub rebind_timer: Option<u32>,
    pub interface: Option<String>,
    pub evaluate_additional_classes: Option<Vec<String>>,

    pub subnet4: Option<Vec<KEAv4Subnet>>,

    pub option_data: Option<Vec<KEAv4OptionData>>,
}
