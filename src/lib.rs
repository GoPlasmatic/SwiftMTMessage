//! # Swift MT Message Parser
//!
//! A comprehensive Rust library for parsing SWIFT MT (Message Type) messages with strong typing,
//! field validation, and extensible architecture. This library provides both high-level message
//! parsing and low-level field access with excellent error reporting.
//!
//! ## Features
//!
//! - **Type-safe field parsing** with dedicated field structs
//! - **Comprehensive validation** using SWIFT format rules
//! - **Extensible field registry** for custom field types
//! - **Rich error diagnostics** with context information
//! - **Generic message support** for unknown message types
//! - **Backward compatibility** with existing APIs
//!
//! ## Supported Message Types
//!
//! - MT102: Multiple Customer Credit Transfer
//! - MT103: Single Customer Credit Transfer
//! - MT103-STP: Single Customer Credit Transfer (Straight Through Processing)
//! - MT192: Request for Cancellation
//! - MT195: Queries
//! - MT196: Answers
//! - MT197: Copy of a Message
//! - MT199: Free Format Message
//! - MT202: General Financial Institution Transfer
//! - MT940: Customer Statement Message
//! - MT941: Balance Report Message
//! - MT942: Interim Transaction Report
//!
//! ## Example Usage
//!
//! ### High-level API (recommended)
//!
//! ```rust
//! use swift_mt_message::{field_parser::SwiftMessage, mt_models::mt103::MT103};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let message_text = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:\n:20:FT21234567890\n:23B:CRED\n:32A:210315EUR1234567,89\n:50K:JOHN DOE\n:59:JANE SMITH\n:71A:OUR\n-}";
//!
//! // Parse as generic message
//! let message = SwiftMessage::parse(message_text)?;
//! println!("Message type: {}", message.message_type);
//!
//! // Convert to specific message type
//! let mt103 = MT103::from_swift_message(message)?;
//! println!("Transaction reference: {}", mt103.field_20.transaction_reference);
//! # Ok(())
//! # }
//! ```
//!
//! ### MT103-STP with Enhanced Validation
//!
//! ```rust
//! use swift_mt_message::{field_parser::SwiftMessage, mt_models::mt103_stp::MT103STP};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let message_text = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:\n:20:STP123456789\n:23B:CRED\n:32A:210315EUR1234567,89\n:50K:JOHN DOE\n:59A:DEUTDEFF\n:71A:OUR\n-}";
//!
//! // Parse as MT103-STP with enhanced validation
//! let message = SwiftMessage::parse(message_text)?;
//! let mt103_stp = MT103STP::from_swift_message(message)?;
//!
//! // Check STP compliance
//! if mt103_stp.is_stp_compliant() {
//!     println!("Message is STP compliant");
//! } else {
//!     let violations = mt103_stp.get_stp_violations();
//!     for violation in violations {
//!         println!("STP Rule {} violated: {}", violation.rule, violation.description);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Field-level Access
//!
//! ```rust
//! use swift_mt_message::field_parser::{SwiftMessage, SwiftField};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let message_text = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:\n:20:FT21234567890\n:23B:CRED\n:32A:210315EUR1234567,89\n:50K:JOHN DOE\n:59:JANE SMITH\n:71A:OUR\n-}";
//! let message = SwiftMessage::parse(message_text)?;
//!
//! // Access individual fields with type safety
//! for (tag, field) in &message.fields {
//!     println!("{}: {}", tag, field.to_swift_string());
//! }
//! # Ok(())
//! # }
//! ```

pub mod config;
pub mod errors;
pub mod field_parser;
pub mod json;
pub mod mt_models;
pub mod tokenizer;
pub mod utils;
pub mod validation;
pub mod validator;

pub use errors::{ErrorCollection, ErrorContext, FieldParseError, ParseError, ValidationError};
pub use field_parser::{SwiftField, SwiftFieldContainer, SwiftMessage};
pub use json::{FromJson, JsonBlocks, JsonMessage, MessageMetadata, ToJson};
pub use mt_models::{MT103, MT103STP, MT202, STPRuleViolation, STPValidationReport};
pub use tokenizer::{SwiftMessageBlocks, extract_blocks, parse_block4_fields};
pub use validation::{ValidationReport, ValidationResult, validate_mt_message_with_rules};
