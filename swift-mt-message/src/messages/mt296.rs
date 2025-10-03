use crate::errors::ParseError;
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MT296 - Answers
///
/// Used to respond to MT295 (Queries) or MT292 (Request for Cancellation)
/// or any message without a dedicated response type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT296 {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Field 21 - Related Reference (Mandatory)
    /// Reference of the original message being responded to
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Field 76 - Answers (Mandatory)
    #[serde(rename = "76")]
    pub field_76: Field76,

    /// Field 77A - Narrative (Optional)
    #[serde(rename = "77A", skip_serializing_if = "Option::is_none")]
    pub field_77a: Option<Field77A>,

    /// Field 11R - MT and Date of the Original Message - Received (Optional)
    #[serde(rename = "11R", skip_serializing_if = "Option::is_none")]
    pub field_11r: Option<Field11R>,

    /// Field 11S - MT and Date of the Original Message - Sent (Optional)
    #[serde(rename = "11S", skip_serializing_if = "Option::is_none")]
    pub field_11s: Option<Field11S>,

    /// Field 79 - Narrative Description of Original Message (Conditional)
    #[serde(rename = "79", skip_serializing_if = "Option::is_none")]
    pub field_79: Option<Field79>,

    /// Copy of mandatory fields from the original message (Conditional)
    /// Stored as additional fields that were part of the original message
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub original_fields: HashMap<String, serde_json::Value>,
}

impl MT296 {
    /// Parse MT296 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "296");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_76 = parser.parse_field::<Field76>("76")?;

        // Parse optional Field 77A
        let field_77a = parser.parse_optional_field::<Field77A>("77A")?;

        // Parse optional Field 11R or 11S
        let field_11r = parser.parse_optional_field::<Field11R>("11R")?;
        let field_11s = parser.parse_optional_field::<Field11S>("11S")?;

        // Parse optional/conditional Field 79
        let field_79 = parser.parse_optional_field::<Field79>("79")?;

        // Collect any remaining fields as original message fields
        // This would need to be implemented in MessageParser but for now use empty HashMap
        let original_fields = HashMap::new();

        // Validation: Only one of Field 79 or original fields should be present (C1)
        if field_79.is_some() && !original_fields.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "MT296: Only one of Field 79 or copy of original message fields should be present (C1)".to_string(),
            });
        }

        Ok(MT296 {
            field_20,
            field_21,
            field_76,
            field_77a,
            field_11r,
            field_11s,
            field_79,
            original_fields,
        })
    }

    /// Static validation rules for MT296
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }
}

impl crate::traits::SwiftMessageBody for MT296 {
    fn message_type() -> &'static str {
        "296"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_field(&mut result, &self.field_21);
        append_field(&mut result, &self.field_76);
        append_optional_field(&mut result, &self.field_77a);
        append_optional_field(&mut result, &self.field_11r);
        append_optional_field(&mut result, &self.field_11s);
        append_optional_field(&mut result, &self.field_79);

        // Note: original_fields are not serialized as they are dynamic
        // and would require special handling

        finalize_mt_string(result, false)
    }
}
