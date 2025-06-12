//! Generic field parser and registry system for SWIFT MT fields
//!
//! This module provides the core field parsing infrastructure with a generic SwiftField trait
//! that all field types implement, plus an extensible registry system for dynamic field parsing.

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::errors::{ParseError, Result, ValidationError};
// Import the modular field implementations
use crate::mt_models::fields::beneficiary::Field59;
use crate::mt_models::fields::charges::{Field71A, Field71F};
use crate::mt_models::fields::common::{
    Field20, Field23B, Field32A, Field33B, Field70, Field72, Field77B,
};
use crate::mt_models::fields::institutions::{
    Field52, Field53, Field54, Field55, Field56, Field57,
};
use crate::mt_models::fields::ordering_customer::Field50;
use crate::mt_models::fields::{Field13C, Field23E, Field26T, Field36, Field51A, Field71G};

/// Generic trait that all SWIFT fields must implement
pub trait SwiftField: Clone + std::fmt::Debug + Serialize + for<'de> Deserialize<'de> {
    /// The field tag (e.g., "20", "50A", "50K")
    const TAG: &'static str;

    /// Parse field content from raw string
    fn parse(content: &str) -> Result<Self>
    where
        Self: Sized;

    /// Serialize field back to SWIFT format
    fn to_swift_string(&self) -> String;

    /// Validate field content according to SWIFT rules
    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError>;

    /// Get field tag at runtime
    fn tag(&self) -> &'static str {
        Self::TAG
    }

    /// Get field options if any (e.g., "A", "K", "F" for field 50)
    fn options() -> Vec<&'static str> {
        vec![]
    }

    /// Check if field is mandatory for a specific message type
    fn is_mandatory_for_message_type(message_type: &str) -> bool {
        // Use centralized configuration by default
        crate::utils::is_field_mandatory(Self::TAG, message_type)
    }

    /// Get field description for documentation/debugging
    fn description() -> &'static str;
}

/// Container for all possible field types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwiftFieldContainer {
    Field13C(Field13C),
    Field20(Field20),
    Field23B(Field23B),
    Field23E(Field23E),
    Field26T(Field26T),
    Field32A(Field32A),
    Field33B(Field33B),
    Field36(Field36),
    Field50(Field50),
    Field51A(Field51A),
    Field52(Field52),
    Field53(Field53),
    Field54(Field54),
    Field55(Field55),
    Field56(Field56),
    Field57(Field57),
    Field59(Field59),
    Field70(Field70),
    Field71A(Field71A),
    Field71F(Field71F),
    Field71G(Field71G),
    Field72(Field72),
    Field77B(Field77B),
    // Add more fields as needed
    Unknown(UnknownField),
}

impl SwiftFieldContainer {
    pub fn tag(&self) -> &str {
        match self {
            SwiftFieldContainer::Field13C(f) => f.tag(),
            SwiftFieldContainer::Field20(f) => f.tag(),
            SwiftFieldContainer::Field23B(f) => f.tag(),
            SwiftFieldContainer::Field23E(f) => f.tag(),
            SwiftFieldContainer::Field26T(f) => f.tag(),
            SwiftFieldContainer::Field32A(f) => f.tag(),
            SwiftFieldContainer::Field33B(f) => f.tag(),
            SwiftFieldContainer::Field36(f) => f.tag(),
            SwiftFieldContainer::Field50(f) => f.tag(),
            SwiftFieldContainer::Field51A(f) => f.tag(),
            SwiftFieldContainer::Field52(f) => f.tag(),
            SwiftFieldContainer::Field53(f) => f.tag(),
            SwiftFieldContainer::Field54(f) => f.tag(),
            SwiftFieldContainer::Field55(f) => f.tag(),
            SwiftFieldContainer::Field56(f) => f.tag(),
            SwiftFieldContainer::Field57(f) => f.tag(),
            SwiftFieldContainer::Field59(f) => f.tag(),
            SwiftFieldContainer::Field70(f) => f.tag(),
            SwiftFieldContainer::Field71A(f) => f.tag(),
            SwiftFieldContainer::Field71F(f) => f.tag(),
            SwiftFieldContainer::Field71G(f) => f.tag(),
            SwiftFieldContainer::Field72(f) => f.tag(),
            SwiftFieldContainer::Field77B(f) => f.tag(),
            SwiftFieldContainer::Unknown(f) => &f.tag,
        }
    }

