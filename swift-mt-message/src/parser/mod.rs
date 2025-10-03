//! SWIFT MT Message Parser modules

mod generated;
mod parser_impl;
pub mod sequence_parser;

// Re-export generated parser functions
pub use generated::{extract_base_tag, normalize_field_tag, parse_block4_fields};

// Re-export main parser types
pub use parser_impl::{
    FieldConsumptionTracker, ParsingContext, SwiftParser,
    find_field_with_variant_sequential_constrained, find_field_with_variant_sequential_numbered,
    parse_sequences,
};

// Re-export sequence parser types
pub use sequence_parser::{
    ParsedSequences, SequenceConfig, get_sequence_config, parse_repetitive_sequence,
    split_into_sequences,
};
