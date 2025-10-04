use crate::errors::{ParseError, SwiftValidationError};
use crate::fields::*;
use crate::parser::MessageParser;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// MT200 - Financial Institution Transfer for Own Account
///
/// Used by financial institutions to transfer funds for their own account,
/// typically for nostro account funding, liquidity management, or internal transfers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT200 {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Field 32A - Value Date, Currency Code, Amount (Mandatory)
    #[serde(rename = "32A")]
    pub field_32a: Field32A,

    /// Field 53B - Sender's Correspondent (Optional)
    #[serde(rename = "53B", skip_serializing_if = "Option::is_none")]
    pub field_53b: Option<Field53B>,

    /// Field 56 - Intermediary Institution (Optional)
    /// Can be 56A or 56D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_56: Option<Field56IntermediaryAD>,

    /// Field 57 - Account With Institution (Mandatory)
    #[serde(flatten)]
    pub field_57: Field57DebtInstitution,

    /// Field 72 - Sender to Receiver Information (Optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,
}

impl MT200 {
    /// Parse MT200 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "200");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_32a = parser.parse_field::<Field32A>("32A")?;

        // Parse optional Field 53B - Sender's Correspondent
        let field_53b = parser.parse_optional_field::<Field53B>("53B")?;

        // Parse optional Field 56 - Intermediary Institution
        let field_56 = parser.parse_optional_variant_field::<Field56IntermediaryAD>("56")?;

        // Parse mandatory Field 57 - Account With Institution
        let field_57 = parser.parse_variant_field::<Field57DebtInstitution>("57")?;

        // Parse optional Field 72
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT200 {
            field_20,
            field_32a,
            field_53b,
            field_56,
            field_57,
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
        append_field(&mut result, &self.field_32a);
        append_optional_field(&mut result, &self.field_53b);
        append_optional_field(&mut result, &self.field_56);
        append_field(&mut result, &self.field_57);
        append_optional_field(&mut result, &self.field_72);

        result.push('-');
        result
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT200)
    // ========================================================================

    /// Special codes requiring SWIFT Payments Reject/Return Guidelines compliance
    const SPECIAL_72_CODES: &'static [&'static str] = &["REJT", "RETN"];

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Extract codes from field 72 content
    fn extract_72_codes(&self) -> Vec<String> {
        let mut codes = Vec::new();

        if let Some(ref field_72) = self.field_72 {
            // Parse field 72 content for codes (format: /CODE/additional text)
            for line in &field_72.information {
                let trimmed = line.trim();
                if let Some(without_prefix) = trimmed.strip_prefix('/') {
                    if let Some(end_idx) = without_prefix.find('/') {
                        let code = &without_prefix[..end_idx];
                        codes.push(code.to_uppercase());
                    } else if !without_prefix.is_empty() {
                        // Code without trailing slash
                        if let Some(space_idx) = without_prefix.find(|c: char| c.is_whitespace()) {
                            codes.push(without_prefix[..space_idx].to_uppercase());
                        } else {
                            codes.push(without_prefix.to_uppercase());
                        }
                    }
                }
            }
        }

        codes
    }

    // ========================================================================
    // VALIDATION RULES
    // ========================================================================

    /// T80: Field 72 REJT/RETN Compliance
    /// If /REJT/ or /RETN/ present, must follow SWIFT Payments Reject/Return Guidelines
    fn validate_t80_field_72_special_codes(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        let codes = self.extract_72_codes();

        for code in &codes {
            if Self::SPECIAL_72_CODES.contains(&code.as_str()) {
                // Note: This is a guideline check - actual compliance verification
                // would require checking the full message structure according to
                // SWIFT Payments Reject/Return Guidelines
                errors.push(SwiftValidationError::content_error(
                    "T80",
                    "72",
                    code,
                    &format!(
                        "Field 72 contains code /{}/. Message must comply with SWIFT Payments Reject/Return Guidelines",
                        code
                    ),
                    "When field 72 contains /REJT/ or /RETN/, the message must follow SWIFT Payments Reject/Return Guidelines",
                ));
            }
        }

        errors
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // Note: Per SR 2025 specification, MT200 has no standard network validated rules.
        // However, field-specific validation rules still apply.

        // T80: Field 72 REJT/RETN Guidelines Compliance
        let t80_errors = self.validate_t80_field_72_special_codes();
        all_errors.extend(t80_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        all_errors
    }
}

impl crate::traits::SwiftMessageBody for MT200 {
    fn message_type() -> &'static str {
        "200"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT200::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT200::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT200::validate_network_rules(self, stop_on_first_error)
    }
}
