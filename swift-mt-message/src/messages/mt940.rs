use crate::fields::*;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};

// MT940: Customer Statement Message
// Used to transmit detailed account statement information to the account owner,
// showing all debits and credits for a specific period.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT940 {
    // Transaction Reference Number
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Related Reference (optional)
    #[serde(rename = "21", skip_serializing_if = "Option::is_none")]
    pub field_21: Option<Field21NoOption>,

    // Account Identification
    #[serde(rename = "25")]
    pub field_25: Field25NoOption,

    // Statement Number/Sequence Number
    #[serde(rename = "28C")]
    pub field_28c: Field28C,

    // Opening Balance
    #[serde(rename = "60F")]
    pub field_60f: Field60F,

    // Statement Lines (1-500 occurrences)
    #[serde(rename = "statement_lines")]
    pub statement_lines: Vec<MT940StatementLine>,

    // Closing Balance
    #[serde(rename = "62F")]
    pub field_62f: Field62F,

    // Available Balance (optional)
    #[serde(rename = "64", skip_serializing_if = "Option::is_none")]
    pub field_64: Option<Field64>,

    // Forward Available Balance (optional, repetitive)
    #[serde(rename = "65", skip_serializing_if = "Option::is_none")]
    pub field_65: Option<Vec<Field65>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT940StatementLine {
    // Statement Line
    #[serde(rename = "61")]
    pub field_61: Field61,

    // Information to Account Owner (optional)
    #[serde(rename = "86", skip_serializing_if = "Option::is_none")]
    pub field_86: Option<Field86>,
}

impl MT940 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "940");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_optional_field::<Field21NoOption>("21")?;
        let field_25 = parser.parse_field::<Field25NoOption>("25")?;
        let field_28c = parser.parse_field::<Field28C>("28C")?;
        let field_60f = parser.parse_field::<Field60F>("60F")?;

        // Enable duplicate field handling for statement lines
        parser = parser.with_duplicates(true);

        // Parse statement lines (1-500)
        let mut statement_lines = Vec::new();

        while parser.detect_field("61") && statement_lines.len() < 500 {
            let field_61 = parser.parse_field::<Field61>("61")?;
            let field_86 = parser.parse_optional_field::<Field86>("86")?;

            statement_lines.push(MT940StatementLine { field_61, field_86 });
        }

        // Must have at least one statement line
        if statement_lines.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT940: At least one statement line (field 61) is required".to_string(),
            });
        }

        // Parse mandatory closing balance
        let field_62f = parser.parse_field::<Field62F>("62F")?;

        // Parse optional fields
        let field_64 = parser.parse_optional_field::<Field64>("64")?;

        // Parse optional repetitive Field 65 (Forward Available Balance)
        let mut forward_balances = Vec::new();
        while let Ok(field_65) = parser.parse_field::<Field65>("65") {
            forward_balances.push(field_65);
        }
        let field_65 = if forward_balances.is_empty() {
            None
        } else {
            Some(forward_balances)
        };

        Ok(MT940 {
            field_20,
            field_21,
            field_25,
            field_28c,
            field_60f,
            statement_lines,
            field_62f,
            field_64,
            field_65,
        })
    }

    /// Static validation rules for MT940
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "C1", "description": "The repetitive sequence starting with field 61 must appear at least once and no more than 500 times"},
            {"id": "C2", "description": "If field 64 is present, fields 60F and 62F must also be present"}
        ]}"#
    }

    /// Validate the message instance according to MT940 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // C1: Statement lines must occur 1-500 times
        if self.statement_lines.is_empty() || self.statement_lines.len() > 500 {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "MT940: Statement lines must occur 1-500 times, found {}",
                    self.statement_lines.len()
                ),
            });
        }

        // C2 is automatically satisfied as fields 60F and 62F are mandatory

        Ok(())
    }
}

// Implement the SwiftMessageBody trait for MT940
impl crate::traits::SwiftMessageBody for MT940 {
    fn message_type() -> &'static str {
        "940"
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
        append_field(&mut result, &self.field_60f);

        // Statement lines
        for statement_line in &self.statement_lines {
            append_field(&mut result, &statement_line.field_61);
            append_optional_field(&mut result, &statement_line.field_86);
        }

        append_field(&mut result, &self.field_62f);
        append_optional_field(&mut result, &self.field_64);
        append_vec_field(&mut result, &self.field_65);

        finalize_mt_string(result, false)
    }
}
