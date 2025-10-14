use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// **MT900: Confirmation of Debit**
///
/// Confirms debit to account servicing institution's account.
///
/// **Usage:** Debit confirmations, account reconciliation
/// **Category:** Category 9 (Cash Management & Customer Status)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT900 {
    /// Transaction Reference Number (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Related Reference (Field 21)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Account Identification (Field 25)
    #[serde(rename = "25")]
    pub field_25: Field25AccountIdentification,

    /// Date/Time Indication (Field 13D)
    #[serde(rename = "13D")]
    pub field_13d: Option<Field13D>,

    /// Value Date, Currency Code, Amount (Field 32A)
    #[serde(rename = "32A")]
    pub field_32a: Field32A,

    /// Ordering Institution (Field 52)
    #[serde(flatten)]
    pub field_52: Option<Field52OrderingInstitution>,

    /// Sender to Receiver Information (Field 72)
    #[serde(rename = "72")]
    pub field_72: Option<Field72>,
}

impl MT900 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "900");

        // Parse mandatory fields in order matching to_mt_string
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_25 = parser.parse_field::<Field25AccountIdentification>("25")?;

        // Parse optional Field 13D before Field 32A
        let field_13d = parser.parse_optional_field::<Field13D>("13D")?;

        // Parse mandatory Field 32A
        let field_32a = parser.parse_field::<Field32A>("32A")?;

        // Parse optional fields
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        // Verify all content is consumed
        verify_parser_complete(&parser)?;

        Ok(Self {
            field_20,
            field_21,
            field_25,
            field_13d,
            field_32a,
            field_52,
            field_72,
        })
    }

    /// Parse from SWIFT MT text format
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
        append_optional_field(&mut result, &self.field_13d);
        append_field(&mut result, &self.field_32a);
        append_optional_field(&mut result, &self.field_52);
        append_optional_field(&mut result, &self.field_72);

        finalize_mt_string(result, false)
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT900)
    // ========================================================================

    /// Main validation method - validates all network rules
    ///
    /// **Note**: According to SR 2025 specifications, MT900 has no network validated rules.
    /// This method always returns an empty vector.
    ///
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, _stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // MT900 has no network validated rules per SR 2025 specifications
        Vec::new()
    }
}

impl crate::traits::SwiftMessageBody for MT900 {
    fn message_type() -> &'static str {
        "900"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT900::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT900::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT900::validate_network_rules(self, stop_on_first_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt900_parse() {
        let mt900_text = r#":20:20240719001
:21:REF20240719001
:25:12345678901234567890
:32A:240719USD1000,00
-"#;
        let result = MT900::parse_from_block4(mt900_text);
        assert!(result.is_ok());
        let mt900 = result.unwrap();
        assert_eq!(mt900.field_20.reference, "20240719001");
        assert_eq!(mt900.field_21.reference, "REF20240719001");
    }

    #[test]
    fn test_mt900_network_validation() {
        let mt900_text = r#":20:20240719001
:21:REF20240719001
:25:12345678901234567890
:32A:240719USD1000,00
-"#;
        let mt900 = MT900::parse_from_block4(mt900_text).unwrap();

        // MT900 has no network validation rules, should always return empty vector
        let errors = mt900.validate_network_rules(false);
        assert!(errors.is_empty(), "MT900 should have no validation errors");

        // Test with stop_on_first_error=true as well
        let errors = mt900.validate_network_rules(true);
        assert!(
            errors.is_empty(),
            "MT900 should have no validation errors with stop_on_first_error"
        );
    }

    #[test]
    fn test_mt900_trait_validate_network_rules() {
        use crate::traits::SwiftMessageBody;

        let mt900_text = r#":20:20240719001
:21:REF20240719001
:25:12345678901234567890
:32A:240719USD1000,00
-"#;
        let mt900 = MT900::parse_from_block4(mt900_text).unwrap();

        // Test trait method implementation
        let errors = <MT900 as SwiftMessageBody>::validate_network_rules(&mt900, false);
        assert!(
            errors.is_empty(),
            "Trait implementation should return empty vector"
        );
    }
}
