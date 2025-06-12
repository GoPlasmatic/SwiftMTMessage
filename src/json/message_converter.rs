//! Message-level JSON conversion utilities
//!
//! This module handles the conversion of complete SWIFT messages to/from JSON format.

use crate::errors::{ParseError, Result};
use crate::field_parser::SwiftMessage;
use crate::json::field_converter::{extract_field_data, recreate_field_container};
use crate::json::traits::{FromJson, ToJson};
use crate::mt_models::mt103::MT103;
use crate::tokenizer::{ApplicationHeader, BasicHeader, SwiftMessageBlocks, Trailer, UserHeader};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

/// JSON representation of a SWIFT message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonMessage {
    /// Message type (e.g., "103", "202")
    pub message_type: String,

    /// Message header blocks (raw)
    pub blocks: JsonBlocks,

    /// Parsed header information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_header: Option<BasicHeader>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_header: Option<ApplicationHeader>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_header: Option<UserHeader>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailer_block: Option<Trailer>,

    /// Message fields with their values
    pub fields: Map<String, Value>,

    /// Original field order for faithful reconstruction
    pub field_order: Vec<String>,
}

/// JSON representation of SWIFT message blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonBlocks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block3: Option<String>,

    pub block4: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block5: Option<String>,
}

/// Optional metadata for enhanced JSON representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMetadata {
    /// Parsing timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Validation status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,

    /// Additional custom metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

/// Helper function to create a JSON error
fn json_error<T: std::fmt::Display>(context: &str, error: T) -> ParseError {
    ParseError::JsonError {
        message: format!("{}: {}", context, error),
    }
}

/// Convert fields map to JSON
fn convert_fields_to_json(
    fields: &HashMap<String, crate::field_parser::SwiftFieldContainer>,
) -> Result<Map<String, Value>> {
    let mut fields_map = Map::new();
    for (tag, field) in fields {
        let field_value = extract_field_data(field)?;
        fields_map.insert(tag.clone(), field_value);
    }
    Ok(fields_map)
}

/// Convert JSON fields back to SwiftFieldContainer map
fn convert_json_to_fields(
    json_fields: &Map<String, Value>,
) -> Result<HashMap<String, crate::field_parser::SwiftFieldContainer>> {
    let mut fields = HashMap::new();
    for (tag, field_value) in json_fields {
        let field_container = recreate_field_container(tag, field_value)?;
        fields.insert(tag.clone(), field_container);
    }
    Ok(fields)
}

/// Parse headers from blocks with fallback
fn parse_headers_with_fallback(
    blocks: &SwiftMessageBlocks,
    json_headers: (
        Option<BasicHeader>,
        Option<ApplicationHeader>,
        Option<UserHeader>,
        Option<Trailer>,
    ),
) -> (
    Option<BasicHeader>,
    Option<ApplicationHeader>,
    Option<UserHeader>,
    Option<Trailer>,
) {
    let (json_basic, json_app, json_user, json_trailer) = json_headers;

    let basic_header = json_basic.or_else(|| {
        blocks
            .block_1
            .as_ref()
            .and_then(|block1| crate::tokenizer::parse_basic_header(block1).ok())
    });

    let application_header = json_app.or_else(|| {
        blocks
            .block_2
            .as_ref()
            .and_then(|block2| crate::tokenizer::parse_application_header(block2).ok())
    });

    let user_header = json_user.or_else(|| {
        blocks
            .block_3
            .as_ref()
            .and_then(|block3| crate::tokenizer::parse_user_header(block3).ok())
    });

    let trailer_block = json_trailer.or_else(|| {
        blocks
            .block_5
            .as_ref()
            .and_then(|block5| crate::tokenizer::parse_trailer_block(block5).ok())
    });

    (basic_header, application_header, user_header, trailer_block)
}

impl ToJson for SwiftMessage {
    fn to_json(&self) -> Result<Value> {
        let fields_map = convert_fields_to_json(&self.fields)?;

        let json_message = JsonMessage {
            message_type: self.message_type.clone(),
            blocks: JsonBlocks {
                block1: self.blocks.block_1.clone(),
                block2: self.blocks.block_2.clone(),
                block3: self.blocks.block_3.clone(),
                block4: self.blocks.block_4.clone().unwrap_or_default(),
                block5: self.blocks.block_5.clone(),
            },
            fields: fields_map,
            field_order: self.field_order.clone(),
            basic_header: self.basic_header.clone(),
            application_header: self.application_header.clone(),
            user_header: self.user_header.clone(),
            trailer_block: self.trailer_block.clone(),
        };

        serde_json::to_value(json_message)
            .map_err(|e| json_error("Failed to convert SwiftMessage to JSON", e))
    }
}

