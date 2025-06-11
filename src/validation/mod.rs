use crate::errors::{ParseError, Result};
use datalogic_rs::DataLogic;
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Validation result for a single rule
#[derive(Debug)]
pub struct ValidationResult {
    pub rule_name: String,
    pub passed: bool,
    pub message: String,
}

/// Collection of validation results
#[derive(Debug)]
pub struct ValidationReport {
    pub results: Vec<ValidationResult>,
    pub overall_valid: bool,
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationReport {
    /// Create a new empty validation report
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            overall_valid: true,
        }
    }

    /// Add a validation result
    pub fn add_result(&mut self, result: ValidationResult) {
        if !result.passed {
            self.overall_valid = false;
        }
        self.results.push(result);
    }

    /// Get the number of failed validations
    pub fn failure_count(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    /// Get all failed validation results
    pub fn get_failures(&self) -> Vec<&ValidationResult> {
        self.results.iter().filter(|r| !r.passed).collect()
    }
}

/// Validates an MT message using JSONLogic rules from a file
///
/// # Arguments
/// * `message_json` - The MT message converted to JSON Value
/// * `rules_file_path` - Path to the JSON file containing validation rules
///
/// # Returns
/// * `ValidationReport` containing detailed validation results
pub fn validate_mt_message_with_rules<P: AsRef<Path>>(
    message_json: Value,
    rules_file_path: P,
) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Read the rules file
    let rules_content =
        fs::read_to_string(rules_file_path).map_err(|e| ParseError::ValidationError {
            message: format!("Failed to read rules file: {}", e),
        })?;

    // Parse rules JSON
    let rules_json: Value =
        serde_json::from_str(&rules_content).map_err(|e| ParseError::ValidationError {
            message: format!("Failed to parse rules JSON: {}", e),
        })?;

    // Extract rules array
    let rules_array = rules_json
        .get("rules")
        .and_then(|r| r.as_array())
        .ok_or_else(|| ParseError::ValidationError {
            message: "Rules file must contain a 'rules' array".to_string(),
        })?;

    // Create DataLogic engine
    let engine = DataLogic::new();

    // Process each rule
    for rule in rules_array {
        let rule_name = rule
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("Unknown Rule")
            .to_string();

        let rule_message = rule
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("Validation failed")
            .to_string();

        // Extract logic from rule
        let logic = rule
            .get("logic")
            .ok_or_else(|| ParseError::ValidationError {
                message: format!("Rule '{}' missing 'logic' field", rule_name),
            })?;

        // Evaluate the rule
        let rule_result = engine
            .evaluate_json(logic, &message_json, None)
            .map_err(|e| ParseError::ValidationError {
                message: format!("Failed to evaluate rule '{}': {}", rule_name, e),
            })?;

        // Convert result to boolean
        let passed = rule_result.is_boolean() && rule_result.as_bool().unwrap();

        report.add_result(ValidationResult {
            rule_name,
            passed,
            message: rule_message,
        });
    }

    Ok(report)
}
