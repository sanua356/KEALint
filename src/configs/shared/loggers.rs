#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]

use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum KEALoggerSeverityTypes {
    FATAL,
    ERROR,
    WARN,
    INFO,
    DEBUG,
}

impl Display for KEALoggerSeverityTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            KEALoggerSeverityTypes::FATAL => write!(f, "FATAL"),
            KEALoggerSeverityTypes::ERROR => write!(f, "ERROR"),
            KEALoggerSeverityTypes::WARN => write!(f, "WARN"),
            KEALoggerSeverityTypes::INFO => write!(f, "INFO"),
            KEALoggerSeverityTypes::DEBUG => write!(f, "DEBUG"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEALoggerOutputOptions {
    pub output: Option<String>,
    pub pattern: Option<String>,
    pub maxsize: Option<u64>,
    pub maxver: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEALogger {
    pub name: String,
    pub severity: Option<KEALoggerSeverityTypes>,
    pub debuglevel: Option<u32>,

    #[serde(rename = "output_options")]
    pub output_options: Option<Vec<KEALoggerOutputOptions>>,
}
