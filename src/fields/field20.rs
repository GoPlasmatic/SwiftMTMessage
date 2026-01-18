use super::swift_utils::{parse_max_length, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 20: Sender's Reference**
///
/// Transaction reference assigned by sender to identify the message.
/// Must be unique for audit and payment tracking.
///
/// **Format:** `16x` (max 16 alphanumeric chars)
/// **Constraints:** Cannot start/end with `/` or contain `//`
///
/// **Example:**
/// ```text
/// :20:PAYMENT123456
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field20 {
    /// Sender's reference (max 16 chars, no leading/trailing slashes)
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
