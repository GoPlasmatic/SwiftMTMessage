use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
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
        let mut parser = crate::parser::MessageParser::new(block4, "199");

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

    /// Parse from generic SWIFT input (tries to detect blocks)
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        let block4 = extract_block4(input)?;
        Self::parse_from_block4(&block4)
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MTn99)
    // ========================================================================

    /// Main validation method - validates all network rules
    ///
    /// According to SR 2025 MTn99 specification:
    /// "There are no network validated rules for this message type beyond the standard
    /// field-specific rules."
    ///
    /// Returns array of validation errors, respects stop_on_first_error flag.
    ///
    /// ## Parameters
    /// - `stop_on_first_error`: If true, returns immediately upon finding the first error
    ///
    /// ## Returns
    /// - Empty vector (MT199 has no network validation rules)
    pub fn validate_network_rules(&self, _stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // MT199 has no network validated rules according to SR 2025 specification
        // All validation is performed at the field level during parsing
        Vec::new()
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

impl crate::traits::SwiftMessageBody for MT199 {
    fn message_type() -> &'static str {
        "199"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT199::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_optional_field(&mut result, &self.field_21);
        append_field(&mut result, &self.field_79);

        finalize_mt_string(result, false)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT199::validate_network_rules(self, stop_on_first_error)
    }
}
