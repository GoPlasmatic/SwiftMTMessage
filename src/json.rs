//! JSON conversion utilities for SWIFT MT messages
//!
//! This module provides bidirectional conversion between SWIFT MT messages
//! and JSON format, supporting both message-level and field-level serialization.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::errors::{ParseError, Result};
use crate::field_parser::{SwiftFieldContainer, SwiftMessage};
use crate::mt_models::fields::beneficiary::Field59;
use crate::mt_models::fields::institutions::{
    Field52, Field53, Field54, Field55, Field56, Field57,
};
use crate::mt_models::fields::ordering_customer::Field50;
use crate::mt_models::mt103::MT103;
use crate::tokenizer::{SwiftMessageBlocks, BasicHeader, ApplicationHeader, UserHeader, Trailer};

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

/// Convert SWIFT message to JSON
pub trait ToJson {
    /// Convert to JSON Value
    fn to_json(&self) -> Result<Value>;

    /// Convert to JSON string
    fn to_json_string(&self) -> Result<String> {
        let json_value = self.to_json()?;
        serde_json::to_string_pretty(&json_value).map_err(|e| ParseError::JsonError {
            message: format!("Failed to serialize to JSON: {}", e),
        })
    }

    /// Convert to compact JSON string
    fn to_json_compact(&self) -> Result<String> {
        let json_value = self.to_json()?;
        serde_json::to_string(&json_value).map_err(|e| ParseError::JsonError {
            message: format!("Failed to serialize to JSON: {}", e),
        })
    }
}

/// Convert from JSON to SWIFT message
pub trait FromJson<T> {
    /// Parse from JSON Value
    fn from_json(json: &Value) -> Result<T>;

    /// Parse from JSON string
    fn from_json_string(json_str: &str) -> Result<T> {
        let json_value: Value =
            serde_json::from_str(json_str).map_err(|e| ParseError::JsonError {
                message: format!("Invalid JSON: {}", e),
            })?;
        Self::from_json(&json_value)
    }
}

impl ToJson for SwiftMessage {
    fn to_json(&self) -> Result<Value> {
        let mut fields_map = Map::new();

        // Convert each field to JSON based on its type
        for (tag, field) in &self.fields {
            // Extract the inner field data without the container wrapper
            let field_value = extract_field_data(field)?;
            fields_map.insert(tag.clone(), field_value);
        }

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

        serde_json::to_value(json_message).map_err(|e| ParseError::JsonError {
            message: format!("Failed to convert to JSON: {}", e),
        })
    }
}

