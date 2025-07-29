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
pub mod messages;
pub mod parsed_message;
pub mod parser;
pub mod scenario_config;
pub mod swift_error_codes;
pub mod swift_message;
pub mod traits;
pub mod utils;
pub mod validation_result;

// Re-export all message types
pub use messages::*;

// Re-export core types
pub use errors::{
    error_codes, ParseError, Result, SwiftBusinessError, SwiftContentError, SwiftFormatError,
    SwiftGeneralError, SwiftRelationError, SwiftValidationError, SwiftValidationResult,
    ValidationError,
};
pub use headers::{ApplicationHeader, BasicHeader, Trailer, UserHeader};
pub use parsed_message::ParsedSwiftMessage;
pub use parser::{extract_base_tag, SwiftParser};
pub use swift_error_codes as swift_codes;
pub use swift_message::SwiftMessage;
pub use traits::{SwiftField, SwiftMessageBody};
pub use utils::{get_field_tag_for_mt, get_field_tag_with_variant};
pub use validation_result::ValidationResult;

// Re-export derive macros
pub use swift_mt_message_macros::{serde_swift_fields, SwiftField, SwiftMessage};

/// Simplified result type for SWIFT operations
pub type SwiftResult<T> = std::result::Result<T, crate::errors::ParseError>;

use datafake_rs::DataGenerator;
use scenario_config::{find_scenario_by_name, find_scenario_for_message_type};

/// Generate a sample SWIFT MT message based on test scenarios
///
/// This function loads a test scenario configuration for the specified message type
/// and generates a realistic SWIFT message using datafake-rs for dynamic data generation.
///
/// # Arguments
///
/// * `message_type` - The MT message type (e.g., "MT103", "MT202")
/// * `scenario_name` - Optional scenario name. If None, uses the default scenario
///
/// # Returns
///
/// Returns a complete SwiftMessage object with headers and the message body
///
/// # Example
///
/// ```no_run
/// # use swift_mt_message::{generate_sample, SwiftMessage, messages::mt103::MT103};
/// // Generate a standard MT103 message
/// let mt103_msg: SwiftMessage<MT103> = generate_sample("MT103", None).unwrap();
/// println!("{}", mt103_msg.to_mt_message());
///
/// // Generate a specific scenario
/// let mt103_high_value: SwiftMessage<MT103> = generate_sample("MT103", Some("high_value_payment")).unwrap();
/// ```
pub fn generate_sample<T>(
    message_type: &str,
    scenario_name: Option<&str>,
) -> crate::errors::Result<SwiftMessage<T>>
where
    T: crate::traits::SwiftMessageBody + serde::de::DeserializeOwned,
{
    // Load the scenario configuration JSON
    let scenario_json = if let Some(name) = scenario_name {
        find_scenario_by_name(message_type, name)?
    } else {
        find_scenario_for_message_type(message_type)?
    };

    // Create datafake-rs generator from the scenario
    let generator = DataGenerator::from_value(scenario_json).map_err(|e| {
        errors::ParseError::InvalidFormat {
            message: format!("Failed to create datafake generator: {e:?}"),
        }
    })?;

    // Generate the data
    let generated_data = generator
        .generate()
        .map_err(|e| errors::ParseError::InvalidFormat {
            message: format!("datafake-rs generation failed: {e:?}"),
        })?;

    // Convert generated data to string for parsing
    let generated_json = serde_json::to_string_pretty(&generated_data).map_err(|e| {
        errors::ParseError::InvalidFormat {
            message: format!("Failed to serialize generated data: {e}"),
        }
    })?;

    // Parse the generated JSON into the complete SwiftMessage
    serde_json::from_str(&generated_json).map_err(|e| errors::ParseError::InvalidFormat {
        message: format!(
            "Failed to parse generated JSON into SwiftMessage<{message_type}>: {e}"
        ),
    })
}
