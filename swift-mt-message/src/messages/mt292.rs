use crate::errors::{ParseError, ParseResult, ParserConfig};
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MT292 - Request for Cancellation
///
/// Used to request a cancellation of a previously sent SWIFT message.
/// Can be used for full or partial cancellation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT292 {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Field 21 - Related Reference (Mandatory)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Field 11S - MT and Date of the Original Message (Mandatory)
    #[serde(rename = "11S")]
    pub field_11s: Field11S,

    /// Field 79 - Narrative Description of Original Message (Conditional)
    /// Must be present if copy of original message fields is not included
    #[serde(rename = "79", skip_serializing_if = "Option::is_none")]
    pub field_79: Option<Field79>,

    /// Copy of mandatory fields from the original message (Conditional)
    /// Stored as additional fields that were part of the original message
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub original_fields: HashMap<String, serde_json::Value>,
}

impl MT292 {
    /// Parse MT292 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "292");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_11s = parser.parse_field::<Field11S>("11S")?;

        // Parse optional/conditional Field 79
        let field_79 = parser.parse_optional_field::<Field79>("79")?;

        // Collect any remaining fields as original message fields
        // This would need to be implemented in MessageParser but for now use empty HashMap
        let original_fields = HashMap::new();

        // Validation: Either Field 79 or original fields must be present
        if field_79.is_none() && original_fields.is_empty() {
            return Err(ParseError::InvalidFormat {
                message:
                    "MT292: Either Field 79 or copy of original message fields must be present"
                        .to_string(),
            });
        }

        Ok(MT292 {
            field_20,
            field_21,
            field_11s,
            field_79,
            original_fields,
        })
    }

    /// Static validation rules for MT292
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }
}

impl crate::traits::SwiftMessageBody for MT292 {
    fn message_type() -> &'static str {
        "292"
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

        // Note: Field order matters in SWIFT - 20 and 21 must come before 11S
        fields.insert("20".to_string(), vec![self.field_20.to_swift_string()]);
        fields.insert("21".to_string(), vec![self.field_21.to_swift_string()]);
        fields.insert("11S".to_string(), vec![self.field_11s.to_swift_string()]);

        if let Some(ref narrative) = self.field_79 {
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
        vec!["20", "21", "11S"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["79"]
    }

    fn to_ordered_fields(&self) -> Vec<(String, String)> {
        // MT292 has specific field order requirements:
        // Fields 20 and 21 must come before Field 11S
        let mut ordered_fields = Vec::new();
        let field_map = self.to_fields();

        // Add fields in the correct SWIFT order
        if let Some(values) = field_map.get("20") {
            for value in values {
                ordered_fields.push(("20".to_string(), value.clone()));
            }
        }

        if let Some(values) = field_map.get("21") {
            for value in values {
                ordered_fields.push(("21".to_string(), value.clone()));
            }
        }

        if let Some(values) = field_map.get("11S") {
            for value in values {
                ordered_fields.push(("11S".to_string(), value.clone()));
            }
        }

        if let Some(values) = field_map.get("79") {
            for value in values {
                ordered_fields.push(("79".to_string(), value.clone()));
            }
        }

        // Add any other fields (from original_fields) in numeric order
        let mut other_tags: Vec<String> = field_map
            .keys()
            .filter(|k| !["20", "21", "11S", "79"].contains(&k.as_str()))
            .cloned()
            .collect();
        other_tags.sort();

        for tag in other_tags {
            if let Some(values) = field_map.get(&tag) {
                for value in values {
                    ordered_fields.push((tag.clone(), value.clone()));
                }
            }
        }

        ordered_fields
    }
}
