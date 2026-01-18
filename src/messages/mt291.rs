use crate::errors::{ParseError, SwiftValidationError};
use crate::fields::*;
use crate::parser::MessageParser;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// **MT291: Request for Payment of Charges, Interest and Other Expenses**
///
/// Request payment of charges, interest, and expenses.
///
/// **Usage:** Interbank charge payment requests, expense reimbursement
/// **Category:** Category 2 (Financial Institution Transfers)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MT291 {
    /// Transaction Reference Number (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Related Reference (Field 21)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Currency Code, Amount (Field 32B)
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    /// Ordering Institution (Field 52)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    /// Account With Institution (Field 57)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_57: Option<Field57AccountWithABD>,

    /// Details of Charges (Field 71B)
    #[serde(rename = "71B")]
    pub field_71b: Field71B,

    /// Sender to Receiver Information (Field 72)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,
}

impl MT291 {
    /// Parse MT291 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "291");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_32b = parser.parse_field::<Field32B>("32B")?;

        // Parse optional Field 52 - Ordering Institution
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;

        // Parse optional Field 57 - Account With Institution (A, B, D only per spec)
        let field_57 = parser.parse_optional_variant_field::<Field57AccountWithABD>("57")?;

        // Parse mandatory Field 71B
        let field_71b = parser.parse_field::<Field71B>("71B")?;

        // Parse optional Field 72
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT291 {
            field_20,
            field_21,
            field_32b,
            field_52,
            field_57,
            field_71b,
            field_72,
        })
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
        append_field(&mut result, &self.field_32b);
        append_optional_field(&mut result, &self.field_52);
        append_optional_field(&mut result, &self.field_57);
        append_field(&mut result, &self.field_71b);
        append_optional_field(&mut result, &self.field_72);

        finalize_mt_string(result, false)
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MTn91)
    // ========================================================================

    /// Main validation method - validates all network rules
    ///
    /// **Note**: According to SR 2025 MTn91 specification, there are no network
    /// validated rules for this message type. This method is provided for API
    /// consistency and returns an empty vector.
    ///
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, _stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Per SR 2025 MTn91 specification:
        // "There are no network validated rules for this message type."
        Vec::new()
    }
}

impl crate::traits::SwiftMessageBody for MT291 {
    fn message_type() -> &'static str {
        "291"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT291::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT291::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT291::validate_network_rules(self, stop_on_first_error)
    }
}
