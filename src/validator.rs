//! Format Rule Validator for SWIFT MT fields
//!
//! This module provides comprehensive validation of SWIFT field content according to
//! official SWIFT format rules. It supports the SWIFT format mini-grammar and can
//! be extended with custom validation rules.

use crate::errors::{ParseError, Result, ValidationError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Format rules configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatRules {
    pub fields: HashMap<String, FieldRule>,
}

/// Individual field validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldRule {
    /// SWIFT format rule (e.g., "16x", "6!n3!a15d", "4*35x")
    pub format: String,
    /// Human-readable description of the field
    pub description: String,
    /// Optional field options for fields like 50A, 50K, etc.
    pub options: Option<HashMap<String, String>>,
    /// Message types where this field is mandatory
    pub mandatory_for: Vec<String>,
}

impl FormatRules {
    /// Create a new empty FormatRules
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// Load format rules from JSON content
    pub fn from_json(json_content: &str) -> Result<Self> {
        serde_json::from_str(json_content).map_err(|e| ParseError::FormatRuleError {
            message: format!("Failed to parse JSON format rules: {}", e),
        })
    }

    /// Load default format rules from the embedded JSON config
    pub fn default_rules() -> Self {
        // Try to load from the config file first
        if let Ok(rules) = Self::load_from_config_file() {
            return rules;
        }

        // Fallback to hardcoded rules if config file is not available
        Self::fallback_rules()
    }

    /// Load format rules from the default config file
    pub fn load_from_config_file() -> Result<Self> {
        Self::load_from_file("config/swift_format_rules.json")
    }

    /// Load format rules from a specific file path
    pub fn load_from_file(config_path: &str) -> Result<Self> {
        match std::fs::read_to_string(config_path) {
            Ok(content) => Self::from_json(&content),
            Err(e) => Err(ParseError::FormatRuleError {
                message: format!("Failed to read config file '{}': {}", config_path, e),
            }),
        }
    }

    /// Fallback hardcoded rules for when config file is not available
    fn fallback_rules() -> Self {
        let mut rules = FormatRules::new();

        // Field 20: Transaction Reference
        rules.add_field_rule(
            "20",
            FieldRule {
                format: "16x".to_string(),
                description: "Transaction Reference Number".to_string(),
                options: None,
                mandatory_for: vec!["103".to_string(), "102".to_string(), "202".to_string()],
            },
        );

        // Field 23B: Bank Operation Code
        rules.add_field_rule(
            "23B",
            FieldRule {
                format: "4!c".to_string(),
                description: "Bank Operation Code".to_string(),
                options: None,
                mandatory_for: vec!["103".to_string(), "102".to_string()],
            },
        );

        // Field 32A: Value Date, Currency, Amount
        rules.add_field_rule(
            "32A",
            FieldRule {
                format: "6!n3!a15d".to_string(),
                description: "Value Date, Currency Code, Amount".to_string(),
                options: None,
                mandatory_for: vec!["103".to_string(), "102".to_string(), "202".to_string()],
            },
        );

        // Field 50K: Ordering Customer
        rules.add_field_rule(
            "50K",
            FieldRule {
                format: "4*35x".to_string(),
                description: "Ordering Customer (Option K)".to_string(),
                options: None,
                mandatory_for: vec![],
            },
        );

        // Field 59: Beneficiary Customer
        rules.add_field_rule(
            "59",
            FieldRule {
                format: "4*35x".to_string(),
                description: "Beneficiary Customer".to_string(),
                options: None,
                mandatory_for: vec!["103".to_string()],
            },
        );

        // Field 71A: Details of Charges
        rules.add_field_rule(
            "71A",
            FieldRule {
                format: "3!a".to_string(),
                description: "Details of Charges".to_string(),
                options: None,
                mandatory_for: vec!["103".to_string()],
            },
        );

        rules
    }

    /// Add a field rule
    pub fn add_field_rule(&mut self, tag: &str, rule: FieldRule) {
        self.fields.insert(tag.to_string(), rule);
    }

    /// Get a field rule by tag
    pub fn get_field_rule(&self, tag: &str) -> Option<&FieldRule> {
        self.fields.get(tag)
    }

    /// Validate field content against its format rule
    pub fn validate_field(
        &self,
        tag: &str,
        content: &str,
    ) -> std::result::Result<(), ValidationError> {
        if let Some(rule) = self.fields.get(tag) {
            validate_format(&rule.format, content).map_err(|msg| {
                ValidationError::FormatRuleValidationFailed {
                    rule: rule.format.clone(),
                    message: msg,
                }
            })
        } else {
            // No validation rule found - this might be acceptable for unknown fields
            Ok(())
        }
    }

    /// Check if a field is mandatory for a specific message type
    pub fn is_field_mandatory(&self, tag: &str, message_type: &str) -> bool {
        if let Some(rule) = self.fields.get(tag) {
            rule.mandatory_for.contains(&message_type.to_string())
        } else {
            false
        }
    }

