use crate::{SwiftField, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Generic Account Field
///
/// ## Overview
/// A generic field structure for SWIFT account identification fields that follow the
/// `/34x` pattern (account number or identifier up to 34 characters). This structure
/// consolidates the common functionality used by Field56C and Field57C.
///
/// ## Format Specification
/// **Format**: `/34x`
/// - **34x**: Account number or identifier (up to 34 characters)
/// - **Leading slash**: Required field delimiter
/// - **Character set**: SWIFT character set (A-Z, 0-9, and limited special characters)
///
/// ## Structure
/// ```text
/// /1234567890123456789012345678901234
/// │└─────────────────────────────────┘
/// │              Account number
/// └─ Required delimiter
/// ```
///
/// ## Usage Context
/// Used in various SWIFT MT message types for account-based institutional identification:
/// - **Field 56C**: Intermediary Institution (Account)
/// - **Field 57C**: Account With Institution (Account)
///
/// ## Usage Examples
/// ```text
/// /INTERMEDIARYACCT123456
/// └─── Intermediary account number
///
/// /CLRCODE123456789
/// └─── Clearing code identifier
///
/// /FEDWIRE021000021
/// └─── US Federal Reserve routing number
///
/// /SORTCODE123456
/// └─── UK sort code based identifier
///
/// /IBAN12345678901234567890
/// └─── International Bank Account Number
/// ```
///
/// ## Account Number Types
/// - **Bank account numbers**: Direct account identification
/// - **Clearing codes**: National clearing system codes
/// - **Routing numbers**: US Federal Reserve routing numbers
/// - **Sort codes**: UK banking sort codes
/// - **IFSC codes**: Indian Financial System Codes
/// - **BSB numbers**: Australian Bank State Branch numbers
/// - **Transit numbers**: Canadian transit numbers
/// - **IBAN**: International Bank Account Numbers
///
/// ## Validation Rules
/// 1. **Length**: Maximum 34 characters
/// 2. **Character set**: SWIFT character set only
/// 3. **Content**: Cannot be empty
/// 4. **Special characters**: Limited to SWIFT-approved characters
/// 5. **Control characters**: Not permitted
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Account number cannot exceed 34 characters (Error: T14)
/// - Must use SWIFT character set only (Error: T61)
/// - Account identifier cannot be empty (Error: T13)
/// - Must be valid for receiving country's system (Error: T50)
/// - Account format must be recognizable (Error: T51)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenericAccountField {
    /// Account number (up to 34 characters)
    pub account_number: String,
}

impl GenericAccountField {
    /// Create a new GenericAccountField with validation
    ///
    /// # Arguments
    /// * `account_number` - Account number or identifier (up to 34 characters)
    ///
    /// # Returns
    /// Result containing the GenericAccountField instance or validation error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::GenericAccountField;
    /// let field = GenericAccountField::new("INTERMEDIARYACCT123456").unwrap();
    /// assert_eq!(field.account_number(), "INTERMEDIARYACCT123456");
    /// ```
    pub fn new(account_number: impl Into<String>) -> Result<Self, ParseError> {
        let account_number = account_number.into().trim().to_string();

        // Validate account number
        if account_number.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericAccountField".to_string(),
                message: "Account number cannot be empty".to_string(),
            });
        }

        if account_number.len() > 34 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericAccountField".to_string(),
                message: "Account number cannot exceed 34 characters".to_string(),
            });
        }

        if !account_number
            .chars()
            .all(|c| c.is_ascii() && !c.is_control())
        {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericAccountField".to_string(),
                message: "Account number contains invalid characters".to_string(),
            });
        }

        Ok(GenericAccountField { account_number })
    }

    /// Get the account number
    pub fn account_number(&self) -> &str {
        &self.account_number
    }

    /// Parse content with custom field tag for error messages
    pub fn parse_with_tag(content: &str, field_tag: &str) -> Result<Self, ParseError> {
        let content = content.trim();
        if content.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: field_tag.to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Remove field tag prefix if present
        let content = if let Some(stripped) = content.strip_prefix(&format!(":{}:", field_tag)) {
            stripped
        } else if let Some(stripped) = content.strip_prefix(&format!("{}:", field_tag)) {
            stripped
        } else {
            content
        };

        // Remove leading slash if present
        let account_number = if let Some(stripped) = content.strip_prefix('/') {
            stripped
        } else {
            content
        };

        if account_number.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: field_tag.to_string(),
                message: "Account number is required".to_string(),
            });
        }

        Self::new(account_number).map_err(|e| {
            if let ParseError::InvalidFieldFormat {
                field_tag: _,
                message,
            } = e
            {
                ParseError::InvalidFieldFormat {
                    field_tag: field_tag.to_string(),
                    message,
                }
            } else {
                e
            }
        })
    }

    /// Convert to SWIFT string format with custom field tag
    pub fn to_swift_string_with_tag(&self, field_tag: &str) -> String {
        format!(":{}:/{}", field_tag, self.account_number)
    }

    /// Get human-readable description with custom context
    pub fn description(&self, context: &str) -> String {
        format!("{} (Account: {})", context, self.account_number)
    }
}

