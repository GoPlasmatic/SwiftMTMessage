use crate::fields::*;
use crate::parsing_utils::*;
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
        let mut parser = crate::message_parser::MessageParser::new(block4, "941");

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

    /// Static validation rules for MT941
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "C1", "description": "The first two characters of the three-character currency code in fields 60F, 90D, 90C, 62F, 64, and 65 must be the same for all occurrences of these fields"}
        ]}"#
    }

    /// Validate the message instance according to MT941 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // C1: Currency consistency validation
        // Extract currency from mandatory field 62F
        let base_currency = &self.field_62f.currency[0..2];

        // Check 60F if present
        if let Some(ref field_60f) = self.field_60f
            && &field_60f.currency[0..2] != base_currency
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "MT941: Currency code mismatch - field 60F currency '{}' does not match field 62F currency '{}'",
                    &field_60f.currency[0..2],
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
                    "MT941: Currency code mismatch - field 90D currency '{}' does not match field 62F currency '{}'",
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
                    "MT941: Currency code mismatch - field 90C currency '{}' does not match field 62F currency '{}'",
                    &field_90c.currency[0..2],
                    base_currency
                ),
            });
        }

        // Check 64 if present
        if let Some(ref field_64) = self.field_64
            && &field_64.currency[0..2] != base_currency
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "MT941: Currency code mismatch - field 64 currency '{}' does not match field 62F currency '{}'",
                    &field_64.currency[0..2],
                    base_currency
                ),
            });
        }

        // Check 65 if present
        if let Some(ref field_65_vec) = self.field_65 {
            for (idx, field_65) in field_65_vec.iter().enumerate() {
                if &field_65.currency[0..2] != base_currency {
                    return Err(crate::errors::ParseError::InvalidFormat {
                        message: format!(
                            "MT941: Currency code mismatch - field 65[{}] currency '{}' does not match field 62F currency '{}'",
                            idx,
                            &field_65.currency[0..2],
                            base_currency
                        ),
                    });
                }
            }
        }

        Ok(())
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
}
