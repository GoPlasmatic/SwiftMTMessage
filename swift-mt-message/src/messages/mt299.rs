use crate::errors::ParseError;
use crate::fields::*;
use crate::message_parser::MessageParser;
use serde::{Deserialize, Serialize};

/// MT299 - Free Format Message
///
/// Generic message format used to exchange information for which
/// no specific message type exists.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT299 {
    /// Field 20 - Transaction Reference (Mandatory)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Field 21 - Related Reference (Optional)
    #[serde(rename = "21", skip_serializing_if = "Option::is_none")]
    pub field_21: Option<Field21NoOption>,

    /// Field 79 - Narrative (Mandatory)
    #[serde(rename = "79")]
    pub field_79: Field79,
}

impl MT299 {
    /// Parse MT299 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "299");

        // Parse mandatory Field 20
        let field_20 = parser.parse_field::<Field20>("20")?;

        // Parse optional Field 21
        let field_21 = parser.parse_optional_field::<Field21NoOption>("21")?;

        // Parse mandatory Field 79
        let field_79 = parser.parse_field::<Field79>("79")?;

        Ok(MT299 {
            field_20,
            field_21,
            field_79,
        })
    }

    /// Static validation rules for MT299
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }
}

impl crate::traits::SwiftMessageBody for MT299 {
    fn message_type() -> &'static str {
        "299"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        use crate::traits::SwiftField;
        let mut result = String::new();

        result.push_str(&self.field_20.to_swift_string());
        result.push_str("\r\n");

        if let Some(ref field) = self.field_21 {
            result.push_str(&field.to_swift_string());
            result.push_str("\r\n");
        }

        result.push_str(&self.field_79.to_swift_string());
        result.push_str("\r\n");

        // Remove trailing \r\n
        if result.ends_with("\r\n") {
            result.truncate(result.len() - 2);
        }

        result
    }
}
