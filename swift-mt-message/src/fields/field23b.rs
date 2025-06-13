use crate::SwiftField;
use serde::{Deserialize, Serialize};

/// Field 23B: Bank Operation Code
///
/// Format: 4!c (exactly 4 alphabetic characters)
///
/// This field specifies the type of operation.
/// Common values: CRED, CRTS, SPAY, SSTD
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
#[format("4!c")]
pub struct Field23B {
    /// Bank operation code (4 characters)
    #[format("4!c")]
    pub bank_operation_code: String,
}

impl Field23B {
    /// Create a new Field23B with the given operation code
    pub fn new(bank_operation_code: String) -> Self {
        Self {
            bank_operation_code: bank_operation_code.to_uppercase(),
        }
    }

    /// Get the operation code
    pub fn operation_code(&self) -> &str {
        &self.bank_operation_code
    }

    /// Check if this is a standard operation code
    pub fn is_standard_code(&self) -> bool {
        matches!(
            self.bank_operation_code.as_str(),
            "CRED" | "CRTS" | "SPAY" | "SSTD"
        )
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

    #[test]
    fn test_field23b_creation() {
        let field = Field23B::new("CRED".to_string());
        assert_eq!(field.operation_code(), "CRED");
        assert!(field.is_standard_code());
    }

    #[test]
    fn test_field23b_parse() {
        let field = Field23B::parse("CRED").unwrap();
        assert_eq!(field.bank_operation_code, "CRED");
    }

    #[test]
    fn test_field23b_case_insensitive() {
        let field = Field23B::new("cred".to_string());
        assert_eq!(field.bank_operation_code, "CRED");
    }
}
