use crate::errors::SwiftValidationError;
use crate::fields::Field52DrawerBank;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// Cheque details (repeating sequence)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT110Cheque {
    /// Cheque number (Field 21)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Date of issue (Field 30)
    #[serde(rename = "30")]
    pub field_30: Field30,

    /// Amount (Field 32)
    #[serde(flatten)]
    pub field_32: Field32AB,

    /// Payer (Field 50)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_50: Option<Field50OrderingCustomerAFK>,

    /// Drawer bank (Field 52)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52DrawerBank>,

    /// Payee (Field 59)
    #[serde(flatten)]
    pub field_59: Field59,
}

/// **MT110: Advice of Cheque(s)**
///
/// Advice from drawer bank to drawee bank confirming issuance of one or more cheques.
/// Supports multiple cheque details in a single message.
///
/// **Usage:** Cheque issuance advice, payment notifications
/// **Category:** Category 1 (Customer Payments)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT110 {
    /// Sender's reference (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Sender's correspondent (Field 53)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_53a: Option<Field53SenderCorrespondent>,

    /// Receiver's correspondent (Field 54)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_54a: Option<Field54ReceiverCorrespondent>,

    /// Sender to receiver information (Field 72)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,

    /// Cheque details (max 10)
    #[serde(rename = "#", default)]
    pub cheques: Vec<MT110Cheque>,
}

impl MT110 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "110");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;

        // Parse optional fields before cheque details
        let field_53a = parser.parse_optional_variant_field::<Field53SenderCorrespondent>("53")?;
        let field_54a =
            parser.parse_optional_variant_field::<Field54ReceiverCorrespondent>("54")?;
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        // Parse cheque details - enable duplicates for repeating fields
        let mut cheques = Vec::new();
        parser = parser.with_duplicates(true);

        // Parse each cheque detail - they start with field 21
        while parser.detect_field("21") {
            let field_21 = parser.parse_field::<Field21NoOption>("21")?;
            let field_30 = parser.parse_field::<Field30>("30")?;

            // Parse field 32 (amount) - only A or B per spec
            let field_32 = parser.parse_variant_field::<Field32AB>("32")?;

            // Parse optional fields
            let field_50 =
                parser.parse_optional_variant_field::<Field50OrderingCustomerAFK>("50")?;
            let field_52 = parser.parse_optional_variant_field::<Field52DrawerBank>("52")?;

            // Parse field 59 (payee)
            let field_59 = parser.parse_variant_field::<Field59>("59")?;

            cheques.push(MT110Cheque {
                field_21,
                field_30,
                field_32,
                field_50,
                field_52,
                field_59,
            });
        }

        // Validate we have at least one cheque detail
        if cheques.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT110: At least one cheque detail is required".to_string(),
            });
        }

        // Note: Max 10 repetitions (NVR C1) and currency consistency (NVR C2)
        // are validated in validate_network_rules(), not during parsing

        Ok(MT110 {
            field_20,
            field_53a,
            field_54a,
            field_72,
            cheques,
        })
    }

    /// Parse from generic SWIFT input (tries to detect blocks)
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        let block4 = extract_block4(input)?;
        Self::parse_from_block4(&block4)
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT110)
    // ========================================================================

    /// C1: Maximum 10 repetitive sequences (Error code: T10)
    /// The repetitive sequence must not be present more than ten times
    fn validate_c1_max_repetitions(&self) -> Option<SwiftValidationError> {
        if self.cheques.len() > 10 {
            return Some(SwiftValidationError::content_error(
                "T10",
                "21-59a",
                "",
                &format!(
                    "The repetitive sequence (cheque details) appears {} times, but maximum 10 occurrences are allowed",
                    self.cheques.len()
                ),
                "The repetitive sequence containing fields 21, 30, 32a, 50a, 52a, and 59a must not be present more than ten times in the message",
            ));
        }

        None
    }

    /// C2: Currency Code Consistency (Error code: C02)
    /// The currency code in field 32a must be the same for all occurrences
    fn validate_c2_currency_consistency(&self) -> Option<SwiftValidationError> {
        if self.cheques.is_empty() {
            return None;
        }

        // Get currency from first cheque
        let first_currency = match &self.cheques[0].field_32 {
            Field32AB::A(amt) => &amt.currency,
            Field32AB::B(amt) => &amt.currency,
        };

        // Check all subsequent cheques have the same currency
        for (idx, cheque) in self.cheques.iter().enumerate().skip(1) {
            let cheque_currency = match &cheque.field_32 {
                Field32AB::A(amt) => &amt.currency,
                Field32AB::B(amt) => &amt.currency,
            };

            if cheque_currency != first_currency {
                return Some(SwiftValidationError::content_error(
                    "C02",
                    "32a",
                    cheque_currency,
                    &format!(
                        "Cheque {}: Currency code in field 32a ({}) must be the same as in other occurrences ({})",
                        idx + 1,
                        cheque_currency,
                        first_currency
                    ),
                    "The currency code in the amount field 32a must be the same for all occurrences of this field in the message",
                ));
            }
        }

        None
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Maximum 10 repetitions
        if let Some(error) = self.validate_c1_max_repetitions() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C2: Currency Code Consistency
        if let Some(error) = self.validate_c2_currency_consistency() {
            all_errors.push(error);
        }

        all_errors
    }
}

