use crate::errors::{ParseError, SwiftValidationError};
use crate::fields::*;
use crate::parser::MessageParser;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **MT292: Request for Cancellation**
///
/// Request to cancel previously sent message.
///
/// **Usage:** Payment cancellation requests
/// **Category:** Category 2 (Financial Institution Transfers)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MT292 {
    /// Transaction Reference Number (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Related Reference (Field 21)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// MT and Date of the Original Message (Field 11S)
    #[serde(rename = "11S")]
    pub field_11s: Field11S,

    /// Narrative Description of Original Message (Field 79)
    #[serde(rename = "79", skip_serializing_if = "Option::is_none")]
    pub field_79: Option<Field79>,

    /// Copy of mandatory fields from original message
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

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT292)
    // ========================================================================

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Check if field 79 is present
    fn has_field_79(&self) -> bool {
        self.field_79.is_some()
    }

    /// Check if copy of original message fields is present
    fn has_original_fields(&self) -> bool {
        !self.original_fields.is_empty()
    }

    // ========================================================================
    // VALIDATION RULES (C1)
    // ========================================================================

    /// C1: Field 79 or Copy of Mandatory Fields Requirement (Error code: C25)
    /// Field 79 or a copy of at least the mandatory fields of the original message
    /// or both must be present
    fn validate_c1_field_79_or_original_fields(&self) -> Option<SwiftValidationError> {
        let has_79 = self.has_field_79();
        let has_original = self.has_original_fields();

        if !has_79 && !has_original {
            return Some(SwiftValidationError::content_error(
                "C25",
                "79",
                "",
                "Field 79 (Narrative Description) or a copy of at least the mandatory fields of the original message must be present",
                "Either field 79 or a copy of at least the mandatory fields of the original message or both must be present",
            ));
        }

        None
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Field 79 or Copy of Mandatory Fields Requirement
        if let Some(error) = self.validate_c1_field_79_or_original_fields() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        all_errors
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

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT292::validate_network_rules(self, stop_on_first_error)
    }
}