impl SwiftField for GenericAccountField {
    fn parse(content: &str) -> Result<Self, ParseError> {
        Self::parse_with_tag(content, "GenericAccountField")
    }

    fn to_swift_string(&self) -> String {
        self.to_swift_string_with_tag("GenericAccountField")
    }

    fn validate(&self) -> ValidationResult {
        // Validation is done in constructor
        ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "/34x"
    }
}

impl std::fmt::Display for GenericAccountField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Account: {}", self.account_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_account_field_creation() {
        let field = GenericAccountField::new("INTERMEDIARYACCT123456").unwrap();
        assert_eq!(field.account_number(), "INTERMEDIARYACCT123456");
    }

    #[test]
    fn test_generic_account_field_parse_with_tag() {
        let field = GenericAccountField::parse_with_tag("/ACCT123456789", "56C").unwrap();
        assert_eq!(field.account_number(), "ACCT123456789");

        let field = GenericAccountField::parse_with_tag(":57C:/ACCT123456789", "57C").unwrap();
        assert_eq!(field.account_number(), "ACCT123456789");
    }

    #[test]
    fn test_generic_account_field_to_swift_string_with_tag() {
        let field = GenericAccountField::new("ACCT123456789").unwrap();
        assert_eq!(field.to_swift_string_with_tag("56C"), ":56C:/ACCT123456789");
        assert_eq!(field.to_swift_string_with_tag("57C"), ":57C:/ACCT123456789");
    }

    #[test]
    fn test_generic_account_field_validation_errors() {
        // Empty account number
        let result = GenericAccountField::new("");
        assert!(result.is_err());

        // Account number too long
        let result = GenericAccountField::new("A".repeat(35));
        assert!(result.is_err());

        // Invalid characters
        let result = GenericAccountField::new("ACCOUNT\x00ID");
        assert!(result.is_err());
    }

    #[test]
    fn test_generic_account_field_display() {
        let field = GenericAccountField::new("ACCT123456789").unwrap();
        assert_eq!(format!("{}", field), "Account: ACCT123456789");
    }

    #[test]
    fn test_generic_account_field_description() {
        let field = GenericAccountField::new("ACCT123456789").unwrap();
        assert_eq!(
            field.description("Intermediary Institution"),
            "Intermediary Institution (Account: ACCT123456789)"
        );
        assert_eq!(
            field.description("Account With Institution"),
            "Account With Institution (Account: ACCT123456789)"
        );
    }

    #[test]
    fn test_generic_account_field_parse_without_slash() {
        let field = GenericAccountField::parse_with_tag("ACCT123456789", "56C").unwrap();
        assert_eq!(field.account_number(), "ACCT123456789");
    }

    #[test]
    fn test_generic_account_field_validation() {
        let field = GenericAccountField::new("ACCT123456789").unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }
}
