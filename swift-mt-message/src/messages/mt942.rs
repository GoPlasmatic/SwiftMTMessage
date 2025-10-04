use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};

/// MT942: Interim Transaction Report
///
/// ## Purpose
/// Used to report interim account information including real-time or intraday transaction
/// details and balance updates. This message provides timely account information between
/// regular statement periods for enhanced cash management and liquidity monitoring.
///
/// ## Scope
/// This message is:
/// - Sent for real-time or intraday account reporting
/// - Used when immediate transaction visibility is required
/// - Applied for active cash management and treasury operations
/// - Essential for intraday liquidity management and position monitoring
/// - Part of real-time cash management and payment system integration
///
/// ## Key Features
/// - **Real-time Reporting**: Immediate transaction and balance information
/// - **Intraday Updates**: Multiple reports possible within a single business day
/// - **Balance Limits**: Credit and debit limit information for account management
/// - **Transaction Details**: Individual transaction entries with real-time processing
/// - **Summary Information**: Debit and credit entry summaries for quick analysis
/// - **Available Balance**: Current available balance for immediate decision making
///
/// ## Common Use Cases
/// - Intraday liquidity monitoring
/// - Real-time cash position management
/// - Payment system integration
/// - Overdraft and credit limit monitoring
/// - High-frequency trading account management
/// - Treasury operations requiring immediate visibility
/// - Risk management and exposure monitoring
/// - Automated cash sweeping and positioning
///
/// ## Field Structure
/// - **20**: Transaction Reference (mandatory) - Unique report reference
/// - **21**: Related Reference (optional) - Reference to related period or statement
/// - **25**: Account Identification (mandatory) - Account being reported
/// - **28C**: Statement Number/Sequence (mandatory) - Report numbering
/// - **34F**: Debit Floor Limit Indicator (mandatory) - Minimum debit transaction amount for reporting
/// - **34F**: Credit Floor Limit Indicator (optional) - Minimum credit transaction amount for reporting
/// - **13D**: Date/Time Indication (mandatory) - Precise timing of report
/// - **Statement Lines**: Repetitive sequence of transaction details (Field 61 + optional Field 86)
/// - **90D**: Number/Sum of Debit Entries (optional) - Debit transaction summary
/// - **90C**: Number/Sum of Credit Entries (optional) - Credit transaction summary
/// - **86**: Information to Account Owner (optional) - Additional information
///
/// ## Network Validation Rules
/// - **Currency Consistency**: All floor limit and entry summary fields must use consistent currency
/// - **Entry Currency Consistency**: Entry summaries must use same currency as floor limits
/// - **Floor Limit DC Mark**: Second occurrence of field 34F must have debit/credit mark 'C'
/// - **Reference Format**: Transaction references must follow SWIFT standards
/// - **Required Fields**: All mandatory fields must be present and properly formatted
/// - **Real-time Constraints**: Timing information must reflect current processing

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT942 {
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
    #[serde(rename = "28C")]
    pub field_28c: Field28C,

    // Debit Floor Limit Indicator (mandatory)
    #[serde(rename = "34F_debit")]
    pub floor_limit_debit: Field34F,

    // Credit Floor Limit Indicator (optional)
    #[serde(rename = "34F_credit", skip_serializing_if = "Option::is_none")]
    pub floor_limit_credit: Option<Field34F>,

    // Date/Time Indication (mandatory)
    #[serde(rename = "13D")]
    pub field_13d: Field13D,

    // Statement Lines (repetitive)
    #[serde(rename = "statement_lines")]
    pub statement_lines: Vec<MT942StatementLine>,

    // Number and Sum of Debits (optional)
    #[serde(rename = "90D", skip_serializing_if = "Option::is_none")]
    pub field_90d: Option<Field90D>,

    // Number and Sum of Credits (optional)
    #[serde(rename = "90C", skip_serializing_if = "Option::is_none")]
    pub field_90c: Option<Field90C>,

    // Information to Account Owner (optional)
    #[serde(rename = "86", skip_serializing_if = "Option::is_none")]
    pub field_86: Option<Field86>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT942StatementLine {
    // Statement Line
    #[serde(rename = "61")]
    pub field_61: Field61,

    // Information to Account Owner (optional)
    #[serde(rename = "86", skip_serializing_if = "Option::is_none")]
    pub field_86: Option<Field86>,
}

impl MT942 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "942");

        // Parse mandatory fields in flexible order
        // Field 13D might appear first due to HashMap ordering issues

        // Check if Field 13D appears early (out of standard order)
        let field_13d_early = if parser.detect_field("13D") {
            Some(parser.parse_field::<Field13D>("13D")?)
        } else {
            None
        };

        // Parse fields in standard order
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_optional_field::<Field21NoOption>("21")?;
        let field_25 = parser.parse_field::<Field25AccountIdentification>("25")?;
        let field_28c = parser.parse_field::<Field28C>("28C")?;

        // Parse floor limit indicators (Field 34F appears twice)
        let floor_limit_debit = parser.parse_field::<Field34F>("34F")?;
        let floor_limit_credit = parser.parse_optional_field::<Field34F>("34F")?;

        // Parse Field 13D if not already parsed
        let field_13d = if let Some(early_13d) = field_13d_early {
            early_13d
        } else {
            parser.parse_field::<Field13D>("13D")?
        };

        // Enable duplicate field handling for statement lines
        parser = parser.with_duplicates(true);

        // Parse statement lines (optional, repetitive)
        let mut statement_lines = Vec::new();

        while parser.detect_field("61") {
            let field_61 = parser.parse_field::<Field61>("61")?;
            let field_86 = parser.parse_optional_field::<Field86>("86")?;

            statement_lines.push(MT942StatementLine { field_61, field_86 });
        }

        // Parse optional summary fields
        let field_90d = parser.parse_optional_field::<Field90D>("90D")?;
        let field_90c = parser.parse_optional_field::<Field90C>("90C")?;

        // Parse optional information to account owner
        let field_86 = parser.parse_optional_field::<Field86>("86")?;

        Ok(MT942 {
            field_20,
            field_21,
            field_25,
            field_28c,
            floor_limit_debit,
            floor_limit_credit,
            field_13d,
            statement_lines,
            field_90d,
            field_90c,
            field_86,
        })
    }

    /// Validation rules for the message (legacy method for backward compatibility)
    ///
    /// **Note**: This method returns a static JSON string for legacy validation systems.
    /// For actual validation, use `validate_network_rules()` which returns detailed errors.
    pub fn validate() -> &'static str {
        r#"{"rules": [{"id": "MT942_VALIDATION", "description": "Use validate_network_rules() for detailed validation", "condition": true}]}"#
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT942)
    // ========================================================================

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Get the base currency from the mandatory debit floor limit
    fn get_base_currency(&self) -> &str {
        &self.floor_limit_debit.currency[0..2]
    }

    // ========================================================================
    // VALIDATION RULES (C1-C3)
    // ========================================================================

    /// C1: Currency Code Consistency (Error code: C27)
    /// The first two characters of the three character currency code in fields 34F,
    /// 90D, and 90C must be the same for all occurrences
    fn validate_c1_currency_consistency(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();
        let base_currency = self.get_base_currency();

        // Check floor limit credit if present
        if let Some(ref floor_limit_credit) = self.floor_limit_credit {
            let credit_currency = &floor_limit_credit.currency[0..2];
            if credit_currency != base_currency {
                errors.push(SwiftValidationError::content_error(
                    "C27",
                    "34F",
                    &floor_limit_credit.currency,
                    &format!(
                        "Currency code in second field 34F ({}) must match first field 34F ({}). First two characters must be the same for all currency fields",
                        credit_currency, base_currency
                    ),
                    "The first two characters of the three character currency code in fields 34F, 90D, and 90C must be the same for all occurrences",
                ));
            }
        }

        // Check field 90D if present
        if let Some(ref field_90d) = self.field_90d {
            let field_90d_currency = &field_90d.currency[0..2];
            if field_90d_currency != base_currency {
                errors.push(SwiftValidationError::content_error(
                    "C27",
                    "90D",
                    &field_90d.currency,
                    &format!(
                        "Currency code in field 90D ({}) must match field 34F ({}). First two characters must be the same for all currency fields",
                        field_90d_currency, base_currency
                    ),
                    "The first two characters of the three character currency code in fields 34F, 90D, and 90C must be the same for all occurrences",
                ));
            }
        }

        // Check field 90C if present
        if let Some(ref field_90c) = self.field_90c {
            let field_90c_currency = &field_90c.currency[0..2];
            if field_90c_currency != base_currency {
                errors.push(SwiftValidationError::content_error(
                    "C27",
                    "90C",
                    &field_90c.currency,
                    &format!(
                        "Currency code in field 90C ({}) must match field 34F ({}). First two characters must be the same for all currency fields",
                        field_90c_currency, base_currency
                    ),
                    "The first two characters of the three character currency code in fields 34F, 90D, and 90C must be the same for all occurrences",
                ));
            }
        }

        errors
    }

    /// C2: Floor Limit Indicator D/C Mark (Error code: C23)
    /// When only one field 34F is present, the second subfield (D/C Mark) must not be used.
    /// When both fields 34F are present, subfield 2 of the first 34F must contain 'D',
    /// and subfield 2 of the second 34F must contain 'C'
    fn validate_c2_floor_limit_dc_mark(&self) -> Option<SwiftValidationError> {
        if let Some(ref floor_limit_credit) = self.floor_limit_credit {
            // Two occurrences - first must have 'D', second must have 'C'

            // Check first occurrence (debit) has 'D'
            if self.floor_limit_debit.indicator != Some('D') {
                return Some(SwiftValidationError::content_error(
                    "C23",
                    "34F",
                    &format!("{:?}", self.floor_limit_debit.indicator),
                    &format!(
                        "When two field 34F are present, first occurrence must have D/C mark 'D', found '{:?}'",
                        self.floor_limit_debit.indicator
                    ),
                    "When both fields 34F are present, subfield 2 of the first 34F must contain the value 'D', and subfield 2 of the second 34F must contain the value 'C'",
                ));
            }

            // Check second occurrence (credit) has 'C'
            if floor_limit_credit.indicator != Some('C') {
                return Some(SwiftValidationError::content_error(
                    "C23",
                    "34F",
                    &format!("{:?}", floor_limit_credit.indicator),
                    &format!(
                        "When two field 34F are present, second occurrence must have D/C mark 'C', found '{:?}'",
                        floor_limit_credit.indicator
                    ),
                    "When both fields 34F are present, subfield 2 of the first 34F must contain the value 'D', and subfield 2 of the second 34F must contain the value 'C'",
                ));
            }
        } else {
            // Single occurrence - D/C mark must not be used
            if self.floor_limit_debit.indicator.is_some() {
                return Some(SwiftValidationError::content_error(
                    "C23",
                    "34F",
                    &format!("{:?}", self.floor_limit_debit.indicator),
                    &format!(
                        "When only one field 34F is present, D/C mark must not be used, found '{:?}'",
                        self.floor_limit_debit.indicator
                    ),
                    "When only one field 34F is present, the second subfield (D/C Mark) must not be used",
                ));
            }
        }

        None
    }

    /// C3: Field 86 Positioning and Relationship to Field 61 (Error code: C24)
    /// If field 86 is present in any occurrence of the repetitive sequence, it must be
    /// preceded by a field 61 except if that field 86 is the last field in the message,
    /// then field 61 is optional
    fn validate_c3_field_86_positioning(&self) -> Vec<SwiftValidationError> {
        let errors = Vec::new();

        // Check each statement line
        for statement_line in self.statement_lines.iter() {
            if statement_line.field_86.is_some() {
                // Within the repetitive sequence, field 86 must be preceded by field 61
                // This is structurally enforced by our data model (field_86 is part of MT942StatementLine)
                // So this check is always satisfied for statement_lines

                // The rule is primarily about ensuring field 86 within statement lines
                // is properly associated with a field 61, which our structure guarantees
            }
        }

        // If there's a message-level field 86 (self.field_86), it's the last field
        // and doesn't need to be preceded by field 61, so it's valid

        // The structural validation is implicitly handled by the parsing logic
        // No explicit validation error needed here as the structure enforces the rule

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

        // C2: Floor Limit Indicator D/C Mark
        if let Some(error) = self.validate_c2_floor_limit_dc_mark() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C3: Field 86 Positioning
        let c3_errors = self.validate_c3_field_86_positioning();
        all_errors.extend(c3_errors);

        all_errors
    }
}

// Implement the SwiftMessageBody trait for MT942
impl crate::traits::SwiftMessageBody for MT942 {
    fn message_type() -> &'static str {
        "942"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_optional_field(&mut result, &self.field_21);
        append_field(&mut result, &self.field_25);
        append_field(&mut result, &self.field_28c);
        append_field(&mut result, &self.floor_limit_debit);
        append_optional_field(&mut result, &self.floor_limit_credit);
        append_field(&mut result, &self.field_13d);

        // Statement lines
        for statement_line in &self.statement_lines {
            append_field(&mut result, &statement_line.field_61);
            append_optional_field(&mut result, &statement_line.field_86);
        }

        append_optional_field(&mut result, &self.field_90d);
        append_optional_field(&mut result, &self.field_90c);
        append_optional_field(&mut result, &self.field_86);

        finalize_mt_string(result, false)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT942::validate_network_rules(self, stop_on_first_error)
    }
}
