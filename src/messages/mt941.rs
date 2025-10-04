use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// MT941: Balance Report Message
///
/// ## Purpose
/// Used to report account balance information with summary details for a specific period.
/// This message provides balance reporting with optional transaction summaries and is typically
/// used for balance monitoring and cash management without detailed transaction information.
///
/// ## Scope
/// This message is:
/// - Sent by account servicing institutions for balance reporting
/// - Used for periodic balance reporting (daily, weekly, monthly)
/// - Applied when detailed transaction information is not required
/// - Essential for cash position monitoring and liquidity management
/// - Part of streamlined cash management and treasury operations
///
/// ## Key Features
/// - **Balance Focus**: Emphasis on balance information rather than transaction detail
/// - **Summary Information**: Optional transaction summaries without individual entries
/// - **Period Reporting**: Statement numbering and period identification
/// - **Available Balance**: Forward available balance information for cash planning
/// - **Simplified Structure**: Streamlined format for efficient balance reporting
/// - **Cash Management**: Optimized for automated cash management systems
///
/// ## Common Use Cases
/// - Daily balance reporting for cash management
/// - Automated liquidity monitoring
/// - Treasury position reporting
/// - Balance verification and confirmation
/// - Cash forecasting and planning support
/// - Correspondent banking balance monitoring
/// - Investment account balance reporting
/// - Multi-currency position reporting
///
/// ## Field Structure
/// - **20**: Transaction Reference (mandatory) - Unique report reference
/// - **21**: Related Reference (optional) - Reference to related period or statement
/// - **25**: Account Identification (mandatory) - Account being reported
/// - **28**: Statement Number (mandatory) - Report numbering and sequence
/// - **13D**: Date/Time Indication (optional) - Report timing information
/// - **60F**: Opening Balance (optional) - Starting balance for reporting period
/// - **90D**: Number/Sum of Debit Entries (optional) - Debit transaction summary
/// - **90C**: Number/Sum of Credit Entries (optional) - Credit transaction summary
/// - **62F**: Closing Balance (mandatory) - Ending balance for reporting period
/// - **64**: Available Balance (optional) - Available balance information
/// - **65**: Forward Available Balance (optional, repetitive) - Future balance projections
/// - **86**: Information to Account Owner (optional) - Additional balance information
///
/// ## Network Validation Rules
/// - **Currency Consistency**: All balance fields must use the same currency code
/// - **Reference Format**: Transaction references must follow SWIFT formatting standards
/// - **Required Fields**: All mandatory fields must be present and properly formatted
/// - **Date Validation**: All dates must be valid and properly sequenced
/// - **Amount Validation**: All amounts must be properly formatted with currency precision
/// - **Summary Consistency**: Entry summaries must be consistent with balance calculations
/// - **Account Validation**: Account identification must be valid and properly formatted

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT941 {
    // Transaction Reference Number
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Related Reference (optional)
    #[serde(rename = "21", skip_serializing_if = "Option::is_none")]
    pub field_21: Option<Field21NoOption>,

    // Account Identification
    #[serde(rename = "25")]
    pub field_25: Field25AccountIdentification,

    // Statement Number/Sequence Number
    #[serde(rename = "28")]
    pub field_28: Field28,

    // Date/Time Indication (optional)
    #[serde(rename = "13D", skip_serializing_if = "Option::is_none")]
    pub field_13d: Option<Field13D>,

    // Opening Balance (optional)
    #[serde(rename = "60F", skip_serializing_if = "Option::is_none")]
    pub field_60f: Option<Field60F>,

    // Number and Sum of Debits (optional)
    #[serde(rename = "90D", skip_serializing_if = "Option::is_none")]
    pub field_90d: Option<Field90D>,

    // Number and Sum of Credits (optional)
    #[serde(rename = "90C", skip_serializing_if = "Option::is_none")]
    pub field_90c: Option<Field90C>,

    // Closing Balance (mandatory)
    #[serde(rename = "62F")]
    pub field_62f: Field62F,

    // Closing Available Balance (optional)
    #[serde(rename = "64", skip_serializing_if = "Option::is_none")]
    pub field_64: Option<Field64>,

    // Forward Available Balance (optional, repetitive)
    #[serde(rename = "65", skip_serializing_if = "Option::is_none")]
    pub field_65: Option<Vec<Field65>>,

    // Information to Account Owner (optional)
    #[serde(rename = "86", skip_serializing_if = "Option::is_none")]
    pub field_86: Option<Field86>,
}

