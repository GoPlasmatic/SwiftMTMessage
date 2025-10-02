use crate::fields::*;
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
/// - **13D**: Date/Time Indication (mandatory) - Precise timing of report
/// - **60F**: Opening Balance (optional) - Starting balance for reporting period
/// - **Statement Lines**: Repetitive sequence of transaction details (Field 61 + optional Field 86)
/// - **90D**: Number/Sum of Debit Entries (optional) - Debit transaction summary
/// - **90C**: Number/Sum of Credit Entries (optional) - Credit transaction summary
/// - **62F**: Closing Balance (optional) - Ending balance for reporting period
/// - **64**: Available Balance (optional) - Available balance information
/// - **65**: Forward Available Balance (optional, repetitive) - Future balance projections
///
/// ## Network Validation Rules
/// - **Currency Consistency**: All balance and limit fields must use consistent currency
/// - **Entry Currency Consistency**: Entry summaries must use same currency as balances
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

    // Date/Time Indication (mandatory)
    #[serde(rename = "13D")]
    pub field_13d: Field13D,

    // Opening Balance (optional)
    #[serde(rename = "60F", skip_serializing_if = "Option::is_none")]
    pub field_60f: Option<Field60F>,

    // Statement Lines (repetitive)
    #[serde(rename = "statement_lines")]
    pub statement_lines: Vec<MT942StatementLine>,

    // Number and Sum of Debits (optional)
    #[serde(rename = "90D", skip_serializing_if = "Option::is_none")]
    pub field_90d: Option<Field90D>,

    // Number and Sum of Credits (optional)
    #[serde(rename = "90C", skip_serializing_if = "Option::is_none")]
    pub field_90c: Option<Field90C>,

    // Closing Balance (optional)
    #[serde(rename = "62F", skip_serializing_if = "Option::is_none")]
    pub field_62f: Option<Field62F>,

    // Closing Available Balance (optional)
    #[serde(rename = "64", skip_serializing_if = "Option::is_none")]
    pub field_64: Option<Field64>,

    // Forward Available Balance (optional, repetitive)
    #[serde(rename = "65", skip_serializing_if = "Option::is_none")]
    pub field_65: Option<Vec<Field65>>,
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

        // Parse Field 13D if not already parsed
        let field_13d = if let Some(early_13d) = field_13d_early {
            early_13d
        } else {
            parser.parse_field::<Field13D>("13D")?
        };

        // Parse optional opening balance
        let field_60f = parser.parse_optional_field::<Field60F>("60F")?;

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

        // Parse optional closing balance
        let field_62f = parser.parse_optional_field::<Field62F>("62F")?;

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

        Ok(MT942 {
            field_20,
            field_21,
            field_25,
            field_28c,
            field_13d,
            field_60f,
            statement_lines,
            field_90d,
            field_90c,
            field_62f,
            field_64,
            field_65,
        })
    }

    /// Static validation rules for MT942
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "C1", "description": "The first two characters of the three-character currency code in fields 60F, 61, 90D, 90C, 62F, 64, and 65 must be the same for all occurrences"}
        ]}"#
    }

    /// Validate the message instance according to MT942 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // C1: Currency consistency validation
        // We need at least one field with currency to validate against
        let base_currency = if let Some(ref field_60f) = self.field_60f {
            &field_60f.currency[0..2]
        } else if let Some(ref field_62f) = self.field_62f {
            &field_62f.currency[0..2]
        } else if let Some(ref field_90d) = self.field_90d {
            &field_90d.currency[0..2]
        } else if let Some(ref field_90c) = self.field_90c {
            &field_90c.currency[0..2]
        } else {
            // No currency fields to validate
            return Ok(());
        };

        // Check 60F if present
        if let Some(ref field_60f) = self.field_60f
            && &field_60f.currency[0..2] != base_currency
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "MT942: Currency code mismatch - field 60F currency '{}' does not match base currency '{}'",
                    &field_60f.currency[0..2],
                    base_currency
                ),
            });
        }

        // Check 62F if present
        if let Some(ref field_62f) = self.field_62f
            && &field_62f.currency[0..2] != base_currency
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "MT942: Currency code mismatch - field 62F currency '{}' does not match base currency '{}'",
                    &field_62f.currency[0..2],
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

        // Check 64 if present
        if let Some(ref field_64) = self.field_64
            && &field_64.currency[0..2] != base_currency
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "MT942: Currency code mismatch - field 64 currency '{}' does not match base currency '{}'",
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
                            "MT942: Currency code mismatch - field 65[{}] currency '{}' does not match base currency '{}'",
                            idx,
                            &field_65.currency[0..2],
                            base_currency
                        ),
                    });
                }
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

    fn from_fields(
        fields: std::collections::HashMap<String, Vec<(String, usize)>>,
    ) -> crate::SwiftResult<Self> {
        // Collect all fields with their positions
        let mut all_fields: Vec<(String, String, usize)> = Vec::new();
        for (tag, values) in fields {
            for (value, position) in values {
                all_fields.push((tag.clone(), value, position));
            }
        }

        // Sort by position to preserve field order
        all_fields.sort_by_key(|(_, _, pos)| *pos);

        // Reconstruct block4 in the correct order
        let mut block4 = String::new();
        for (tag, value, _) in all_fields {
            block4.push_str(&format!(":{}:{}\n", tag, value));
        }
        Self::parse_from_block4(&block4)
    }

    fn from_fields_with_config(
        fields: std::collections::HashMap<String, Vec<(String, usize)>>,
        _config: &crate::errors::ParserConfig,
    ) -> std::result::Result<crate::errors::ParseResult<Self>, crate::errors::ParseError> {
        match Self::from_fields(fields) {
            Ok(msg) => Ok(crate::errors::ParseResult::Success(msg)),
            Err(e) => Err(e),
        }
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        use crate::traits::SwiftField;
        let mut fields = std::collections::HashMap::new();

        // Add mandatory fields
        fields.insert("20".to_string(), vec![self.field_20.reference.clone()]);

        if let Some(ref field_21) = self.field_21 {
            fields.insert("21".to_string(), vec![field_21.reference.clone()]);
        }

        // Handle Field25AccountIdentification enum
        match &self.field_25 {
            Field25AccountIdentification::NoOption(field) => {
                fields.insert("25".to_string(), vec![field.authorisation.clone()]);
            }
            Field25AccountIdentification::P(field) => {
                fields.insert("25P".to_string(), vec![field.to_swift_string()]);
            }
        }

        fields.insert("28C".to_string(), vec![self.field_28c.to_swift_string()]);
        fields.insert("13D".to_string(), vec![self.field_13d.to_swift_string()]);

        if let Some(ref field_60f) = self.field_60f {
            fields.insert("60F".to_string(), vec![field_60f.to_swift_string()]);
        }

        // Add statement lines
        let mut field_61_values = Vec::new();
        let mut field_86_values = Vec::new();

        for line in &self.statement_lines {
            field_61_values.push(line.field_61.to_swift_string());
            if let Some(ref field_86) = line.field_86 {
                field_86_values.push(field_86.to_swift_string());
            }
        }

        if !field_61_values.is_empty() {
            fields.insert("61".to_string(), field_61_values);
        }
        if !field_86_values.is_empty() {
            fields.insert("86".to_string(), field_86_values);
        }

        if let Some(ref field_90d) = self.field_90d {
            fields.insert("90D".to_string(), vec![field_90d.to_swift_string()]);
        }

        if let Some(ref field_90c) = self.field_90c {
            fields.insert("90C".to_string(), vec![field_90c.to_swift_string()]);
        }

        if let Some(ref field_62f) = self.field_62f {
            fields.insert("62F".to_string(), vec![field_62f.to_swift_string()]);
        }

        if let Some(ref field_64) = self.field_64 {
            fields.insert("64".to_string(), vec![field_64.to_swift_string()]);
        }

        if let Some(ref field_65_vec) = self.field_65 {
            let values: Vec<String> = field_65_vec.iter().map(|f| f.to_swift_string()).collect();
            fields.insert("65".to_string(), values);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "25", "28C", "13D"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["21", "60F", "61", "86", "90D", "90C", "62F", "64", "65"]
    }
}
