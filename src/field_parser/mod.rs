//! Field parsing infrastructure for SWIFT MT messages
//!
//! This module provides basic field parsing support without complex macros.

use crate::errors::{Result, ValidationError};
pub use crate::validator::FormatRules;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait that all SWIFT fields must implement
pub trait SwiftField {
    /// The field tag (e.g., "20", "32A")
    const TAG: &'static str;

    /// Parse field content from string
    fn parse(content: &str) -> Result<Self>
    where
        Self: Sized;

    /// Convert field back to SWIFT format
    fn to_swift_string(&self) -> String;

    /// Validate field according to SWIFT standards
    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError>;

    /// Get available options for this field (if any)
    fn options() -> Vec<&'static str> {
        vec![]
    }

    /// Get human-readable description
    fn description() -> &'static str;

    /// Get the field tag
    fn tag(&self) -> &'static str {
        Self::TAG
    }
}

/// Unknown field that couldn't be parsed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownField {
    pub tag: String,
    pub content: String,
}

impl UnknownField {
    pub fn to_swift_string(&self) -> String {
        self.content.clone()
    }
}

/// Container for all possible field types
/// This will be expanded as we add more message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwiftFieldContainer {
    // Ordering Customer fields
    Field50(crate::mt_models::fields::ordering_customer::Field50),

    // Institution fields
    Field51A(crate::mt_models::fields::institutions::Field51A),
    Field52(crate::mt_models::fields::institutions::Field52),
    Field53(crate::mt_models::fields::institutions::Field53),
    Field54(crate::mt_models::fields::institutions::Field54),
    Field55(crate::mt_models::fields::institutions::Field55),
    Field56(crate::mt_models::fields::institutions::Field56),
    Field57(crate::mt_models::fields::institutions::Field57),

    // Beneficiary fields
    Field59(crate::mt_models::fields::beneficiary::Field59),

    // Common fields
    Field13C(crate::mt_models::fields::common::Field13C),
    Field20(crate::mt_models::fields::common::Field20),
    Field23B(crate::mt_models::fields::common::Field23B),
    Field23E(crate::mt_models::fields::common::Field23E),
    Field26T(crate::mt_models::fields::common::Field26T),
    Field32A(crate::mt_models::fields::common::Field32A),
    Field33B(crate::mt_models::fields::common::Field33B),
    Field36(crate::mt_models::fields::common::Field36),
    Field70(crate::mt_models::fields::common::Field70),
    Field72(crate::mt_models::fields::common::Field72),
    Field77B(crate::mt_models::fields::common::Field77B),

    // Charges fields
    Field71A(crate::mt_models::fields::charges::Field71A),
    Field71F(crate::mt_models::fields::charges::Field71F),
    Field71G(crate::mt_models::fields::charges::Field71G),

    Unknown(UnknownField),
}

