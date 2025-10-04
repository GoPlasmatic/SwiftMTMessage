use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// MT950: Statement Message
///
/// ## Purpose
/// Used to transmit account statement information with a simplified structure focusing
/// on balance information and essential transaction data. This message provides streamlined
/// account reporting for efficient processing and communication.
///
/// ## Scope
/// This message is:
/// - Sent by account servicing institutions for streamlined statement delivery
/// - Used for simplified account reporting with essential information
/// - Applied when detailed narrative information is not required
/// - Essential for automated processing and high-volume account reporting
/// - Part of efficient account management and customer communication systems
///
/// ## Key Features
/// - **Simplified Structure**: Streamlined format for efficient processing
/// - **Essential Information**: Focus on key balance and transaction data
/// - **Multiple Transactions**: Support for multiple statement line entries
/// - **Balance Information**: Opening and closing balance with currency consistency
/// - **Available Balance**: Optional available balance information
/// - **Automated Processing**: Optimized for automated statement processing systems
///
/// ## Common Use Cases
/// - High-volume account statement processing
/// - Automated statement delivery systems
/// - Simplified account reporting for operational accounts
/// - Batch processing of multiple account statements
/// - System-to-system account information exchange
/// - Streamlined cash management reporting
/// - Efficient correspondent banking statement delivery
/// - Simplified regulatory reporting requirements
///
/// ## Field Structure
/// - **20**: Transaction Reference (mandatory) - Unique statement reference
/// - **25**: Account Identification (mandatory) - Account being reported
/// - **28C**: Statement Number/Sequence (mandatory) - Statement numbering
/// - **60**: Opening Balance (mandatory) - Starting balance for statement period
/// - **61**: Statement Line (optional, repetitive) - Individual transaction entries
/// - **62**: Closing Balance (mandatory) - Ending balance for statement period
/// - **64**: Available Balance (optional) - Available balance information
///
/// ## Field Details
/// ### Field 61 - Statement Line
/// Multiple statement lines can be included, each containing:
/// - **Value Date**: Date when transaction becomes effective
/// - **Entry Date**: Date when transaction was posted (optional)
/// - **Credit/Debit Mark**: C (Credit) or D (Debit) entry
/// - **Amount**: Transaction amount
/// - **Transaction Type**: SWIFT transaction type identification
/// - **Reference**: Transaction reference number
///
/// ## Network Validation Rules
/// - **Currency Consistency**: Opening and closing balances must use the same currency
/// - **Available Balance Currency**: Available balances must use same currency as main balances
/// - **Reference Format**: Transaction references must follow SWIFT formatting standards
/// - **Required Fields**: All mandatory fields must be present and properly formatted
/// - **Balance Logic**: Closing balance should reflect opening balance plus/minus transactions
/// - **Date Validation**: All dates must be valid and properly sequenced

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT950 {
    // Transaction Reference Number
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Account Identification
    #[serde(rename = "25")]
    pub field_25: Field25NoOption,

    // Statement Number/Sequence Number
    #[serde(rename = "28C")]
    pub field_28c: Field28C,

    // Opening Balance
    #[serde(rename = "60")]
    pub field_60: Field60,

    // Statement Lines (optional, repetitive)
    #[serde(rename = "61", skip_serializing_if = "Option::is_none")]
    pub field_61: Option<Vec<Field61>>,

    // Closing Balance
    #[serde(rename = "62")]
    pub field_62: Field62,

    // Closing Available Balance (optional)
    #[serde(rename = "64", skip_serializing_if = "Option::is_none")]
    pub field_64: Option<Field64>,
}

