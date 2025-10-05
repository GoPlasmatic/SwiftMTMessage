use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// **MT191: Request for Payment of Charges, Interest and Other Expenses**
///
/// Request for payment of charges, interest, and expenses from another financial institution.
///
/// **Usage:** Charge reimbursement requests, expense recovery
/// **Category:** Category 1 (Customer Payments & Cheques)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT191 {
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
    pub field_57: Option<Field57AccountWithInstitution>,

    /// Details of Charges (Field 71B)
    #[serde(rename = "71B")]
    pub field_71b: Field71B,

    /// Sender to Receiver Information (Field 72)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,
}

impl MT191 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "191");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_32b = parser.parse_field::<Field32B>("32B")?;

        // Parse optional fields
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_57 =
            parser.parse_optional_variant_field::<Field57AccountWithInstitution>("57")?;

        // Parse mandatory field 71B
        let field_71b = parser.parse_field::<Field71B>("71B")?;

        // Parse optional field 72
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT191 {
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

        result.push('-');
        result
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT191)
    // ========================================================================
    //
    // Per SR 2025 MTn91 specification (line 35):
    // "There are no network validated rules for this message type."
    //
    // This implementation follows the MT101 pattern for consistency,
    // but returns an empty vector as there are no validation rules.
    // ========================================================================

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    ///
    /// **Note**: MT191 has no network validated rules per SR 2025 specification.
    /// This method is provided for consistency with other message types and
    /// returns an empty vector.
    pub fn validate_network_rules(&self, _stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // No network validated rules for MT191
        Vec::new()
    }
}

impl crate::traits::SwiftMessageBody for MT191 {
    fn message_type() -> &'static str {
        "191"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT191::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT191::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT191::validate_network_rules(self, stop_on_first_error)
    }
}
