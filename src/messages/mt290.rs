use crate::errors::{ParseError, SwiftValidationError};
use crate::fields::*;
use crate::parser::MessageParser;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// **MT290: Advice of Charges, Interest and Other Adjustments**
///
/// Notification to beneficiary institution of charges and adjustments.
///
/// **Usage:** Interbank charge notifications, interest advice
/// **Category:** Category 2 (Financial Institution Transfers)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT290 {
    /// Transaction Reference Number (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Related Reference (Field 21)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Account Identification (Field 25)
    #[serde(rename = "25")]
    pub field_25: Field25NoOption,

    /// Value Date, Currency Code, Amount (Field 32)
    #[serde(flatten)]
    pub field_32: Field32AmountCD,

    /// Ordering Institution (Field 52)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    /// Details of Charges (Field 71B)
    #[serde(rename = "71B")]
    pub field_71b: Field71B,

    /// Sender to Receiver Information (Field 72)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,
}

impl MT290 {
    /// Parse MT290 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "290");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;

        // Parse Field 25 - Account Identification
        let field_25 = parser.parse_field::<Field25NoOption>("25")?;

        // Parse Field 32 - variant field (32C or 32D only per spec)
        let field_32 = parser.parse_variant_field::<Field32AmountCD>("32")?;

        // Parse optional Field 52 - Ordering Institution
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;

        // Parse mandatory Field 71B
        let field_71b = parser.parse_field::<Field71B>("71B")?;

        // Parse optional Field 72
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT290 {
            field_20,
            field_21,
            field_25,
            field_32,
            field_52,
            field_71b,
            field_72,
        })
    }

    /// Parse from generic SWIFT input (tries to detect blocks)
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let block4 = extract_block4(input)?;
        Self::parse_from_block4(&block4)
    }

    /// Convert to SWIFT MT text format
    pub fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_field(&mut result, &self.field_21);
        append_field(&mut result, &self.field_25);
        append_field(&mut result, &self.field_32);
        append_optional_field(&mut result, &self.field_52);
        append_field(&mut result, &self.field_71b);
        append_optional_field(&mut result, &self.field_72);

        result.push('-');
        result
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT290)
    // ========================================================================

    /// Main validation method - validates all network rules
    ///
    /// **Note**: MT290 has no network validated rules according to SR 2025 specification.
    /// This method is provided for API consistency and always returns an empty vector.
    ///
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, _stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // MT290 has no network validated rules per SR 2025 specification
        Vec::new()
    }
}

impl crate::traits::SwiftMessageBody for MT290 {
    fn message_type() -> &'static str {
        "290"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT290::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT290::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT290::validate_network_rules(self, stop_on_first_error)
    }
}
