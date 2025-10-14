use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// **MT190: Advice of Charges, Interest and Other Adjustments**
///
/// Notification of charges, interest, and adjustments debited or credited to account.
///
/// **Usage:** Charge notifications, interest advice
/// **Category:** Category 1 (Customer Payments & Cheques)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT190 {
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

impl MT190 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "190");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_25 = parser.parse_field::<Field25NoOption>("25")?;

        // Parse amount - can be 32C or 32D for credit/debit adjustments
        let field_32 = parser.parse_variant_field::<Field32AmountCD>("32")?;

        // Parse optional fields
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;

        // Parse mandatory field 71B
        let field_71b = parser.parse_field::<Field71B>("71B")?;

        // Parse optional field 72
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT190 {
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
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
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

        finalize_mt_string(result, false)
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT190)
    // ========================================================================

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    ///
    /// **Note**: According to SR 2025 specification, MT190 has NO network validated rules.
    /// This method always returns an empty vector.
    pub fn validate_network_rules(&self, _stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // SR 2025 MT190 specification states:
        // "There are no network validated rules for this message type."
        Vec::new()
    }
}

impl crate::traits::SwiftMessageBody for MT190 {
    fn message_type() -> &'static str {
        "190"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT190::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT190::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT190::validate_network_rules(self, stop_on_first_error)
    }
}
