use crate::errors::{ParseError, ParseResult, ParserConfig};
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MT299 - Free Format Message
///
/// Generic message format used to exchange information for which
/// no specific message type exists.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT299 {
    /// Field 20 - Transaction Reference (Mandatory)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Field 21 - Related Reference (Optional)
    #[serde(rename = "21", skip_serializing_if = "Option::is_none")]
    pub field_21: Option<Field21NoOption>,

    /// Field 79 - Narrative (Mandatory)
    #[serde(rename = "79")]
    pub field_79: Field79,
}

impl MT299 {
    /// Parse MT299 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "299");

        // Parse mandatory Field 20
        let field_20 = parser.parse_field::<Field20>("20")?;

        // Parse optional Field 21
        let field_21 = parser.parse_optional_field::<Field21NoOption>("21")?;

        // Parse mandatory Field 79
        let field_79 = parser.parse_field::<Field79>("79")?;

        Ok(MT299 {
            field_20,
            field_21,
            field_79,
        })
    }

    /// Static validation rules for MT299
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }
}

impl crate::traits::SwiftMessageBody for MT299 {
    fn message_type() -> &'static str {
        "299"
    }

    fn from_fields(fields: HashMap<String, Vec<(String, usize)>>) -> crate::SwiftResult<Self> {
        // Reconstruct block4 from fields
        let mut all_fields: Vec<(String, String, usize)> = Vec::new();
        for (tag, values) in fields {
            for (value, position) in values {
                all_fields.push((tag.clone(), value, position));
            }
        }

        // Sort by position
        all_fields.sort_by_key(|f| f.2);

        // Build block4
        let mut block4 = String::new();
        for (tag, value, _) in all_fields {
            block4.push_str(&format!(
                ":{}:{}
",
                tag, value
            ));
        }

        Self::parse_from_block4(&block4)
    }

    fn from_fields_with_config(
        fields: HashMap<String, Vec<(String, usize)>>,
        _config: &ParserConfig,
    ) -> Result<ParseResult<Self>, ParseError> {
        match Self::from_fields(fields) {
            Ok(msg) => Ok(ParseResult::Success(msg)),
            Err(e) => Err(e),
        }
    }

    fn to_fields(&self) -> HashMap<String, Vec<String>> {
        let mut fields = HashMap::new();

        fields.insert("20".to_string(), vec![self.field_20.to_swift_string()]);

        if let Some(ref related) = self.field_21 {
            fields.insert("21".to_string(), vec![related.to_swift_string()]);
        }

        fields.insert("79".to_string(), vec![self.field_79.to_swift_string()]);

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "79"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["21"]
    }
}
