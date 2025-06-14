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
//! ## Example Usage
//!
//! ```rust
//! use swift_mt_message::{SwiftParser, SwiftMessage, messages::MT103};
//!
//! let raw_mt103 = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:\n:20:FT21234567890\n:23B:CRED\n:32A:210315EUR1234567,89\n:50K:JOHN DOE\n:52A:BANKDEFF\n:57A:DEUTDEFF\n:59A:/DE89370400440532013000\nDEUTDEFF\n:70:PAYMENT\n:71A:OUR\n-}";
//! let parsed: SwiftMessage<MT103> = SwiftParser::parse(raw_mt103)?;
//! let json_output = serde_json::to_string_pretty(&parsed)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

pub mod errors;
pub mod fields;
pub mod headers;
pub mod messages;
pub mod parser;

// Re-export core types
pub use errors::{ParseError, Result, ValidationError};
pub use headers::{ApplicationHeader, BasicHeader, Trailer, UserHeader};
pub use parser::SwiftParser;

// Re-export derive macros
pub use swift_mt_message_macros::{SwiftField, SwiftMessage, swift_serde};

/// Simplified result type for SWIFT operations
pub type SwiftResult<T> = std::result::Result<T, crate::errors::ParseError>;

/// Core trait for all Swift field types
pub trait SwiftField: Serialize + for<'de> Deserialize<'de> + Clone + std::fmt::Debug {
    /// Parse field value from string representation
    fn parse(value: &str) -> Result<Self>
    where
        Self: Sized;

    /// Convert field back to SWIFT string format
    fn to_swift_string(&self) -> String;

    /// Validate field according to SWIFT format rules
    fn validate(&self) -> ValidationResult;

    /// Get field format specification
    fn format_spec() -> &'static str;
}

/// Core trait for Swift message types
pub trait SwiftMessageBody: Debug + Clone + Send + Sync + Serialize {
    /// Get the message type identifier (e.g., "103", "202")
    fn message_type() -> &'static str;

    /// Create from field map
    fn from_fields(fields: HashMap<String, Vec<String>>) -> SwiftResult<Self>
    where
        Self: Sized;

    /// Convert to field map
    fn to_fields(&self) -> HashMap<String, Vec<String>>;

    /// Get required field tags for this message type
    fn required_fields() -> Vec<&'static str>;

    /// Get optional field tags for this message type
    fn optional_fields() -> Vec<&'static str>;
}

/// Complete SWIFT message with headers and body
#[derive(Debug, Clone, Serialize)]
pub struct SwiftMessage<T: SwiftMessageBody> {
    /// Basic Header (Block 1)
    pub basic_header: BasicHeader,

    /// Application Header (Block 2)
    pub application_header: ApplicationHeader,

    /// User Header (Block 3) - Optional
    pub user_header: Option<UserHeader>,

    /// Trailer (Block 5) - Optional
    pub trailer: Option<Trailer>,

    /// Raw message blocks for preservation
    pub blocks: RawBlocks,

    /// Message type identifier
    pub message_type: String,

    /// Field order as they appeared in the original message
    pub field_order: Vec<String>,

    /// Parsed message body with typed fields
    pub fields: T,
}

/// Raw message blocks for preservation and reconstruction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RawBlocks {
    pub block1: Option<String>,
    pub block2: Option<String>,
    pub block3: Option<String>,
    pub block4: String,
    pub block5: Option<String>,
}

/// Validation result for field and message validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn with_error(error: ValidationError) -> Self {
        Self {
            is_valid: false,
            errors: vec![error],
            warnings: Vec::new(),
        }
    }

    pub fn with_errors(errors: Vec<ValidationError>) -> Self {
        Self {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }
}

/// Common data types used across multiple fields
pub mod common {
    use serde::{Deserialize, Serialize};

    /// SWIFT BIC (Bank Identifier Code)
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct BIC {
        pub value: String,
    }

    impl BIC {
        pub fn new(value: String) -> Self {
            Self { value }
        }

        pub fn validate(&self) -> bool {
            // BIC validation logic: 8 or 11 characters, alphanumeric
            let len = self.value.len();
            (len == 8 || len == 11) && self.value.chars().all(|c| c.is_alphanumeric())
        }
    }

    /// SWIFT Currency Code (ISO 4217)
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct Currency {
        pub code: String,
    }

    impl Currency {
        pub fn new(code: String) -> Self {
            Self {
                code: code.to_uppercase(),
            }
        }
    }

    /// SWIFT Amount with decimal handling
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct Amount {
        pub value: String,
        pub decimal_places: u8,
    }

    impl Amount {
        pub fn new(value: String) -> Self {
            let decimal_places = if value.contains(',') {
                value.split(',').nth(1).map(|s| s.len() as u8).unwrap_or(0)
            } else {
                0
            };

            Self {
                value,
                decimal_places,
            }
        }

        pub fn to_decimal(&self) -> Result<f64, std::num::ParseFloatError> {
            self.value.replace(',', ".").parse()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::mt103::MT103;

    #[test]
    fn test_full_mt103_parsing() {
        let raw_message = r#"{1:F01BNPAFRPPXXX0000000000}{2:O1031234240101DEUTDEFFXXXX12345678952401011234N}{3:{103:EBA}}{4:
:20:FT21001234567890
:23B:CRED
:32A:240101USD1000,00
:50K:/1234567890
ACME CORPORATION
123 MAIN STREET
NEW YORK NY 10001
:52A:BNPAFRPPXXX
:57A:DEUTDEFFXXX
:59:/DE89370400440532013000
MUELLER GMBH
HAUPTSTRASSE 1
10115 BERLIN
:70:PAYMENT FOR INVOICE 12345
:71A:OUR
-}"#;

        let result = SwiftParser::parse::<MT103>(raw_message);
        assert!(result.is_ok(), "Parsing should succeed: {:?}", result.err());

        let parsed = result.unwrap();
        assert_eq!(parsed.message_type, "103");

        // Test JSON serialization
        let json = serde_json::to_string_pretty(&parsed);
        assert!(json.is_ok(), "JSON serialization should work");
        println!("Parsed MT103 JSON:\n{}", json.unwrap());
    }

    #[test]
    fn test_field_parsing() {
        use crate::fields::field20::Field20;

        let result = Field20::parse(":20:FT21001234567890");
        assert!(result.is_ok());

        let field = result.unwrap();
        assert_eq!(field.to_swift_string(), ":20:FT21001234567890");
    }
}
