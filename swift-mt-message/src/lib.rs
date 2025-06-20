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
pub use swift_mt_message_macros::{SwiftField, SwiftMessage, field, serde_swift_fields};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_header: Option<UserHeader>,

    /// Trailer (Block 5) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block3: Option<String>,
    pub block4: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// Enumeration of all supported SWIFT message types for automatic parsing
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "message_type")]
pub enum ParsedSwiftMessage {
    #[serde(rename = "103")]
    MT103(Box<SwiftMessage<messages::MT103>>),
    #[serde(rename = "103STP")]
    MT103STP(Box<SwiftMessage<messages::MT103STP>>),
    #[serde(rename = "103REMIT")]
    MT103REMIT(Box<SwiftMessage<messages::MT103REMIT>>),
    #[serde(rename = "202")]
    MT202(Box<SwiftMessage<messages::MT202>>),
    #[serde(rename = "205")]
    MT205(Box<SwiftMessage<messages::MT205>>),
}

impl ParsedSwiftMessage {
    /// Get the message type as a string
    pub fn message_type(&self) -> &'static str {
        match self {
            ParsedSwiftMessage::MT103(_) => "103",
            ParsedSwiftMessage::MT103STP(_) => "103STP",
            ParsedSwiftMessage::MT103REMIT(_) => "103REMIT",
            ParsedSwiftMessage::MT202(_) => "202",
            ParsedSwiftMessage::MT205(_) => "205",
        }
    }

    /// Convert to a specific message type if it matches
    pub fn as_mt103(&self) -> Option<&SwiftMessage<messages::MT103>> {
        match self {
            ParsedSwiftMessage::MT103(msg) => Some(msg),
            _ => None,
        }
    }

    pub fn as_mt103stp(&self) -> Option<&SwiftMessage<messages::MT103STP>> {
        match self {
            ParsedSwiftMessage::MT103STP(msg) => Some(msg),
            _ => None,
        }
    }

    pub fn as_mt103remit(&self) -> Option<&SwiftMessage<messages::MT103REMIT>> {
        match self {
            ParsedSwiftMessage::MT103REMIT(msg) => Some(msg),
            _ => None,
        }
    }

    pub fn as_mt202(&self) -> Option<&SwiftMessage<messages::MT202>> {
        match self {
            ParsedSwiftMessage::MT202(msg) => Some(msg),
            _ => None,
        }
    }

    pub fn as_mt205(&self) -> Option<&SwiftMessage<messages::MT205>> {
        match self {
            ParsedSwiftMessage::MT205(msg) => Some(msg),
            _ => None,
        }
    }

    /// Convert into a specific message type if it matches
    pub fn into_mt103(self) -> Option<SwiftMessage<messages::MT103>> {
        match self {
            ParsedSwiftMessage::MT103(msg) => Some(*msg),
            _ => None,
        }
    }

    pub fn into_mt202(self) -> Option<SwiftMessage<messages::MT202>> {
        match self {
            ParsedSwiftMessage::MT202(msg) => Some(*msg),
            _ => None,
        }
    }

    pub fn into_mt205(self) -> Option<SwiftMessage<messages::MT205>> {
        match self {
            ParsedSwiftMessage::MT205(msg) => Some(*msg),
            _ => None,
        }
    }
}

