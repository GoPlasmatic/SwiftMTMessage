//! # SWIFT MT Message Parser
//!
//! Rust library for parsing, validating, and generating SWIFT MT financial messages.
//!
//! ## Features
//! - **Type-safe parsing** with dedicated field structures
//! - **SWIFT validation** with 1,335 error codes (T/C/D/E/G series)
//! - **Sample generation** with configurable scenarios
//! - **JSON serialization** with clean flattened output
//! - **30+ message types** (MT101-MT950)
//!
//! ## Quick Start
//! ```rust
//! use swift_mt_message::parser::SwiftParser;
//!
//! # fn main() -> swift_mt_message::Result<()> {
//! let message = "{1:F01BANKDEFF...}{2:I103...}{4:\n:20:REF123\n:23B:CRED\n-}";
//! let parsed = SwiftParser::parse_auto(message)?;
//! # Ok(())
//! # }
//! ```

pub mod errors;
pub mod fields;
pub mod headers;
pub mod messages;
pub mod parsed_message;
pub mod parser;
pub mod sample;
pub mod scenario_config;
pub mod swift_error_codes;
pub mod swift_message;
pub mod traits;
pub mod utils;
pub mod validation_result;

// Plugin module for dataflow-rs integration
pub mod plugin;

// Re-export all message types
pub use messages::*;

// Re-export core types
pub use errors::{
    ParseError, Result, SwiftBusinessError, SwiftContentError, SwiftFormatError, SwiftGeneralError,
    SwiftRelationError, SwiftValidationError, SwiftValidationResult, ValidationError, error_codes,
};
pub use headers::{ApplicationHeader, BasicHeader, Trailer, UserHeader};
pub use parsed_message::ParsedSwiftMessage;
pub use parser::{SwiftParser, extract_base_tag};
pub use swift_error_codes as swift_codes;
pub use swift_message::SwiftMessage;
pub use traits::{SwiftField, SwiftMessageBody};
pub use utils::{
    get_field_tag_for_mt, get_field_tag_with_variant, is_numbered_field, map_variant_to_numbered,
};
pub use validation_result::ValidationResult;

// Re-export sample generation
pub use sample::{SampleGenerator, generate_sample, generate_sample_with_config};
pub use scenario_config::ScenarioConfig;

/// Simplified result type for SWIFT operations
pub type SwiftResult<T> = std::result::Result<T, crate::errors::ParseError>;
