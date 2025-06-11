//! Configuration management for SWIFT MT message processing
//!
//! This module handles loading and managing configuration from JSON files,
//! including mandatory field mappings and field validation rules.

use crate::errors::{ParseError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Configuration for mandatory fields per message type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MandatoryFieldsConfig {
    /// Map of message type -> list of mandatory field tags
    pub message_types: HashMap<String, Vec<String>>,
}

/// Field validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValidationConfig {
    /// Field-specific validation rules
    pub fields: HashMap<String, FieldValidationRule>,
    /// Common validation patterns that can be reused
    pub patterns: HashMap<String, ValidationPattern>,
}

/// Validation rule for a specific field
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FieldValidationRule {
    /// Field description
    pub description: String,
    /// Maximum length (optional)
    pub max_length: Option<usize>,
    /// Exact length (optional)
    pub exact_length: Option<usize>,
    /// Minimum length (optional)
    pub min_length: Option<usize>,
    /// Character validation pattern
    pub pattern: Option<String>,
    /// Custom validation pattern reference
    pub pattern_ref: Option<String>,
    /// Whether empty values are allowed
    pub allow_empty: Option<bool>,
    /// Maximum number of lines for multi-line fields
    pub max_lines: Option<usize>,
    /// Maximum characters per line for multi-line fields
    pub max_chars_per_line: Option<usize>,
    /// BIC validation required
    pub bic_validation: Option<bool>,
    /// Account validation required
    pub account_validation: Option<bool>,
    /// Case normalization (upper, lower, none)
    pub case_normalization: Option<String>,
    /// Valid values list (for enumerated fields)
    pub valid_values: Option<Vec<String>>,
}

/// Reusable validation pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationPattern {
    /// Pattern name
    pub name: String,
    /// Pattern description
    pub description: String,
    /// Regex pattern
    pub regex: Option<String>,
    /// Character set validation
    pub charset: Option<CharsetValidation>,
}

/// Character set validation options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharsetValidation {
    /// Allow alphabetic characters
    pub alphabetic: Option<bool>,
    /// Allow numeric characters
    pub numeric: Option<bool>,
    /// Allow alphanumeric characters
    pub alphanumeric: Option<bool>,
    /// Allow ASCII printable characters
    pub ascii_printable: Option<bool>,
    /// Specific allowed characters
    pub allowed_chars: Option<String>,
    /// Specific forbidden characters
    pub forbidden_chars: Option<String>,
}

/// Main configuration container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwiftConfig {
    /// Mandatory fields configuration
    pub mandatory_fields: MandatoryFieldsConfig,
    /// Field validation configuration
    pub field_validations: FieldValidationConfig,
}

/// Configuration loader
pub struct ConfigLoader {
    config: SwiftConfig,
}

impl ConfigLoader {
    /// Load configuration from JSON files
    pub fn load_from_directory<P: AsRef<Path>>(config_dir: P) -> Result<Self> {
        let config_dir = config_dir.as_ref();

        // Load mandatory fields
        let mandatory_fields_path = config_dir.join("mandatory_fields.json");
        let mandatory_fields = if mandatory_fields_path.exists() {
            let content =
                fs::read_to_string(&mandatory_fields_path).map_err(|e| ParseError::IoError {
                    message: e.to_string(),
                })?;
            serde_json::from_str(&content)?
        } else {
            // Fallback to default configuration
            Self::default_mandatory_fields_config()
        };

        // Load field validations
        let validations_path = config_dir.join("field_validations.json");
        let field_validations = if validations_path.exists() {
            let content =
                fs::read_to_string(&validations_path).map_err(|e| ParseError::IoError {
                    message: e.to_string(),
                })?;
            serde_json::from_str(&content)?
        } else {
            // Fallback to default configuration
            Self::default_field_validations_config()
        };

        Ok(ConfigLoader {
            config: SwiftConfig {
                mandatory_fields,
                field_validations,
            },
        })
    }

    /// Load configuration with defaults (used when config files don't exist)
    pub fn load_defaults() -> Self {
        ConfigLoader {
            config: SwiftConfig {
                mandatory_fields: Self::default_mandatory_fields_config(),
                field_validations: Self::default_field_validations_config(),
            },
        }
    }

    /// Get the loaded configuration
    pub fn config(&self) -> &SwiftConfig {
        &self.config
    }

