use serde::Deserialize;
use std::{fmt::Display, net::Ipv4Addr, str::FromStr};

use super::KEAv4OptionData;

use crate::{
    configs::reservations::KEAReservation,
    constants::{CIDR_V4_REGEXP, IPV4_RANGE_REGEXP},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAv4Subnet {
    pub subnet: KEAv4PoolVariant,
    pub id: Option<u32>,
    pub pools: Option<Vec<KEAv4Pool>>,

    pub valid_lifetime: Option<u32>,
    pub renew_timer: Option<u32>,
    pub rebind_timer: Option<u32>,
    pub evaluate_additional_classes: Option<Vec<String>>,
    pub reservations_out_of_pool: Option<bool>,

    pub option_data: Option<Vec<KEAv4OptionData>>,

    pub reservations: Option<Vec<KEAReservation>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KEAv4PoolVariant {
    Range(Ipv4Addr, Ipv4Addr),
    Cidr(Ipv4Addr, u8),
}

impl Display for KEAv4PoolVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KEAv4PoolVariant::Cidr(start, prefix) => {
                write!(f, "{}/{}", start, prefix)
            }
            KEAv4PoolVariant::Range(start, end) => {
                write!(f, "{}-{}", start, end)
            }
        }
    }
}

impl FromStr for KEAv4PoolVariant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if IPV4_RANGE_REGEXP.is_match(s) {
            let (start, end): (&str, &str) = s.split_once("-").unwrap();
            let start_ip: Ipv4Addr = start.trim().parse().map_err(|_| "Bad start ip in range.")?;
            let end_ip: Ipv4Addr = end.trim().parse().map_err(|_| "Bad start ip in range.")?;

            return Ok(KEAv4PoolVariant::Range(start_ip, end_ip));
        }

        if CIDR_V4_REGEXP.is_match(s) {
            let (start, prefix): (&str, &str) = s.split_once("/").unwrap();

            let start_ip: Ipv4Addr = start.trim().parse().map_err(|_| "Bad start ip in range.")?;
            let prefix_value: u8 = prefix.trim().parse().map_err(|_| "Bad prefix in CIDR.")?;

            if prefix_value > 32 {
                return Err(format!("Bad prefix value in CIDR: {}", s));
            }

            return Ok(KEAv4PoolVariant::Cidr(start_ip, prefix_value));
        }

        Err(format!("Bad pool range or CIDR: {}", s))
    }
}

impl<'de> Deserialize<'de> for KEAv4PoolVariant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEAv4Pool {
    pub pool: KEAv4PoolVariant,
    pub evaluate_additional_classes: Option<Vec<String>>,
}
