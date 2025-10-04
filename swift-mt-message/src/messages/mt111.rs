use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};

// MT111: Request for Stop Payment of a Cheque
// Sent by the drawer bank (or its agent) to the drawee bank to request
// stop payment of a cheque.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT111 {
    // Sender's Reference
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Cheque Number
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    // Date of Issue
    #[serde(rename = "30")]
    pub field_30: Field30,

    // Amount (can be 32A or 32B per SWIFT spec)
    #[serde(flatten)]
    pub field_32: Field32AB,

    // Drawer Bank (optional) - can be A, B, or D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    // Payee (optional) - name and address only
    #[serde(rename = "59", skip_serializing_if = "Option::is_none")]
    pub field_59: Option<Field59NoOption>,

    // Queries (optional)
    #[serde(rename = "75", skip_serializing_if = "Option::is_none")]
    pub field_75: Option<Field75>,
}

impl MT111 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "111");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_30 = parser.parse_field::<Field30>("30")?;

        // Parse amount - can be 32A or 32B per spec
        let field_32 = parser.parse_variant_field::<Field32AB>("32")?;

        // Parse optional fields
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_59 = parser.parse_optional_field::<Field59NoOption>("59")?;
        let field_75 = parser.parse_optional_field::<Field75>("75")?;

        Ok(MT111 {
            field_20,
            field_21,
            field_30,
            field_32,
            field_52,
            field_59,
            field_75,
        })
    }

    /// Validation rules for the message (legacy method for backward compatibility)
    ///
    /// **Note**: This method returns a static JSON string for legacy validation systems.
    /// For actual validation, use `validate_network_rules()` which returns detailed errors.
    pub fn validate() -> &'static str {
        r#"{"rules": [{"id": "MT111_VALIDATION", "description": "Use validate_network_rules() for detailed validation", "condition": true}]}"#
    }

    /// Parse from generic SWIFT input (tries to detect blocks)
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        let block4 = extract_block4(input)?;
        Self::parse_from_block4(&block4)
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT111)
    // ========================================================================

    // NOTE: MT111 has no message-level network validation rules (C-series, D-series, E-series, T-series)
    // per SR 2025 specification. All validation is performed at the field level:
    // - Field 20, 21: Reference format validation (implemented in field types)
    // - Field 30: Date validation (implemented in field types)
    // - Field 32a: Currency and amount validation (implemented in field types)
    // - Field 52a: BIC validation (implemented in field types)
    // - Field 59: Account must not be used (enforced by Field59NoOption type)
    // - Field 75: Narrative format (implemented in field types)

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, _stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // No message-level network validation rules for MT111
        Vec::new()
    }
}

impl crate::traits::SwiftMessageBody for MT111 {
    fn message_type() -> &'static str {
        "111"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT111::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_field(&mut result, &self.field_21);
        append_field(&mut result, &self.field_30);
        append_field(&mut result, &self.field_32);
        append_optional_field(&mut result, &self.field_52);
        append_optional_field(&mut result, &self.field_59);
        append_optional_field(&mut result, &self.field_75);

        finalize_mt_string(result, false)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT111::validate_network_rules(self, stop_on_first_error)
    }
}
