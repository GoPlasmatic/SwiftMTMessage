//! SWIFT MT message implementations

use serde::{Deserialize, Serialize};

use crate::common::{Field, MessageBlock};
use crate::error::{MTError, Result};

// Import all message types
pub mod mt102;
pub mod mt103;
pub mod mt192;
pub mod mt195;
pub mod mt196;
pub mod mt197;
pub mod mt199;
pub mod mt202;
pub mod mt940;
pub mod mt941;
pub mod mt942;

// Re-export message types
pub use mt102::MT102;
pub use mt103::MT103;
pub use mt192::MT192;
pub use mt195::MT195;
pub use mt196::MT196;
pub use mt197::MT197;
pub use mt199::MT199;
pub use mt202::MT202;
pub use mt940::MT940;
pub use mt941::MT941;
pub use mt942::MT942;

/// Main enum representing all supported MT message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MTMessage {
    /// MT102: Multiple Customer Credit Transfer
    MT102(MT102),
    /// MT103: Single Customer Credit Transfer
    MT103(MT103),
    /// MT192: Request for Cancellation
    MT192(MT192),
    /// MT195: Queries
    MT195(MT195),
    /// MT196: Answers
    MT196(MT196),
    /// MT197: Copy of a Message
    MT197(MT197),
    /// MT199: Free Format Message
    MT199(MT199),
    /// MT202: General Financial Institution Transfer
    MT202(MT202),
    /// MT940: Customer Statement Message
    MT940(MT940),
    /// MT941: Balance Report Message
    MT941(MT941),
    /// MT942: Interim Transaction Report
    MT942(MT942),
}

impl MTMessage {
    /// Get the message type as a string
    pub fn message_type(&self) -> &'static str {
        match self {
            MTMessage::MT102(_) => "102",
            MTMessage::MT103(_) => "103",
            MTMessage::MT192(_) => "192",
            MTMessage::MT195(_) => "195",
            MTMessage::MT196(_) => "196",
            MTMessage::MT197(_) => "197",
            MTMessage::MT199(_) => "199",
            MTMessage::MT202(_) => "202",
            MTMessage::MT940(_) => "940",
            MTMessage::MT941(_) => "941",
            MTMessage::MT942(_) => "942",
        }
    }

    /// Get a field by tag from the message
    pub fn get_field(&self, tag: &str) -> Option<&Field> {
        match self {
            MTMessage::MT102(msg) => msg.get_field(tag),
            MTMessage::MT103(msg) => msg.get_field(tag),
            MTMessage::MT192(msg) => msg.get_field(tag),
            MTMessage::MT195(msg) => msg.get_field(tag),
            MTMessage::MT196(msg) => msg.get_field(tag),
            MTMessage::MT197(msg) => msg.get_field(tag),
            MTMessage::MT199(msg) => msg.get_field(tag),
            MTMessage::MT202(msg) => msg.get_field(tag),
            MTMessage::MT940(msg) => msg.get_field(tag),
            MTMessage::MT941(msg) => msg.get_field(tag),
            MTMessage::MT942(msg) => msg.get_field(tag),
        }
    }

    /// Get all fields with a specific tag from the message
    pub fn get_fields(&self, tag: &str) -> Vec<&Field> {
        match self {
            MTMessage::MT102(msg) => msg.get_fields(tag),
            MTMessage::MT103(msg) => msg.get_fields(tag),
            MTMessage::MT192(msg) => msg.get_fields(tag),
            MTMessage::MT195(msg) => msg.get_fields(tag),
            MTMessage::MT196(msg) => msg.get_fields(tag),
            MTMessage::MT197(msg) => msg.get_fields(tag),
            MTMessage::MT199(msg) => msg.get_fields(tag),
            MTMessage::MT202(msg) => msg.get_fields(tag),
            MTMessage::MT940(msg) => msg.get_fields(tag),
            MTMessage::MT941(msg) => msg.get_fields(tag),
            MTMessage::MT942(msg) => msg.get_fields(tag),
        }
    }

    /// Get all fields from the message
    pub fn get_all_fields(&self) -> Vec<&Field> {
        match self {
            MTMessage::MT102(msg) => msg.get_all_fields(),
            MTMessage::MT103(msg) => msg.get_all_fields(),
            MTMessage::MT192(msg) => msg.get_all_fields(),
            MTMessage::MT195(msg) => msg.get_all_fields(),
            MTMessage::MT196(msg) => msg.get_all_fields(),
            MTMessage::MT197(msg) => msg.get_all_fields(),
            MTMessage::MT199(msg) => msg.get_all_fields(),
            MTMessage::MT202(msg) => msg.get_all_fields(),
            MTMessage::MT940(msg) => msg.get_all_fields(),
            MTMessage::MT941(msg) => msg.get_all_fields(),
            MTMessage::MT942(msg) => msg.get_all_fields(),
        }
    }
}

/// Common trait for all MT message types
pub trait MTMessageType {
    /// Create message from parsed blocks
    fn from_blocks(blocks: Vec<MessageBlock>) -> Result<Self>
    where
        Self: Sized;

    /// Get a field by tag
    fn get_field(&self, tag: &str) -> Option<&Field>;

    /// Get all fields with a specific tag
    fn get_fields(&self, tag: &str) -> Vec<&Field>;

    /// Get all fields
    fn get_all_fields(&self) -> Vec<&Field>;

    /// Get the text block fields
    fn text_fields(&self) -> &[Field];
}

/// Helper function to extract text block from message blocks
pub fn extract_text_block(blocks: &[MessageBlock]) -> Result<Vec<Field>> {
    for block in blocks {
        if let MessageBlock::TextBlock { fields } = block {
            return Ok(fields.clone());
        }
    }
    Err(MTError::InvalidMessageStructure {
        message: "No text block found in message".to_string(),
    })
}

/// Helper function to find field by tag in a list of fields
pub fn find_field<'a>(fields: &'a [Field], tag: &str) -> Option<&'a Field> {
    fields.iter().find(|field| field.tag.as_str() == tag)
}

/// Helper function to find all fields with a specific tag
pub fn find_fields<'a>(fields: &'a [Field], tag: &str) -> Vec<&'a Field> {
    fields.iter().filter(|field| field.tag.as_str() == tag).collect()
}

/// Helper function to get required field value
pub fn get_required_field_value(fields: &[Field], tag: &str) -> Result<String> {
    find_field(fields, tag)
        .map(|field| field.value().to_string())
        .ok_or_else(|| MTError::missing_required_field(tag))
}

/// Helper function to get optional field value
pub fn get_optional_field_value(fields: &[Field], tag: &str) -> Option<String> {
    find_field(fields, tag).map(|field| field.value().to_string())
} 