/// Main SWIFT message structure
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
    /// Parse a generic SWIFT message (simplified implementation)
    pub fn parse(raw_message: &str) -> Result<Self> {
        let blocks = crate::tokenizer::extract_blocks(raw_message)?;
        let message_type = crate::tokenizer::extract_message_type(&blocks)?;

        // Parse headers
        let basic_header = blocks
            .block_1
            .as_ref()
            .and_then(|block1| crate::tokenizer::parse_basic_header(block1).ok());

        let application_header = blocks
            .block_2
            .as_ref()
            .and_then(|block2| crate::tokenizer::parse_application_header(block2).ok());

        let user_header = blocks
            .block_3
            .as_ref()
            .and_then(|block3| crate::tokenizer::parse_user_header(block3).ok());

        let trailer_block = blocks
            .block_5
            .as_ref()
            .and_then(|block5| crate::tokenizer::parse_trailer_block(block5).ok());

        // Parse fields (simplified)
        let parsed_fields = if let Some(block4) = &blocks.block_4 {
            crate::tokenizer::parse_block4_fields(block4)?
        } else {
            Vec::new()
        };

        let mut fields = HashMap::new();
        let mut field_order = Vec::new();

        for parsed_field in parsed_fields {
            // Parse field based on its tag
            let field_container = parse_field_by_tag(&parsed_field.tag, &parsed_field.content)?;
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
}

impl SwiftFieldContainer {
    /// Get the field tag for this container
    pub fn tag(&self) -> &str {
        match self {
            SwiftFieldContainer::Field50(field) => field.tag(),
            SwiftFieldContainer::Field51A(_) => "51A",
            SwiftFieldContainer::Field52(field) => field.tag(),
            SwiftFieldContainer::Field53(field) => field.tag(),
            SwiftFieldContainer::Field54(field) => field.tag(),
            SwiftFieldContainer::Field55(field) => field.tag(),
            SwiftFieldContainer::Field56(field) => field.tag(),
            SwiftFieldContainer::Field57(field) => field.tag(),
            SwiftFieldContainer::Field59(field) => field.tag(),
            SwiftFieldContainer::Field13C(_) => "13C",
            SwiftFieldContainer::Field20(_) => "20",
            SwiftFieldContainer::Field23B(_) => "23B",
            SwiftFieldContainer::Field23E(_) => "23E",
            SwiftFieldContainer::Field26T(_) => "26T",
            SwiftFieldContainer::Field32A(_) => "32A",
            SwiftFieldContainer::Field33B(_) => "33B",
            SwiftFieldContainer::Field36(_) => "36",
            SwiftFieldContainer::Field70(_) => "70",
            SwiftFieldContainer::Field72(_) => "72",
            SwiftFieldContainer::Field77B(_) => "77B",
            SwiftFieldContainer::Field71A(_) => "71A",
            SwiftFieldContainer::Field71F(_) => "71F",
            SwiftFieldContainer::Field71G(_) => "71G",
            SwiftFieldContainer::Unknown(field) => &field.tag,
        }
    }

    /// Convert field back to SWIFT format
    pub fn to_swift_string(&self) -> String {
        match self {
            SwiftFieldContainer::Field50(field) => field.to_swift_string(),
            SwiftFieldContainer::Field51A(field) => field.to_swift_string(),
            SwiftFieldContainer::Field52(field) => field.to_swift_string(),
            SwiftFieldContainer::Field53(field) => field.to_swift_string(),
            SwiftFieldContainer::Field54(field) => field.to_swift_string(),
            SwiftFieldContainer::Field55(field) => field.to_swift_string(),
            SwiftFieldContainer::Field56(field) => field.to_swift_string(),
            SwiftFieldContainer::Field57(field) => field.to_swift_string(),
            SwiftFieldContainer::Field59(field) => field.to_swift_string(),
            SwiftFieldContainer::Field13C(field) => field.to_swift_string(),
            SwiftFieldContainer::Field20(field) => field.to_swift_string(),
            SwiftFieldContainer::Field23B(field) => field.to_swift_string(),
            SwiftFieldContainer::Field23E(field) => field.to_swift_string(),
            SwiftFieldContainer::Field26T(field) => field.to_swift_string(),
            SwiftFieldContainer::Field32A(field) => field.to_swift_string(),
            SwiftFieldContainer::Field33B(field) => field.to_swift_string(),
            SwiftFieldContainer::Field36(field) => field.to_swift_string(),
            SwiftFieldContainer::Field70(field) => field.to_swift_string(),
            SwiftFieldContainer::Field72(field) => field.to_swift_string(),
            SwiftFieldContainer::Field77B(field) => field.to_swift_string(),
            SwiftFieldContainer::Field71A(field) => field.to_swift_string(),
            SwiftFieldContainer::Field71F(field) => field.to_swift_string(),
            SwiftFieldContainer::Field71G(field) => field.to_swift_string(),
            SwiftFieldContainer::Unknown(field) => field.to_swift_string(),
        }
    }
}

/// Parse a field based on its tag
fn parse_field_by_tag(tag: &str, content: &str) -> Result<SwiftFieldContainer> {
    use crate::mt_models::fields::institutions::{
        Field52A, Field52D, Field53A, Field53B, Field53D, Field54A, Field54B, Field54D, Field55A,
        Field55B, Field55D, Field56A, Field56C, Field56D, Field57A, Field57B, Field57C, Field57D,
    };
    use crate::mt_models::fields::ordering_customer::{Field50A, Field50F, Field50K};
    // Note: Field59 and Field59A are used via their full paths below
    use crate::mt_models::fields::common::*;

    match tag {
        // Field 50 variants (Ordering Customer)
        "50A" => {
            let field = Field50A::parse(content)?;
            Ok(SwiftFieldContainer::Field50(
                crate::mt_models::fields::ordering_customer::Field50::A(field),
            ))
        }
        "50F" => {
            let field = Field50F::parse(content)?;
            Ok(SwiftFieldContainer::Field50(
                crate::mt_models::fields::ordering_customer::Field50::F(field),
            ))
        }
        "50K" => {
            let field = Field50K::parse(content)?;
            Ok(SwiftFieldContainer::Field50(
                crate::mt_models::fields::ordering_customer::Field50::K(field),
            ))
        }

        // Field 52 variants (Ordering Institution)
        "52A" => {
            let field = Field52A::parse(content)?;
            Ok(SwiftFieldContainer::Field52(
                crate::mt_models::fields::institutions::Field52::A(field),
            ))
        }
        "52D" => {
            let field = Field52D::parse(content)?;
            Ok(SwiftFieldContainer::Field52(
                crate::mt_models::fields::institutions::Field52::D(field),
            ))
        }

        // Field 53 variants (Sender's Correspondent)
        "53A" => {
            let field = Field53A::parse(content)?;
            Ok(SwiftFieldContainer::Field53(
                crate::mt_models::fields::institutions::Field53::A(field),
            ))
        }
        "53B" => {
            let field = Field53B::parse(content)?;
            Ok(SwiftFieldContainer::Field53(
                crate::mt_models::fields::institutions::Field53::B(field),
            ))
        }
        "53D" => {
            let field = Field53D::parse(content)?;
            Ok(SwiftFieldContainer::Field53(
                crate::mt_models::fields::institutions::Field53::D(field),
            ))
        }

        // Field 54 variants (Receiver's Correspondent)
        "54A" => {
            let field = Field54A::parse(content)?;
            Ok(SwiftFieldContainer::Field54(
                crate::mt_models::fields::institutions::Field54::A(field),
            ))
        }
        "54B" => {
            let field = Field54B::parse(content)?;
            Ok(SwiftFieldContainer::Field54(
                crate::mt_models::fields::institutions::Field54::B(field),
            ))
        }
        "54D" => {
            let field = Field54D::parse(content)?;
            Ok(SwiftFieldContainer::Field54(
                crate::mt_models::fields::institutions::Field54::D(field),
            ))
        }

        // Field 55 variants (Third Reimbursement Institution)
        "55A" => {
            let field = Field55A::parse(content)?;
            Ok(SwiftFieldContainer::Field55(
                crate::mt_models::fields::institutions::Field55::A(field),
            ))
        }
        "55B" => {
            let field = Field55B::parse(content)?;
            Ok(SwiftFieldContainer::Field55(
                crate::mt_models::fields::institutions::Field55::B(field),
            ))
        }
        "55D" => {
            let field = Field55D::parse(content)?;
            Ok(SwiftFieldContainer::Field55(
                crate::mt_models::fields::institutions::Field55::D(field),
            ))
        }

        // Field 56 variants (Intermediary Institution)
        "56A" => {
            let field = Field56A::parse(content)?;
            Ok(SwiftFieldContainer::Field56(
                crate::mt_models::fields::institutions::Field56::A(field),
            ))
        }
        "56C" => {
            let field = Field56C::parse(content)?;
            Ok(SwiftFieldContainer::Field56(
                crate::mt_models::fields::institutions::Field56::C(field),
            ))
        }
        "56D" => {
            let field = Field56D::parse(content)?;
            Ok(SwiftFieldContainer::Field56(
                crate::mt_models::fields::institutions::Field56::D(field),
            ))
        }

        // Field 57 variants (Account With Institution)
        "57A" => {
            let field = Field57A::parse(content)?;
            Ok(SwiftFieldContainer::Field57(
                crate::mt_models::fields::institutions::Field57::A(field),
            ))
        }
        "57B" => {
            let field = Field57B::parse(content)?;
            Ok(SwiftFieldContainer::Field57(
                crate::mt_models::fields::institutions::Field57::B(field),
            ))
        }
        "57C" => {
            let field = Field57C::parse(content)?;
            Ok(SwiftFieldContainer::Field57(
                crate::mt_models::fields::institutions::Field57::C(field),
            ))
        }
        "57D" => {
            let field = Field57D::parse(content)?;
            Ok(SwiftFieldContainer::Field57(
                crate::mt_models::fields::institutions::Field57::D(field),
            ))
        }

        // Field 59 variants (Beneficiary)
        "59" => {
            let field = crate::mt_models::fields::beneficiary::Field59::parse("59", content)?;
            Ok(SwiftFieldContainer::Field59(field))
        }
        "59A" => {
            let field = crate::mt_models::fields::beneficiary::Field59::parse("59A", content)?;
            Ok(SwiftFieldContainer::Field59(field))
        }

        // Common fields
        "20" => {
            let field = Field20::parse(content)?;
            Ok(SwiftFieldContainer::Field20(field))
        }
        "23B" => {
            let field = Field23B::parse(content)?;
            Ok(SwiftFieldContainer::Field23B(field))
        }
        "23E" => {
            let field = Field23E::parse(content)?;
            Ok(SwiftFieldContainer::Field23E(field))
        }
        "32A" => {
            let field = Field32A::parse(content)?;
            Ok(SwiftFieldContainer::Field32A(field))
        }
        "33B" => {
            let field = Field33B::parse(content)?;
            Ok(SwiftFieldContainer::Field33B(field))
        }
        "71A" => {
            let field = crate::mt_models::fields::charges::Field71A::parse(content)?;
            Ok(SwiftFieldContainer::Field71A(field))
        }
        "71F" => {
            let field = crate::mt_models::fields::charges::Field71F::parse(content)?;
            Ok(SwiftFieldContainer::Field71F(field))
        }
        "71G" => {
            let field = crate::mt_models::fields::charges::Field71G::parse(content)?;
            Ok(SwiftFieldContainer::Field71G(field))
        }

        // Unknown fields
        _ => Ok(SwiftFieldContainer::Unknown(UnknownField {
            tag: tag.to_string(),
            content: content.to_string(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_field_creation() {
        let field = SwiftFieldContainer::Unknown(UnknownField {
            tag: "99Z".to_string(),
            content: "UNKNOWN_CONTENT".to_string(),
        });
        assert!(matches!(field, SwiftFieldContainer::Unknown(_)));
        assert_eq!(field.tag(), "99Z");
    }

    #[test]
    fn test_field_container_tag_methods() {
        let unknown_field = SwiftFieldContainer::Unknown(UnknownField {
            tag: "99Z".to_string(),
            content: "TEST".to_string(),
        });
        assert_eq!(unknown_field.tag(), "99Z");
        assert_eq!(unknown_field.to_swift_string(), "TEST");
    }

    #[test]
    fn test_swift_message_parsing() {
        let swift_text = r#"{1:F01DEUTDEFFAXXX0123456789}{2:I103CHASUS33AXXXU3003}{4:
:20:FT21234567890
:23B:CRED
:32A:210315EUR1234567,89
:50K:JOHN DOE
:59:JANE SMITH
:71A:OUR
-}"#;

        let message = SwiftMessage::parse(swift_text).unwrap();
        assert_eq!(message.message_type, "103");
        // Since we use simplified parsing, fields will be Unknown for now
        assert!(message.get_field("20").is_some());
        assert!(message.get_field("23B").is_some());
    }
}
