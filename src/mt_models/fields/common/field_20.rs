//! Field 20: Transaction Reference Number
//!
//! The sender's reference to uniquely identify the message.
//! Format: 16x (up to 16 characters)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use crate::utils::character;
use serde::{Deserialize, Serialize};

/// Field 20: Transaction Reference Number
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field20 {
    /// Transaction reference (up to 16 characters)
    pub transaction_reference: String,
}

impl Field20 {
    /// Create a new Field20 with validation
    pub fn new(transaction_reference: impl Into<String>) -> Result<Self> {
        let reference = transaction_reference.into();

        if reference.is_empty() {
            return Err(FieldParseError::missing_data(
                "20",
                "Transaction reference cannot be empty",
            )
            .into());
        }

        character::validate_max_length("20", &reference, 16, "Transaction reference")?;
        character::validate_ascii_printable("20", &reference, "Transaction reference")?;

        Ok(Field20 {
            transaction_reference: reference.to_string(),
        })
    }

    /// Get the transaction reference
    pub fn reference(&self) -> &str {
        &self.transaction_reference
    }
}

impl SwiftField for Field20 {
    const TAG: &'static str = "20";

    fn parse(content: &str) -> Result<Self> {
        Self::new(content.trim())
    }

    fn to_swift_string(&self) -> String {
        format!(":20:{}", self.transaction_reference)
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        rules.validate_field("20", &self.transaction_reference)
    }

    fn description() -> &'static str {
        "Transaction Reference Number - Sender's reference to uniquely identify the message"
    }
}

impl std::fmt::Display for Field20 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.transaction_reference)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_parser::FormatRules;
    use std::collections::HashMap;

    #[test]
    fn test_field20_creation() {
        let field = Field20::new("FT21234567890").unwrap();
        assert_eq!(field.reference(), "FT21234567890");
        assert_eq!(field.to_swift_string(), ":20:FT21234567890");
    }

    #[test]
    fn test_field20_empty_reference() {
        let result = Field20::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_field20_too_long() {
        let long_reference = "A".repeat(17); // 17 characters
        let result = Field20::new(long_reference);
        assert!(result.is_err());
    }

    #[test]
    fn test_field20_parse() {
        let field = Field20::parse("  FT21234567890  ").unwrap();
        assert_eq!(field.reference(), "FT21234567890");
    }

    #[test]
    fn test_field20_validation() {
        let field = Field20::new("TEST123").unwrap();
        let rules = FormatRules {
            fields: HashMap::new(),
        };

        // Should not fail validation (no specific rule defined)
        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field20_display() {
        let field = Field20::new("TEST123").unwrap();
        assert_eq!(format!("{}", field), "TEST123");
    }

    #[test]
    fn test_field20_invalid_characters() {
        // Test with control characters
        let result = Field20::new("TEST\x00");
        assert!(result.is_err());
    }
}
