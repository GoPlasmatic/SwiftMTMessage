use super::swift_utils::{parse_alphanumeric, parse_exact_length};
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 26T: Transaction Type Code**
///
/// Specifies transaction type using standardized 3-character codes for categorization and processing.
///
/// **Format:** `3!c` (exactly 3 alphanumeric chars)
/// **Common Codes:** PAY, SAL, FXD, DIV, TRD, INT
///
/// **Example:**
/// ```text
/// :26T:PAY
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field26T {
    /// Transaction type code (3 chars, alphanumeric)
    pub type_code: String,
}

impl SwiftField for Field26T {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Must be exactly 3 characters
        let type_code = parse_exact_length(input, 3, "Field 26T type code")?;

        // Must be alphanumeric
        parse_alphanumeric(&type_code, "Field 26T type code")?;

        Ok(Field26T { type_code })
    }

    fn to_swift_string(&self) -> String {
        format!(":26T:{}", self.type_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field26t_valid() {
        let field = Field26T::parse("PAY").unwrap();
        assert_eq!(field.type_code, "PAY");
        assert_eq!(field.to_swift_string(), ":26T:PAY");

        let field = Field26T::parse("FXD").unwrap();
        assert_eq!(field.type_code, "FXD");

        let field = Field26T::parse("123").unwrap();
        assert_eq!(field.type_code, "123");

        let field = Field26T::parse("A1B").unwrap();
        assert_eq!(field.type_code, "A1B");
    }

    #[test]
    fn test_field26t_invalid() {
        // Too short
        assert!(Field26T::parse("PA").is_err());

        // Too long
        assert!(Field26T::parse("PAYM").is_err());

        // Non-alphanumeric characters
        assert!(Field26T::parse("PA-").is_err());
        assert!(Field26T::parse("P@Y").is_err());
    }
}
