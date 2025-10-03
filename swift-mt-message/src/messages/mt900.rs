use crate::fields::*;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};

// MT900: Confirmation of Debit
// Used to confirm that a debit entry has been posted to an account.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT900 {
    #[serde(rename = "20")]
    pub field_20: Field20,

    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    #[serde(rename = "25")]
    pub field_25: Field25AccountIdentification,

    #[serde(rename = "13D")]
    pub field_13d: Option<Field13D>,

    #[serde(rename = "32A")]
    pub field_32a: Field32A,

    #[serde(flatten)]
    pub field_52: Option<Field52OrderingInstitution>,

    #[serde(rename = "72")]
    pub field_72: Option<Field72>,
}

impl MT900 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "900");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_25 = parser.parse_field::<Field25AccountIdentification>("25")?;
        let field_32a = parser.parse_field::<Field32A>("32A")?;

        // Parse optional fields
        let field_13d = parser.parse_optional_field::<Field13D>("13D")?;
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        // Verify all content is consumed
        verify_parser_complete(&parser)?;

        Ok(Self {
            field_20,
            field_21,
            field_25,
            field_13d,
            field_32a,
            field_52,
            field_72,
        })
    }

    /// Validation rules for the message
    pub fn validate() -> &'static str {
        r#"{"rules": [{"id": "BASIC", "description": "Basic validation", "condition": true}]}"#
    }

    /// Parse from SWIFT MT text format
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        let block4 = extract_block4(input)?;
        Self::parse_from_block4(&block4)
    }

    /// Convert to SWIFT MT text format
    pub fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_field(&mut result, &self.field_21);
        append_field(&mut result, &self.field_25);
        append_optional_field(&mut result, &self.field_13d);
        append_field(&mut result, &self.field_32a);
        append_optional_field(&mut result, &self.field_52);
        append_optional_field(&mut result, &self.field_72);

        result.push('-');
        result
    }
}

impl crate::traits::SwiftMessageBody for MT900 {
    fn message_type() -> &'static str {
        "900"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT900::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT900::to_mt_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt900_parse() {
        let mt900_text = r#":20:20240719001
:21:REF20240719001
:25:12345678901234567890
:32A:240719USD1000,00
-"#;
        let result = MT900::parse_from_block4(mt900_text);
        assert!(result.is_ok());
        let mt900 = result.unwrap();
        assert_eq!(mt900.field_20.reference, "20240719001");
        assert_eq!(mt900.field_21.reference, "REF20240719001");
    }
}
