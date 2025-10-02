use crate::fields::*;
use serde::{Deserialize, Serialize};

// MT196: Answers
// Used to provide comprehensive answers and responses to various queries and
// requests related to customer payments and transactions.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT196 {
    // Sender's Reference
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Related Reference
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    // Answers (mandatory)
    #[serde(rename = "76")]
    pub field_76: Field76,

    // Proprietary Message (optional)
    #[serde(rename = "77A", skip_serializing_if = "Option::is_none")]
    pub field_77a: Option<Field77A>,

    // Message Type and Date (optional)
    #[serde(rename = "11", skip_serializing_if = "Option::is_none")]
    pub field_11: Option<Field11>,

    // Narrative (optional)
    #[serde(rename = "79", skip_serializing_if = "Option::is_none")]
    pub field_79: Option<Field79>,
}

impl MT196 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "196");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_76 = parser.parse_field::<Field76>("76")?;

        // Parse optional fields
        let field_77a = parser.parse_optional_field::<Field77A>("77A")?;
        let field_11 = parser.parse_optional_field::<Field11>("11")?;
        let field_79 = parser.parse_optional_field::<Field79>("79")?;

        Ok(MT196 {
            field_20,
            field_21,
            field_76,
            field_77a,
            field_11,
            field_79,
        })
    }

    /// Static validation rules for MT196
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "F20", "description": "Field 20 must not start or end with '/', and must not contain '//'"},
            {"id": "F21", "description": "Field 21 must not start or end with '/', and must not contain '//'"},
            {"id": "F76", "description": "Field 76 must contain at least one line of answer information"},
            {"id": "C1", "description": "Only one of the following may be present: Field 79, or a copy of mandatory fields of the original message"}
        ]}"#
    }

    /// Validate the message instance according to MT196 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // Validate Field 20 - must not start/end with '/' or contain '//'
        let reference = &self.field_20.reference;
        if reference.starts_with('/') || reference.ends_with('/') || reference.contains("//") {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT196: Field 20 must not start or end with '/', and must not contain '//'".to_string(),
            });
        }

        // Validate Field 21 - same rules as Field 20
        let related_ref = &self.field_21.reference;
        if related_ref.starts_with('/') || related_ref.ends_with('/') || related_ref.contains("//") {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT196: Field 21 must not start or end with '/', and must not contain '//'".to_string(),
            });
        }

        // Validate Field 76 has content
        if self.field_76.information.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT196: Field 76 must contain at least one line of answer information".to_string(),
            });
        }

        Ok(())
    }
}

// Implement the SwiftMessageBody trait for MT196
impl crate::traits::SwiftMessageBody for MT196 {
    fn message_type() -> &'static str {
        "196"
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
        use chrono::Datelike;
        let mut fields = std::collections::HashMap::new();

        // Add mandatory fields
        fields.insert("20".to_string(), vec![self.field_20.reference.clone()]);
        fields.insert("21".to_string(), vec![self.field_21.reference.clone()]);
        fields.insert("76".to_string(), vec![self.field_76.information.join("\n")]);

        // Add optional fields
        if let Some(ref field_77a) = self.field_77a {
            fields.insert("77A".to_string(), vec![field_77a.narrative.join("\n")]);
        }

        if let Some(ref field_11) = self.field_11 {
            let field_11_value = format!("{}{:02}{:02}{:02}",
                field_11.message_type,
                field_11.date.year() % 100,
                field_11.date.month(),
                field_11.date.day()
            );
            fields.insert("11".to_string(), vec![field_11_value]);
        }

        if let Some(ref field_79) = self.field_79 {
            fields.insert("79".to_string(), vec![field_79.information.join("\n")]);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "21", "76"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["77A", "11", "79"]
    }
}