    /// Check if a field is mandatory for a specific message type
    pub fn is_field_mandatory(&self, field_tag: &str, message_type: &str) -> bool {
        if let Some(mandatory_fields) = self.config.mandatory_fields.message_types.get(message_type)
        {
            // First check exact match
            if mandatory_fields.contains(&field_tag.to_string()) {
                return true;
            }

            // Then handle field options (e.g., "50A", "50K", "50F" all map to "50")
            // This should only apply to fields that are purely numeric base + alpha option
            if field_tag.len() > 2 {
                let chars: Vec<char> = field_tag.chars().collect();
                // Check if it's all digits followed by a single alpha (like "50A", "51A")
                let is_option_pattern = chars.len() == 3
                    && chars[0].is_ascii_digit()
                    && chars[1].is_ascii_digit()
                    && chars[2].is_alphabetic();

                if is_option_pattern {
                    let base_tag = &field_tag[..2];
                    return mandatory_fields.contains(&base_tag.to_string());
                }
            }

            false
        } else {
            false
        }
    }

    /// Get validation rule for a field
    pub fn get_field_validation(&self, field_tag: &str) -> Option<&FieldValidationRule> {
        self.config.field_validations.fields.get(field_tag)
    }

    /// Get validation pattern by name
    pub fn get_validation_pattern(&self, pattern_name: &str) -> Option<&ValidationPattern> {
        self.config.field_validations.patterns.get(pattern_name)
    }

    /// Get all mandatory fields for a message type
    pub fn get_mandatory_fields(&self, message_type: &str) -> Vec<String> {
        self.config
            .mandatory_fields
            .message_types
            .get(message_type)
            .cloned()
            .unwrap_or_default()
    }

    /// Default mandatory fields configuration
    fn default_mandatory_fields_config() -> MandatoryFieldsConfig {
        let mut message_types = HashMap::new();

        // MT103: Single Customer Credit Transfer
        message_types.insert(
            "103".to_string(),
            vec![
                "20".to_string(),
                "23B".to_string(),
                "32A".to_string(),
                "50".to_string(),
                "59".to_string(),
                "71A".to_string(),
            ],
        );

        // MT102: Multiple Customer Credit Transfer
        message_types.insert(
            "102".to_string(),
            vec![
                "20".to_string(),
                "23B".to_string(),
                "32A".to_string(),
                "50".to_string(),
                "71A".to_string(),
            ],
        );

        // MT202: General Financial Institution Transfer
        message_types.insert(
            "202".to_string(),
            vec![
                "20".to_string(),
                "32A".to_string(),
                "52A".to_string(),
                "58A".to_string(),
            ],
        );

        // MT199: Free Format Message
        message_types.insert("199".to_string(), vec!["20".to_string(), "79".to_string()]);

        // MT192: Request for Cancellation
        message_types.insert("192".to_string(), vec!["20".to_string(), "21".to_string()]);

        // MT195: Queries
        message_types.insert("195".to_string(), vec!["20".to_string(), "21".to_string()]);

        // MT196: Answers
        message_types.insert("196".to_string(), vec!["20".to_string(), "21".to_string()]);

        // MT197: Copy of a Message
        message_types.insert("197".to_string(), vec!["20".to_string(), "21".to_string()]);

        // MT940: Customer Statement Message
        message_types.insert(
            "940".to_string(),
            vec![
                "20".to_string(),
                "25".to_string(),
                "28C".to_string(),
                "60F".to_string(),
                "62F".to_string(),
            ],
        );

        // MT941: Balance Report Message
        message_types.insert(
            "941".to_string(),
            vec!["20".to_string(), "25".to_string(), "28C".to_string()],
        );

        // MT942: Interim Transaction Report
        message_types.insert(
            "942".to_string(),
            vec![
                "20".to_string(),
                "25".to_string(),
                "28C".to_string(),
                "34F".to_string(),
            ],
        );

        MandatoryFieldsConfig { message_types }
    }

