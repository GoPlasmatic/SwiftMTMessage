use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// **MT935: Rate Change Advice**
///
/// Advises changes in interest rates affecting accounts or agreements.
///
/// **Usage:** Rate change notifications, interest rate updates
/// **Category:** Category 9 (Cash Management & Customer Status)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MT935 {
    /// Transaction Reference Number (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Rate change sequences (1-10 occurrences)
    #[serde(rename = "#")]
    pub rate_changes: Vec<MT935RateChange>,

    /// Sender to Receiver Information (Field 72)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,
}

/// Rate change sequence for MT935
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MT935RateChange {
    /// Further Identification (Field 23)
    #[serde(rename = "23", skip_serializing_if = "Option::is_none")]
    pub field_23: Option<Field23>,

    /// Account Identification (Field 25)
    #[serde(rename = "25", skip_serializing_if = "Option::is_none")]
    pub field_25: Option<Field25NoOption>,

    /// Effective Date of New Rate (Field 30)
    #[serde(rename = "30")]
    pub field_30: Field30,

    /// New Interest Rate (Field 37H)
    #[serde(rename = "37H")]
    pub field_37h: Vec<Field37H>,
}

impl MT935 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "935");

        // Parse mandatory field 20
        let field_20 = parser.parse_field::<Field20>("20")?;

        // Enable duplicate field handling for repetitive sequences
        parser = parser.with_duplicates(true);

        // Parse rate change sequences (1-10 occurrences)
        let mut rate_changes = Vec::new();

        while (parser.detect_field("23") || parser.detect_field("25")) && rate_changes.len() < 10 {
            // Parse optional Field 23 - Further Identification (mutually exclusive with field_25)
            let field_23 = parser.parse_optional_field::<Field23>("23")?;

            // Parse optional Field 25 - Account Identification (mutually exclusive with field_23)
            let field_25 = parser.parse_optional_field::<Field25NoOption>("25")?;

            // Parse mandatory Field 30 - Effective Date of New Rate
            let field_30 = parser.parse_field::<Field30>("30")?;

            // Parse field 37H (New Interest Rate) - can be multiple per sequence
            let mut field_37h = Vec::new();
            while let Ok(rate) = parser.parse_field::<Field37H>("37H") {
                field_37h.push(rate);
                // Keep parsing 37H until we hit the next sequence marker or end
                if !parser.detect_field("37H") {
                    break;
                }
            }

            // At least one field 37H is required per sequence
            if field_37h.is_empty() {
                return Err(crate::errors::ParseError::InvalidFormat {
                    message: format!(
                        "MT935: At least one field 37H is required for sequence {}",
                        rate_changes.len() + 1
                    ),
                });
            }

            rate_changes.push(MT935RateChange {
                field_23,
                field_25,
                field_30,
                field_37h,
            });
        }

        // Disable duplicates mode after parsing sequences
        parser = parser.with_duplicates(false);

        // At least one sequence is required
        if rate_changes.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT935: At least one rate change sequence is required".to_string(),
            });
        }

        // Parse optional field 72
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT935 {
            field_20,
            rate_changes,
            field_72,
        })
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT935)
    // ========================================================================

    /// Valid function codes for field 23 (Further Identification)
    const VALID_23_FUNCTION_CODES: &'static [&'static str] = &[
        "BASE",
        "CALL",
        "COMMERCIAL",
        "CURRENT",
        "DEPOSIT",
        "NOTICE",
        "PRIME",
    ];

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Check if field 23 is present in a sequence
    fn has_field_23(seq: &MT935RateChange) -> bool {
        seq.field_23.is_some()
    }

    /// Check if field 25 is present in a sequence
    fn has_field_25(seq: &MT935RateChange) -> bool {
        seq.field_25.is_some()
    }

    // ========================================================================
    // VALIDATION RULES (C1-C2 and Field-Specific)
    // ========================================================================

    /// C1: Repetitive Sequence Occurrence (Error code: T10)
    /// The repetitive sequence must appear at least once, but not more than ten times
    fn validate_c1_sequence_occurrence(&self) -> Option<SwiftValidationError> {
        let num_sequences = self.rate_changes.len();

        if num_sequences == 0 {
            return Some(SwiftValidationError::content_error(
                "T10",
                "RateChangeSequence",
                "0",
                "The repetitive sequence must appear at least once",
                "The repetitive sequence (fields 23/25, 30, 37H) must appear at least once",
            ));
        }

        if num_sequences > 10 {
            return Some(SwiftValidationError::content_error(
                "T10",
                "RateChangeSequence",
                &num_sequences.to_string(),
                &format!(
                    "The repetitive sequence must not appear more than ten times, found {}",
                    num_sequences
                ),
                "The repetitive sequence (fields 23/25, 30, 37H) must not appear more than ten times",
            ));
        }

        None
    }

    /// C2: Further Identification and Account Identification Mutual Exclusivity (Error code: C83)
    /// Either field 23 or field 25, but not both, must be present in any repetitive sequence
    fn validate_c2_field_23_25_mutual_exclusivity(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, seq) in self.rate_changes.iter().enumerate() {
            let has_23 = Self::has_field_23(seq);
            let has_25 = Self::has_field_25(seq);

            if has_23 && has_25 {
                // Both present - NOT ALLOWED
                errors.push(SwiftValidationError::relation_error(
                    "C83",
                    "23/25",
                    vec!["23".to_string(), "25".to_string()],
                    &format!(
                        "Sequence {}: Both field 23 and field 25 are present. Either field 23 or field 25, but not both, must be present",
                        idx + 1
                    ),
                    "Either field 23 or field 25, but not both, must be present in any repetitive sequence",
                ));
            } else if !has_23 && !has_25 {
                // Neither present - NOT ALLOWED
                errors.push(SwiftValidationError::relation_error(
                    "C83",
                    "23/25",
                    vec!["23".to_string(), "25".to_string()],
                    &format!(
                        "Sequence {}: Neither field 23 nor field 25 is present. Either field 23 or field 25 must be present",
                        idx + 1
                    ),
                    "Either field 23 or field 25, but not both, must be present in any repetitive sequence",
                ));
            }
        }

        errors
    }

    /// Validate field 23 (Further Identification) format and content
    /// Field 23 must be formatted as: 3!a[2!n]11x (Currency)(Number of Days)(Function)
    fn validate_field_23(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, seq) in self.rate_changes.iter().enumerate() {
            if let Some(ref field_23) = seq.field_23 {
                // Reconstruct the full value as it appears in SWIFT format
                let mut value = field_23.function_code.clone();
                if let Some(days) = field_23.days {
                    value.push_str(&format!("{:02}", days));
                }
                value.push_str(&field_23.reference);

                // Minimum length check: 3 (currency) + at least one function character
                if value.len() < 4 {
                    errors.push(SwiftValidationError::format_error(
                        "T26",
                        "23",
                        &value,
                        "3!a[2!n]11x",
                        &format!(
                            "Sequence {}: Field 23 must be at least 4 characters (currency code + function)",
                            idx + 1
                        ),
                    ));
                    continue;
                }

                // Extract currency (first 3 characters)
                let currency = &value[..3];

                // Validate currency is alphabetic
                if !currency.chars().all(|c| c.is_ascii_alphabetic()) {
                    errors.push(SwiftValidationError::format_error(
                        "T26",
                        "23",
                        &value,
                        "3!a[2!n]11x",
                        &format!(
                            "Sequence {}: Currency code '{}' must be 3 alphabetic characters",
                            idx + 1,
                            currency
                        ),
                    ));
                }

                // Extract remaining part (could be [2!n]function or just function)
                let remaining = &value[3..];

                // Check if next 2 characters are digits (Number of Days)
                let (num_days, function_start) =
                    if remaining.len() >= 2 && remaining[..2].chars().all(|c| c.is_ascii_digit()) {
                        (Some(&remaining[..2]), 2)
                    } else {
                        (None, 0)
                    };

                // Extract function code
                let function = &remaining[function_start..];

                // Validate function code
                if !Self::VALID_23_FUNCTION_CODES.contains(&function) {
                    errors.push(SwiftValidationError::content_error(
                        "T26",
                        "23",
                        function,
                        &format!(
                            "Sequence {}: Function code '{}' is not valid. Valid codes: {}",
                            idx + 1,
                            function,
                            Self::VALID_23_FUNCTION_CODES.join(", ")
                        ),
                        &format!(
                            "Function code must be one of: {}",
                            Self::VALID_23_FUNCTION_CODES.join(", ")
                        ),
                    ));
                }

                // Validate Number of Days only allowed with NOTICE
                if let Some(days) = num_days
                    && function != "NOTICE"
                {
                    errors.push(SwiftValidationError::content_error(
                            "T26",
                            "23",
                            &value,
                            &format!(
                                "Sequence {}: Number of Days '{}' is only allowed when Function is NOTICE, but found '{}'",
                                idx + 1, days, function
                            ),
                            "Number of Days must only be used when Function is NOTICE",
                        ));
                }
            }
        }

        errors
    }

    /// Validate field 37H (New Interest Rate) content rules
    /// - Indicator must be C or D (Error code: T51)
    /// - Sign must not be used if Rate is zero (Error code: T14)
    fn validate_field_37h(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (seq_idx, seq) in self.rate_changes.iter().enumerate() {
            for (field_idx, field_37h) in seq.field_37h.iter().enumerate() {
                let indicator = field_37h.rate_indicator;
                let is_negative = field_37h.is_negative;
                let rate = field_37h.rate;

                // T51: Validate indicator is C or D
                if indicator != 'C' && indicator != 'D' {
                    errors.push(SwiftValidationError::format_error(
                        "T51",
                        "37H",
                        &indicator.to_string(),
                        "C or D",
                        &format!(
                            "Sequence {}, Rate {}: Indicator '{}' is not valid. Must be C (Credit) or D (Debit)",
                            seq_idx + 1,
                            field_idx + 1,
                            indicator
                        ),
                    ));
                }

                // T14: Sign must not be used if rate is zero
                if rate.abs() < 0.00001 && is_negative.is_some() {
                    errors.push(SwiftValidationError::content_error(
                        "T14",
                        "37H",
                        &rate.to_string(),
                        &format!(
                            "Sequence {}, Rate {}: Sign must not be used when rate is zero",
                            seq_idx + 1,
                            field_idx + 1
                        ),
                        "Sign (N for negative) must not be used if Rate is zero",
                    ));
                }
            }
        }

        errors
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Repetitive Sequence Occurrence
        if let Some(error) = self.validate_c1_sequence_occurrence() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C2: Field 23/25 Mutual Exclusivity
        let c2_errors = self.validate_c2_field_23_25_mutual_exclusivity();
        all_errors.extend(c2_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // Field 23 Validation
        let f23_errors = self.validate_field_23();
        all_errors.extend(f23_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // Field 37H Validation
        let f37h_errors = self.validate_field_37h();
        all_errors.extend(f37h_errors);

        all_errors
    }
}

// Implement the SwiftMessageBody trait for MT935
impl crate::traits::SwiftMessageBody for MT935 {
    fn message_type() -> &'static str {
        "935"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        use crate::traits::SwiftField;
        let mut result = String::new();

        append_field(&mut result, &self.field_20);

        // Rate change sequences
        for rate_change in &self.rate_changes {
            append_optional_field(&mut result, &rate_change.field_23);
            append_optional_field(&mut result, &rate_change.field_25);
            append_field(&mut result, &rate_change.field_30);

            // Manually append vec field
            for field_37h in &rate_change.field_37h {
                result.push_str(&field_37h.to_swift_string());
                result.push_str("\r\n");
            }
        }

        append_optional_field(&mut result, &self.field_72);

        finalize_mt_string(result, false)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT935::validate_network_rules(self, stop_on_first_error)
    }
}
