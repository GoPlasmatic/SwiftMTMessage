use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// **MT192: Request for Cancellation**
///
/// Request to cancel a previously sent payment message before execution.
///
/// **Usage:** Payment cancellation requests, transaction reversal
/// **Category:** Category 1 (Customer Payments & Cheques)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT192 {
    /// Sender's Reference (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Related Reference (Field 21)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// MT and Date (Field 11S)
    #[serde(rename = "11S")]
    pub field_11s: Field11S,

    /// Narrative (Field 79)
    #[serde(rename = "79", skip_serializing_if = "Option::is_none")]
    pub field_79: Option<Field79>,
}

impl MT192 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "192");

        // Parse mandatory fields in order: 20, 21, 11S
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_11s = parser.parse_field::<Field11S>("11S")?;

        // Parse optional field 79
        let field_79 = parser.parse_optional_field::<Field79>("79")?;

        Ok(MT192 {
            field_20,
            field_21,
            field_11s,
            field_79,
        })
    }

    /// Parse from generic SWIFT input (tries to detect blocks)
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        let block4 = extract_block4(input)?;
        Self::parse_from_block4(&block4)
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT192)
    // ========================================================================

    /// Field 79 valid cancellation reason codes for MT192
    const MT192_VALID_79_CODES: &'static [&'static str] = &[
        "AGNT", // Incorrect Agent
        "AM09", // Wrong Amount
        "COVR", // Cover Cancelled or Returned
        "CURR", // Incorrect Currency
        "CUST", // Requested by Customer
        "CUTA", // Cancel upon Unable to Apply
        "DUPL", // Duplicate Payment
        "FRAD", // Fraudulent Origin
        "TECH", // Technical Problem
        "UPAY", // Undue Payment
    ];

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Check if Field 79 is present
    fn has_field_79(&self) -> bool {
        self.field_79.is_some()
    }

    /// Extract cancellation reason code from Field 79 if present
    /// Returns the 4-character code from the first line if formatted as /CODE/...
    fn get_field_79_cancellation_code(&self) -> Option<String> {
        if let Some(ref field_79) = self.field_79 {
            // Get the first line from the information vector
            let first_line = field_79.information.first()?;
            if first_line.starts_with('/') {
                let parts: Vec<&str> = first_line.split('/').collect();
                if parts.len() >= 2 && parts[1].len() == 4 {
                    return Some(parts[1].to_string());
                }
            }
        }
        None
    }

    // ========================================================================
    // VALIDATION RULES (C1)
    // ========================================================================

    /// C1: Field 79 or Copy of Mandatory Fields Requirement (Error code: C25)
    /// Field 79 or a copy of at least the mandatory fields of the original message
    /// or both must be present.
    ///
    /// **Implementation Note**: This implementation only validates that Field 79 is present
    /// since we don't currently support parsing the "copy of mandatory fields" section.
    /// A full implementation would check for either Field 79 OR copied fields OR both.
    fn validate_c1_field_79_or_copy(&self) -> Option<SwiftValidationError> {
        // Check if Field 79 is present
        // Note: In a complete implementation, we would also check for copied fields
        // from the original message, but this is not currently supported
        if !self.has_field_79() {
            return Some(SwiftValidationError::content_error(
                "C25",
                "79",
                "",
                "Field 79 (Narrative Description) or a copy of at least the mandatory fields of the original message must be present",
                "Either field 79 or a copy of at least the mandatory fields of the original message or both must be present to identify the transaction to be cancelled",
            ));
        }

        None
    }

    /// Validate Field 79 cancellation reason codes (if present)
    /// While not explicitly a network validation rule, this validates the cancellation
    /// reason codes are from the allowed list when present in Field 79.
    fn validate_field_79_codes(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        if let Some(code) = self.get_field_79_cancellation_code()
            && !Self::MT192_VALID_79_CODES.contains(&code.as_str())
        {
            errors.push(SwiftValidationError::content_error(
                    "T47",
                    "79",
                    &code,
                    &format!(
                        "Cancellation reason code '{}' is not valid for MT192. Valid codes: {}",
                        code,
                        Self::MT192_VALID_79_CODES.join(", ")
                    ),
                    "Cancellation reason must be one of the allowed codes when using /CODE/ format in field 79",
                ));
        }

        errors
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Field 79 or Copy of Mandatory Fields Requirement
        if let Some(error) = self.validate_c1_field_79_or_copy() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // Validate Field 79 cancellation codes (if present)
        let f79_errors = self.validate_field_79_codes();
        all_errors.extend(f79_errors);

        all_errors
    }
}

// Implement the SwiftMessageBody trait for MT192
impl crate::traits::SwiftMessageBody for MT192 {
    fn message_type() -> &'static str {
        "192"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_field(&mut result, &self.field_21);
        append_field(&mut result, &self.field_11s);
        append_optional_field(&mut result, &self.field_79);

        finalize_mt_string(result, false)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT192::validate_network_rules(self, stop_on_first_error)
    }
}
