//! Field 23B: Bank Operation Code
//!
//! Identifies the type of operation/service/facility requested or provided.
//! Format: 4!c (exactly 4 alphanumeric characters)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use crate::utils::character;
use serde::{Deserialize, Serialize};

/// Field 23B: Bank Operation Code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field23B {
    /// Bank operation code (exactly 4 characters)
    pub bank_operation_code: String,
}

impl Field23B {
    /// Common bank operation codes
    pub const CRED: &'static str = "CRED"; // Credit transfer
    pub const SPAY: &'static str = "SPAY"; // Salary payment
    pub const SSBE: &'static str = "SSBE"; // Social security benefit
    pub const SUPP: &'static str = "SUPP"; // Supplier payment
    pub const CORT: &'static str = "CORT"; // Court ordered payment
    pub const REPA: &'static str = "REPA"; // Reimbursement of advance
    pub const INTC: &'static str = "INTC"; // Intracompany payment
    pub const TRAD: &'static str = "TRAD"; // Trade
    pub const TREA: &'static str = "TREA"; // Treasury payment

    /// Create a new Field23B with validation
    pub fn new(bank_operation_code: impl Into<String>) -> Result<Self> {
        let code = bank_operation_code.into().trim().to_uppercase();

        if code.is_empty() {
            return Err(FieldParseError::missing_data(
                "23B",
                "Bank operation code cannot be empty",
            )
            .into());
        }

        character::validate_exact_length("23B", &code, 4, "Bank operation code")?;
        character::validate_alphanumeric("23B", &code, "Bank operation code")?;

        Ok(Field23B {
            bank_operation_code: code.to_string(),
        })
    }

    /// Get the bank operation code
    pub fn code(&self) -> &str {
        &self.bank_operation_code
    }

    /// Check if this is a credit transfer operation
    pub fn is_credit_transfer(&self) -> bool {
        self.bank_operation_code == Self::CRED
    }

    /// Check if this is a salary payment
    pub fn is_salary_payment(&self) -> bool {
        self.bank_operation_code == Self::SPAY
    }

    /// Get a human-readable description of the operation code
    pub fn description(&self) -> &'static str {
        match self.bank_operation_code.as_str() {
            Self::CRED => "Credit transfer",
            Self::SPAY => "Salary payment",
            Self::SSBE => "Social security benefit",
            Self::SUPP => "Supplier payment",
            Self::CORT => "Court ordered payment",
            Self::REPA => "Reimbursement of advance",
            Self::INTC => "Intracompany payment",
            Self::TRAD => "Trade",
            Self::TREA => "Treasury payment",
            _ => "Unknown operation code",
        }
    }
}

impl SwiftField for Field23B {
    const TAG: &'static str = "23B";

    fn parse(content: &str) -> Result<Self> {
        Self::new(content.trim())
    }

    fn to_swift_string(&self) -> String {
        format!(":23B:{}", self.bank_operation_code)
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        rules.validate_field("23B", &self.bank_operation_code)
    }

    fn description() -> &'static str {
        "Bank Operation Code - Identifies the type of operation/service/facility requested or provided"
    }
}

impl std::fmt::Display for Field23B {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.bank_operation_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_parser::FormatRules;
    use std::collections::HashMap;

    #[test]
    fn test_field23b_creation() {
        let field = Field23B::new("CRED").unwrap();
        assert_eq!(field.code(), "CRED");
        assert_eq!(field.to_swift_string(), ":23B:CRED");
        assert!(field.is_credit_transfer());
    }

    #[test]
    fn test_field23b_lowercase_normalization() {
        let field = Field23B::new("cred").unwrap();
        assert_eq!(field.code(), "CRED");
    }

    #[test]
    fn test_field23b_empty_code() {
        let result = Field23B::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_field23b_wrong_length() {
        let result = Field23B::new("CREDIT"); // 6 characters
        assert!(result.is_err());

        let result = Field23B::new("CR"); // 2 characters
        assert!(result.is_err());
    }

    #[test]
    fn test_field23b_invalid_characters() {
        let result = Field23B::new("CR-D"); // Contains hyphen
        assert!(result.is_err());

        let result = Field23B::new("CR@D"); // Contains special character
        assert!(result.is_err());
    }

    #[test]
    fn test_field23b_parse() {
        let field = Field23B::parse("  SPAY  ").unwrap();
        assert_eq!(field.code(), "SPAY");
        assert!(field.is_salary_payment());
    }

    #[test]
    fn test_field23b_descriptions() {
        let cred_field = Field23B::new("CRED").unwrap();
        assert_eq!(cred_field.description(), "Credit transfer");

        let spay_field = Field23B::new("SPAY").unwrap();
        assert_eq!(spay_field.description(), "Salary payment");

        let unknown_field = Field23B::new("UNKN").unwrap();
        assert_eq!(unknown_field.description(), "Unknown operation code");
    }

    #[test]
    fn test_field23b_validation() {
        let field = Field23B::new("CRED").unwrap();
        let rules = FormatRules {
            fields: HashMap::new(),
        };

        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field23b_display() {
        let field = Field23B::new("TRAD").unwrap();
        assert_eq!(format!("{}", field), "TRAD");
    }
}