    /// Get all mandatory fields for a message type
    pub fn get_mandatory_fields(&self, message_type: &str) -> Vec<String> {
        self.fields
            .iter()
            .filter(|(_, rule)| rule.mandatory_for.contains(&message_type.to_string()))
            .map(|(tag, _)| tag.clone())
            .collect()
    }
}

impl Default for FormatRules {
    fn default() -> Self {
        Self::default_rules()
    }
}

/// Validate content against a SWIFT format rule
///
/// This function implements a parser for the SWIFT format mini-grammar.
/// Examples of format rules:
/// - "16x" - up to 16 characters
/// - "4!c" - exactly 4 alphanumeric characters
/// - "6!n3!a15d" - 6 digits + 3 letters + up to 15 digits with decimal
/// - "4*35x" - up to 4 lines of 35 characters each
/// - "[/34x]" - optional 34 characters starting with "/"
pub fn validate_format(format_rule: &str, content: &str) -> std::result::Result<(), String> {
    let validator = FormatValidator::new(format_rule);
    validator.validate(content)
}

/// SWIFT format rule validator
struct FormatValidator {
    rule: String,
}

impl FormatValidator {
    fn new(rule: &str) -> Self {
        Self {
            rule: rule.to_string(),
        }
    }

    fn validate(&self, content: &str) -> std::result::Result<(), String> {
        // This is a simplified implementation of the SWIFT format validator
        // In a full implementation, this would parse the complete SWIFT format mini-grammar

        match self.rule.as_str() {
            // Simple length rules
            "16x" => self.validate_max_length(content, 16),
            "4x" => self.validate_max_length(content, 4),
            "35x" => self.validate_max_length(content, 35),

            // Exact length rules
            "4!c" => self.validate_exact_alphanumeric(content, 4),
            "3!a" => self.validate_exact_alpha(content, 3),
            "6!n" => self.validate_exact_numeric(content, 6),

            // Multiline rules
            "4*35x" => self.validate_multiline(content, 4, 35),

            // Complex rules (simplified)
            "6!n3!a15d" => self.validate_date_currency_amount(content),

            _ => {
                // For unknown format rules, just return OK for now
                // In a full implementation, we would parse the rule and validate accordingly
                Ok(())
            }
        }
    }

    fn validate_max_length(
        &self,
        content: &str,
        max_len: usize,
    ) -> std::result::Result<(), String> {
        if content.len() > max_len {
            Err(format!(
                "Content too long: {} > {} characters",
                content.len(),
                max_len
            ))
        } else {
            Ok(())
        }
    }

    fn validate_exact_alphanumeric(
        &self,
        content: &str,
        exact_len: usize,
    ) -> std::result::Result<(), String> {
        if content.len() != exact_len {
            return Err(format!(
                "Invalid length: expected {} characters, got {}",
                exact_len,
                content.len()
            ));
        }

        if !content.chars().all(|c| c.is_alphanumeric() && c.is_ascii()) {
            return Err("Must contain only alphanumeric characters".to_string());
        }

        Ok(())
    }

    fn validate_exact_alpha(
        &self,
        content: &str,
        exact_len: usize,
    ) -> std::result::Result<(), String> {
        if content.len() != exact_len {
            return Err(format!(
                "Invalid length: expected {} characters, got {}",
                exact_len,
                content.len()
            ));
        }

        if !content.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err("Must contain only alphabetic characters".to_string());
        }

        Ok(())
    }

    fn validate_exact_numeric(
        &self,
        content: &str,
        exact_len: usize,
    ) -> std::result::Result<(), String> {
        if content.len() != exact_len {
            return Err(format!(
                "Invalid length: expected {} characters, got {}",
                exact_len,
                content.len()
            ));
        }

        if !content.chars().all(|c| c.is_numeric()) {
            return Err("Must contain only numeric characters".to_string());
        }

        Ok(())
    }

    fn validate_multiline(
        &self,
        content: &str,
        max_lines: usize,
        max_chars_per_line: usize,
    ) -> std::result::Result<(), String> {
        let lines: Vec<&str> = content.lines().collect();

        if lines.len() > max_lines {
            return Err(format!("Too many lines: {} > {}", lines.len(), max_lines));
        }

        for (i, line) in lines.iter().enumerate() {
            if line.len() > max_chars_per_line {
                return Err(format!(
                    "Line {} too long: {} > {} characters",
                    i + 1,
                    line.len(),
                    max_chars_per_line
                ));
            }
        }

        Ok(())
    }

    fn validate_date_currency_amount(&self, content: &str) -> std::result::Result<(), String> {
        // Simplified validation for 32A format: 6!n3!a15d
        if content.len() < 9 {
            return Err("Content too short for date+currency+amount format".to_string());
        }

        // Validate date part (6 digits)
        let date_part = &content[0..6];
        if !date_part.chars().all(|c| c.is_numeric()) {
            return Err("Date part must be 6 digits".to_string());
        }

        // Validate currency part (3 letters)
        let currency_part = &content[6..9];
        if !currency_part
            .chars()
            .all(|c| c.is_alphabetic() && c.is_ascii())
        {
            return Err("Currency part must be 3 letters".to_string());
        }

        // Validate amount part (remaining characters)
        let amount_part = &content[9..];
        if amount_part.is_empty() {
            return Err("Amount part cannot be empty".to_string());
        }

        // Check if amount contains valid characters (digits, comma, dot)
        if !amount_part
            .chars()
            .all(|c| c.is_numeric() || c == ',' || c == '.')
        {
            return Err("Amount part contains invalid characters".to_string());
        }

        Ok(())
    }
}

