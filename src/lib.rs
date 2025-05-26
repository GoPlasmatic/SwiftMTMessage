//! # Swift MT Message Parser
//!
//! A Rust library for parsing SWIFT MT (Message Type) messages and extracting their fields.
//! This library focuses purely on parsing and field extraction, not message building or transformation.
//!
//! ## Supported Message Types
//!
//! - MT102: Multiple Customer Credit Transfer
//! - MT103: Single Customer Credit Transfer
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
//! ```rust
//! use swift_mt_message::{parse_message, MTMessage};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let message_text = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:\n:20:FT21234567890\n:23B:CRED\n:32A:210315EUR1234567,89\n:50K:JOHN DOE\n:59:JANE SMITH\n-}";
//! let message = parse_message(message_text)?;
//!
//! if let MTMessage::MT103(mt103) = message {
//!     println!("Sender Reference: {}", mt103.sender_reference()?);
//!     println!("Amount: {:?}", mt103.amount()?);
//!     println!("Currency: {}", mt103.currency()?);
//! }
//! # Ok(())
//! # }
//! ```

pub mod common;
pub mod error;
pub mod messages;
pub mod parser;
pub mod validation;

// Re-export main types for convenience
pub use error::{MTError, Result};
pub use messages::MTMessage;
pub use parser::parse_message;
pub use validation::{validate_message, ValidationLevel, ValidationResult};

// Re-export common types that users might need
pub use common::{Field, MessageBlock, Tag};