impl FromJson<SwiftMessage> for SwiftMessage {
    fn from_json(json: &Value) -> Result<SwiftMessage> {
        let json_message: JsonMessage =
            serde_json::from_value(json.clone()).map_err(|e| ParseError::JsonError {
                message: format!("Invalid JSON message format: {}", e),
            })?;

        let mut fields = HashMap::new();

        // Convert JSON fields back to SwiftFieldContainer
        for (tag, field_value) in &json_message.fields {
            // Recreate the field container from the simplified structure
            let field_container = recreate_field_container(tag, field_value)?;
            fields.insert(tag.clone(), field_container);
        }

        let blocks = SwiftMessageBlocks {
            block_1: json_message.blocks.block1,
            block_2: json_message.blocks.block2,
            block_3: json_message.blocks.block3,
            block_4: Some(json_message.blocks.block4),
            block_5: json_message.blocks.block5,
        };

        // Use headers from JSON if available, otherwise parse from blocks
        let basic_header = json_message.basic_header.or_else(|| {
            blocks.block_1.as_ref()
                .and_then(|block1| crate::tokenizer::parse_basic_header(block1).ok())
        });

        let application_header = json_message.application_header.or_else(|| {
            blocks.block_2.as_ref()
                .and_then(|block2| crate::tokenizer::parse_application_header(block2).ok())
        });

        let user_header = json_message.user_header.or_else(|| {
            blocks.block_3.as_ref()
                .and_then(|block3| crate::tokenizer::parse_user_header(block3).ok())
        });

        let trailer_block = json_message.trailer_block.or_else(|| {
            blocks.block_5.as_ref()
                .and_then(|block5| crate::tokenizer::parse_trailer_block(block5).ok())
        });

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

/// Extract the inner field data without the SwiftFieldContainer wrapper
fn extract_field_data(field: &SwiftFieldContainer) -> Result<Value> {
    let result = match field {
        SwiftFieldContainer::Field20(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field23B(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field32A(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field50(f) => return extract_field50_data(f),
        SwiftFieldContainer::Field59(f) => return extract_field59_data(f),
        SwiftFieldContainer::Field71A(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field13C(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field23E(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field26T(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field33B(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field36(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field51A(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field52(f) => return extract_field52_data(f),
        SwiftFieldContainer::Field53(f) => return extract_field53_data(f),
        SwiftFieldContainer::Field54(f) => return extract_field54_data(f),
        SwiftFieldContainer::Field55(f) => return extract_field55_data(f),
        SwiftFieldContainer::Field56(f) => return extract_field56_data(f),
        SwiftFieldContainer::Field57(f) => return extract_field57_data(f),
        SwiftFieldContainer::Field70(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field71F(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field71G(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field72(f) => serde_json::to_value(f),
        SwiftFieldContainer::Field77B(f) => serde_json::to_value(f),
        SwiftFieldContainer::Unknown(f) => {
            let mut map = Map::new();
            map.insert("tag".to_string(), Value::String(f.tag.clone()));
            map.insert("content".to_string(), Value::String(f.content.clone()));
            return Ok(Value::Object(map));
        }
    };

    result.map_err(|e| ParseError::JsonError {
        message: format!("Failed to serialize field: {}", e),
    })
}

/// Extract Field50 data and flatten enum structure
fn extract_field50_data(field: &Field50) -> Result<Value> {
    match field {
        Field50::A(f) => serde_json::to_value(f),
        Field50::F(f) => serde_json::to_value(f),
        Field50::K(f) => serde_json::to_value(f),
    }
    .map_err(|e| ParseError::JsonError {
        message: format!("Failed to serialize Field50: {}", e),
    })
}

/// Extract Field59 data and flatten enum structure  
fn extract_field59_data(field: &Field59) -> Result<Value> {
    match field {
        Field59::A(f) => serde_json::to_value(f),
        Field59::NoOption(f) => serde_json::to_value(f),
    }
    .map_err(|e| ParseError::JsonError {
        message: format!("Failed to serialize Field59: {}", e),
    })
}

/// Extract Field52 data and flatten enum structure
fn extract_field52_data(field: &Field52) -> Result<Value> {
    match field {
        Field52::A(f) => serde_json::to_value(f),
        Field52::D(f) => serde_json::to_value(f),
    }
    .map_err(|e| ParseError::JsonError {
        message: format!("Failed to serialize Field52: {}", e),
    })
}

/// Extract Field53 data and flatten enum structure
fn extract_field53_data(field: &Field53) -> Result<Value> {
    match field {
        Field53::A(f) => serde_json::to_value(f),
        Field53::B(f) => serde_json::to_value(f),
        Field53::D(f) => serde_json::to_value(f),
    }
    .map_err(|e| ParseError::JsonError {
        message: format!("Failed to serialize Field53: {}", e),
    })
}

/// Extract Field54 data and flatten enum structure
fn extract_field54_data(field: &Field54) -> Result<Value> {
    match field {
        Field54::A(f) => serde_json::to_value(f),
        Field54::B(f) => serde_json::to_value(f),
        Field54::D(f) => serde_json::to_value(f),
    }
    .map_err(|e| ParseError::JsonError {
        message: format!("Failed to serialize Field54: {}", e),
    })
}

/// Extract Field55 data and flatten enum structure
fn extract_field55_data(field: &Field55) -> Result<Value> {
    match field {
        Field55::A(f) => serde_json::to_value(f),
        Field55::B(f) => serde_json::to_value(f),
        Field55::D(f) => serde_json::to_value(f),
    }
    .map_err(|e| ParseError::JsonError {
        message: format!("Failed to serialize Field55: {}", e),
    })
}

/// Extract Field56 data and flatten enum structure
fn extract_field56_data(field: &Field56) -> Result<Value> {
    match field {
        Field56::A(f) => serde_json::to_value(f),
        Field56::C(f) => serde_json::to_value(f),
        Field56::D(f) => serde_json::to_value(f),
    }
    .map_err(|e| ParseError::JsonError {
        message: format!("Failed to serialize Field56: {}", e),
    })
}

/// Extract Field57 data and flatten enum structure
fn extract_field57_data(field: &Field57) -> Result<Value> {
    match field {
        Field57::A(f) => serde_json::to_value(f),
        Field57::B(f) => serde_json::to_value(f),
        Field57::C(f) => serde_json::to_value(f),
        Field57::D(f) => serde_json::to_value(f),
    }
    .map_err(|e| ParseError::JsonError {
        message: format!("Failed to serialize Field57: {}", e),
    })
}

/// Recreate Field50 from flattened JSON structure
fn recreate_field50(tag: &str, value: &Value) -> Result<Field50> {
    use crate::mt_models::fields::ordering_customer::{Field50A, Field50F, Field50K};

    match tag {
        "50A" => {
            let field: Field50A =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field50A: {}", e),
                })?;
            Ok(Field50::A(field))
        }
        "50F" => {
            let field: Field50F =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field50F: {}", e),
                })?;
            Ok(Field50::F(field))
        }
        "50K" => {
            let field: Field50K =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field50K: {}", e),
                })?;
            Ok(Field50::K(field))
        }
        _ => Err(ParseError::JsonError {
            message: format!("Unknown Field50 variant: {}", tag),
        }),
    }
}

/// Recreate Field52 from flattened JSON structure
fn recreate_field52(tag: &str, value: &Value) -> Result<Field52> {
    use crate::mt_models::fields::institutions::{Field52A, Field52D};

    match tag {
        "52A" => {
            let field: Field52A =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field52A: {}", e),
                })?;
            Ok(Field52::A(field))
        }
        "52D" => {
            let field: Field52D =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field52D: {}", e),
                })?;
            Ok(Field52::D(field))
        }
        _ => Err(ParseError::JsonError {
            message: format!("Unknown Field52 variant: {}", tag),
        }),
    }
}

/// Recreate Field53 from flattened JSON structure
fn recreate_field53(tag: &str, value: &Value) -> Result<Field53> {
    use crate::mt_models::fields::institutions::{Field53A, Field53B, Field53D};

    match tag {
        "53A" => {
            let field: Field53A =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field53A: {}", e),
                })?;
            Ok(Field53::A(field))
        }
        "53B" => {
            let field: Field53B =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field53B: {}", e),
                })?;
            Ok(Field53::B(field))
        }
        "53D" => {
            let field: Field53D =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field53D: {}", e),
                })?;
            Ok(Field53::D(field))
        }
        _ => Err(ParseError::JsonError {
            message: format!("Unknown Field53 variant: {}", tag),
        }),
    }
}

