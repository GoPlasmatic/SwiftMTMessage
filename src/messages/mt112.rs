use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// **MT112: Status of a Request for Stop Payment of a Cheque**
///
/// Response from drawee bank to drawer bank confirming actions taken on MT111 stop payment request.
/// Provides status information about the stop payment instruction.
///
/// **Usage:** Stop payment status notifications
/// **Category:** Category 1 (Customer Payments)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MT112 {
    /// Transaction reference (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Cheque number (Field 21)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Date of issue (Field 30)
    #[serde(rename = "30")]
    pub field_30: Field30,

    /// Amount (Field 32)
    #[serde(flatten)]
    pub field_32: Field32AB,

    /// Drawer bank (Field 52)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    /// Payee (Field 59)
    #[serde(rename = "59", skip_serializing_if = "Option::is_none")]
    pub field_59: Option<Field59NoOption>,

    /// Answers (Field 76)
    #[serde(rename = "76")]
    pub field_76: Field76,
}

impl MT112 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "112");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_30 = parser.parse_field::<Field30>("30")?;

        // Parse amount - can be 32A or 32B per spec
        let field_32 = parser.parse_variant_field::<Field32AB>("32")?;

        // Parse optional fields
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_59 = parser.parse_optional_field::<Field59NoOption>("59")?;

        // Parse mandatory field 76
        let field_76 = parser.parse_field::<Field76>("76")?;

        Ok(MT112 {
            field_20,
            field_21,
            field_30,
            field_32,
            field_52,
            field_59,
            field_76,
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
        append_field(&mut result, &self.field_30);
        append_field(&mut result, &self.field_32);
        append_optional_field(&mut result, &self.field_52);
        append_optional_field(&mut result, &self.field_59);
        append_field(&mut result, &self.field_76);

        finalize_mt_string(result, false)
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT112)
    // ========================================================================

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    ///
    /// **Note**: According to SR 2025 specifications, MT112 has no network validated rules.
    /// This method is provided for consistency with other message types and future extensibility.
    pub fn validate_network_rules(&self, _stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // MT112 has no network validated rules according to SR 2025
        // All validation is handled at the field level
        Vec::new()
    }
}

impl crate::traits::SwiftMessageBody for MT112 {
    fn message_type() -> &'static str {
        "112"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT112::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT112::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT112::validate_network_rules(self, stop_on_first_error)
    }
}