impl MT950 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "950");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_25 = parser.parse_field::<Field25NoOption>("25")?;
        let field_28c = parser.parse_field::<Field28C>("28C")?;

        // Parse Field60 - check for both 60F and 60M variants
        let field_60 = if parser.detect_field("60F") {
            Field60::F(parser.parse_field::<Field60F>("60F")?)
        } else if parser.detect_field("60M") {
            Field60::M(parser.parse_field::<Field60M>("60M")?)
        } else if parser.detect_field("60") {
            // Try to parse as generic Field60
            parser.parse_field::<Field60>("60")?
        } else {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT950: Missing required field 60 (opening balance)".to_string(),
            });
        };

        // Enable duplicate field handling for statement lines
        parser = parser.with_duplicates(true);

        // Parse optional statement lines (repetitive)
        let mut field_61_vec = Vec::new();
        while parser.detect_field("61") {
            if let Ok(field) = parser.parse_field::<Field61>("61") {
                field_61_vec.push(field);
            } else {
                break;
            }
        }
        let field_61 = if field_61_vec.is_empty() {
            None
        } else {
            Some(field_61_vec)
        };

        // Parse mandatory closing balance - check for both 62F and 62M variants
        let field_62 = if parser.detect_field("62F") {
            Field62::F(parser.parse_field::<Field62F>("62F")?)
        } else if parser.detect_field("62M") {
            Field62::M(parser.parse_field::<Field62M>("62M")?)
        } else if parser.detect_field("62") {
            // Try to parse as generic Field62
            parser.parse_field::<Field62>("62")?
        } else {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT950: Missing required field 62 (closing balance)".to_string(),
            });
        };

        // Parse optional available balance
        let field_64 = parser.parse_optional_field::<Field64>("64")?;

        Ok(MT950 {
            field_20,
            field_25,
            field_28c,
            field_60,
            field_61,
            field_62,
            field_64,
        })
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT950)
    // ========================================================================

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Get the first two characters of currency code from field 60
    fn get_field_60_currency_prefix(&self) -> &str {
        match &self.field_60 {
            Field60::F(field) => &field.currency[0..2],
            Field60::M(field) => &field.currency[0..2],
        }
    }

    /// Get the first two characters of currency code from field 62
    fn get_field_62_currency_prefix(&self) -> &str {
        match &self.field_62 {
            Field62::F(field) => &field.currency[0..2],
            Field62::M(field) => &field.currency[0..2],
        }
    }

    /// Get the full currency code from field 60 for error messages
    fn get_field_60_currency(&self) -> &str {
        match &self.field_60 {
            Field60::F(field) => &field.currency,
            Field60::M(field) => &field.currency,
        }
    }

    /// Get the full currency code from field 62 for error messages
    fn get_field_62_currency(&self) -> &str {
        match &self.field_62 {
            Field62::F(field) => &field.currency,
            Field62::M(field) => &field.currency,
        }
    }

    // ========================================================================
    // VALIDATION RULES
    // ========================================================================

    /// C1: Currency Code Consistency (Error code: C27)
    /// The first two characters of the three character currency code in fields 60a, 62a and 64 must be the same
    fn validate_c1_currency_consistency(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        // Get base currency from field 60 (mandatory)
        let base_currency_prefix = self.get_field_60_currency_prefix();
        let base_currency = self.get_field_60_currency();

        // Check field 62 (mandatory)
        let field_62_prefix = self.get_field_62_currency_prefix();
        let field_62_currency = self.get_field_62_currency();

        if field_62_prefix != base_currency_prefix {
            errors.push(SwiftValidationError::business_error(
                "C27",
                "62a",
                vec!["60a".to_string()],
                &format!(
                    "Currency code mismatch: field 62a currency '{}' (prefix '{}') must have the same first two characters as field 60a currency '{}' (prefix '{}')",
                    field_62_currency, field_62_prefix, base_currency, base_currency_prefix
                ),
                "The first two characters of the three character currency code in fields 60a, 62a and 64 must be the same",
            ));
        }

        // Check field 64 if present (optional)
        if let Some(ref field_64) = self.field_64 {
            let field_64_prefix = &field_64.currency[0..2];
            if field_64_prefix != base_currency_prefix {
                errors.push(SwiftValidationError::business_error(
                    "C27",
                    "64",
                    vec!["60a".to_string(), "62a".to_string()],
                    &format!(
                        "Currency code mismatch: field 64 currency '{}' (prefix '{}') must have the same first two characters as field 60a currency '{}' (prefix '{}')",
                        field_64.currency, field_64_prefix, base_currency, base_currency_prefix
                    ),
                    "The first two characters of the three character currency code in fields 60a, 62a and 64 must be the same",
                ));
            }
        }

        errors
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Currency Code Consistency
        let c1_errors = self.validate_c1_currency_consistency();
        all_errors.extend(c1_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        all_errors
    }
}

// Implement the SwiftMessageBody trait for MT950
impl crate::traits::SwiftMessageBody for MT950 {
    fn message_type() -> &'static str {
        "950"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_field(&mut result, &self.field_25);
        append_field(&mut result, &self.field_28c);
        append_field(&mut result, &self.field_60);
        append_vec_field(&mut result, &self.field_61);
        append_field(&mut result, &self.field_62);
        append_optional_field(&mut result, &self.field_64);

        finalize_mt_string(result, false)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT950::validate_network_rules(self, stop_on_first_error)
    }
}
