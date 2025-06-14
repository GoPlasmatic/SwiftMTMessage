use crate::{SwiftField, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 57C: Account With Institution (Option C)
///
/// ## Overview
/// Field 57C identifies the account with institution in SWIFT payment messages using an account
/// number or identifier. This field provides a direct account-based identification method when
/// the beneficiary's bank is identified through an account number, clearing code, or other
/// identifier system. This option is particularly useful in domestic payment systems or when
/// specific account-based routing is required for the final credit destination.
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
/// ## Field Components
/// - **Account Number**: Beneficiary's bank account identifier
///   - Can be account number, clearing code, or routing identifier
///   - Maximum 34 characters
///   - Must comply with SWIFT character set
///
/// ## Usage Context
/// Field 57C is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Domestic routing**: Using national clearing codes for beneficiary banks
/// - **Account-based identification**: When BIC is not available or preferred
/// - **Clearing system integration**: Interfacing with local clearing systems
/// - **Direct account crediting**: Specifying exact account for final credit
/// - **Cost optimization**: Reducing correspondent banking complexity
/// - **Regional payments**: Supporting regional payment networks
///
/// ## Examples
/// ```text
/// :57C:/BENEFICIARYACCT123456
/// └─── Beneficiary's bank account number
///
/// :57C:/CLRCODE987654321
/// └─── Clearing code for beneficiary's bank
///
/// :57C:/FEDWIRE021000021
/// └─── US Federal Reserve routing number
///
/// :57C:/SORTCODE654321
/// └─── UK sort code for beneficiary's bank
///
/// :57C:/IBAN12345678901234567890
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
/// 2. **Format**: Must start with forward slash (/)
/// 3. **Character set**: SWIFT character set only
/// 4. **Content**: Cannot be empty after delimiter
/// 5. **Special characters**: Limited to SWIFT-approved characters
/// 6. **Control characters**: Not permitted
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Account number cannot exceed 34 characters (Error: T14)
/// - Must use SWIFT character set only (Error: T61)
/// - Leading slash is mandatory (Error: T26)
/// - Account identifier cannot be empty (Error: T13)
/// - Field 57C alternative to 57A/57B/57D (Error: C57)
/// - Must be valid for receiving country's system (Error: T50)
/// - Account format must be recognizable (Error: T51)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field57C {
    /// Account number (up to 34 characters)
    pub account_number: String,
}

impl Field57C {
    /// Create a new Field57C with validation
    pub fn new(account_number: impl Into<String>) -> Result<Self, crate::ParseError> {
        let account_number = account_number.into().trim().to_string();

        // Validate account number
        if account_number.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57C".to_string(),
                message: "Account number cannot be empty".to_string(),
            });
        }

        if account_number.len() > 34 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57C".to_string(),
                message: "Account number cannot exceed 34 characters".to_string(),
            });
        }

        if !account_number
            .chars()
            .all(|c| c.is_ascii() && !c.is_control())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57C".to_string(),
                message: "Account number contains invalid characters".to_string(),
            });
        }

        Ok(Field57C { account_number })
    }

    /// Get the account number
    pub fn account_number(&self) -> &str {
        &self.account_number
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!(
            "Account With Institution (Account: {})",
            self.account_number
        )
    }
}

impl SwiftField for Field57C {
    fn parse(content: &str) -> crate::Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57C".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let content = if let Some(stripped) = content.strip_prefix(":57C:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("57C:") {
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

        Field57C::new(account_number)
    }

    fn to_swift_string(&self) -> String {
        format!(":57C:/{}", self.account_number)
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

impl std::fmt::Display for Field57C {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Account: {}", self.account_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field57c_creation() {
        let field = Field57C::new("ACCT123456789").unwrap();
        assert_eq!(field.account_number(), "ACCT123456789");
    }

    #[test]
    fn test_field57c_parse_basic() {
        let field = Field57C::parse("/ACCT123456789").unwrap();
        assert_eq!(field.account_number(), "ACCT123456789");
    }

    #[test]
    fn test_field57c_parse_without_slash() {
        let field = Field57C::parse("ACCT123456789").unwrap();
        assert_eq!(field.account_number(), "ACCT123456789");
    }

    #[test]
    fn test_field57c_parse_with_tag() {
        let field = Field57C::parse(":57C:/ACCT123456789").unwrap();
        assert_eq!(field.account_number(), "ACCT123456789");
    }

    #[test]
    fn test_field57c_to_swift_string() {
        let field = Field57C::new("ACCT123456789").unwrap();
        assert_eq!(field.to_swift_string(), ":57C:/ACCT123456789");
    }

    #[test]
    fn test_field57c_display() {
        let field = Field57C::new("ACCT123456789").unwrap();
        assert_eq!(format!("{}", field), "Account: ACCT123456789");
    }

    #[test]
    fn test_field57c_description() {
        let field = Field57C::new("ACCT123456789").unwrap();
        assert_eq!(
            field.description(),
            "Account With Institution (Account: ACCT123456789)"
        );
    }

    #[test]
    fn test_field57c_validation_empty() {
        let result = Field57C::new("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_field57c_validation_too_long() {
        let account = "A".repeat(35); // 35 characters, max is 34
        let result = Field57C::new(account);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("cannot exceed 34 characters")
        );
    }

    #[test]
    fn test_field57c_validation_invalid_characters() {
        let result = Field57C::new("ACCT\x00123"); // Contains null character
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid characters")
        );
    }

    #[test]
    fn test_field57c_validate() {
        let field = Field57C::new("ACCT123456789").unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }
}