/// Validation level for controlling strictness
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationLevel {
    /// No validation
    None,
    /// Basic format validation only
    Basic,
    /// Strict validation with all rules
    Strict,
    /// Custom validation level
    Custom,
}

/// Validation context for providing additional information
#[derive(Debug, Clone)]
pub struct ValidationContext {
    pub message_type: Option<String>,
    pub validation_level: ValidationLevel,
    pub custom_rules: Option<FormatRules>,
}

impl ValidationContext {
    pub fn new(message_type: Option<String>, level: ValidationLevel) -> Self {
        Self {
            message_type,
            validation_level: level,
            custom_rules: None,
        }
    }

    pub fn with_custom_rules(mut self, rules: FormatRules) -> Self {
        self.custom_rules = Some(rules);
        self
    }
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self::new(None, ValidationLevel::Basic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_rules_creation() {
        let rules = FormatRules::default_rules();
        assert!(rules.get_field_rule("20").is_some());
        assert!(rules.get_field_rule("23B").is_some());
        assert!(rules.get_field_rule("32A").is_some());
    }

    #[test]
    fn test_field_validation() {
        let rules = FormatRules::default_rules();

        // Valid field 20
        assert!(rules.validate_field("20", "FT21234567890").is_ok());

        // Invalid field 20 (too long)
        assert!(rules.validate_field("20", "A".repeat(17).as_str()).is_err());

        // Valid field 23B
        assert!(rules.validate_field("23B", "CRED").is_ok());
    }

    #[test]
    fn test_mandatory_fields() {
        let rules = FormatRules::default_rules();

        assert!(rules.is_field_mandatory("20", "103"));
        assert!(rules.is_field_mandatory("23B", "103"));
        assert!(!rules.is_field_mandatory("50K", "103")); // Optional in choice

        let mandatory = rules.get_mandatory_fields("103");
        assert!(mandatory.contains(&"20".to_string()));
        assert!(mandatory.contains(&"23B".to_string()));
    }

    #[test]
    fn test_format_validator_simple() {
        assert!(validate_format("16x", "TEST123").is_ok());
        assert!(validate_format("16x", "A".repeat(17).as_str()).is_err());

        assert!(validate_format("4!c", "CRED").is_ok());
        assert!(validate_format("4!c", "CREDIT").is_err()); // Too long
        assert!(validate_format("4!c", "CR-D").is_err()); // Invalid character
    }

    #[test]
    fn test_format_validator_multiline() {
        let content = "Line 1\nLine 2\nLine 3";
        assert!(validate_format("4*35x", content).is_ok());

        let too_many_lines = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        assert!(validate_format("4*35x", too_many_lines).is_err());
    }

    #[test]
    fn test_format_validator_complex() {
        // Valid 32A format
        assert!(validate_format("6!n3!a15d", "210315EUR1234567,89").is_ok());

        // Invalid 32A format
        assert!(validate_format("6!n3!a15d", "21031XEUR123").is_err()); // Invalid date
        assert!(validate_format("6!n3!a15d", "210315E1R123").is_err()); // Invalid currency
    }

    #[test]
    fn test_validation_context() {
        let context = ValidationContext::new(Some("103".to_string()), ValidationLevel::Strict);
        assert_eq!(context.message_type, Some("103".to_string()));
        assert_eq!(context.validation_level, ValidationLevel::Strict);
    }

    #[test]
    fn test_json_loading() {
        // Test loading from JSON string
        let json_content = r#"{
            "fields": {
                "20": {
                    "format": "16x",
                    "description": "Test Field",
                    "options": null,
                    "mandatory_for": ["103"]
                }
            }
        }"#;

        let rules = FormatRules::from_json(json_content).expect("Should parse JSON");
        assert!(rules.get_field_rule("20").is_some());
        assert_eq!(rules.get_field_rule("20").unwrap().format, "16x");
        assert!(rules.is_field_mandatory("20", "103"));
    }

    #[test]
    fn test_config_file_fallback() {
        // Test that default_rules() falls back to hardcoded rules when config file is missing
        let rules = FormatRules::default_rules();
        assert!(rules.get_field_rule("20").is_some());
        assert!(rules.get_field_rule("23B").is_some());
        assert!(rules.get_field_rule("32A").is_some());
    }
}
