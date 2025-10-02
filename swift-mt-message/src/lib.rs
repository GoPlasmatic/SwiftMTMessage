//! # Swift MT Message Parser - Enhanced Architecture
//!
//! A comprehensive Rust library for parsing SWIFT MT (Message Type) messages with strong typing,
//! complex field structures, comprehensive validation, and flattened JSON serialization.
//!
//! ## Key Features
//!
//! - **Complex Field Structures**: Full enum-based field variants (Field50: A/F/K, Field59: A/Basic)
//! - **Flattened JSON Serialization**: Clean JSON output without enum wrapper layers
//! - **Type-safe field parsing** with dedicated field structs and automatic validation
//! - **Comprehensive Field Support**: All MT103 fields with proper SWIFT compliance
//! - **Bidirectional Serialization**: Perfect round-trip JSON serialization/deserialization
//! - **Extensive Validation**: BIC validation, field length checks, format compliance
//!
//! ## Supported Field Types
//!
//! ### Complex Enum Fields
//! - **Field50** (Ordering Customer): 50A (Account+BIC), 50F (Party+Address), 50K (Name+Address)
//! - **Field59** (Beneficiary Customer): 59A (Account+BIC), 59 (Basic lines)
//!
//! ### Institution Fields (with account_line_indicator)
//! - **Field52A** (Ordering Institution): BIC + optional account + account_line_indicator
//! - **Field53A-57A** (Correspondent/Intermediary): All with account_line_indicator support
//!
//! ### Simple Type Fields
//! - **Field32A** (Value Date/Currency/Amount): NaiveDate + String + f64
//! - **Field20, 23B, 70, 71A**: Proper field name alignment with old version
//!
//! ## JSON Output Structure
//!
//! The library produces clean, flattened JSON without enum wrapper layers:
//!
//! ```json
//! {
//!   "50": {
//!     "name_and_address": ["JOHN DOE", "123 MAIN ST"]
//!   },
//!   "59": {
//!     "account": "DE89370400440532013000",
//!     "bic": "DEUTDEFFXXX"
//!   }
//! }
//! ```
//!
//! Instead of nested enum structures like `{"50": {"K": {...}}}`.

pub mod errors;
pub mod fields;
pub mod headers;
pub mod message_parser;
pub mod messages;
pub mod parsed_message;
pub mod parser;
pub mod sample;
pub mod scenario_config;
pub mod serde_helpers;
pub mod slash_handler;
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
