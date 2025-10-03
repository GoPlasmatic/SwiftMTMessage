use crate::fields::*;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};

// MT196: Answers
// Used to provide comprehensive answers and responses to various queries and
// requests related to customer payments and transactions.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT196 {
    // Sender's Reference
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Related Reference
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    // Answers (mandatory)
    #[serde(rename = "76")]
    pub field_76: Field76,

    // Proprietary Message (optional)
    #[serde(rename = "77A", skip_serializing_if = "Option::is_none")]
    pub field_77a: Option<Field77A>,

    // Message Type and Date (optional)
    #[serde(rename = "11", skip_serializing_if = "Option::is_none")]
    pub field_11: Option<Field11>,

    // Narrative (optional)
    #[serde(rename = "79", skip_serializing_if = "Option::is_none")]
    pub field_79: Option<Field79>,
}

impl MT196 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "196");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_76 = parser.parse_field::<Field76>("76")?;

        // Parse optional fields
        let field_77a = parser.parse_optional_field::<Field77A>("77A")?;
        let field_11 = parser.parse_optional_field::<Field11>("11")?;
        let field_79 = parser.parse_optional_field::<Field79>("79")?;

        Ok(MT196 {
            field_20,
            field_21,
            field_76,
            field_77a,
            field_11,
            field_79,
        })
    }

    /// Static validation rules for MT196
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "F20", "description": "Field 20 must not start or end with '/', and must not contain '//'"},
            {"id": "F21", "description": "Field 21 must not start or end with '/', and must not contain '//'"},
            {"id": "F76", "description": "Field 76 must contain at least one line of answer information"},
            {"id": "C1", "description": "Only one of the following may be present: Field 79, or a copy of mandatory fields of the original message"}
        ]}"#
    }

    /// Validate the message instance according to MT196 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // Validate Field 20 - must not start/end with '/' or contain '//'
        let reference = &self.field_20.reference;
        if reference.starts_with('/') || reference.ends_with('/') || reference.contains("//") {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT196: Field 20 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Validate Field 21 - same rules as Field 20
        let related_ref = &self.field_21.reference;
        if related_ref.starts_with('/') || related_ref.ends_with('/') || related_ref.contains("//")
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT196: Field 21 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Validate Field 76 has content
        if self.field_76.information.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT196: Field 76 must contain at least one line of answer information"
                    .to_string(),
            });
        }

        Ok(())
    }
}

// Implement the SwiftMessageBody trait for MT196
impl crate::traits::SwiftMessageBody for MT196 {
    fn message_type() -> &'static str {
        "196"
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
        append_optional_field(&mut result, &self.field_11);
        append_optional_field(&mut result, &self.field_79);

        finalize_mt_string(result, false)
    }
}
