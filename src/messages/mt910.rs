use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// **MT910: Confirmation of Credit**
///
/// Confirms credit to account servicing institution's account.
///
/// **Usage:** Credit confirmations, account reconciliation
/// **Category:** Category 9 (Cash Management & Customer Status)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT910 {
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

    /// Ordering Customer (Field 50)
    #[serde(flatten)]
    pub field_50: Option<Field50OrderingCustomerAFK>,

    /// Ordering Institution (Field 52)
    #[serde(flatten)]
    pub field_52: Option<Field52OrderingInstitution>,

    /// Intermediary (Field 56)
    #[serde(flatten)]
    pub field_56: Option<Field56Intermediary>,

    /// Sender to Receiver Information (Field 72)
    #[serde(rename = "72")]
    pub field_72: Option<Field72>,
}

impl MT910 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "910");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_25 = parser.parse_field::<Field25AccountIdentification>("25")?;

        // Parse optional field 13D (comes before 32A)
        let field_13d = parser.parse_optional_field::<Field13D>("13D")?;

        // Parse mandatory field 32A
        let field_32a = parser.parse_field::<Field32A>("32A")?;

        // Parse remaining optional fields
        let field_50 = parser.parse_optional_variant_field::<Field50OrderingCustomerAFK>("50")?;
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_56 = parser.parse_optional_variant_field::<Field56Intermediary>("56")?;
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        // Verify all content is consumed
        verify_parser_complete(&parser)?;

        Ok(Self {
            field_20,
            field_21,
            field_25,
            field_13d,
            field_32a,
            field_50,
            field_52,
            field_56,
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
        append_optional_field(&mut result, &self.field_50);
        append_optional_field(&mut result, &self.field_52);
        append_optional_field(&mut result, &self.field_56);
        append_optional_field(&mut result, &self.field_72);

        finalize_mt_string(result, false)
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT910)
    // ========================================================================

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Check if ordering customer (field 50a) is present
    fn has_ordering_customer(&self) -> bool {
        self.field_50.is_some()
    }

    /// Check if ordering institution (field 52a) is present
    fn has_ordering_institution(&self) -> bool {
        self.field_52.is_some()
    }

    // ========================================================================
    // VALIDATION RULES (C1)
    // ========================================================================

    /// C1: Ordering Customer or Ordering Institution Required (Error code: C06)
    /// Either field 50a or field 52a must be present
    fn validate_c1_ordering_party(&self) -> Option<SwiftValidationError> {
        let has_50 = self.has_ordering_customer();
        let has_52 = self.has_ordering_institution();

        if !has_50 && !has_52 {
            return Some(SwiftValidationError::business_error(
                "C06",
                "50a/52a",
                vec![],
                "Either field 50a (Ordering Customer) or field 52a (Ordering Institution) must be present",
                "At least one of fields 50a or 52a must be present in the message. If field 50a is present, field 52a is optional. If field 52a is present, field 50a is optional. Both fields cannot be absent",
            ));
        }

        None
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Ordering Customer or Ordering Institution
        if let Some(error) = self.validate_c1_ordering_party() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        all_errors
    }
}

impl crate::traits::SwiftMessageBody for MT910 {
    fn message_type() -> &'static str {
        "910"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT910::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT910::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT910::validate_network_rules(self, stop_on_first_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt910_parse() {
        let mt910_text = r#":20:20240719001
:21:REF20240719001
:25:12345678901234567890
:32A:240719USD1000,00
:50K:JOHN DOE
123 MAIN STREET
NEW YORK
-"#;
        let result = MT910::parse_from_block4(mt910_text);
        assert!(result.is_ok());
        let mt910 = result.unwrap();
        assert_eq!(mt910.field_20.reference, "20240719001");
        assert_eq!(mt910.field_21.reference, "REF20240719001");
    }

    #[test]
    fn test_mt910_validation_c1_fails_without_ordering_party() {
        // Test without field 50 and 52 - should fail validation
        let mt910_text = r#":20:20240719001
:21:REF20240719001
:25:12345678901234567890
:32A:240719USD1000,00
-"#;
        let result = MT910::parse_from_block4(mt910_text);
        assert!(result.is_ok());
        let mt910 = result.unwrap();

        // Validation should fail
        let errors = mt910.validate_network_rules(false);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code(), "C06");
        assert!(errors[0].message().contains("Either field 50a"));
    }

    #[test]
    fn test_mt910_validation_c1_passes_with_field_50() {
        // Test with field 50 only - should pass validation
        let mt910_text = r#":20:20240719001
:21:REF20240719001
:25:12345678901234567890
:32A:240719USD1000,00
:50K:JOHN DOE
123 MAIN STREET
NEW YORK
-"#;
        let result = MT910::parse_from_block4(mt910_text);
        assert!(result.is_ok());
        let mt910 = result.unwrap();

        // Validation should pass
        let errors = mt910.validate_network_rules(false);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_mt910_validation_c1_passes_with_field_52() {
        // Test with field 52 only - should pass validation
        let mt910_text = r#":20:20240719001
:21:REF20240719001
:25:12345678901234567890
:32A:240719USD1000,00
:52A:DEUTDEFFXXX
-"#;
        let result = MT910::parse_from_block4(mt910_text);
        if let Err(ref e) = result {
            eprintln!("Parse error: {:?}", e);
        }
        assert!(result.is_ok());
        let mt910 = result.unwrap();

        // Validation should pass
        let errors = mt910.validate_network_rules(false);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_mt910_validation_c1_passes_with_both_fields() {
        // Test with both fields 50 and 52 - should pass validation
        let mt910_text = r#":20:20240719001
:21:REF20240719001
:25:12345678901234567890
:32A:240719USD1000,00
:50K:JOHN DOE
123 MAIN STREET
NEW YORK
:52A:DEUTDEFFXXX
-"#;
        let result = MT910::parse_from_block4(mt910_text);
        assert!(result.is_ok());
        let mt910 = result.unwrap();

        // Validation should pass
        let errors = mt910.validate_network_rules(false);
        assert_eq!(errors.len(), 0);
    }
}
