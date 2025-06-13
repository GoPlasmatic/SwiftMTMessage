use crate::SwiftField;
use serde::{Deserialize, Serialize};

/// Field 20: Transaction Reference
///
/// Format: 16x (up to 16 alphanumeric characters)
///
/// This field contains the sender's reference to the related transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
#[format("16x")]
pub struct Field20 {
    /// Transaction reference value (max 16 characters)
    #[format("16x")]
    pub transaction_reference: String,
}

impl Field20 {
    /// Create a new Field20 with the given transaction reference
    pub fn new(transaction_reference: String) -> Self {
        Self {
            transaction_reference,
        }
    }

    /// Get the transaction reference value
    pub fn transaction_reference(&self) -> &str {
        &self.transaction_reference
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

    #[test]
    fn test_field20_creation() {
        let field = Field20::new("FT21234567890".to_string());
        assert_eq!(field.transaction_reference(), "FT21234567890");
    }

    #[test]
    fn test_field20_parse() {
        let field = Field20::parse("FT21234567890").unwrap();
        assert_eq!(field.transaction_reference, "FT21234567890");
    }

    #[test]
    fn test_field20_parse_with_prefix() {
        let field = Field20::parse(":20:FT21234567890").unwrap();
        assert_eq!(field.transaction_reference, "FT21234567890");
    }

    #[test]
    fn test_field20_to_swift_string() {
        let field = Field20::new("FT21234567890".to_string());
        assert_eq!(field.to_swift_string(), ":20:FT21234567890");
    }

    #[test]
    fn test_field20_validation() {
        let valid_field = Field20::new("FT12345".to_string());
        let result = valid_field.validate();
        assert!(result.is_valid);

        let invalid_field = Field20::new("THIS_IS_TOO_LONG_FOR_FIELD20".to_string());
        let result = invalid_field.validate();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_field20_format_spec() {
        assert_eq!(Field20::format_spec(), "16x");
    }
}