impl MT941 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "941");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_optional_field::<Field21NoOption>("21")?;
        let field_25 = parser.parse_field::<Field25AccountIdentification>("25")?;
        let field_28 = parser.parse_field::<Field28>("28")?;

        // Parse optional date/time indication
        let field_13d = parser.parse_optional_field::<Field13D>("13D")?;

        // Parse optional opening balance
        let field_60f = parser.parse_optional_field::<Field60F>("60F")?;

        // Parse optional summary fields
        let field_90d = parser.parse_optional_field::<Field90D>("90D")?;
        let field_90c = parser.parse_optional_field::<Field90C>("90C")?;

        // Parse mandatory closing balance
        let field_62f = parser.parse_field::<Field62F>("62F")?;

        // Parse optional available balance
        let field_64 = parser.parse_optional_field::<Field64>("64")?;

        // Parse optional forward available balance (can be repetitive)
        let mut field_65_vec = Vec::new();
        while parser.detect_field("65") {
            if let Ok(field) = parser.parse_field::<Field65>("65") {
                field_65_vec.push(field);
            } else {
                break;
            }
        }
        let field_65 = if field_65_vec.is_empty() {
            None
        } else {
            Some(field_65_vec)
        };

        // Parse optional information field
        let field_86 = parser.parse_optional_field::<Field86>("86")?;

        Ok(MT941 {
            field_20,
            field_21,
            field_25,
            field_28,
            field_13d,
            field_60f,
            field_90d,
            field_90c,
            field_62f,
            field_64,
            field_65,
            field_86,
        })
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT941)
    // ========================================================================

    // No validation constants needed for MT941 - only currency consistency check

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Get the base currency (first two characters) from mandatory field 62F
    fn get_base_currency(&self) -> &str {
        &self.field_62f.currency[0..2]
    }

    // ========================================================================
    // VALIDATION RULES
    // ========================================================================

    /// C1: Currency Code Consistency (Error code: C27)
    /// The first two characters of the three character currency code in fields 60F, 90D,
    /// 90C, 62F, 64 and 65 must be the same for all occurrences of these fields
    fn validate_c1_currency_consistency(
        &self,
        stop_on_first_error: bool,
    ) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();
        let base_currency = self.get_base_currency();

        // Check 60F if present
        if let Some(ref field_60f) = self.field_60f
            && &field_60f.currency[0..2] != base_currency
        {
            errors.push(SwiftValidationError::content_error(
                    "C27",
                    "60F",
                    &field_60f.currency,
                    &format!(
                        "Currency code in field 60F ({}) must have the same first two characters as field 62F ({})",
                        &field_60f.currency[0..2],
                        base_currency
                    ),
                    "The first two characters of the three character currency code in fields 60F, 90D, 90C, 62F, 64 and 65 must be the same for all occurrences of these fields",
                ));
            if stop_on_first_error {
                return errors;
            }
        }

        // Check 90D if present
        if let Some(ref field_90d) = self.field_90d
            && &field_90d.currency[0..2] != base_currency
        {
            errors.push(SwiftValidationError::content_error(
                    "C27",
                    "90D",
                    &field_90d.currency,
                    &format!(
                        "Currency code in field 90D ({}) must have the same first two characters as field 62F ({})",
                        &field_90d.currency[0..2],
                        base_currency
                    ),
                    "The first two characters of the three character currency code in fields 60F, 90D, 90C, 62F, 64 and 65 must be the same for all occurrences of these fields",
                ));
            if stop_on_first_error {
                return errors;
            }
        }

        // Check 90C if present
        if let Some(ref field_90c) = self.field_90c
            && &field_90c.currency[0..2] != base_currency
        {
            errors.push(SwiftValidationError::content_error(
                    "C27",
                    "90C",
                    &field_90c.currency,
                    &format!(
                        "Currency code in field 90C ({}) must have the same first two characters as field 62F ({})",
                        &field_90c.currency[0..2],
                        base_currency
                    ),
                    "The first two characters of the three character currency code in fields 60F, 90D, 90C, 62F, 64 and 65 must be the same for all occurrences of these fields",
                ));
            if stop_on_first_error {
                return errors;
            }
        }

        // Check 64 if present
        if let Some(ref field_64) = self.field_64
            && &field_64.currency[0..2] != base_currency
        {
            errors.push(SwiftValidationError::content_error(
                    "C27",
                    "64",
                    &field_64.currency,
                    &format!(
                        "Currency code in field 64 ({}) must have the same first two characters as field 62F ({})",
                        &field_64.currency[0..2],
                        base_currency
                    ),
                    "The first two characters of the three character currency code in fields 60F, 90D, 90C, 62F, 64 and 65 must be the same for all occurrences of these fields",
                ));
            if stop_on_first_error {
                return errors;
            }
        }

        // Check 65 if present (can be repetitive)
        if let Some(ref field_65_vec) = self.field_65 {
            for (idx, field_65) in field_65_vec.iter().enumerate() {
                if &field_65.currency[0..2] != base_currency {
                    errors.push(SwiftValidationError::content_error(
                        "C27",
                        "65",
                        &field_65.currency,
                        &format!(
                            "Currency code in field 65[{}] ({}) must have the same first two characters as field 62F ({})",
                            idx,
                            &field_65.currency[0..2],
                            base_currency
                        ),
                        "The first two characters of the three character currency code in fields 60F, 90D, 90C, 62F, 64 and 65 must be the same for all occurrences of these fields",
                    ));
                    if stop_on_first_error {
                        return errors;
                    }
                }
            }
        }

        errors
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Currency Code Consistency
        let c1_errors = self.validate_c1_currency_consistency(stop_on_first_error);
        all_errors.extend(c1_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        all_errors
    }
}