impl<T: SwiftMessageBody> SwiftMessage<T> {
    /// Validate message against business rules using JSONLogic
    /// This validation method has access to both headers and message fields,
    /// allowing for comprehensive validation of MT103 and other message types.
    pub fn validate_business_rules(&self) -> ValidationResult {
        // Check if the message type has validation rules
        let validation_rules = match T::message_type() {
            "103" => messages::MT103::validation_rules(),
            "103STP" => messages::MT103STP::validation_rules(),
            "103REMIT" => messages::MT103REMIT::validation_rules(),
            "202" => messages::MT202::validation_rules(),
            "202COV" => messages::MT202COV::validation_rules(),
            "205" => messages::MT205::validation_rules(),
            "104" => messages::MT104::validation_rules(),
            "107" => messages::MT107::validation_rules(),
            "110" => messages::MT110::validation_rules(),
            "111" => messages::MT111::validation_rules(),
            "112" => messages::MT112::validation_rules(),
            "210" => messages::MT210::validation_rules(),
            "900" => messages::MT900::validation_rules(),
            "910" => messages::MT910::validation_rules(),
            "920" => messages::MT920::validation_rules(),
            "935" => messages::MT935::validation_rules(),
            "940" => messages::MT940::validation_rules(),
            "941" => messages::MT941::validation_rules(),
            "942" => messages::MT942::validation_rules(),
            "950" => messages::MT950::validation_rules(),
            _ => {
                return ValidationResult::with_error(ValidationError::BusinessRuleValidation {
                    rule_name: "UNSUPPORTED_MESSAGE_TYPE".to_string(),
                    message: format!(
                        "No validation rules defined for message type {}",
                        T::message_type()
                    ),
                });
            }
        };

        // Parse the validation rules JSON
        let rules_json: serde_json::Value = match serde_json::from_str(validation_rules) {
            Ok(json) => json,
            Err(e) => {
                return ValidationResult::with_error(ValidationError::BusinessRuleValidation {
                    rule_name: "JSON_PARSE".to_string(),
                    message: format!("Failed to parse validation rules JSON: {}", e),
                });
            }
        };

        // Extract rules array from the JSON
        let rules = match rules_json.get("rules").and_then(|r| r.as_array()) {
            Some(rules) => rules,
            None => {
                return ValidationResult::with_error(ValidationError::BusinessRuleValidation {
                    rule_name: "RULES_FORMAT".to_string(),
                    message: "Validation rules must contain a 'rules' array".to_string(),
                });
            }
        };

        // Get constants if they exist
        let constants = rules_json
            .get("constants")
            .and_then(|c| c.as_object())
            .cloned()
            .unwrap_or_default();

        // Create comprehensive data context with headers and fields
        let context_value = match self.create_validation_context(&constants) {
            Ok(context) => context,
            Err(e) => {
                return ValidationResult::with_error(ValidationError::BusinessRuleValidation {
                    rule_name: "CONTEXT_CREATION".to_string(),
                    message: format!("Failed to create validation context: {}", e),
                });
            }
        };

        // Validate each rule using datalogic-rs
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        for (rule_index, rule) in rules.iter().enumerate() {
            let rule_id = rule
                .get("id")
                .and_then(|id| id.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("RULE_{}", rule_index));

            let rule_description = rule
                .get("description")
                .and_then(|desc| desc.as_str())
                .unwrap_or("No description");

            if let Some(condition) = rule.get("condition") {
                // Create DataLogic instance for evaluation
                let dl = datalogic_rs::DataLogic::new();
                match dl.evaluate_json(condition, &context_value, None) {
                    Ok(result) => {
                        match result.as_bool() {
                            Some(true) => {
                                // Rule passed
                                continue;
                            }
                            Some(false) => {
                                // Rule failed
                                errors.push(ValidationError::BusinessRuleValidation {
                                    rule_name: rule_id.clone(),
                                    message: format!(
                                        "Business rule validation failed: {} - {}",
                                        rule_id, rule_description
                                    ),
                                });
                            }
                            None => {
                                // Rule returned non-boolean value
                                warnings.push(format!(
                                    "Rule {} returned non-boolean value: {:?}",
                                    rule_id, result
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        // JSONLogic evaluation error
                        errors.push(ValidationError::BusinessRuleValidation {
                            rule_name: rule_id.clone(),
                            message: format!(
                                "JSONLogic evaluation error for rule {}: {}",
                                rule_id, e
                            ),
                        });
                    }
                }
            } else {
                warnings.push(format!("Rule {} has no condition", rule_id));
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Create a comprehensive validation context that includes headers, fields, and constants
    fn create_validation_context(
        &self,
        constants: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<serde_json::Value> {
        // Serialize the entire message (including headers) to JSON for data context
        let full_message_data = match serde_json::to_value(self) {
            Ok(data) => data,
            Err(e) => {
                return Err(ParseError::SerializationError {
                    message: format!("Failed to serialize complete message: {}", e),
                });
            }
        };

        // Create a comprehensive data context
        let mut data_context = serde_json::Map::new();

        // Add the complete message data
        if let serde_json::Value::Object(msg_obj) = full_message_data {
            for (key, value) in msg_obj {
                data_context.insert(key, value);
            }
        }

        // Add constants to data context
        for (key, value) in constants {
            data_context.insert(key.clone(), value.clone());
        }

        // Extract sender and receiver BIC from headers for enhanced validation context
        let (sender_country, receiver_country) = self.extract_country_codes_from_bics();

        // Add enhanced message context including BIC-derived information
        data_context.insert("message_context".to_string(), serde_json::json!({
            "message_type": self.message_type,
            "sender_country": sender_country,
            "receiver_country": receiver_country,
            "sender_bic": self.basic_header.logical_terminal,
            "receiver_bic": &self.application_header.destination_address,
            "message_priority": &self.application_header.priority,
            "delivery_monitoring": self.application_header.delivery_monitoring.as_ref().unwrap_or(&"3".to_string()),
        }));

        Ok(serde_json::Value::Object(data_context))
    }

    /// Extract country codes from BIC codes in the headers
    fn extract_country_codes_from_bics(&self) -> (String, String) {
        // Extract sender country from basic header BIC (positions 4-5)
        let sender_country = if self.basic_header.logical_terminal.len() >= 6 {
            self.basic_header.logical_terminal[4..6].to_string()
        } else {
            "XX".to_string() // Unknown country
        };

        // Extract receiver country from application header destination BIC
        let receiver_country = if self.application_header.destination_address.len() >= 6 {
            self.application_header.destination_address[4..6].to_string()
        } else {
            "XX".to_string()
        };

        (sender_country, receiver_country)
    }
}
