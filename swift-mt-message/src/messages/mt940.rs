use crate::fields::*;
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

    // Forward Available Balance (optional)
    #[serde(rename = "65", skip_serializing_if = "Option::is_none")]
    pub field_65: Option<Field65>,
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

            statement_lines.push(MT940StatementLine {
                field_61,
                field_86,
            });
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
        let field_65 = parser.parse_optional_field::<Field65>("65")?;

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
                message: format!("MT940: Statement lines must occur 1-500 times, found {}", self.statement_lines.len()),
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

        fields.insert("25".to_string(), vec![self.field_25.authorisation.clone()]);
        fields.insert("28C".to_string(), vec![self.field_28c.to_swift_string()]);
        fields.insert("60F".to_string(), vec![self.field_60f.to_swift_string()]);

        // Add statement lines
        let mut field_61_values = Vec::new();
        let mut field_86_values = Vec::new();

        for line in &self.statement_lines {
            field_61_values.push(line.field_61.to_swift_string());
            if let Some(ref field_86) = line.field_86 {
                field_86_values.push(field_86.to_swift_string());
            }
        }

        fields.insert("61".to_string(), field_61_values);
        if !field_86_values.is_empty() {
            fields.insert("86".to_string(), field_86_values);
        }

        // Add closing balance
        fields.insert("62F".to_string(), vec![self.field_62f.to_swift_string()]);

        // Add optional fields
        if let Some(ref field_64) = self.field_64 {
            fields.insert("64".to_string(), vec![field_64.to_swift_string()]);
        }

        if let Some(ref field_65) = self.field_65 {
            fields.insert("65".to_string(), vec![field_65.to_swift_string()]);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "25", "28C", "60F", "61", "62F"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["21", "86", "64", "65"]
    }
}