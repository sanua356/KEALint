#![allow(dead_code)]

use regex::bytes::Regex;

use crate::{
    common::RuleResult,
    configs::loggers::{KEALogger, KEALoggerSeverityTypes},
};

pub fn get_debug_loggers_rule(loggers: &Option<Vec<KEALogger>>) -> Option<Vec<RuleResult>> {
    if let Some(loggers) = loggers {
        let mut results: Vec<RuleResult> = Vec::new();

        for (idx_logger, logger) in loggers.iter().enumerate() {
            if let Some(severity) = &logger.severity
                && *severity == KEALoggerSeverityTypes::DEBUG
            {
                results.push(RuleResult {
                description: format!(
	                "The logger named '{}' has severity 'DEBUG'. Change severity if you are using a production server.",
	                logger.name
                ),
                places: Some(vec![format!("loggers.{}.severity", idx_logger)]),
                links: None,
            });
            }
        }

        if !results.is_empty() {
            return Some(results);
        }
    }

    None
}

pub fn get_no_percent_m_in_pattern_rule(
    loggers: &Option<Vec<KEALogger>>,
) -> Option<Vec<RuleResult>> {
    if let Some(loggers) = loggers {
        let mut results: Vec<RuleResult> = Vec::new();

        let remove_datetime_regex = Regex::new(r#"\{[^}]*[%Dd][^}]*\}"#).unwrap();

        for (idx_logger, logger) in loggers.iter().enumerate() {
            if let Some(output_options) = &logger.output_options {
                for (idx_options, options) in output_options.iter().enumerate() {
                    if let Some(pattern) = &options.pattern {
                        let replaced = String::from_utf8(
                            remove_datetime_regex
                                .replace_all(pattern.as_bytes(), b"")
                                .into_owned(),
                        )
                        .unwrap();

                        if !replaced.contains("%m") {
                            results.push(RuleResult {
			                    description: format!(
									"The logger named '{}' by the key 'pattern' does not have the literals '%m' outside datetime. The log message will not be available without it.",
									logger.name
								),
			                    places: Some(vec![format!("loggers.{}.output-options.{}.pattern", idx_logger, idx_options)]),
			                    links: Some(&["https://kea.readthedocs.io/en/latest/arm/logging.html#logging-message-format"]),
                        });
                        }
                    }
                }
            }
        }

        if !results.is_empty() {
            return Some(results);
        }
    }

    None
}

pub fn get_no_linebreak_in_pattern_rule(
    loggers: &Option<Vec<KEALogger>>,
) -> Option<Vec<RuleResult>> {
    if let Some(loggers) = loggers {
        let mut results: Vec<RuleResult> = Vec::new();

        for (idx_logger, logger) in loggers.iter().enumerate() {
            if let Some(output_options) = &logger.output_options {
                for (idx_options, options) in output_options.iter().enumerate() {
                    if let Some(pattern) = &options.pattern
                        && !pattern.ends_with('\n')
                    {
                        results.push(RuleResult {
	                        description: format!(
								r#"The logger named '{}' by the key 'pattern' does not have the literals '\n'. Log messages will not be transferred to a new line."#,
								logger.name
							),
	                     	places: Some(vec![format!("loggers.{}.output-options.{}.pattern", idx_logger, idx_options)]),
	                      	links: Some(&["https://kea.readthedocs.io/en/latest/arm/logging.html#logging-message-format"]),
                        });
                    }
                }
            }
        }

        if !results.is_empty() {
            return Some(results);
        }
    }

    None
}

#[cfg(test)]
pub mod _tests;

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::configs::{KEAD2Config, loggers::KEALoggerSeverityTypes};

    use super::{
        _tests::{
            DEBUG_LOGGERS_D2_RULE_TEMPLATE, NO_LINEBREAK_MESSAGES_LOGGERS_D2_RULE_TEMPLATE,
            NO_PERCENT_M_MESSAGES_LOGGERS_D2_RULE_TEMPLATE,
        },
        get_debug_loggers_rule, get_no_linebreak_in_pattern_rule, get_no_percent_m_in_pattern_rule,
    };

    #[test]
    fn check_expected_debug_loggers_trigger() {
        let data: KEAD2Config = serde_json::from_str(DEBUG_LOGGERS_D2_RULE_TEMPLATE).unwrap();

        let rule = get_debug_loggers_rule(&data.loggers);
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_debug_loggers_trigger() {
        let mut json_value: Value = serde_json::from_str(DEBUG_LOGGERS_D2_RULE_TEMPLATE).unwrap();
        json_value["loggers"].as_array_mut().unwrap()[0]["severity"] =
            Value::from(KEALoggerSeverityTypes::INFO.to_string());

        let data: KEAD2Config = serde_json::from_value(json_value).unwrap();

        let rule = get_debug_loggers_rule(&data.loggers);
        assert!(rule.is_none());
    }

    #[test]
    fn check_expected_no_linebreak_in_pattern_trigger() {
        let data: KEAD2Config =
            serde_json::from_str(NO_LINEBREAK_MESSAGES_LOGGERS_D2_RULE_TEMPLATE).unwrap();

        let rule = get_no_linebreak_in_pattern_rule(&data.loggers);
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_no_linebreak_in_pattern_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_LINEBREAK_MESSAGES_LOGGERS_D2_RULE_TEMPLATE).unwrap();
        json_value["loggers"][0]["output-options"][0]["pattern"] =
            Value::from("%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m\n");

        let data: KEAD2Config = serde_json::from_value(json_value).unwrap();

        let rule = get_no_linebreak_in_pattern_rule(&data.loggers);
        assert!(rule.is_none());
    }

    #[test]
    fn check_expected_no_percent_m_in_pattern_trigger() {
        let data: KEAD2Config =
            serde_json::from_str(NO_PERCENT_M_MESSAGES_LOGGERS_D2_RULE_TEMPLATE).unwrap();

        let rule = get_no_percent_m_in_pattern_rule(&data.loggers);
        assert!(rule.is_some());
    }

    #[test]
    fn check_absense_no_percent_m_in_pattern_trigger() {
        let mut json_value: Value =
            serde_json::from_str(NO_PERCENT_M_MESSAGES_LOGGERS_D2_RULE_TEMPLATE).unwrap();
        json_value["loggers"][0]["output-options"][0]["pattern"] =
            Value::from("%d{%Y-%m-%d %H:%M:%S.%q} %-5p [%c/%i.%t] %m \\n");

        let data: KEAD2Config = serde_json::from_value(json_value).unwrap();

        let rule = get_no_percent_m_in_pattern_rule(&data.loggers);
        assert!(rule.is_none());
    }
}