/// Recreate Field54 from flattened JSON structure
fn recreate_field54(tag: &str, value: &Value) -> Result<Field54> {
    use crate::mt_models::fields::institutions::{Field54A, Field54B, Field54D};

    match tag {
        "54A" => {
            let field: Field54A =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field54A: {}", e),
                })?;
            Ok(Field54::A(field))
        }
        "54B" => {
            let field: Field54B =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field54B: {}", e),
                })?;
            Ok(Field54::B(field))
        }
        "54D" => {
            let field: Field54D =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field54D: {}", e),
                })?;
            Ok(Field54::D(field))
        }
        _ => Err(ParseError::JsonError {
            message: format!("Unknown Field54 variant: {}", tag),
        }),
    }
}

/// Recreate Field55 from flattened JSON structure
fn recreate_field55(tag: &str, value: &Value) -> Result<Field55> {
    use crate::mt_models::fields::institutions::{Field55A, Field55B, Field55D};

    match tag {
        "55A" => {
            let field: Field55A =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field55A: {}", e),
                })?;
            Ok(Field55::A(field))
        }
        "55B" => {
            let field: Field55B =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field55B: {}", e),
                })?;
            Ok(Field55::B(field))
        }
        "55D" => {
            let field: Field55D =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field55D: {}", e),
                })?;
            Ok(Field55::D(field))
        }
        _ => Err(ParseError::JsonError {
            message: format!("Unknown Field55 variant: {}", tag),
        }),
    }
}

