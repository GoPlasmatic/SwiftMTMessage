//! Field 26T: Transaction Type Code
//!
//! Code indicating the transaction type according to EUROSTAT Balance of Payments guidelines.
//! Format: 3!c (3 alphanumeric characters)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 26T: Transaction Type Code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field26T {
    /// Transaction type code (3 alphanumeric characters)
    pub transaction_type_code: String,
}

impl Field26T {
    /// Create a new Field26T with validation
    pub fn new(transaction_type_code: impl Into<String>) -> Result<Self> {
        let code = transaction_type_code.into().trim().to_uppercase();

        if code.is_empty() {
            return Err(FieldParseError::missing_data(
                "26T",
                "Transaction type code cannot be empty",
            )
            .into());
        }

        if code.len() != 3 {
            return Err(FieldParseError::invalid_format(
                "26T",
                "Transaction type code must be exactly 3 characters",
            )
            .into());
        }

        // Validate characters (alphanumeric)
        if !code.chars().all(|c| c.is_alphanumeric() && c.is_ascii()) {
            return Err(FieldParseError::invalid_format(
                "26T",
                "Transaction type code must contain only alphanumeric characters",
            )
            .into());
        }

        Ok(Field26T {
            transaction_type_code: code,
        })
    }

    /// Get the transaction type code
    pub fn code(&self) -> &str {
        &self.transaction_type_code
    }
}

impl SwiftField for Field26T {
    const TAG: &'static str = "26T";

    fn parse(content: &str) -> Result<Self> {
        Self::new(content.trim())
    }

    fn to_swift_string(&self) -> String {
        format!(":26T:{}", self.transaction_type_code)
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        rules.validate_field("26T", &self.transaction_type_code)
    }

    fn description() -> &'static str {
        "Transaction Type Code - EUROSTAT Balance of Payments code"
    }
}

impl std::fmt::Display for Field26T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.transaction_type_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_parser::FormatRules;
    use std::collections::HashMap;

    #[test]
    fn test_field26t_creation() {
        let field = Field26T::new("A01").unwrap();
        assert_eq!(field.transaction_type_code, "A01");
        assert_eq!(field.code(), "A01");
    }

    #[test]
    fn test_field26t_parse() {
        let field = Field26T::parse("B02").unwrap();
        assert_eq!(field.transaction_type_code, "B02");
    }

    #[test]
    fn test_field26t_case_normalization() {
        let field = Field26T::new("c03").unwrap();
        assert_eq!(field.transaction_type_code, "C03");
    }

    #[test]
    fn test_field26t_invalid_length() {
        let result = Field26T::new("AB"); // Too short
        assert!(result.is_err());

        let result = Field26T::new("ABCD"); // Too long
        assert!(result.is_err());
    }

    #[test]
    fn test_field26t_invalid_characters() {
        let result = Field26T::new("A@1"); // Invalid character @
        assert!(result.is_err());

        let result = Field26T::new("A-1"); // Invalid character -
        assert!(result.is_err());
    }

    #[test]
    fn test_field26t_empty() {
        let result = Field26T::new("");
        assert!(result.is_err());

        let result = Field26T::new("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_field26t_to_swift_string() {
        let field = Field26T::new("D04").unwrap();
        assert_eq!(field.to_swift_string(), ":26T:D04");
    }

    #[test]
    fn test_field26t_validation() {
        let field = Field26T::new("E05").unwrap();
        let rules = FormatRules {
            fields: HashMap::new(),
        };
        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field26t_display() {
        let field = Field26T::new("F06").unwrap();
        assert_eq!(format!("{}", field), "F06");
    }
}
