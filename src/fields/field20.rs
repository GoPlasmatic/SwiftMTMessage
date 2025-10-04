use super::swift_utils::{parse_max_length, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 20: Sender's Reference**
///
/// ## Purpose
/// Specifies the reference assigned by the Sender to unambiguously identify the message.
/// This reference must be unique within the sender's system and is used to link related
/// messages and confirmations throughout the payment chain.
///
/// ## Format
/// - **Swift Format**: `16x`
/// - **Description**: Up to 16 alphanumeric characters
/// - **Pattern**: May include letters, digits, and limited special characters
/// - **Restrictions**: Must not start or end with `/` and must not contain `//`
///
/// ## Presence
/// - **Status**: Mandatory in all MT messages containing this field
/// - **Swift Error Codes**: T26 (invalid format), T13 (field too long)
/// - **Referenced in Rules**: Network validation rules across multiple message types
///
/// ## Network Validation Rules
/// - **Format Validation**: Must conform to 16x pattern (alphanumeric, max 16 chars)
/// - **Content Validation**: Cannot start/end with `/` or contain consecutive slashes `//`
/// - **Uniqueness**: Should be unique within sender's daily operations for audit purposes
///
/// ## Usage Rules
/// - **Confirmations**: Must be quoted unchanged in related MT900/MT910/MT950 messages
/// - **Cover Payments**: When using cover method, copy to field 21 of associated MT202 COV
/// - **Audit Trail**: Used by institutions to track payment lifecycle and exceptions
/// - **Reference Format**: Common patterns include transaction IDs, invoice numbers, or internal references
///
/// ## Examples
/// ```logic
/// :20:PAYMENT123456
/// :20:INV2024001234
/// :20:TXN20240719001
/// :20:URGPAY240719
/// ```
///
/// ## Related Fields
/// - **Field 21**: Transaction Reference Number (often contains Field 20 value in cover payments)
/// - **Field 61**: Statement Line Reference (in account statements)
/// - **Block 3 {108}**: Message User Reference (system-level tracking)
///
/// ## STP Compliance
/// Field 20 has no specific STP restrictions but must meet standard format requirements.
/// STP processing relies on this field for automated matching and exception handling.
///
/// ## See Also
/// - Swift FIN User Handbook: Message Structure and Field Specifications
/// - MT103 Specification: Customer Credit Transfer requirements
/// - Cover Payment Guidelines: Field 20/21 relationship rules
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field20 {
    /// The sender's reference string (max 16 characters)
    ///
    /// Format: 16x - Up to 16 alphanumeric characters
    /// Validation: No leading/trailing slashes, no consecutive slashes
    pub reference: String,
}

impl SwiftField for Field20 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Parse the reference with max length of 16
        let reference = parse_max_length(input, 16, "Field 20 reference")?;

        // Validate SWIFT character set
        parse_swift_chars(&reference, "Field 20 reference")?;

        // Additional validation: no leading/trailing slashes
        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 20 reference cannot start or end with '/'".to_string(),
            });
        }

        // Additional validation: no consecutive slashes
        if reference.contains("//") {
            return Err(ParseError::InvalidFormat {
                message: "Field 20 reference cannot contain consecutive slashes '//'".to_string(),
            });
        }

        Ok(Field20 { reference })
    }

    fn to_swift_string(&self) -> String {
        format!(":20:{}", self.reference)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field20_parse_valid() {
        let field = Field20::parse("PAYMENT123456").unwrap();
        assert_eq!(field.reference, "PAYMENT123456");

        let field = Field20::parse("INV2024001234").unwrap();
        assert_eq!(field.reference, "INV2024001234");

        let field = Field20::parse("TXN20240719001").unwrap();
        assert_eq!(field.reference, "TXN20240719001");
    }

    #[test]
    fn test_field20_max_length() {
        // Exactly 16 characters should work
        let field = Field20::parse("1234567890ABCDEF").unwrap();
        assert_eq!(field.reference, "1234567890ABCDEF");

        // 17 characters should fail
        assert!(Field20::parse("1234567890ABCDEFG").is_err());
    }

    #[test]
    fn test_field20_slash_validation() {
        // Leading slash should fail
        assert!(Field20::parse("/PAYMENT123").is_err());

        // Trailing slash should fail
        assert!(Field20::parse("PAYMENT123/").is_err());

        // Consecutive slashes should fail
        assert!(Field20::parse("PAY//MENT123").is_err());

        // Single slash in middle should be ok
        let field = Field20::parse("PAY/MENT123").unwrap();
        assert_eq!(field.reference, "PAY/MENT123");
    }

    #[test]
    fn test_field20_to_swift_string() {
        let field = Field20 {
            reference: "PAYMENT123456".to_string(),
        };
        assert_eq!(field.to_swift_string(), ":20:PAYMENT123456");
    }
}