/// Recreate Field56 from flattened JSON structure
fn recreate_field56(tag: &str, value: &Value) -> Result<Field56> {
    use crate::mt_models::fields::institutions::{Field56A, Field56C, Field56D};

    match tag {
        "56A" => {
            let field: Field56A =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field56A: {}", e),
                })?;
            Ok(Field56::A(field))
        }
        "56C" => {
            let field: Field56C =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field56C: {}", e),
                })?;
            Ok(Field56::C(field))
        }
        "56D" => {
            let field: Field56D =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field56D: {}", e),
                })?;
            Ok(Field56::D(field))
        }
        _ => Err(ParseError::JsonError {
            message: format!("Unknown Field56 variant: {}", tag),
        }),
    }
}

/// Recreate Field57 from flattened JSON structure
fn recreate_field57(tag: &str, value: &Value) -> Result<Field57> {
    use crate::mt_models::fields::institutions::{Field57A, Field57B, Field57C, Field57D};

    match tag {
        "57A" => {
            let field: Field57A =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field57A: {}", e),
                })?;
            Ok(Field57::A(field))
        }
        "57B" => {
            let field: Field57B =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field57B: {}", e),
                })?;
            Ok(Field57::B(field))
        }
        "57C" => {
            let field: Field57C =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field57C: {}", e),
                })?;
            Ok(Field57::C(field))
        }
        "57D" => {
            let field: Field57D =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field57D: {}", e),
                })?;
            Ok(Field57::D(field))
        }
        _ => Err(ParseError::JsonError {
            message: format!("Unknown Field57 variant: {}", tag),
        }),
    }
}

/// Recreate Field59 from flattened JSON structure
fn recreate_field59(tag: &str, value: &Value) -> Result<Field59> {
    use crate::mt_models::fields::beneficiary::{Field59A, Field59Basic};

    match tag {
        "59A" => {
            let field: Field59A =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field59A: {}", e),
                })?;
            Ok(Field59::A(field))
        }
        "59" => {
            let field: Field59Basic =
                serde_json::from_value(value.clone()).map_err(|e| ParseError::JsonError {
                    message: format!("Failed to parse Field59: {}", e),
                })?;
            Ok(Field59::NoOption(field))
        }
        _ => Err(ParseError::JsonError {
            message: format!("Unknown Field59 variant: {}", tag),
        }),
    }
}

/// Recreate a SwiftFieldContainer from simplified JSON structure
fn recreate_field_container(tag: &str, value: &Value) -> Result<SwiftFieldContainer> {
    // For unknown fields, handle special case
    if value.is_object() && value.get("tag").is_some() && value.get("content").is_some() {
        if let Some(obj) = value.as_object() {
            if let (Some(tag_val), Some(content_val)) = (obj.get("tag"), obj.get("content")) {
                if let (Some(tag_str), Some(content_str)) = (tag_val.as_str(), content_val.as_str())
                {
                    return Ok(SwiftFieldContainer::Unknown(
                        crate::field_parser::UnknownField {
                            tag: tag_str.to_string(),
                            content: content_str.to_string(),
                        },
                    ));
                }
            }
        }
    }

    // Convert simplified JSON back to appropriate field type
    match tag {
        "13C" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field13C(field))
        }
        "20" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field20(field))
        }
        "23B" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field23B(field))
        }
        "23E" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field23E(field))
        }
        "26T" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field26T(field))
        }
        "32A" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field32A(field))
        }
        "33B" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field33B(field))
        }
        "36" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field36(field))
        }
        tag if tag.starts_with("50") => {
            let field = recreate_field50(tag, value)?;
            Ok(SwiftFieldContainer::Field50(field))
        }
        "51A" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field51A(field))
        }
        tag if tag.starts_with("52") => {
            let field = recreate_field52(tag, value)?;
            Ok(SwiftFieldContainer::Field52(field))
        }
        tag if tag.starts_with("53") => {
            let field = recreate_field53(tag, value)?;
            Ok(SwiftFieldContainer::Field53(field))
        }
        tag if tag.starts_with("54") => {
            let field = recreate_field54(tag, value)?;
            Ok(SwiftFieldContainer::Field54(field))
        }
        tag if tag.starts_with("55") => {
            let field = recreate_field55(tag, value)?;
            Ok(SwiftFieldContainer::Field55(field))
        }
        tag if tag.starts_with("56") => {
            let field = recreate_field56(tag, value)?;
            Ok(SwiftFieldContainer::Field56(field))
        }
        tag if tag.starts_with("57") => {
            let field = recreate_field57(tag, value)?;
            Ok(SwiftFieldContainer::Field57(field))
        }
        tag if tag.starts_with("59") => {
            let field = recreate_field59(tag, value)?;
            Ok(SwiftFieldContainer::Field59(field))
        }
        "70" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field70(field))
        }
        "71A" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field71A(field))
        }
        "71F" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field71F(field))
        }
        "71G" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field71G(field))
        }
        "72" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field72(field))
        }
        "77B" => {
            let field = serde_json::from_value(value.clone())?;
            Ok(SwiftFieldContainer::Field77B(field))
        }
        _ => {
            // Fallback to unknown field
            Ok(SwiftFieldContainer::Unknown(
                crate::field_parser::UnknownField {
                    tag: tag.to_string(),
                    content: value.to_string(),
                },
            ))
        }
    }
    .map_err(|e: serde_json::Error| ParseError::JsonError {
        message: format!("Failed to parse field {}: {}", tag, e),
    })
}

