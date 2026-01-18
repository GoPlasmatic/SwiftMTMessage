//! **Field 12: Sub Message Type**
//!
//! Provides additional categorization within main MT message types for routing and processing.

use super::swift_utils::{parse_exact_length, parse_numeric};
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 12: Sub Message Type**
///
/// Specifies sub-message type code for additional categorization within main MT message types.
///
/// **Format:** `3!n` (exactly 3 numeric digits)
///
/// **Example:**
/// ```text
/// :12:103
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field12 {
    /// Sub-message type code (3 digits, numeric only)
    pub type_code: String,
}

impl SwiftField for Field12 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Must be exactly 3 numeric digits
        let type_code = parse_exact_length(input, 3, "Field 12 type code")?;
        parse_numeric(&type_code, "Field 12 type code")?;

        Ok(Field12 { type_code })
    }

    fn to_swift_string(&self) -> String {
        format!(":12:{}", self.type_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field12_valid() {
        let field = Field12::parse("103").unwrap();
        assert_eq!(field.type_code, "103");
        assert_eq!(field.to_swift_string(), ":12:103");

        let field = Field12::parse("001").unwrap();
        assert_eq!(field.type_code, "001");
        assert_eq!(field.to_swift_string(), ":12:001");

        let field = Field12::parse("950").unwrap();
        assert_eq!(field.type_code, "950");
        assert_eq!(field.to_swift_string(), ":12:950");
    }

    #[test]
    fn test_field12_invalid() {
        // Too short
        assert!(Field12::parse("12").is_err());

        // Too long
        assert!(Field12::parse("1234").is_err());

        // Non-numeric
        assert!(Field12::parse("ABC").is_err());
        assert!(Field12::parse("12A").is_err());
    }
}
