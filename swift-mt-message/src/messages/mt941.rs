use crate::fields::*;
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
        if let Some(ref field_60f) = self.field_60f {
            if &field_60f.currency[0..2] != base_currency {
                return Err(crate::errors::ParseError::InvalidFormat {
                    message: format!(
                        "MT941: Currency code mismatch - field 60F currency '{}' does not match field 62F currency '{}'",
                        &field_60f.currency[0..2],
                        base_currency
                    ),
                });
            }
        }

        // Check 90D if present
        if let Some(ref field_90d) = self.field_90d {
            if &field_90d.currency[0..2] != base_currency {
                return Err(crate::errors::ParseError::InvalidFormat {
                    message: format!(
                        "MT941: Currency code mismatch - field 90D currency '{}' does not match field 62F currency '{}'",
                        &field_90d.currency[0..2],
                        base_currency
                    ),
                });
            }
        }

        // Check 90C if present
        if let Some(ref field_90c) = self.field_90c {
            if &field_90c.currency[0..2] != base_currency {
                return Err(crate::errors::ParseError::InvalidFormat {
                    message: format!(
                        "MT941: Currency code mismatch - field 90C currency '{}' does not match field 62F currency '{}'",
                        &field_90c.currency[0..2],
                        base_currency
                    ),
                });
            }
        }

        // Check 64 if present
        if let Some(ref field_64) = self.field_64 {
            if &field_64.currency[0..2] != base_currency {
                return Err(crate::errors::ParseError::InvalidFormat {
                    message: format!(
                        "MT941: Currency code mismatch - field 64 currency '{}' does not match field 62F currency '{}'",
                        &field_64.currency[0..2],
                        base_currency
                    ),
                });
            }
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

        fields.insert("28".to_string(), vec![self.field_28.to_swift_string()]);

        if let Some(ref field_13d) = self.field_13d {
            fields.insert("13D".to_string(), vec![field_13d.to_swift_string()]);
        }

        if let Some(ref field_60f) = self.field_60f {
            fields.insert("60F".to_string(), vec![field_60f.to_swift_string()]);
        }

        if let Some(ref field_90d) = self.field_90d {
            fields.insert("90D".to_string(), vec![field_90d.to_swift_string()]);
        }

        if let Some(ref field_90c) = self.field_90c {
            fields.insert("90C".to_string(), vec![field_90c.to_swift_string()]);
        }

        fields.insert("62F".to_string(), vec![self.field_62f.to_swift_string()]);

        if let Some(ref field_64) = self.field_64 {
            fields.insert("64".to_string(), vec![field_64.to_swift_string()]);
        }

        if let Some(ref field_65_vec) = self.field_65 {
            let values: Vec<String> = field_65_vec.iter().map(|f| f.to_swift_string()).collect();
            fields.insert("65".to_string(), values);
        }

        if let Some(ref field_86) = self.field_86 {
            fields.insert("86".to_string(), vec![field_86.to_swift_string()]);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "25", "28", "62F"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["21", "13D", "60F", "90D", "90C", "64", "65", "86"]
    }
}
