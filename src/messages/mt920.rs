use crate::errors::SwiftValidationError;
use crate::fields::{field34::Field34F, *};
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// **MT920: Request Message**
///
/// Requests specific account information or statement messages.
///
/// **Usage:** Statement requests, account information inquiries
/// **Category:** Category 9 (Cash Management & Customer Status)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT920 {
    /// Transaction Reference Number (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Repetitive sequence (1-100 occurrences)
    #[serde(rename = "#")]
    pub sequence: Vec<MT920Sequence>,
}

/// Repetitive sequence for MT920 request message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT920Sequence {
    /// Message Type (Field 12)
    #[serde(rename = "12")]
    pub field_12: Field12,

    /// Account Identification (Field 25)
    #[serde(rename = "25")]
    pub field_25: Field25,

    /// Debit Floor Limit (Field 34F)
    #[serde(rename = "34F_1")]
    pub floor_limit_debit: Option<Field34F>,

    /// Credit Floor Limit (Field 34F)
    #[serde(rename = "34F_2")]
    pub floor_limit_credit: Option<Field34F>,
}

impl MT920 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "920");

        // Parse header field
        let field_20 = parser.parse_field::<Field20>("20")?;

        // Parse repetitive sequences
        let mut sequence = Vec::new();

        // Enable duplicate field handling for repeated sequences
        parser = parser.with_duplicates(true);

        // Detect and parse each sequence (field 12 marks the start of a new sequence)
        while parser.detect_field("12") {
            let field_12 = parser.parse_field::<Field12>("12")?;
            let field_25 = parser.parse_field::<Field25>("25")?;
            let floor_limit_debit = parser.parse_optional_field::<Field34F>("34F")?;
            let floor_limit_credit = parser.parse_optional_field::<Field34F>("34F")?;

            // Apply max repetitions validation
            if sequence.len() >= 100 {
                return Err(crate::errors::ParseError::InvalidFormat {
                    message: "Maximum 100 repetitions allowed".to_string(),
                });
            }

            sequence.push(MT920Sequence {
                field_12,
                field_25,
                floor_limit_debit,
                floor_limit_credit,
            });
        }

        // Validate at least one sequence is present
        if sequence.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "At least one sequence is required in MT920".to_string(),
            });
        }

        // Verify all content is consumed
        verify_parser_complete(&parser)?;

        Ok(Self { field_20, sequence })
    }

    /// Parse from SWIFT MT text format
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        let block4 = extract_block4(input)?;
        Self::parse_from_block4(&block4)
    }

    /// Convert to SWIFT MT text format
    pub fn to_mt_string(&self) -> String {
        let mut result = String::new();

        // Add header field
        append_field(&mut result, &self.field_20);

        // Add sequences
        for seq in &self.sequence {
            append_field(&mut result, &seq.field_12);
            append_field(&mut result, &seq.field_25);
            append_optional_field(&mut result, &seq.floor_limit_debit);
            append_optional_field(&mut result, &seq.floor_limit_credit);
        }

        finalize_mt_string(result, false)
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT920)
    // ========================================================================

    /// Field 12 valid message type codes for MT920
    const VALID_MESSAGE_TYPES: &'static [&'static str] = &["940", "941", "942", "950"];

    // ========================================================================
    // VALIDATION RULES (T88, C1, C2, C3)
    // ========================================================================

    /// T88: Field 12 Message Type Validation (Error code: T88)
    /// Field 12 must contain one of: 940, 941, 942, 950
    fn validate_t88_message_type(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, seq) in self.sequence.iter().enumerate() {
            let type_code = &seq.field_12.type_code;
            if !Self::VALID_MESSAGE_TYPES.contains(&type_code.as_str()) {
                errors.push(SwiftValidationError::format_error(
                    "T88",
                    "12",
                    type_code,
                    &format!("One of: {}", Self::VALID_MESSAGE_TYPES.join(", ")),
                    &format!(
                        "Sequence {}: Field 12 message type '{}' is not valid. Valid types: {}",
                        idx + 1,
                        type_code,
                        Self::VALID_MESSAGE_TYPES.join(", ")
                    ),
                ));
            }
        }

        errors
    }

    /// C1: Field 34F Requirement for MT 942 Requests (Error code: C22)
    /// If field 12 contains '942', at least field 34F Debit/(Debit and Credit) must be present
    fn validate_c1_field_34f_requirement(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, seq) in self.sequence.iter().enumerate() {
            if seq.field_12.type_code == "942" && seq.floor_limit_debit.is_none() {
                errors.push(SwiftValidationError::business_error(
                    "C22",
                    "34F",
                    vec!["12".to_string()],
                    &format!(
                        "Sequence {}: Field 34F (Debit/Debit and Credit Floor Limit) is mandatory when field 12 contains '942'",
                        idx + 1
                    ),
                    "When requesting MT 942 (Interim Transaction Report), at least the debit floor limit must be specified",
                ));
            }
        }

        errors
    }

    /// C2: Field 34F D/C Mark Usage (Error code: C23)
    /// When only one 34F present: D/C Mark must NOT be used
    /// When both 34F present: First must have 'D', second must have 'C'
    fn validate_c2_dc_mark_usage(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, seq) in self.sequence.iter().enumerate() {
            let has_debit = seq.floor_limit_debit.is_some();
            let has_credit = seq.floor_limit_credit.is_some();

            if has_debit && !has_credit {
                // Only debit field present - D/C Mark must NOT be used
                if let Some(ref debit_field) = seq.floor_limit_debit
                    && debit_field.indicator.is_some()
                {
                    errors.push(SwiftValidationError::business_error(
                            "C23",
                            "34F",
                            vec![],
                            &format!(
                                "Sequence {}: D/C Mark must not be used when only one field 34F is present",
                                idx + 1
                            ),
                            "When only one field 34F is present, the floor limit applies to both debit and credit amounts, and the D/C Mark subfield must not be used",
                        ));
                }
            } else if has_debit && has_credit {
                // Both fields present - First must have 'D', second must have 'C'
                let debit_field = seq.floor_limit_debit.as_ref().unwrap();
                let credit_field = seq.floor_limit_credit.as_ref().unwrap();

                if debit_field.indicator != Some('D') {
                    errors.push(SwiftValidationError::business_error(
                        "C23",
                        "34F",
                        vec![],
                        &format!(
                            "Sequence {}: First field 34F must contain D/C Mark 'D' when both floor limits are present",
                            idx + 1
                        ),
                        "When both fields 34F are present, the first field (debit floor limit) must contain D/C Mark = 'D'",
                    ));
                }

                if credit_field.indicator != Some('C') {
                    errors.push(SwiftValidationError::business_error(
                        "C23",
                        "34F",
                        vec![],
                        &format!(
                            "Sequence {}: Second field 34F must contain D/C Mark 'C' when both floor limits are present",
                            idx + 1
                        ),
                        "When both fields 34F are present, the second field (credit floor limit) must contain D/C Mark = 'C'",
                    ));
                }
            }
        }

        errors
    }

    /// C3: Currency Consistency Within Repetitive Sequence (Error code: C40)
    /// Currency code must be the same for each occurrence of field 34F within each sequence
    fn validate_c3_currency_consistency(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, seq) in self.sequence.iter().enumerate() {
            if let (Some(debit_field), Some(credit_field)) =
                (&seq.floor_limit_debit, &seq.floor_limit_credit)
                && debit_field.currency != credit_field.currency
            {
                errors.push(SwiftValidationError::business_error(
                        "C40",
                        "34F",
                        vec![],
                        &format!(
                            "Sequence {}: Currency code must be the same for all field 34F occurrences. Found '{}' and '{}'",
                            idx + 1,
                            debit_field.currency,
                            credit_field.currency
                        ),
                        "Within each repetitive sequence, the currency code must be the same for each occurrence of field 34F",
                    ));
            }
        }

        errors
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // T88: Message Type Validation
        let t88_errors = self.validate_t88_message_type();
        all_errors.extend(t88_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C1: Field 34F Requirement for MT 942
        let c1_errors = self.validate_c1_field_34f_requirement();
        all_errors.extend(c1_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C2: Field 34F D/C Mark Usage
        let c2_errors = self.validate_c2_dc_mark_usage();
        all_errors.extend(c2_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C3: Currency Consistency
        let c3_errors = self.validate_c3_currency_consistency();
        all_errors.extend(c3_errors);

        all_errors
    }
}

impl crate::traits::SwiftMessageBody for MT920 {
    fn message_type() -> &'static str {
        "920"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT920::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT920::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT920::validate_network_rules(self, stop_on_first_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt920_parse() {
        let mt920_text = r#":20:REQ123456
:12:100
:25:/GB12ABCD12345678901234
:12:200
:25:/US98EFGH98765432109876
-"#;
        let result = MT920::parse_from_block4(mt920_text);
        if let Err(ref e) = result {
            eprintln!("MT920 parse error: {:?}", e);
        }
        assert!(result.is_ok());
        let mt920 = result.unwrap();
        assert_eq!(mt920.field_20.reference, "REQ123456");
        assert_eq!(mt920.sequence.len(), 2);
        assert_eq!(mt920.sequence[0].field_12.type_code, "100");
        assert_eq!(mt920.sequence[1].field_12.type_code, "200");
    }

    #[test]
    fn test_mt920_validation() {
        // Test empty sequence - should fail
        let mt920_text = r#":20:REQ123456
-"#;
        let result = MT920::parse(mt920_text);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("At least one sequence")
        );
    }

    #[test]
    fn test_mt920_json_deserialization() {
        // Test JSON deserialization for MT920
        let json = r##"{
            "20": {
                "reference": "REQ123456"
            },
            "#": [
                {
                    "12": {
                        "type_code": "940"
                    },
                    "25": {
                        "authorisation": "1234567890"
                    }
                }
            ]
        }"##;

        let result = serde_json::from_str::<MT920>(json);
        if let Err(ref e) = result {
            eprintln!("MT920 JSON deserialization error: {}", e);
        }
        assert!(result.is_ok(), "Failed to deserialize MT920 from JSON");
        let mt920 = result.unwrap();
        assert_eq!(mt920.field_20.reference, "REQ123456");
        assert_eq!(mt920.sequence.len(), 1);
        assert_eq!(mt920.sequence[0].field_12.type_code, "940");
        assert_eq!(mt920.sequence[0].field_25.authorisation, "1234567890");
    }

    #[test]
    fn test_mt920_swift_message_json() {
        use crate::swift_message::SwiftMessage;

        // Test complete SwiftMessage<MT920> JSON deserialization
        let json = r##"{
            "basic_header": {
                "application_id": "F",
                "service_id": "01",
                "sender_bic": "DEUTDEFF",
                "logical_terminal": "DEUTDEFFXXXX",
                "session_number": "0001",
                "sequence_number": "000123"
            },
            "application_header": {
                "direction": "I",
                "message_type": "920",
                "receiver_bic": "DEUTDEFF",
                "destination_address": "DEUTDEFFXXXX",
                "priority": "N"
            },
            "message_type": "920",
            "fields": {
                "20": {
                    "reference": "REQ123456"
                },
                "#": [
                    {
                        "12": {
                            "type_code": "940"
                        },
                        "25": {
                            "authorisation": "1234567890"
                        }
                    }
                ]
            }
        }"##;

        let result = serde_json::from_str::<SwiftMessage<MT920>>(json);
        if let Err(ref e) = result {
            eprintln!("SwiftMessage<MT920> JSON deserialization error: {}", e);
        }
        assert!(
            result.is_ok(),
            "Failed to deserialize SwiftMessage<MT920> from JSON"
        );
        let swift_msg = result.unwrap();
        assert_eq!(swift_msg.message_type, "920");
        assert_eq!(swift_msg.fields.field_20.reference, "REQ123456");
        assert_eq!(swift_msg.fields.sequence.len(), 1);
    }
}