    pub fn parse(tag: &str, content: &str) -> Result<Self> {
        match tag {
            "13C" => Ok(SwiftFieldContainer::Field13C(Field13C::parse(content)?)),
            "20" => Ok(SwiftFieldContainer::Field20(Field20::parse(content)?)),
            "23B" => Ok(SwiftFieldContainer::Field23B(Field23B::parse(content)?)),
            "23E" => Ok(SwiftFieldContainer::Field23E(Field23E::parse(content)?)),
            "26T" => Ok(SwiftFieldContainer::Field26T(Field26T::parse(content)?)),
            "32A" => Ok(SwiftFieldContainer::Field32A(Field32A::parse(content)?)),
            "33B" => Ok(SwiftFieldContainer::Field33B(Field33B::parse(content)?)),
            "36" => Ok(SwiftFieldContainer::Field36(Field36::parse(content)?)),
            tag if tag.starts_with("50") => {
                Ok(SwiftFieldContainer::Field50(Field50::parse(tag, content)?))
            }
            "51A" => Ok(SwiftFieldContainer::Field51A(Field51A::parse(content)?)),
            tag if tag.starts_with("52") => {
                Ok(SwiftFieldContainer::Field52(Field52::parse(tag, content)?))
            }
            tag if tag.starts_with("53") => {
                Ok(SwiftFieldContainer::Field53(Field53::parse(tag, content)?))
            }
            tag if tag.starts_with("54") => {
                Ok(SwiftFieldContainer::Field54(Field54::parse(tag, content)?))
            }
            tag if tag.starts_with("55") => {
                Ok(SwiftFieldContainer::Field55(Field55::parse(tag, content)?))
            }
            tag if tag.starts_with("56") => {
                Ok(SwiftFieldContainer::Field56(Field56::parse(tag, content)?))
            }
            tag if tag.starts_with("57") => {
                Ok(SwiftFieldContainer::Field57(Field57::parse(tag, content)?))
            }
            tag if tag.starts_with("59") => {
                Ok(SwiftFieldContainer::Field59(Field59::parse(tag, content)?))
            }
            "70" => Ok(SwiftFieldContainer::Field70(Field70::parse(content)?)),
            "71A" => Ok(SwiftFieldContainer::Field71A(Field71A::parse(content)?)),
            "71F" => Ok(SwiftFieldContainer::Field71F(Field71F::parse(content)?)),
            "71G" => Ok(SwiftFieldContainer::Field71G(Field71G::parse(content)?)),
            "72" => Ok(SwiftFieldContainer::Field72(Field72::parse(content)?)),
            "77B" => Ok(SwiftFieldContainer::Field77B(Field77B::parse(content)?)),
            _ => {
                // Try the registry first
                if let Ok(container) = parse_field_from_registry(tag, content) {
                    Ok(container)
                } else {
                    // Create unknown field as fallback
                    Ok(SwiftFieldContainer::Unknown(UnknownField {
                        tag: tag.to_string(),
                        content: content.to_string(),
                    }))
                }
            }
        }
    }

