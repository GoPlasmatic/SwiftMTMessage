use crate::errors::{ParseError, ParseResult, ParserConfig};
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MT296 - Answers
///
/// Used to respond to MT295 (Queries) or MT292 (Request for Cancellation)
/// or any message without a dedicated response type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT296 {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub transaction_reference: Field20,

    /// Field 21 - Related Reference (Mandatory)
    /// Reference of the original message being responded to
    #[serde(rename = "21")]
    pub related_reference: Field21NoOption,

    /// Field 76 - Answers (Mandatory)
    #[serde(rename = "76")]
    pub answers: Field76,

    /// Field 77A - Narrative (Optional)
    #[serde(rename = "77A", skip_serializing_if = "Option::is_none")]
    pub narrative: Option<Field77A>,

    /// Field 11R - MT and Date of the Original Message - Received (Optional)
    #[serde(rename = "11R", skip_serializing_if = "Option::is_none")]
    pub original_message_type_r: Option<Field11R>,

    /// Field 11S - MT and Date of the Original Message - Sent (Optional)
    #[serde(rename = "11S", skip_serializing_if = "Option::is_none")]
    pub original_message_type_s: Option<Field11S>,

    /// Field 79 - Narrative Description of Original Message (Conditional)
    #[serde(rename = "79", skip_serializing_if = "Option::is_none")]
    pub narrative_description: Option<Field79>,

    /// Copy of mandatory fields from the original message (Conditional)
    /// Stored as additional fields that were part of the original message
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub original_fields: HashMap<String, serde_json::Value>,
}

impl MT296 {
    /// Parse MT296 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "296");

        // Parse mandatory fields
        let transaction_reference = parser.parse_field::<Field20>("20")?;
        let related_reference = parser.parse_field::<Field21NoOption>("21")?;
        let answers = parser.parse_field::<Field76>("76")?;

        // Parse optional Field 77A
        let narrative = parser.parse_optional_field::<Field77A>("77A")?;

        // Parse optional Field 11R or 11S
        let original_message_type_r = parser.parse_optional_field::<Field11R>("11R")?;
        let original_message_type_s = parser.parse_optional_field::<Field11S>("11S")?;

        // Parse optional/conditional Field 79
        let narrative_description = parser.parse_optional_field::<Field79>("79")?;

        // Collect any remaining fields as original message fields
        // This would need to be implemented in MessageParser but for now use empty HashMap
        let original_fields = HashMap::new();

        // Validation: Only one of Field 79 or original fields should be present (C1)
        if narrative_description.is_some() && !original_fields.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "MT296: Only one of Field 79 or copy of original message fields should be present (C1)".to_string(),
            });
        }

        Ok(MT296 {
            transaction_reference,
            related_reference,
            answers,
            narrative,
            original_message_type_r,
            original_message_type_s,
            narrative_description,
            original_fields,
        })
    }

    /// Static validation rules for MT296
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }
}

impl crate::traits::SwiftMessageBody for MT296 {
    fn message_type() -> &'static str {
        "296"
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
            block4.push_str(&format!(":{}:{}
", tag, value));
        }

        Self::parse_from_block4(&block4)
    }

    fn from_fields_with_config(
        fields: HashMap<String, Vec<(String, usize)>>,
        _config: &ParserConfig,
    ) -> Result<ParseResult<Self>, ParseError> {
        match Self::from_fields(fields) {
            Ok(msg) => Ok(ParseResult::Success(msg)),
            Err(e) => Err(e)
        }
    }

    fn to_fields(&self) -> HashMap<String, Vec<String>> {
        let mut fields = HashMap::new();

        fields.insert("20".to_string(), vec![self.transaction_reference.to_swift_string()]);
        fields.insert("21".to_string(), vec![self.related_reference.to_swift_string()]);
        fields.insert("76".to_string(), vec![self.answers.to_swift_string()]);

        if let Some(ref narr) = self.narrative {
            fields.insert("77A".to_string(), vec![narr.to_swift_string()]);
        }

        if let Some(ref orig_msg_r) = self.original_message_type_r {
            fields.insert("11R".to_string(), vec![orig_msg_r.to_swift_string()]);
        }

        if let Some(ref orig_msg_s) = self.original_message_type_s {
            fields.insert("11S".to_string(), vec![orig_msg_s.to_swift_string()]);
        }

        if let Some(ref narrative) = self.narrative_description {
            fields.insert("79".to_string(), vec![narrative.to_swift_string()]);
        }

        // Add original message fields
        for (key, value) in &self.original_fields {
            if let Some(str_val) = value.as_str() {
                fields.insert(key.clone(), vec![str_val.to_string()]);
            } else if let Some(arr_val) = value.as_array() {
                let str_vals: Vec<String> = arr_val
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                if !str_vals.is_empty() {
                    fields.insert(key.clone(), str_vals);
                }
            }
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