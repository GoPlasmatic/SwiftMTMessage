use crate::errors::ParseError;
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MT292 - Request for Cancellation
///
/// Used to request a cancellation of a previously sent SWIFT message.
/// Can be used for full or partial cancellation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT292 {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Field 21 - Related Reference (Mandatory)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Field 11S - MT and Date of the Original Message (Mandatory)
    #[serde(rename = "11S")]
    pub field_11s: Field11S,

    /// Field 79 - Narrative Description of Original Message (Conditional)
    /// Must be present if copy of original message fields is not included
    #[serde(rename = "79", skip_serializing_if = "Option::is_none")]
    pub field_79: Option<Field79>,

    /// Copy of mandatory fields from the original message (Conditional)
    /// Stored as additional fields that were part of the original message
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub original_fields: HashMap<String, serde_json::Value>,
}

impl MT292 {
    /// Parse MT292 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "292");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_11s = parser.parse_field::<Field11S>("11S")?;

        // Parse optional/conditional Field 79
        let field_79 = parser.parse_optional_field::<Field79>("79")?;

        // Collect any remaining fields as original message fields
        // This would need to be implemented in MessageParser but for now use empty HashMap
        let original_fields = HashMap::new();

        // Validation: Either Field 79 or original fields must be present
        if field_79.is_none() && original_fields.is_empty() {
            return Err(ParseError::InvalidFormat {
                message:
                    "MT292: Either Field 79 or copy of original message fields must be present"
                        .to_string(),
            });
        }

        Ok(MT292 {
            field_20,
            field_21,
            field_11s,
            field_79,
            original_fields,
        })
    }

    /// Static validation rules for MT292
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }
}

impl crate::traits::SwiftMessageBody for MT292 {
    fn message_type() -> &'static str {
        "292"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        // Add mandatory fields in the correct SWIFT order
        append_field(&mut result, &self.field_20);
        append_field(&mut result, &self.field_21);
        append_field(&mut result, &self.field_11s);

        // Add optional field 79
        append_optional_field(&mut result, &self.field_79);

        finalize_mt_string(result, false)
    }
}