    pub fn to_swift_string(&self) -> String {
        match self {
            SwiftFieldContainer::Field13C(f) => f.to_swift_string(),
            SwiftFieldContainer::Field20(f) => f.to_swift_string(),
            SwiftFieldContainer::Field23B(f) => f.to_swift_string(),
            SwiftFieldContainer::Field23E(f) => f.to_swift_string(),
            SwiftFieldContainer::Field26T(f) => f.to_swift_string(),
            SwiftFieldContainer::Field32A(f) => f.to_swift_string(),
            SwiftFieldContainer::Field33B(f) => f.to_swift_string(),
            SwiftFieldContainer::Field36(f) => f.to_swift_string(),
            SwiftFieldContainer::Field50(f) => f.to_swift_string(),
            SwiftFieldContainer::Field51A(f) => f.to_swift_string(),
            SwiftFieldContainer::Field52(f) => f.to_swift_string(),
            SwiftFieldContainer::Field53(f) => f.to_swift_string(),
            SwiftFieldContainer::Field54(f) => f.to_swift_string(),
            SwiftFieldContainer::Field55(f) => f.to_swift_string(),
            SwiftFieldContainer::Field56(f) => f.to_swift_string(),
            SwiftFieldContainer::Field57(f) => f.to_swift_string(),
            SwiftFieldContainer::Field59(f) => f.to_swift_string(),
            SwiftFieldContainer::Field70(f) => f.to_swift_string(),
            SwiftFieldContainer::Field71A(f) => f.to_swift_string(),
            SwiftFieldContainer::Field71F(f) => f.to_swift_string(),
            SwiftFieldContainer::Field71G(f) => f.to_swift_string(),
            SwiftFieldContainer::Field72(f) => f.to_swift_string(),
            SwiftFieldContainer::Field77B(f) => f.to_swift_string(),
            SwiftFieldContainer::Unknown(f) => f.to_swift_string(),
        }
    }
}

/// Unknown field type for unsupported fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownField {
    pub tag: String,
    pub content: String,
}

impl UnknownField {
    pub fn to_swift_string(&self) -> String {
        format!(":{}:{}", self.tag, self.content)
    }
}

// Import FormatRules from validator module
pub use crate::validator::FormatRules;

/// Field registry type
type FieldParserFn = fn(&str) -> Result<SwiftFieldContainer>;

/// Global field registry
static FIELD_REGISTRY: Lazy<std::sync::Mutex<HashMap<String, FieldParserFn>>> =
    Lazy::new(|| std::sync::Mutex::new(HashMap::new()));

/// Register a custom field parser
pub fn register_field_parser(tag: &str, parser: FieldParserFn) {
    if let Ok(mut registry) = FIELD_REGISTRY.lock() {
        registry.insert(tag.to_string(), parser);
    }
}

/// Parse field using the registry
pub fn parse_field_from_registry(tag: &str, content: &str) -> Result<SwiftFieldContainer> {
    if let Ok(registry) = FIELD_REGISTRY.lock() {
        if let Some(parser) = registry.get(tag) {
            return parser(content);
        }
    }

    Err(ParseError::UnknownField {
        tag: tag.to_string(),
        block: "4".to_string(),
    })
}

/// Generic message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwiftMessage {
    pub message_type: String,
    pub basic_header: Option<crate::tokenizer::BasicHeader>,
    pub application_header: Option<crate::tokenizer::ApplicationHeader>,
    pub user_header: Option<crate::tokenizer::UserHeader>,
    pub trailer_block: Option<crate::tokenizer::Trailer>,
    pub blocks: crate::tokenizer::SwiftMessageBlocks,
    pub fields: HashMap<String, SwiftFieldContainer>,
    pub field_order: Vec<String>, // Preserve field order
}

