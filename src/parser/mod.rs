//! # Message Parser
//!
//! Core parsing functionality for SWIFT MT messages.
//!
//! - **SwiftParser**: Main entry point for complete message parsing
//! - **MessageParser**: Field-level sequential parsing
//! - **SequenceParser**: Repetitive sequence handling (MT101, MT104, etc.)

pub mod field_extractor;
mod generated;
pub mod message_parser;
pub mod sequence_parser;
mod swift_parser;
pub mod utils;

// Re-export generated parser functions
pub use generated::{extract_base_tag, normalize_field_tag, parse_block4_fields};

// Re-export main parser types
pub use swift_parser::{
    FieldConsumptionTracker, ParsingContext, SwiftParser,
    find_field_with_variant_sequential_constrained, find_field_with_variant_sequential_numbered,
    parse_sequences,
};

// Re-export sequence parser types
pub use sequence_parser::{
    ParsedSequences, SequenceConfig, get_sequence_config, parse_repetitive_sequence,
    split_into_sequences,
};

// Re-export message parser for internal use
pub use field_extractor::extract_field_content;
pub use message_parser::MessageParser;

// Re-export utility functions
pub use utils::*;
