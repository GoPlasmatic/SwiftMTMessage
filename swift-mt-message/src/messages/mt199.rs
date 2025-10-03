use crate::fields::*;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};

// MT199: Free Format Message
// Used for free format communication between financial institutions regarding
// customer payments and related matters.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT199 {
    // Sender's Reference
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Related Reference (optional)
    #[serde(rename = "21", skip_serializing_if = "Option::is_none")]
    pub field_21: Option<Field21NoOption>,

    // Narrative (mandatory)
    #[serde(rename = "79")]
    pub field_79: Field79,
}

impl MT199 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "199");

        // Parse mandatory field 20
        let field_20 = parser.parse_field::<Field20>("20")?;

        // Parse optional field 21
        let field_21 = parser.parse_optional_field::<Field21NoOption>("21")?;

        // Parse mandatory field 79
        let field_79 = parser.parse_field::<Field79>("79")?;

        Ok(MT199 {
            field_20,
            field_21,
            field_79,
        })
    }

    /// Static validation rules for MT199
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "F20", "description": "Field 20 must not start or end with '/', and must not contain '//'"},
            {"id": "F21", "description": "Field 21 must not start or end with '/', and must not contain '//'"},
            {"id": "F79", "description": "Field 79 must contain meaningful communication content"},
            {"id": "REJT", "description": "If narrative starts with /REJT/, must follow Payments Guidelines"},
            {"id": "RETN", "description": "If narrative starts with /RETN/, must follow Payments Guidelines"}
        ]}"#
    }

    /// Validate the message instance according to MT199 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // Validate Field 20 - must not start/end with '/' or contain '//'
        let reference = &self.field_20.reference;
        if reference.starts_with('/') || reference.ends_with('/') || reference.contains("//") {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT199: Field 20 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Validate Field 21 if present - same rules as Field 20
        if let Some(ref field_21) = self.field_21 {
            let related_ref = &field_21.reference;
            if related_ref.starts_with('/')
                || related_ref.ends_with('/')
                || related_ref.contains("//")
            {
                return Err(crate::errors::ParseError::InvalidFormat {
                    message:
                        "MT199: Field 21 must not start or end with '/', and must not contain '//'"
                            .to_string(),
                });
            }
        }

        // Validate Field 79 has content
        if self.field_79.information.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT199: Field 79 must contain at least one line of narrative".to_string(),
            });
        }

        // Check for reject/return codes in narrative (informational only)
        if let Some(first_line) = self.field_79.information.first()
            && (first_line.starts_with("/REJT/") || first_line.starts_with("/RETN/"))
        {
            // Note: In production, additional validation for Payments Guidelines would be needed
            // For now, we just acknowledge these special cases exist
        }

        Ok(())
    }

    /// Check if this is a reject message
    pub fn is_reject_message(&self) -> bool {
        self.field_79
            .information
            .first()
            .map(|line| line.starts_with("/REJT/"))
            .unwrap_or(false)
    }

    /// Check if this is a return message
    pub fn is_return_message(&self) -> bool {
        self.field_79
            .information
            .first()
            .map(|line| line.starts_with("/RETN/"))
            .unwrap_or(false)
    }
}

// Implement the SwiftMessageBody trait for MT199
impl crate::traits::SwiftMessageBody for MT199 {
    fn message_type() -> &'static str {
        "199"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_optional_field(&mut result, &self.field_21);
        append_field(&mut result, &self.field_79);

        finalize_mt_string(result, false)
    }
}
