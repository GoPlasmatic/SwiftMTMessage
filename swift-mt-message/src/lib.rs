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
use std::any::Any;
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
pub trait SwiftMessageBody: Debug + Clone + Send + Sync + Serialize + std::any::Any {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub blocks: Option<RawBlocks>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block4: Option<String>,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "message_type")]
pub enum ParsedSwiftMessage {
    #[serde(rename = "101")]
    MT101(Box<SwiftMessage<messages::MT101>>),
    #[serde(rename = "103")]
    MT103(Box<SwiftMessage<messages::MT103>>),
    #[serde(rename = "104")]
    MT104(Box<SwiftMessage<messages::MT104>>),
    #[serde(rename = "107")]
    MT107(Box<SwiftMessage<messages::MT107>>),
    #[serde(rename = "110")]
    MT110(Box<SwiftMessage<messages::MT110>>),
    #[serde(rename = "111")]
    MT111(Box<SwiftMessage<messages::MT111>>),
    #[serde(rename = "112")]
    MT112(Box<SwiftMessage<messages::MT112>>),
    #[serde(rename = "202")]
    MT202(Box<SwiftMessage<messages::MT202>>),
    #[serde(rename = "205")]
    MT205(Box<SwiftMessage<messages::MT205>>),
    #[serde(rename = "210")]
    MT210(Box<SwiftMessage<messages::MT210>>),
    #[serde(rename = "900")]
    MT900(Box<SwiftMessage<messages::MT900>>),
    #[serde(rename = "910")]
    MT910(Box<SwiftMessage<messages::MT910>>),
    #[serde(rename = "920")]
    MT920(Box<SwiftMessage<messages::MT920>>),
    #[serde(rename = "935")]
    MT935(Box<SwiftMessage<messages::MT935>>),
    #[serde(rename = "940")]
    MT940(Box<SwiftMessage<messages::MT940>>),
    #[serde(rename = "941")]
    MT941(Box<SwiftMessage<messages::MT941>>),
    #[serde(rename = "942")]
    MT942(Box<SwiftMessage<messages::MT942>>),
    #[serde(rename = "950")]
    MT950(Box<SwiftMessage<messages::MT950>>),
    #[serde(rename = "192")]
    MT192(Box<SwiftMessage<messages::MT192>>),
    #[serde(rename = "196")]
    MT196(Box<SwiftMessage<messages::MT196>>),
    #[serde(rename = "292")]
    MT292(Box<SwiftMessage<messages::MT292>>),
    #[serde(rename = "296")]
    MT296(Box<SwiftMessage<messages::MT296>>),
}

impl ParsedSwiftMessage {
    /// Get the message type as a string
    pub fn message_type(&self) -> &'static str {
        match self {
            ParsedSwiftMessage::MT101(_) => "101",
            ParsedSwiftMessage::MT103(_) => "103",
            ParsedSwiftMessage::MT104(_) => "104",
            ParsedSwiftMessage::MT107(_) => "107",
            ParsedSwiftMessage::MT110(_) => "110",
            ParsedSwiftMessage::MT111(_) => "111",
            ParsedSwiftMessage::MT112(_) => "112",
            ParsedSwiftMessage::MT202(_) => "202",
            ParsedSwiftMessage::MT205(_) => "205",
            ParsedSwiftMessage::MT210(_) => "210",
            ParsedSwiftMessage::MT900(_) => "900",
            ParsedSwiftMessage::MT910(_) => "910",
            ParsedSwiftMessage::MT920(_) => "920",
            ParsedSwiftMessage::MT935(_) => "935",
            ParsedSwiftMessage::MT940(_) => "940",
            ParsedSwiftMessage::MT941(_) => "941",
            ParsedSwiftMessage::MT942(_) => "942",
            ParsedSwiftMessage::MT950(_) => "950",
            ParsedSwiftMessage::MT192(_) => "192",
            ParsedSwiftMessage::MT196(_) => "196",
            ParsedSwiftMessage::MT292(_) => "292",
            ParsedSwiftMessage::MT296(_) => "296",
        }
    }
}