impl FromJson<SwiftMessage> for SwiftMessage {
    fn from_json(json: &Value) -> Result<SwiftMessage> {
        let json_message: JsonMessage = serde_json::from_value(json.clone())
            .map_err(|e| json_error("Invalid JSON message format", e))?;

        let fields = convert_json_to_fields(&json_message.fields)?;

        let blocks = SwiftMessageBlocks {
            block_1: json_message.blocks.block1,
            block_2: json_message.blocks.block2,
            block_3: json_message.blocks.block3,
            block_4: Some(json_message.blocks.block4),
            block_5: json_message.blocks.block5,
        };

        let (basic_header, application_header, user_header, trailer_block) =
            parse_headers_with_fallback(
                &blocks,
                (
                    json_message.basic_header,
                    json_message.application_header,
                    json_message.user_header,
                    json_message.trailer_block,
                ),
            );

        Ok(SwiftMessage {
            message_type: json_message.message_type,
            basic_header,
            application_header,
            user_header,
            trailer_block,
            blocks,
            fields,
            field_order: json_message.field_order,
        })
    }
}

impl ToJson for MT103 {
    fn to_json(&self) -> Result<Value> {
        // Convert to SwiftMessage first, then to JSON
        let swift_message = self.to_swift_message();
        swift_message.to_json()
    }
}

impl FromJson<MT103> for MT103 {
    fn from_json(json: &Value) -> Result<MT103> {
        let swift_message = SwiftMessage::from_json(json)?;
        MT103::from_swift_message(swift_message)
    }
}

/// Convert SwiftMessage back to SWIFT format string
pub fn swift_message_to_swift_format(message: &SwiftMessage) -> Result<String> {
    let mut swift_content = String::new();

    // Helper function to add block if present
    let add_block = |content: &mut String, block_num: u8, block_data: &Option<String>| {
        if let Some(data) = block_data {
            content.push_str(&format!("{{{}: {}}}", block_num, data));
        }
    };

    // Add header blocks
    add_block(&mut swift_content, 1, &message.blocks.block_1);
    add_block(&mut swift_content, 2, &message.blocks.block_2);
    add_block(&mut swift_content, 3, &message.blocks.block_3);

    // Add Block 4 with fields
    swift_content.push_str("{4:\n");
    for tag in &message.field_order {
        if let Some(field) = message.fields.get(tag) {
            swift_content.push_str(&format!(":{}: {}\n", tag, field.to_swift_string()));
        }
    }
    swift_content.push_str("-}");

    // Add trailer block
    add_block(&mut swift_content, 5, &message.blocks.block_5);

    Ok(swift_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::traits::{FromJson, ToJson};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_swift_message_json_roundtrip() {
        let swift_text = r#"{1:F01DEUTDEFFAXXX0123456789}{2:I103CHASUS33AXXXU3003}{4:
:20:FT21234567890
:23B:CRED
:32A:210315EUR1234567,89
:50K:JOHN DOE
:59:JANE SMITH
:71A:OUR
-}"#;

        // Parse SWIFT -> JSON -> SWIFT
        let message = SwiftMessage::parse(swift_text).unwrap();
        let json_str = message.to_json_string().unwrap();
        let parsed_back = SwiftMessage::from_json_string(&json_str).unwrap();

        assert_eq!(message.message_type, parsed_back.message_type);
        assert_eq!(message.fields.len(), parsed_back.fields.len());
    }

    #[test]
    fn test_json_error_helper() {
        let error = json_error("test context", "test error");
        assert!(matches!(error, ParseError::JsonError { .. }));
    }

    #[test]
    fn test_swift_format_conversion() {
        let message = SwiftMessage {
            message_type: "103".to_string(),
            basic_header: None,
            application_header: None,
            user_header: None,
            trailer_block: None,
            blocks: SwiftMessageBlocks {
                block_1: Some("F01DEUTDEFFAXXX0123456789".to_string()),
                block_2: Some("I103CHASUS33AXXXU3003".to_string()),
                block_3: None,
                block_4: Some("test".to_string()),
                block_5: None,
            },
            fields: HashMap::new(),
            field_order: vec![],
        };

        let result = swift_message_to_swift_format(&message);
        assert!(result.is_ok());
        let swift_format = result.unwrap();
        assert!(swift_format.contains("{1: F01DEUTDEFFAXXX0123456789}"));
        assert!(swift_format.contains("{4:\n-}"));
    }
}
