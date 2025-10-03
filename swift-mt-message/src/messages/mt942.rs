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

    /// Static validation rules for MT942
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "C1", "description": "The first two characters of the three-character currency code in fields 34F, 61, 90D, and 90C must be the same for all occurrences"},
            {"id": "C2", "description": "The debit/credit mark in the first occurrence of field 34F may be D or C, but the second occurrence must have C"}
        ]}"#
    }

    /// Validate the message instance according to MT942 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // C1: Currency consistency validation
        // Base currency from mandatory floor limit debit
        let base_currency = &self.floor_limit_debit.currency[0..2];

        // Check floor limit credit if present
        if let Some(ref floor_limit_credit) = self.floor_limit_credit
            && &floor_limit_credit.currency[0..2] != base_currency
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "MT942: Currency code mismatch - credit floor limit currency '{}' does not match base currency '{}'",
                    &floor_limit_credit.currency[0..2],
                    base_currency
                ),
            });
        }

        // Check 90D if present
        if let Some(ref field_90d) = self.field_90d
            && &field_90d.currency[0..2] != base_currency
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "MT942: Currency code mismatch - field 90D currency '{}' does not match base currency '{}'",
                    &field_90d.currency[0..2],
                    base_currency
                ),
            });
        }

        // Check 90C if present
        if let Some(ref field_90c) = self.field_90c
            && &field_90c.currency[0..2] != base_currency
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "MT942: Currency code mismatch - field 90C currency '{}' does not match base currency '{}'",
                    &field_90c.currency[0..2],
                    base_currency
                ),
            });
        }

        // C2: Debit/Credit mark validation for Field 34F
        if let Some(ref floor_limit_credit) = self.floor_limit_credit {
            // Second occurrence must have C indicator
            if floor_limit_credit.indicator != Some('C') {
                return Err(crate::errors::ParseError::InvalidFormat {
                    message: format!(
                        "MT942: Second occurrence of field 34F must have indicator 'C', found '{:?}'",
                        floor_limit_credit.indicator
                    ),
                });
            }
        }

        // Note: Field 61 does not contain a currency field directly.
        // Currency consistency for Field 61 is implicitly validated through the
        // statement's base currency in other balance fields.

        Ok(())
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
}