/// High-level utility functions for common use cases
pub mod utils {
    use super::*;

    /// Parse MT message from SWIFT format and convert to JSON
    pub fn swift_to_json(swift_message: &str) -> Result<String> {
        let message = SwiftMessage::parse(swift_message)?;
        message.to_json_string()
    }

    /// Parse JSON and convert to SWIFT format (basic implementation)
    pub fn json_to_swift(json_str: &str) -> Result<String> {
        let message = SwiftMessage::from_json_string(json_str)?;

        // Build a basic SWIFT message format
        let mut swift_content = String::new();

        // Add blocks if available
        if let Some(block1) = &message.blocks.block_1 {
            swift_content.push_str(&format!("{{1:{}}}", block1));
        }
        if let Some(block2) = &message.blocks.block_2 {
            swift_content.push_str(&format!("{{2:{}}}", block2));
        }
        if let Some(block3) = &message.blocks.block_3 {
            swift_content.push_str(&format!("{{3:{}}}", block3));
        }

        // Add Block 4 with fields
        swift_content.push_str("{4:\n");
        for tag in &message.field_order {
            if let Some(field) = message.fields.get(tag) {
                swift_content.push_str(&format!(":{}:{}\n", tag, field.to_swift_string()));
            }
        }
        swift_content.push_str("-}");

        if let Some(block5) = &message.blocks.block_5 {
            swift_content.push_str(&format!("{{5:{}}}", block5));
        }

        Ok(swift_content)
    }

    /// Parse specific MT103 from JSON
    pub fn json_to_mt103(json_str: &str) -> Result<MT103> {
        MT103::from_json_string(json_str)
    }

    /// Convert MT103 to JSON
    pub fn mt103_to_json(mt103: &MT103) -> Result<String> {
        mt103.to_json_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_json_utility_functions() {
        let swift_text = r#"{1:F01DEUTDEFFAXXX0123456789}{2:I103CHASUS33AXXXU3003}{4:
:20:FT21234567890
:23B:CRED
:32A:210315EUR1234567,89
:50K:JOHN DOE
:59:JANE SMITH
:71A:OUR
-}"#;

        // Test utility functions
        let json_str = utils::swift_to_json(swift_text).unwrap();
        assert!(json_str.contains("\"message_type\": \"103\""));
        assert!(json_str.contains("\"transaction_reference\": \"FT21234567890\""));

        // Test round-trip conversion
        let swift_reconstructed = utils::json_to_swift(&json_str).unwrap();
        assert!(swift_reconstructed.contains("103"));
        assert!(swift_reconstructed.contains("FT21234567890"));
    }
}
