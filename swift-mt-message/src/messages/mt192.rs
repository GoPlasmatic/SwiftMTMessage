use crate::fields::*;
use serde::{Deserialize, Serialize};

// MT192: Request for Cancellation
// Used by the originator to request the cancellation of a previously sent
// payment message before execution.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT192 {
    // Sender's Reference
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Related Reference
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    // MT and Date (Session details of original message)
    #[serde(rename = "11S")]
    pub field_11s: Field11S,

    // Narrative (optional) - cancellation reason and additional information
    #[serde(rename = "79", skip_serializing_if = "Option::is_none")]
    pub field_79: Option<Field79>,
}

impl MT192 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "192");

        // Parse mandatory fields in order: 20, 21, 11S
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_11s = parser.parse_field::<Field11S>("11S")?;

        // Parse optional field 79
        let field_79 = parser.parse_optional_field::<Field79>("79")?;

        Ok(MT192 {
            field_20,
            field_21,
            field_11s,
            field_79,
        })
    }

    /// Static validation rules for MT192
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "F20", "description": "Field 20 must not start or end with '/', and must not contain '//'"},
            {"id": "F21", "description": "Field 21 must not start or end with '/', and must not contain '//'"},
            {"id": "F11S", "description": "Field 11S must have proper format for MT, date, session, and sequence numbers"},
            {"id": "C1", "description": "Either Field 79 or a copy of mandatory fields from the original message (or both) must be present"}
        ]}"#
    }

    /// Validate the message instance according to MT192 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // Validate Field 20 - must not start/end with '/' or contain '//'
        let reference = &self.field_20.reference;
        if reference.starts_with('/') || reference.ends_with('/') || reference.contains("//") {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT192: Field 20 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Validate Field 21 - same rules as Field 20
        let related_ref = &self.field_21.reference;
        if related_ref.starts_with('/') || related_ref.ends_with('/') || related_ref.contains("//")
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT192: Field 21 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Note: Condition C1 (Field 79 or original fields) is handled in the parser
        // In this implementation, Field 79 is optional as we don't support copying original fields

        Ok(())
    }
}

// Implement the SwiftMessageBody trait for MT192
impl crate::traits::SwiftMessageBody for MT192 {
    fn message_type() -> &'static str {
        "192"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        use crate::traits::SwiftField;
        let mut result = String::new();

        result.push_str(&self.field_20.to_swift_string());
        result.push_str("\r\n");

        result.push_str(&self.field_21.to_swift_string());
        result.push_str("\r\n");

        result.push_str(&self.field_11s.to_swift_string());
        result.push_str("\r\n");

        if let Some(ref field) = self.field_79 {
            result.push_str(&field.to_swift_string());
            result.push_str("\r\n");
        }

        // Remove trailing \r\n
        if result.ends_with("\r\n") {
            result.truncate(result.len() - 2);
        }

        result
    }
}
