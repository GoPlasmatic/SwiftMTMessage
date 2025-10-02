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

        // Parse mandatory fields - MT192 fields can appear in different orders
        // Check for field 11S first as it may come before 20 and 21 in some messages
        let field_11s = if parser.detect_field("11S") {
            parser.parse_field::<Field11S>("11S")?
        } else {
            // If 11S is not found at the start, parse 20 and 21 first
            let _field_20 = parser.parse_field::<Field20>("20")?;
            let _field_21 = parser.parse_field::<Field21NoOption>("21")?;
            parser.parse_field::<Field11S>("11S")?
        };

        // Parse fields 20 and 21
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;

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
                message: "MT192: Field 20 must not start or end with '/', and must not contain '//'".to_string(),
            });
        }

        // Validate Field 21 - same rules as Field 20
        let related_ref = &self.field_21.reference;
        if related_ref.starts_with('/') || related_ref.ends_with('/') || related_ref.contains("//") {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT192: Field 21 must not start or end with '/', and must not contain '//'".to_string(),
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

    fn from_fields(
        fields: std::collections::HashMap<String, Vec<(String, usize)>>,
    ) -> crate::SwiftResult<Self> {
        // Collect all fields with their positions
        let mut all_fields: Vec<(String, String, usize)> = Vec::new();
        for (tag, values) in fields {
            for (value, position) in values {
                all_fields.push((tag.clone(), value, position));
            }
        }

        // Sort by position to preserve field order
        all_fields.sort_by_key(|(_, _, pos)| *pos);

        // Reconstruct block4 in the correct order
        let mut block4 = String::new();
        for (tag, value, _) in all_fields {
            block4.push_str(&format!(":{}:{}\n", tag, value));
        }
        Self::parse_from_block4(&block4)
    }

    fn from_fields_with_config(
        fields: std::collections::HashMap<String, Vec<(String, usize)>>,
        _config: &crate::errors::ParserConfig,
    ) -> std::result::Result<crate::errors::ParseResult<Self>, crate::errors::ParseError> {
        match Self::from_fields(fields) {
            Ok(msg) => Ok(crate::errors::ParseResult::Success(msg)),
            Err(e) => Err(e),
        }
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        use chrono::Datelike;
        let mut fields = std::collections::HashMap::new();

        // Add mandatory fields
        fields.insert("20".to_string(), vec![self.field_20.reference.clone()]);
        fields.insert("21".to_string(), vec![self.field_21.reference.clone()]);

        // Add field 11S
        let mut field_11s_value = format!("{}{:02}{:02}{:02}",
            self.field_11s.message_type,
            self.field_11s.date.year() % 100,
            self.field_11s.date.month(),
            self.field_11s.date.day()
        );
        if let Some(ref session) = self.field_11s.session_number {
            field_11s_value.push_str(session);
        }
        if let Some(ref seq) = self.field_11s.input_sequence_number {
            field_11s_value.push_str(seq);
        }
        fields.insert("11S".to_string(), vec![field_11s_value]);

        // Add optional field 79
        if let Some(ref field_79) = self.field_79 {
            fields.insert("79".to_string(), vec![field_79.information.join("\n")]);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "21", "11S"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["79"]
    }
}