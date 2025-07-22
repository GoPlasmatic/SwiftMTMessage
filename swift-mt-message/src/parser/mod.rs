//! SWIFT MT Message Parser modules

mod parser_impl;
pub mod sequence_parser;

// Re-export main parser types
pub use parser_impl::{
    SwiftParser, 
    ParsingContext, 
    FieldConsumptionTracker, 
    parse_sequences,
    find_field_with_variant_sequential_constrained,
    parse_swift_message_from_string,
    serialize_swift_message_to_string,
};

// Re-export sequence parser types
pub use sequence_parser::{
    SequenceConfig, 
    ParsedSequences, 
    split_into_sequences,
    parse_repetitive_sequence,
    get_sequence_config,
};