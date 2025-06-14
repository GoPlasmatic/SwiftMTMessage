use crate::{SwiftField, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 56C: Intermediary Institution (Option C)
///
/// ## Overview
/// Field 56C identifies an intermediary institution in SWIFT payment messages using an account
/// number or identifier. This field provides an alternative to BIC-based identification when
/// the intermediary institution is identified through an account number, clearing code, or
/// other identifier system. This option is particularly useful in domestic payment systems
/// or when specific account-based routing is required.
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
/// - **Account Number**: Intermediary institution identifier
///   - Can be account number, clearing code, or routing identifier
///   - Maximum 34 characters
///   - Must comply with SWIFT character set
///
/// ## Usage Context
/// Field 56C is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Domestic routing**: Using national clearing codes
/// - **Account-based identification**: When BIC is not available or preferred
/// - **Clearing system integration**: Interfacing with local clearing systems
/// - **Correspondent banking**: Account-based correspondent identification
/// - **Cost optimization**: Reducing correspondent banking fees
/// - **Regional payments**: Supporting regional payment networks
///
/// ## Examples
/// ```text
/// :56C:/INTERMEDIARYACCT123456
/// └─── Intermediary account number
///
/// :56C:/CLRCODE123456789
/// └─── Clearing code identifier
///
/// :56C:/FEDWIRE021000021
/// └─── US Federal Reserve routing number
///
/// :56C:/SORTCODE123456
/// └─── UK sort code based identifier
///
/// :56C:/IFSC0001234
/// └─── Indian Financial System Code
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
/// - Field 56C alternative to 56A/56D (Error: C56)
/// - Must be valid for receiving country's system (Error: T50)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field56C {
    /// Account number (up to 34 characters)
    pub account_number: String,
}

impl Field56C {
    /// Create a new Field56C with validation
    pub fn new(account_number: impl Into<String>) -> Result<Self, crate::ParseError> {
        let account_number = account_number.into().trim().to_string();

        // Validate account number
        if account_number.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "56C".to_string(),
                message: "Account number cannot be empty".to_string(),
            });
        }

        if account_number.len() > 34 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "56C".to_string(),
                message: "Account number cannot exceed 34 characters".to_string(),
            });
        }

        if !account_number
            .chars()
            .all(|c| c.is_ascii() && !c.is_control())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "56C".to_string(),
                message: "Account number contains invalid characters".to_string(),
            });
        }

        Ok(Field56C { account_number })
    }

    /// Get the account number
    pub fn account_number(&self) -> &str {
        &self.account_number
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!(
            "Intermediary Institution (Account: {})",
            self.account_number
        )
    }
}

impl SwiftField for Field56C {
    fn parse(content: &str) -> crate::Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "56C".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let content = if let Some(stripped) = content.strip_prefix(":56C:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("56C:") {
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
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "56C".to_string(),
                message: "Account number is required".to_string(),
            });
        }

        Field56C::new(account_number)
    }

    fn to_swift_string(&self) -> String {
        format!(":56C:/{}", self.account_number)
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

impl std::fmt::Display for Field56C {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Account: {}", self.account_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field56c_creation() {
        let field = Field56C::new("INTERMEDIARYACCT123456").unwrap();
        assert_eq!(field.account_number(), "INTERMEDIARYACCT123456");
    }

    #[test]
    fn test_field56c_parse() {
        let field = Field56C::parse("/INTERMEDIARYACCT123456").unwrap();
        assert_eq!(field.account_number(), "INTERMEDIARYACCT123456");
    }

    #[test]
    fn test_field56c_parse_without_slash() {
        let field = Field56C::parse("INTERMEDIARYACCT123456").unwrap();
        assert_eq!(field.account_number(), "INTERMEDIARYACCT123456");
    }

    #[test]
    fn test_field56c_parse_with_tag() {
        let field = Field56C::parse(":56C:/INTERMEDIARYACCT123456").unwrap();
        assert_eq!(field.account_number(), "INTERMEDIARYACCT123456");
    }

    #[test]
    fn test_field56c_to_swift_string() {
        let field = Field56C::new("INTERMEDIARYACCT123456").unwrap();
        assert_eq!(field.to_swift_string(), ":56C:/INTERMEDIARYACCT123456");
    }

    #[test]
    fn test_field56c_display() {
        let field = Field56C::new("INTERMEDIARYACCT123456").unwrap();
        assert_eq!(format!("{}", field), "Account: INTERMEDIARYACCT123456");
    }

    #[test]
    fn test_field56c_description() {
        let field = Field56C::new("INTERMEDIARYACCT123456").unwrap();
        assert_eq!(
            field.description(),
            "Intermediary Institution (Account: INTERMEDIARYACCT123456)"
        );
    }

    #[test]
    fn test_field56c_validation_empty_account() {
        let result = Field56C::new("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_field56c_validation_account_too_long() {
        let account = "A".repeat(35); // 35 characters, max is 34
        let result = Field56C::new(account);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("cannot exceed 34 characters")
        );
    }

    #[test]
    fn test_field56c_validation_invalid_characters() {
        let result = Field56C::new("ACCOUNT\x00ID"); // Contains null character
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid characters")
        );
    }

    #[test]
    fn test_field56c_validate() {
        let field = Field56C::new("INTERMEDIARYACCT123456").unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }
}