// Implement the SwiftMessageBody trait for MT941
impl crate::traits::SwiftMessageBody for MT941 {
    fn message_type() -> &'static str {
        "941"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_optional_field(&mut result, &self.field_21);
        append_field(&mut result, &self.field_25);
        append_field(&mut result, &self.field_28);
        append_optional_field(&mut result, &self.field_13d);
        append_optional_field(&mut result, &self.field_60f);
        append_optional_field(&mut result, &self.field_90d);
        append_optional_field(&mut result, &self.field_90c);
        append_field(&mut result, &self.field_62f);
        append_optional_field(&mut result, &self.field_64);
        append_vec_field(&mut result, &self.field_65);
        append_optional_field(&mut result, &self.field_86);

        finalize_mt_string(result, false)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT941::validate_network_rules(self, stop_on_first_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::SwiftField;

    #[test]
    fn test_mt941_validate_c1_currency_consistency_pass() {
        // Valid message - all currency codes have the same first two characters
        let mt941 = MT941 {
            field_20: Field20::parse("BALREP001").unwrap(),
            field_21: None,
            field_25: Field25AccountIdentification::parse("1234567890").unwrap(),
            field_28: Field28::parse("1").unwrap(),
            field_13d: None,
            field_60f: Some(Field60F::parse("C251003EUR595771,95").unwrap()),
            field_90d: Some(Field90D::parse("72EUR385920,").unwrap()),
            field_90c: Some(Field90C::parse("44EUR450000,").unwrap()),
            field_62f: Field62F::parse("C251003EUR659851,95").unwrap(),
            field_64: Some(Field64::parse("C251003EUR480525,87").unwrap()),
            field_65: Some(vec![Field65::parse("C251004EUR530691,95").unwrap()]),
            field_86: None,
        };

        let errors = mt941.validate_network_rules(false);
        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_mt941_validate_c1_currency_consistency_fail_60f() {
        // Invalid message - field 60F has different currency prefix
        let mt941 = MT941 {
            field_20: Field20::parse("BALREP001").unwrap(),
            field_21: None,
            field_25: Field25AccountIdentification::parse("1234567890").unwrap(),
            field_28: Field28::parse("1").unwrap(),
            field_13d: None,
            field_60f: Some(Field60F::parse("C251003USD595771,95").unwrap()), // USD instead of EUR
            field_90d: None,
            field_90c: None,
            field_62f: Field62F::parse("C251003EUR659851,95").unwrap(),
            field_64: None,
            field_65: None,
            field_86: None,
        };

        let errors = mt941.validate_network_rules(false);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message().contains("60F"));
        assert!(errors[0].message().contains("US"));
        assert!(errors[0].message().contains("EU"));
    }

    #[test]
    fn test_mt941_validate_c1_currency_consistency_fail_multiple() {
        // Invalid message - multiple fields have different currency prefixes
        let mt941 = MT941 {
            field_20: Field20::parse("BALREP001").unwrap(),
            field_21: None,
            field_25: Field25AccountIdentification::parse("1234567890").unwrap(),
            field_28: Field28::parse("1").unwrap(),
            field_13d: None,
            field_60f: Some(Field60F::parse("C251003USD595771,95").unwrap()), // USD
            field_90d: Some(Field90D::parse("72GBP385920,").unwrap()),        // GBP
            field_90c: Some(Field90C::parse("44JPY450000,").unwrap()),        // JPY
            field_62f: Field62F::parse("C251003EUR659851,95").unwrap(),       // EUR
            field_64: Some(Field64::parse("C251003CHF480525,87").unwrap()),   // CHF
            field_65: Some(vec![
                Field65::parse("C251004AUD530691,95").unwrap(), // AUD
                Field65::parse("C251005CAD530691,95").unwrap(), // CAD
            ]),
            field_86: None,
        };

        let errors = mt941.validate_network_rules(false);
        assert_eq!(errors.len(), 6); // 60F, 90D, 90C, 64, 65[0], 65[1]
    }

    #[test]
    fn test_mt941_validate_c1_stop_on_first_error() {
        // Invalid message - multiple fields have different currency prefixes
        let mt941 = MT941 {
            field_20: Field20::parse("BALREP001").unwrap(),
            field_21: None,
            field_25: Field25AccountIdentification::parse("1234567890").unwrap(),
            field_28: Field28::parse("1").unwrap(),
            field_13d: None,
            field_60f: Some(Field60F::parse("C251003USD595771,95").unwrap()),
            field_90d: Some(Field90D::parse("72GBP385920,").unwrap()),
            field_90c: None,
            field_62f: Field62F::parse("C251003EUR659851,95").unwrap(),
            field_64: None,
            field_65: None,
            field_86: None,
        };

        let errors = mt941.validate_network_rules(true); // stop on first error
        assert_eq!(errors.len(), 1); // Should only return the first error (60F)
    }
}