impl SwiftMessage {
    /// Parse a generic SWIFT message
    pub fn parse(raw_message: &str) -> Result<Self> {
        let blocks = crate::tokenizer::extract_blocks(raw_message)?;
        let message_type = crate::tokenizer::extract_message_type(&blocks)?;

        let mut basic_header = None;
        if let Some(block1) = &blocks.block_1 {
            basic_header = Some(crate::tokenizer::parse_basic_header(block1)?);
        }

        let mut application_header = None;
        if let Some(block2) = &blocks.block_2 {
            application_header = Some(crate::tokenizer::parse_application_header(block2)?);
        }

        let mut user_header = None;
        if let Some(block3) = &blocks.block_3 {
            user_header = Some(crate::tokenizer::parse_user_header(block3)?);
        }

        let mut trailer_block = None;
        if let Some(block5) = &blocks.block_5 {
            trailer_block = Some(crate::tokenizer::parse_trailer_block(block5)?);
        }

        let parsed_fields = if let Some(block4) = &blocks.block_4 {
            crate::tokenizer::parse_block4_fields(block4)?
        } else {
            Vec::new()
        };

        let mut fields = HashMap::new();
        let mut field_order = Vec::new();

        for parsed_field in parsed_fields {
            let field_container =
                SwiftFieldContainer::parse(&parsed_field.tag, &parsed_field.content)?;
            fields.insert(parsed_field.tag.clone(), field_container);
            field_order.push(parsed_field.tag);
        }

        Ok(SwiftMessage {
            message_type,
            basic_header,
            application_header,
            user_header,
            trailer_block,
            blocks,
            fields,
            field_order,
        })
    }

    /// Get a specific field by tag
    pub fn get_field(&self, tag: &str) -> Option<&SwiftFieldContainer> {
        self.fields.get(tag)
    }

    /// Get all fields in order
    pub fn get_all_fields(&self) -> Vec<&SwiftFieldContainer> {
        self.field_order
            .iter()
            .filter_map(|tag| self.fields.get(tag))
            .collect()
    }

    /// Validate the entire message
    pub fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        for field in self.fields.values() {
            match field {
                SwiftFieldContainer::Field13C(f) => f.validate(rules)?,
                SwiftFieldContainer::Field20(f) => f.validate(rules)?,
                SwiftFieldContainer::Field23B(f) => f.validate(rules)?,
                SwiftFieldContainer::Field23E(f) => f.validate(rules)?,
                SwiftFieldContainer::Field26T(f) => f.validate(rules)?,
                SwiftFieldContainer::Field32A(f) => f.validate(rules)?,
                SwiftFieldContainer::Field33B(f) => f.validate(rules)?,
                SwiftFieldContainer::Field36(f) => f.validate(rules)?,
                SwiftFieldContainer::Field50(f) => f.validate(rules)?,
                SwiftFieldContainer::Field51A(f) => f.validate(rules)?,
                SwiftFieldContainer::Field52(f) => f.validate(rules)?,
                SwiftFieldContainer::Field53(f) => f.validate(rules)?,
                SwiftFieldContainer::Field54(f) => f.validate(rules)?,
                SwiftFieldContainer::Field55(f) => f.validate(rules)?,
                SwiftFieldContainer::Field56(f) => f.validate(rules)?,
                SwiftFieldContainer::Field57(f) => f.validate(rules)?,
                SwiftFieldContainer::Field59(f) => f.validate(rules)?,
                SwiftFieldContainer::Field70(f) => f.validate(rules)?,
                SwiftFieldContainer::Field71A(f) => f.validate(rules)?,
                SwiftFieldContainer::Field71F(f) => f.validate(rules)?,
                SwiftFieldContainer::Field71G(f) => f.validate(rules)?,
                SwiftFieldContainer::Field72(f) => f.validate(rules)?,
                SwiftFieldContainer::Field77B(f) => f.validate(rules)?,
                SwiftFieldContainer::Unknown(_) => {} // Skip validation for unknown fields
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_container_parsing() {
        let field = SwiftFieldContainer::parse("20", "TESTREF123").unwrap();
        match field {
            SwiftFieldContainer::Field20(f20) => {
                assert_eq!(f20.transaction_reference, "TESTREF123");
            }
            _ => panic!("Expected Field20"),
        }
    }
}
