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
