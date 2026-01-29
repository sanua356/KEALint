#![allow(dead_code)]

use regex::bytes::Regex;

use crate::{
    common::RuleResult,
    configs::loggers::{KEALogger, KEALoggerSeverityTypes},
};

pub fn get_debug_loggers_rule(
    loggers: &Vec<KEALogger>,
    config_type: &str,
) -> Option<Vec<RuleResult>> {
    let mut results: Vec<RuleResult> = Vec::new();

    for logger in loggers {
        if let Some(severity) = &logger.severity
            && *severity == KEALoggerSeverityTypes::DEBUG
        {
            results.push(RuleResult {
                description: format!("In the configuration '{}', the logger named '{}' has severity 'DEBUG'. Change severity if you are using a production server.", config_type, logger.name),
                snapshot: Some(serde_json::to_string(logger).unwrap()),
                links: None,
            });
        }
    }

    if !results.is_empty() {
        return Some(results);
    }

    None
}

pub fn get_no_percent_m_in_pattern_rule(
    loggers: &Vec<KEALogger>,
    config_type: &str,
) -> Option<Vec<RuleResult>> {
    let mut results: Vec<RuleResult> = Vec::new();

    let remove_datetime_regex = Regex::new(r#"\{[^}]*[%Dd][^}]*\}"#).unwrap();

    for logger in loggers {
        if let Some(output_options) = &logger.output_options {
            for options in output_options {
                if let Some(pattern) = &options.pattern {
                    let replaced = String::from_utf8(
                        remove_datetime_regex
                            .replace_all(pattern.as_bytes(), b"")
                            .into_owned(),
                    )
                    .unwrap();

                    if !replaced.contains("%m") {
                        results.push(RuleResult {
		                    description: format!("In the '{}' configuration, the logger named '{}' by the key 'pattern' does not have the literals '%m' outside datetime. The log message will not be available without it.", config_type, logger.name),
		                    snapshot: Some(serde_json::to_string(logger).unwrap()),
		                    links: None,
                        });
                    }
                }
            }
        }
    }

    if !results.is_empty() {
        return Some(results);
    }

    None
}

pub fn get_no_linebreak_in_pattern_rule(
    loggers: &Vec<KEALogger>,
    config_type: &str,
) -> Option<Vec<RuleResult>> {
    let mut results: Vec<RuleResult> = Vec::new();

    for logger in loggers {
        if let Some(output_options) = &logger.output_options {
            for options in output_options {
                if let Some(pattern) = &options.pattern
                    && !pattern.ends_with("\\n")
                {
                    results.push(RuleResult {
	                    description: format!("In the '{}' configuration, the logger named '{}' by the key 'pattern' does not have the literals '\\n'. Log messages will not be transferred to a new line.", config_type, logger.name),
	                    snapshot: Some(serde_json::to_string(logger).unwrap()),
	                    links: Some(vec!["https://kea.readthedocs.io/en/latest/arm/logging.html#logging-message-format"]),
                    });
                }
            }
        }
    }

    if !results.is_empty() {
        return Some(results);
    }

    None
}
