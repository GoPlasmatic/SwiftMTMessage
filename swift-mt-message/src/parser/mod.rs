//! SWIFT MT Message Parser modules

mod parser_impl;
pub mod sequence_parser;

// Re-export main parser types
pub use parser_impl::{
    find_field_with_variant_sequential_constrained, parse_sequences,
    parse_swift_message_from_string, serialize_swift_message_to_string, FieldConsumptionTracker,
    ParsingContext, SwiftParser,
};

// Re-export sequence parser types
pub use sequence_parser::{
    get_sequence_config, parse_repetitive_sequence, split_into_sequences, ParsedSequences,
    SequenceConfig,
};
