use crate::errors::SwiftValidationError;
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

    /// Validation rules for the message (legacy method for backward compatibility)
    ///
    /// **Note**: This method returns a static JSON string for legacy validation systems.
    /// For actual validation, use `validate_network_rules()` which returns detailed errors.
    pub fn validate() -> &'static str {
        r#"{"rules": [{"id": "MT196_VALIDATION", "description": "Use validate_network_rules() for detailed validation", "condition": true}]}"#
    }

    /// Parse from generic SWIFT input (tries to detect blocks)
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        let block4 = extract_block4(input)?;
        Self::parse_from_block4(&block4)
    }

    /// Convert to SWIFT MT text format
    pub fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_field(&mut result, &self.field_21);
        append_field(&mut result, &self.field_76);
        append_optional_field(&mut result, &self.field_77a);
        append_optional_field(&mut result, &self.field_11);
        append_optional_field(&mut result, &self.field_79);

        result.push('-');
        result
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT196)
    // ========================================================================

    // ========================================================================
    // VALIDATION RULE C1
    // ========================================================================

    /// C1: Field 79 or Copy of Fields Requirement (Error code: C31)
    /// Either field 79 or a "Copy of at least the mandatory fields of the message to
    /// which the answer relates", but not both, may be present in the message.
    ///
    /// **Note**: This implementation currently validates the presence of field 79.
    /// The "Copy of fields" requirement cannot be validated at this level since
    /// copied fields are represented as additional optional fields in the message structure.
    /// Full validation of this rule requires message-level context that is not
    /// available in the current MT196 structure.
    fn validate_c1_field_79_or_copy(&self) -> Option<SwiftValidationError> {
        // Note: MT196 structure currently only has field 79 as defined optional field.
        // The "Copy of at least the mandatory fields of the original message" is
        // represented as additional optional fields that would need to be tracked
        // separately. This validation currently only checks for field 79 presence.
        //
        // In a full implementation, we would need to track whether any copied fields
        // from the original message are present, which would require extending the
        // MT196 structure to include a generic "copied_fields" collection.

        // Current implementation: no validation error since we cannot detect copied fields
        // This is a known limitation of the current structure
        None
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Field 79 or Copy of Fields Requirement
        if let Some(error) = self.validate_c1_field_79_or_copy() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        all_errors
    }
}

impl crate::traits::SwiftMessageBody for MT196 {
    fn message_type() -> &'static str {
        "196"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT196::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT196::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT196::validate_network_rules(self, stop_on_first_error)
    }
}