impl crate::traits::SwiftMessageBody for MT110 {
    fn message_type() -> &'static str {
        "110"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT110::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        // Add header fields
        append_field(&mut result, &self.field_20);
        append_optional_field(&mut result, &self.field_53a);
        append_optional_field(&mut result, &self.field_54a);
        append_optional_field(&mut result, &self.field_72);

        // Add cheque details in sequence
        for cheque in &self.cheques {
            append_field(&mut result, &cheque.field_21);
            append_field(&mut result, &cheque.field_30);
            append_field(&mut result, &cheque.field_32);
            append_optional_field(&mut result, &cheque.field_50);
            append_optional_field(&mut result, &cheque.field_52);
            append_field(&mut result, &cheque.field_59);
        }

        finalize_mt_string(result, false)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT110::validate_network_rules(self, stop_on_first_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_c2_currency_consistency_pass() {
        // Valid message with consistent currency
        let input = r#":20:TESTREF123
:21:CHQ001
:30:250101
:32B:USD1234,56
:59:JOHN DOE
123 MAIN ST
NEW YORK
:21:CHQ002
:30:250102
:32B:USD2345,67
:59:JANE SMITH
456 ELM ST
BOSTON
-"#;

        let msg = MT110::parse_from_block4(input).expect("Should parse successfully");
        let errors = msg.validate_network_rules(false);
        assert!(
            errors.is_empty(),
            "Should have no validation errors for consistent currencies"
        );
    }

    #[test]
    fn test_validate_c2_currency_consistency_fail() {
        // Invalid message with inconsistent currency
        let input = r#":20:TESTREF123
:21:CHQ001
:30:250101
:32B:USD1234,56
:59:JOHN DOE
123 MAIN ST
NEW YORK
:21:CHQ002
:30:250102
:32B:EUR2345,67
:59:JANE SMITH
456 ELM ST
BOSTON
-"#;

        let msg = MT110::parse_from_block4(input).expect("Should parse successfully");
        let errors = msg.validate_network_rules(false);
        assert_eq!(errors.len(), 1, "Should have exactly one validation error");
        assert_eq!(errors[0].code(), "C02");
        assert!(errors[0].message().contains("Currency code in field 32a"));
    }

    #[test]
    fn test_validate_c1_max_repetitions() {
        // Create a message with 11 cheques (exceeds limit)
        let mut cheque_details = String::new();
        for i in 1..=11 {
            cheque_details.push_str(&format!(
                r#":21:CHQ{:03}
:30:250101
:32B:USD100,00
:59:PAYEE {}
ADDRESS LINE
CITY
"#,
                i, i
            ));
        }

        let input = format!(":20:TESTREF123\n{}-", cheque_details);

        let msg = MT110::parse_from_block4(&input).expect("Should parse successfully");
        let errors = msg.validate_network_rules(false);
        assert_eq!(errors.len(), 1, "Should have exactly one validation error");
        assert_eq!(errors[0].code(), "T10");
        assert!(errors[0].message().contains("maximum 10 occurrences"));
    }

    #[test]
    fn test_validate_stop_on_first_error() {
        // Create a message with both errors: too many cheques AND inconsistent currency
        let mut cheque_details = String::new();
        for i in 1..=11 {
            let currency = if i % 2 == 0 { "EUR" } else { "USD" };
            cheque_details.push_str(&format!(
                r#":21:CHQ{:03}
:30:250101
:32B:{}100,00
:59:PAYEE {}
ADDRESS LINE
CITY
"#,
                i, currency, i
            ));
        }

        let input = format!(":20:TESTREF123\n{}-", cheque_details);

        let msg = MT110::parse_from_block4(&input).expect("Should parse successfully");

        // With stop_on_first_error = true, should only get first error
        let errors_stop = msg.validate_network_rules(true);
        assert_eq!(errors_stop.len(), 1, "Should stop after first error");

        // With stop_on_first_error = false, should get all errors
        let errors_all = msg.validate_network_rules(false);
        assert_eq!(errors_all.len(), 2, "Should collect all errors");
        assert!(errors_all.iter().any(|e| e.code() == "T10"));
        assert!(errors_all.iter().any(|e| e.code() == "C02"));
    }
}