impl ParsedSwiftMessage {
    /// Convert to a specific message type if it matches
    pub fn as_mt101(&self) -> Option<&SwiftMessage<messages::MT101>> {
        match self {
            ParsedSwiftMessage::MT101(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt103(&self) -> Option<&SwiftMessage<messages::MT103>> {
        match self {
            ParsedSwiftMessage::MT103(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt104(&self) -> Option<&SwiftMessage<messages::MT104>> {
        match self {
            ParsedSwiftMessage::MT104(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt107(&self) -> Option<&SwiftMessage<messages::MT107>> {
        match self {
            ParsedSwiftMessage::MT107(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt110(&self) -> Option<&SwiftMessage<messages::MT110>> {
        match self {
            ParsedSwiftMessage::MT110(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt111(&self) -> Option<&SwiftMessage<messages::MT111>> {
        match self {
            ParsedSwiftMessage::MT111(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt112(&self) -> Option<&SwiftMessage<messages::MT112>> {
        match self {
            ParsedSwiftMessage::MT112(msg) => Some(msg),
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
    pub fn as_mt210(&self) -> Option<&SwiftMessage<messages::MT210>> {
        match self {
            ParsedSwiftMessage::MT210(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt900(&self) -> Option<&SwiftMessage<messages::MT900>> {
        match self {
            ParsedSwiftMessage::MT900(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt910(&self) -> Option<&SwiftMessage<messages::MT910>> {
        match self {
            ParsedSwiftMessage::MT910(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt920(&self) -> Option<&SwiftMessage<messages::MT920>> {
        match self {
            ParsedSwiftMessage::MT920(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt935(&self) -> Option<&SwiftMessage<messages::MT935>> {
        match self {
            ParsedSwiftMessage::MT935(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt940(&self) -> Option<&SwiftMessage<messages::MT940>> {
        match self {
            ParsedSwiftMessage::MT940(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt941(&self) -> Option<&SwiftMessage<messages::MT941>> {
        match self {
            ParsedSwiftMessage::MT941(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt942(&self) -> Option<&SwiftMessage<messages::MT942>> {
        match self {
            ParsedSwiftMessage::MT942(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt950(&self) -> Option<&SwiftMessage<messages::MT950>> {
        match self {
            ParsedSwiftMessage::MT950(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt192(&self) -> Option<&SwiftMessage<messages::MT192>> {
        match self {
            ParsedSwiftMessage::MT192(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt196(&self) -> Option<&SwiftMessage<messages::MT196>> {
        match self {
            ParsedSwiftMessage::MT196(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt292(&self) -> Option<&SwiftMessage<messages::MT292>> {
        match self {
            ParsedSwiftMessage::MT292(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt296(&self) -> Option<&SwiftMessage<messages::MT296>> {
        match self {
            ParsedSwiftMessage::MT296(msg) => Some(msg),
            _ => None,
        }
    }

    /// Convert into a specific message type if it matches
    pub fn into_mt101(self) -> Option<SwiftMessage<messages::MT101>> {
        match self {
            ParsedSwiftMessage::MT101(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt103(self) -> Option<SwiftMessage<messages::MT103>> {
        match self {
            ParsedSwiftMessage::MT103(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt104(self) -> Option<SwiftMessage<messages::MT104>> {
        match self {
            ParsedSwiftMessage::MT104(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt107(self) -> Option<SwiftMessage<messages::MT107>> {
        match self {
            ParsedSwiftMessage::MT107(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt110(self) -> Option<SwiftMessage<messages::MT110>> {
        match self {
            ParsedSwiftMessage::MT110(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt111(self) -> Option<SwiftMessage<messages::MT111>> {
        match self {
            ParsedSwiftMessage::MT111(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt112(self) -> Option<SwiftMessage<messages::MT112>> {
        match self {
            ParsedSwiftMessage::MT112(msg) => Some(*msg),
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
    pub fn into_mt210(self) -> Option<SwiftMessage<messages::MT210>> {
        match self {
            ParsedSwiftMessage::MT210(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt900(self) -> Option<SwiftMessage<messages::MT900>> {
        match self {
            ParsedSwiftMessage::MT900(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt910(self) -> Option<SwiftMessage<messages::MT910>> {
        match self {
            ParsedSwiftMessage::MT910(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt920(self) -> Option<SwiftMessage<messages::MT920>> {
        match self {
            ParsedSwiftMessage::MT920(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt935(self) -> Option<SwiftMessage<messages::MT935>> {
        match self {
            ParsedSwiftMessage::MT935(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt940(self) -> Option<SwiftMessage<messages::MT940>> {
        match self {
            ParsedSwiftMessage::MT940(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt941(self) -> Option<SwiftMessage<messages::MT941>> {
        match self {
            ParsedSwiftMessage::MT941(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt942(self) -> Option<SwiftMessage<messages::MT942>> {
        match self {
            ParsedSwiftMessage::MT942(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt950(self) -> Option<SwiftMessage<messages::MT950>> {
        match self {
            ParsedSwiftMessage::MT950(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt192(self) -> Option<SwiftMessage<messages::MT192>> {
        match self {
            ParsedSwiftMessage::MT192(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt196(self) -> Option<SwiftMessage<messages::MT196>> {
        match self {
            ParsedSwiftMessage::MT196(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt292(self) -> Option<SwiftMessage<messages::MT292>> {
        match self {
            ParsedSwiftMessage::MT292(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt296(self) -> Option<SwiftMessage<messages::MT296>> {
        match self {
            ParsedSwiftMessage::MT296(msg) => Some(*msg),
            _ => None,
        }
    }
}

impl<T: SwiftMessageBody> SwiftMessage<T> {
    /// Check if this message contains reject codes (MT103 specific)
    ///
    /// Reject messages are identified by checking:
    /// 1. Field 20 (Sender's Reference) for "REJT" prefix
    /// 2. Block 3 field 108 (MUR - Message User Reference) for "REJT"
    /// 3. Field 72 (Sender to Receiver Information) containing `/REJT/` code
    pub fn has_reject_codes(&self) -> bool {
        // Check Block 3 field 108 (MUR - Message User Reference)
        if let Some(ref user_header) = self.user_header {
            if let Some(ref mur) = user_header.message_user_reference {
                if mur.to_uppercase().contains("REJT") {
                    return true;
                }
            }
        }

        if let Some(mt103_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT103>()
        {
            return mt103_fields.has_reject_codes();
        } else if let Some(mt202_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT202>()
        {
            return mt202_fields.has_reject_codes();
        } else if let Some(mt205_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT205>()
        {
            return mt205_fields.has_reject_codes();
        }

        false
    }

    /// Check if this message contains return codes (MT103 specific)
    ///
    /// Return messages are identified by checking:
    /// 1. Field 20 (Sender's Reference) for "RETN" prefix
    /// 2. Block 3 field 108 (MUR - Message User Reference) for "RETN"
    /// 3. Field 72 (Sender to Receiver Information) containing `/RETN/` code
    pub fn has_return_codes(&self) -> bool {
        // Check Block 3 field 108 (MUR - Message User Reference)
        if let Some(ref user_header) = self.user_header {
            if let Some(ref mur) = user_header.message_user_reference {
                if mur.to_uppercase().contains("RETN") {
                    return true;
                }
            }
        }

        if let Some(mt103_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT103>()
        {
            return mt103_fields.has_return_codes();
        } else if let Some(mt202_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT202>()
        {
            return mt202_fields.has_return_codes();
        } else if let Some(mt205_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT205>()
        {
            return mt205_fields.has_return_codes();
        }

        false
    }

    pub fn is_cover_message(&self) -> bool {
        if let Some(mt202_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT202>()
        {
            return mt202_fields.is_cover_message();
        }
        if let Some(mt205_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT205>()
        {
            return mt205_fields.is_cover_message();
        }

        false
    }

    pub fn is_stp_message(&self) -> bool {
        if let Some(mt103_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT103>()
        {
            return mt103_fields.is_stp_compliant();
        }

        false
    }

    /// Validate message against business rules using JSONLogic
    /// This validation method has access to both headers and message fields,
    /// allowing for comprehensive validation of MT103 and other message types.
    pub fn validate_business_rules(&self) -> ValidationResult {
        // Check if the message type has validation rules
        let validation_rules = match T::message_type() {
            "101" => messages::MT101::validation_rules(),
            "103" => messages::MT103::validation_rules(),
            "104" => messages::MT104::validation_rules(),
            "107" => messages::MT107::validation_rules(),
            "110" => messages::MT110::validation_rules(),
            "111" => messages::MT111::validation_rules(),
            "112" => messages::MT112::validation_rules(),
            "202" => messages::MT202::validation_rules(),
            "205" => messages::MT205::validation_rules(),
            "210" => messages::MT210::validation_rules(),
            "900" => messages::MT900::validation_rules(),
            "910" => messages::MT910::validation_rules(),
            "920" => messages::MT920::validation_rules(),
            "935" => messages::MT935::validation_rules(),
            "940" => messages::MT940::validation_rules(),
            "941" => messages::MT941::validation_rules(),
            "942" => messages::MT942::validation_rules(),
            "950" => messages::MT950::validation_rules(),
            "192" => messages::MT192::validation_rules(),
            "196" => messages::MT196::validation_rules(),
            "292" => messages::MT292::validation_rules(),
            "296" => messages::MT296::validation_rules(),
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
                    message: format!("Failed to parse validation rules JSON: {e}"),
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
                    message: format!("Failed to create validation context: {e}"),
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
                .unwrap_or_else(|| format!("RULE_{rule_index}"));

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
                                        "Business rule validation failed: {rule_id} - {rule_description}"
                                    ),
                                });
                            }
                            None => {
                                // Rule returned non-boolean value
                                warnings.push(format!(
                                    "Rule {rule_id} returned non-boolean value: {result:?}"
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        // JSONLogic evaluation error
                        errors.push(ValidationError::BusinessRuleValidation {
                            rule_name: rule_id.clone(),
                            message: format!("JSONLogic evaluation error for rule {rule_id}: {e}"),
                        });
                    }
                }
            } else {
                warnings.push(format!("Rule {rule_id} has no condition"));
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
                    message: format!("Failed to serialize complete message: {e}"),
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

    pub fn to_mt_message(&self) -> String {
        let mut swift_message = String::new();

        // Block 1: Basic Header
        let block1 = &self.basic_header.to_string();
        swift_message.push_str(&format!("{{1:{block1}}}\n"));

        // Block 2: Application Header
        let block2 = &self.application_header.to_string();
        swift_message.push_str(&format!("{{2:{block2}}}\n"));

        // Block 3: User Header (if present)
        if let Some(ref user_header) = self.user_header {
            let block3 = &user_header.to_string();
            swift_message.push_str(&format!("{{3:{block3}}}\n"));
        }

        // Block 4: Text Block with fields
        let field_map = self.fields.to_fields();
        let mut block4 = String::new();

        // Get optional field tags for this message type to determine which fields can be skipped
        let optional_fields: std::collections::HashSet<String> = T::optional_fields()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        // Use field_order to maintain proper field sequence
        for field_tag in &self.field_order {
            if let Some(field_values) = field_map.get(field_tag) {
                for field_value in field_values {
                    // Skip empty optional fields
                    if optional_fields.contains(field_tag) && field_value.trim().is_empty() {
                        continue;
                    }

                    // field_value already includes the field tag prefix from to_swift_string()
                    // but we need to check if it starts with ':' to avoid double prefixing
                    if field_value.starts_with(':') {
                        // Value already has field tag prefix, use as-is
                        block4.push_str(&format!("\n{field_value}"));
                    } else {
                        // Value doesn't have field tag prefix, add it
                        block4.push_str(&format!("\n:{field_tag}:{field_value}"));
                    }
                }
            }
        }

        // Handle any fields not in field_order (shouldn't happen in normal cases)
        for (field_tag, field_values) in &field_map {
            if !self.field_order.contains(field_tag) {
                for field_value in field_values {
                    // Skip empty optional fields
                    if optional_fields.contains(field_tag) && field_value.trim().is_empty() {
                        continue;
                    }

                    if field_value.starts_with(':') {
                        block4.push_str(&format!("\n{field_value}"));
                    } else {
                        block4.push_str(&format!("\n:{field_tag}:{field_value}"));
                    }
                }
            }
        }

        swift_message.push_str(&format!("{{4:{block4}\n-}}"));
        swift_message.push('\n');

        // Block 5: Trailer (if present)
        if let Some(ref trailer) = self.trailer {
            let block5 = &trailer.to_string();
            swift_message.push_str(&format!("{{5:{block5}}}\n"));
        }

        swift_message
    }
}