    /// Default field validations configuration
    fn default_field_validations_config() -> FieldValidationConfig {
        let mut fields = HashMap::new();
        let mut patterns = HashMap::new();

        // Define common patterns
        patterns.insert(
            "bic".to_string(),
            ValidationPattern {
                name: "bic".to_string(),
                description: "Bank Identifier Code format".to_string(),
                regex: Some(r"^[A-Z]{4}[A-Z]{2}[A-Z0-9]{2}([A-Z0-9]{3})?$".to_string()),
                charset: Some(CharsetValidation {
                    alphanumeric: Some(true),
                    ascii_printable: Some(true),
                    ..Default::default()
                }),
            },
        );

        patterns.insert(
            "ascii_printable".to_string(),
            ValidationPattern {
                name: "ascii_printable".to_string(),
                description: "ASCII printable characters".to_string(),
                regex: None,
                charset: Some(CharsetValidation {
                    ascii_printable: Some(true),
                    ..Default::default()
                }),
            },
        );

        patterns.insert(
            "alphanumeric".to_string(),
            ValidationPattern {
                name: "alphanumeric".to_string(),
                description: "Alphanumeric characters only".to_string(),
                regex: None,
                charset: Some(CharsetValidation {
                    alphanumeric: Some(true),
                    ..Default::default()
                }),
            },
        );

        patterns.insert(
            "alphabetic".to_string(),
            ValidationPattern {
                name: "alphabetic".to_string(),
                description: "Alphabetic characters only".to_string(),
                regex: None,
                charset: Some(CharsetValidation {
                    alphabetic: Some(true),
                    ..Default::default()
                }),
            },
        );

        // Define field validation rules
        fields.insert(
            "20".to_string(),
            FieldValidationRule {
                description: "Transaction Reference Number".to_string(),
                max_length: Some(16),
                pattern_ref: Some("ascii_printable".to_string()),
                allow_empty: Some(false),
                ..Default::default()
            },
        );

        fields.insert(
            "23B".to_string(),
            FieldValidationRule {
                description: "Bank Operation Code".to_string(),
                exact_length: Some(4),
                pattern_ref: Some("alphanumeric".to_string()),
                allow_empty: Some(false),
                case_normalization: Some("upper".to_string()),
                ..Default::default()
            },
        );

        fields.insert(
            "32A".to_string(),
            FieldValidationRule {
                description: "Value Date/Currency/Amount".to_string(),
                min_length: Some(9),
                allow_empty: Some(false),
                ..Default::default()
            },
        );

        fields.insert(
            "50".to_string(),
            FieldValidationRule {
                description: "Ordering Customer".to_string(),
                max_lines: Some(4),
                max_chars_per_line: Some(35),
                pattern_ref: Some("ascii_printable".to_string()),
                allow_empty: Some(false),
                account_validation: Some(true),
                ..Default::default()
            },
        );

        fields.insert(
            "52".to_string(),
            FieldValidationRule {
                description: "Ordering Institution".to_string(),
                max_lines: Some(4),
                max_chars_per_line: Some(35),
                pattern_ref: Some("ascii_printable".to_string()),
                allow_empty: Some(false),
                bic_validation: Some(true),
                account_validation: Some(true),
                ..Default::default()
            },
        );

        fields.insert(
            "59".to_string(),
            FieldValidationRule {
                description: "Beneficiary Customer".to_string(),
                max_lines: Some(4),
                max_chars_per_line: Some(35),
                pattern_ref: Some("ascii_printable".to_string()),
                allow_empty: Some(false),
                account_validation: Some(true),
                ..Default::default()
            },
        );

        fields.insert(
            "70".to_string(),
            FieldValidationRule {
                description: "Remittance Information".to_string(),
                max_lines: Some(4),
                max_chars_per_line: Some(35),
                pattern_ref: Some("ascii_printable".to_string()),
                allow_empty: Some(true),
                ..Default::default()
            },
        );

        fields.insert(
            "71A".to_string(),
            FieldValidationRule {
                description: "Details of Charges".to_string(),
                exact_length: Some(3),
                pattern_ref: Some("alphabetic".to_string()),
                allow_empty: Some(false),
                case_normalization: Some("upper".to_string()),
                valid_values: Some(vec![
                    "BEN".to_string(),
                    "OUR".to_string(),
                    "SHA".to_string(),
                ]),
                ..Default::default()
            },
        );

        fields.insert(
            "72".to_string(),
            FieldValidationRule {
                description: "Sender to Receiver Information".to_string(),
                max_lines: Some(6),
                max_chars_per_line: Some(35),
                pattern_ref: Some("ascii_printable".to_string()),
                allow_empty: Some(true),
                ..Default::default()
            },
        );

        FieldValidationConfig { fields, patterns }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_loading() {
        let loader = ConfigLoader::load_defaults();

        // Test mandatory fields
        assert!(loader.is_field_mandatory("20", "103"));
        assert!(loader.is_field_mandatory("23B", "103"));
        assert!(!loader.is_field_mandatory("72", "103"));

        // Test field options
        assert!(loader.is_field_mandatory("50A", "103"));
        assert!(loader.is_field_mandatory("50K", "103"));

        // Test field validation
        let field_20_validation = loader.get_field_validation("20");
        assert!(field_20_validation.is_some());
        assert_eq!(field_20_validation.unwrap().max_length, Some(16));

        // Test patterns
        let ascii_pattern = loader.get_validation_pattern("ascii_printable");
        assert!(ascii_pattern.is_some());
    }

    #[test]
    fn test_get_mandatory_fields() {
        let loader = ConfigLoader::load_defaults();
        let mt103_fields = loader.get_mandatory_fields("103");

        assert!(mt103_fields.contains(&"20".to_string()));
        assert!(mt103_fields.contains(&"23B".to_string()));
        assert!(mt103_fields.contains(&"32A".to_string()));
    }
}
