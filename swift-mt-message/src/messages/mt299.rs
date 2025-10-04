use crate::errors::{ParseError, SwiftValidationError};
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};

/// MT299 - Free Format Message
///
/// Generic message format used to exchange information for which
/// no specific message type exists.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT299 {
    /// Field 20 - Transaction Reference (Mandatory)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Field 21 - Related Reference (Optional)
    #[serde(rename = "21", skip_serializing_if = "Option::is_none")]
    pub field_21: Option<Field21NoOption>,

    /// Field 79 - Narrative (Mandatory)
    #[serde(rename = "79")]
    pub field_79: Field79,
}

impl MT299 {
    /// Parse MT299 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "299");

        // Parse mandatory Field 20
        let field_20 = parser.parse_field::<Field20>("20")?;

        // Parse optional Field 21
        let field_21 = parser.parse_optional_field::<Field21NoOption>("21")?;

        // Parse mandatory Field 79
        let field_79 = parser.parse_field::<Field79>("79")?;

        Ok(MT299 {
            field_20,
            field_21,
            field_79,
        })
    }

    /// Validation rules for the message (legacy method for backward compatibility)
    ///
    /// **Note**: This method returns a static JSON string for legacy validation systems.
    /// For actual validation, use `validate_network_rules()` which returns detailed errors.
    /// According to SR 2025 specifications, MT n99 messages have no network validated
    /// rules beyond standard field-specific rules.
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MTn99)
    // ========================================================================

    /// Main validation method - validates all network rules
    ///
    /// **Note**: According to SR 2025 specifications, MT n99 messages have no
    /// network validated rules beyond standard field-specific rules, which are
    /// already enforced during parsing. This method always returns an empty vector.
    ///
    /// Returns empty vector as there are no network validation rules for MT299
    pub fn validate_network_rules(&self, _stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        Vec::new()
    }
}

impl crate::traits::SwiftMessageBody for MT299 {
    fn message_type() -> &'static str {
        "299"
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

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT299::validate_network_rules(self, stop_on_first_error)
    }
}
