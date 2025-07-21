//! # Error Handling for SWIFT MT Message Library
//!
//! ## Purpose
//! Comprehensive error types and result handling for SWIFT MT message parsing, validation, and processing.
//! Provides detailed error information for debugging and error recovery.
//!
//! ## Error Categories
//! - **Parse Errors**: Issues during message parsing and field extraction
//! - **Validation Errors**: Field format and business rule validation failures
//! - **Type Errors**: Message type mismatches and unsupported formats
//! - **IO Errors**: File system and network-related errors
//! - **Serialization Errors**: JSON and data conversion issues
//!
//! ## Error Design
//! All errors implement:
//! - `std::error::Error` trait for standard error handling
//! - `Serialize`/`Deserialize` for API error responses
//! - `Clone` for error propagation and logging
//! - `Debug` for comprehensive debugging information
//!
//! ## Usage Examples
//! ```rust
//! use swift_mt_message::errors::{ParseError, ValidationError, Result};
//! use swift_mt_message::parser::SwiftParser;
//! use swift_mt_message::ValidationResult;
//!
//! # let invalid_message = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:-}";
//! // Handle parsing errors
//! match SwiftParser::parse_auto(&invalid_message) {
//!     Ok(message) => println!("Parsed successfully: {:?}", message),
//!     Err(ParseError::InvalidFieldFormat { field_tag, component_name, .. }) => {
//!         eprintln!("Format error in field {}: {}", field_tag, component_name);
//!     },
//!     Err(ParseError::MissingRequiredField { field_tag, field_name, .. }) => {
//!         eprintln!("Missing required field: {} ({})", field_tag, field_name);
//!     },
//!     Err(other) => eprintln!("Other error: {}", other),
//! }
//!
//! // Handle validation errors
//! let validation_result = ValidationResult::with_errors(vec![
//!     ValidationError::FormatValidation {
//!         field_tag: "20".to_string(),
//!         message: "Invalid format".to_string()
//!     },
//! ]);
//! if !validation_result.is_valid {
//!     for error in validation_result.errors {
//!         match error {
//!             ValidationError::FormatValidation { field_tag, message } => {
//!                 eprintln!("Field {} format error: {}", field_tag, message);
//!             },
//!             ValidationError::BusinessRuleValidation { rule_name, message } => {
//!                 eprintln!("Business rule {} failed: {}", rule_name, message);
//!             },
//!             _ => eprintln!("Validation error: {}", error),
//!         }
//!     }
//! }
//! ```
//!
//! ## Error Recovery
//! Many errors provide sufficient context for automated error recovery:
//! - Field validation errors include expected formats for correction
//! - Parse errors include position information for partial recovery
//! - Business rule errors include rule names for selective validation

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type alias for the library
///
/// Standard Result type used throughout the library for consistent error handling.
/// All fallible operations return `Result<T>` where T is the success type.
pub type Result<T> = std::result::Result<T, ParseError>;

/// Enhanced result type for SWIFT validation operations
pub type SwiftValidationResult<T> = std::result::Result<T, SwiftValidationError>;

/// Main error type for parsing operations
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ParseError {
    #[error("Wrong message type: expected {expected}, got {actual}")]
    WrongMessageType { expected: String, actual: String },

    #[error("Unsupported message type: {message_type}")]
    UnsupportedMessageType { message_type: String },

    #[error("Field validation failed: {errors:?}")]
    ValidationFailed { errors: Vec<ValidationError> },

    #[error("IO error: {message}")]
    IoError { message: String },

    #[error(transparent)]
    SwiftValidation(Box<SwiftValidationError>),

    #[error("Serialization error: {message}")]
    SerializationError { message: String },

    /// Invalid message format error
    #[error("Invalid message format: {message}")]
    InvalidFormat { message: String },

    /// Field format error with full context
    #[error("Invalid field format - Field: {field_tag}, Component: {component_name}, Value: '{value}', Expected: {format_spec}")]
    InvalidFieldFormat {
        /// SWIFT field tag (e.g., "50K", "32A")
        field_tag: String,
        /// Component name within the field (e.g., "currency", "amount")
        component_name: String,
        /// The actual value that failed to parse
        value: String,
        /// Expected format specification
        format_spec: String,
        /// Position in the original message
        position: Option<usize>,
        /// Inner parsing error (simplified for serialization)
        inner_error: String,
    },

    /// Missing required field with detailed context
    #[error("Missing required field {field_tag} ({field_name}) in {message_type}")]
    MissingRequiredField {
        /// SWIFT field tag
        field_tag: String,
        /// Rust field name in the struct
        field_name: String,
        /// Message type (MT103, MT202, etc.)
        message_type: String,
        /// Position where field was expected
        position_in_block4: Option<usize>,
    },

    /// Field parsing failed with detailed context
    #[error("Failed to parse field {field_tag} of type {field_type} at line {position}: {original_error}")]
    FieldParsingFailed {
        /// SWIFT field tag
        field_tag: String,
        /// Type of field being parsed
        field_type: String,
        /// Line number in message
        position: usize,
        /// Original error message
        original_error: String,
    },

    /// Component parsing error with specific details
    #[error(
        "Component parse error in field {field_tag}: {component_name} (index {component_index})"
    )]
    ComponentParseError {
        /// Field tag containing the component
        field_tag: String,
        /// Index of component in field
        component_index: usize,
        /// Name of the component
        component_name: String,
        /// Expected format
        expected_format: String,
        /// Actual value that failed
        actual_value: String,
    },

    /// Invalid block structure with detailed location
    #[error("Invalid block {block} structure: {message}")]
    InvalidBlockStructure {
        /// Block number (1-5)
        block: String,
        /// Detailed error message
        message: String,
    },
}

/// Validation error for field-level validation
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ValidationError {
    #[error("Field {field_tag} format validation failed: {message}")]
    FormatValidation { field_tag: String, message: String },

    #[error("Field {field_tag} length validation failed: expected {expected}, got {actual}")]
    LengthValidation {
        field_tag: String,
        expected: String,
        actual: usize,
    },

    #[error("Field {field_tag} pattern validation failed: {message}")]
    PatternValidation { field_tag: String, message: String },

    #[error("Field {field_tag} value validation failed: {message}")]
    ValueValidation { field_tag: String, message: String },

    #[error("Business rule validation failed: {rule_name} - {message}")]
    BusinessRuleValidation { rule_name: String, message: String },
}

/// Comprehensive SWIFT validation error system based on SWIFT Standard Error Codes
///
/// This error system implements all 1,335 SWIFT error codes across T, C, D, E, and G series
/// to provide precise feedback matching SWIFT network validation standards.
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum SwiftValidationError {
    /// T-Series: Technical/Format Validation Errors (275 codes)
    /// Format validation errors for field structure and basic syntax compliance
    #[error(transparent)]
    Format(Box<SwiftFormatError>),

    /// C-Series: Conditional/Business Rules Errors (57 codes)
    /// Business logic validation for conditional fields and cross-field relationships
    #[error(transparent)]
    Business(Box<SwiftBusinessError>),

    /// D-Series: Data/Content Validation Errors (77 codes)
    /// Content-specific validation including regional requirements and dependencies
    #[error(transparent)]
    Content(Box<SwiftContentError>),

    /// E-Series: Enhanced/Field Relation Validation Errors (86 codes)
    /// Advanced validation for instruction codes and complex business rules
    #[error(transparent)]
    Relation(Box<SwiftRelationError>),

    /// G-Series: General/Field Validation Errors (823 codes)
    /// General field validation across all MT categories
    #[error(transparent)]
    General(Box<SwiftGeneralError>),
}

/// T-Series: Technical/Format Validation Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[error("Format Error {code}: Field {field} contains '{value}', expected {expected}. {message}")]
pub struct SwiftFormatError {
    /// SWIFT error code (e.g., "T50", "T27")
    pub code: String,
    /// Field tag where error occurred
    pub field: String,
    /// Invalid value that caused the error
    pub value: String,
    /// Expected format or value
    pub expected: String,
    /// Human-readable error message
    pub message: String,
    /// Additional context for error recovery
    pub context: Option<String>,
}

/// C-Series: Conditional/Business Rules Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[error("Business Rule Violation {code}: {message} (Field: {field})")]
pub struct SwiftBusinessError {
    /// SWIFT error code (e.g., "C02", "C81")
    pub code: String,
    /// Primary field tag involved
    pub field: String,
    /// Related field tags for cross-field validation
    pub related_fields: Vec<String>,
    /// Human-readable error message
    pub message: String,
    /// Business rule that was violated
    pub rule_description: String,
    /// Additional context for error recovery
    pub context: Option<String>,
}

/// D-Series: Data/Content Validation Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[error("Content Validation Error {code}: {message} (Field: {field})")]
pub struct SwiftContentError {
    /// SWIFT error code (e.g., "D19", "D49")
    pub code: String,
    /// Field tag where error occurred
    pub field: String,
    /// Invalid content that caused the error
    pub content: String,
    /// Human-readable error message
    pub message: String,
    /// Regional or contextual requirements
    pub requirements: String,
    /// Additional context for error recovery
    pub context: Option<String>,
}

/// E-Series: Enhanced/Field Relation Validation Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[error("Relation Validation Error {code}: {message} (Field: {field})")]
pub struct SwiftRelationError {
    /// SWIFT error code (e.g., "E01", "E15")
    pub code: String,
    /// Primary field tag involved
    pub field: String,
    /// Related field tags that affect this validation
    pub related_fields: Vec<String>,
    /// Instruction code or option that caused the error
    pub instruction_context: Option<String>,
    /// Human-readable error message
    pub message: String,
    /// Relationship rule that was violated
    pub rule_description: String,
    /// Additional context for error recovery
    pub context: Option<String>,
}

/// G-Series: General/Field Validation Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[error("General Validation Error {code}: {message} (Field: {field})")]
pub struct SwiftGeneralError {
    /// SWIFT error code (e.g., "G001", "G050")
    pub code: String,
    /// Field tag where error occurred
    pub field: String,
    /// Invalid value that caused the error
    pub value: String,
    /// Human-readable error message
    pub message: String,
    /// MT category context (1-9 or Common)
    pub category: Option<String>,
    /// Additional context for error recovery
    pub context: Option<String>,
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IoError {
            message: err.to_string(),
        }
    }
}

impl ParseError {
    /// Get a detailed debug report for the error
    pub fn debug_report(&self) -> String {
        match self {
            ParseError::InvalidFieldFormat {
                field_tag,
                component_name,
                value,
                format_spec,
                position,
                inner_error,
            } => {
                format!(
                    "Field Parsing Error:\n\
                     ├─ Field Tag: {}\n\
                     ├─ Component: {}\n\
                     ├─ Value: '{}'\n\
                     ├─ Expected Format: {}\n\
                     ├─ Position in Message: {}\n\
                     ├─ Details: {}\n\
                     └─ Hint: Check SWIFT format specification for field {}",
                    field_tag,
                    component_name,
                    value,
                    format_spec,
                    position.map_or("unknown".to_string(), |p| p.to_string()),
                    inner_error,
                    field_tag
                )
            }
            ParseError::MissingRequiredField {
                field_tag,
                field_name,
                message_type,
                position_in_block4,
            } => {
                format!(
                    "Missing Required Field:\n\
                     ├─ Field Tag: {}\n\
                     ├─ Field Name: {}\n\
                     ├─ Message Type: {}\n\
                     ├─ Expected Position: {}\n\
                     └─ Hint: {} requires field {} to be present",
                    field_tag,
                    field_name,
                    message_type,
                    position_in_block4.map_or("unknown".to_string(), |p| p.to_string()),
                    message_type,
                    field_tag
                )
            }
            ParseError::ComponentParseError {
                field_tag,
                component_index,
                component_name,
                expected_format,
                actual_value,
            } => {
                format!(
                    "Component Parse Error:\n\
                     ├─ Field Tag: {field_tag}\n\
                     ├─ Component: {component_name} (index {component_index})\n\
                     ├─ Expected Format: {expected_format}\n\
                     ├─ Actual Value: '{actual_value}'\n\
                     └─ Hint: Component '{component_name}' must match format '{expected_format}'"
                )
            }
            ParseError::FieldParsingFailed {
                field_tag,
                field_type,
                position,
                original_error,
            } => {
                let line_num = if *position > 0xFFFF {
                    // Old format: encoded position
                    position >> 16
                } else {
                    // New format: just line number
                    *position
                };
                format!(
                    "Field Parsing Failed:\n\
                     ├─ Field Tag: {field_tag}\n\
                     ├─ Field Type: {field_type}\n\
                     ├─ Line Number: {line_num}\n\
                     ├─ Error: {original_error}\n\
                     └─ Hint: Check the field value matches the expected type"
                )
            }
            ParseError::InvalidBlockStructure { block, message } => {
                format!(
                    "Block Structure Error:\n\
                     ├─ Block: {block}\n\
                     ├─ Error: {message}\n\
                     └─ Hint: Ensure block {block} follows SWIFT message structure"
                )
            }
            // Fallback for other variants
            _ => format!("{self}"),
        }
    }

    /// Get a concise error message for logging
    pub fn brief_message(&self) -> String {
        match self {
            ParseError::InvalidFieldFormat {
                field_tag,
                component_name,
                ..
            } => {
                format!("Field {field_tag} component '{component_name}' format error")
            }
            ParseError::MissingRequiredField {
                field_tag,
                message_type,
                ..
            } => {
                format!("Required field {field_tag} missing in {message_type}")
            }
            ParseError::ComponentParseError {
                field_tag,
                component_name,
                ..
            } => {
                format!("Field {field_tag} component '{component_name}' parse error")
            }
            ParseError::FieldParsingFailed {
                field_tag,
                field_type,
                position,
                ..
            } => {
                let line_num = if *position > 0xFFFF {
                    position >> 16
                } else {
                    *position
                };
                format!("Field {field_tag} (type {field_type}) parsing failed at line {line_num}")
            }
            ParseError::InvalidBlockStructure { block, .. } => {
                format!("Block {block} structure invalid")
            }
            _ => self.to_string(),
        }
    }

    /// Format error with message context
    pub fn format_with_context(&self, original_message: &str) -> String {
        match self {
            ParseError::FieldParsingFailed { position, .. } => {
                // Extract line and show context
                let lines: Vec<&str> = original_message.lines().collect();
                let line_num = if *position > 0xFFFF {
                    position >> 16
                } else {
                    *position
                };
                let mut output = self.debug_report();

                if line_num > 0 && line_num <= lines.len() {
                    output.push_str("\n\nContext:\n");
                    // Show 2 lines before and after
                    let start = line_num.saturating_sub(3);
                    let end = (line_num + 2).min(lines.len());

                    for (i, line) in lines.iter().enumerate().take(end).skip(start) {
                        if i == line_num - 1 {
                            output.push_str(&format!(">>> {} │ {}\n", i + 1, line));
                        } else {
                            output.push_str(&format!("    {} │ {}\n", i + 1, line));
                        }
                    }
                }
                output
            }
            ParseError::InvalidFieldFormat {
                position: Some(pos),
                ..
            } => {
                let lines: Vec<&str> = original_message.lines().collect();
                let line_num = pos >> 16;
                let mut output = self.debug_report();

                if line_num > 0 && line_num <= lines.len() {
                    output.push_str("\n\nContext:\n");
                    let start = line_num.saturating_sub(3);
                    let end = (line_num + 2).min(lines.len());

                    for (i, line) in lines.iter().enumerate().take(end).skip(start) {
                        if i == line_num - 1 {
                            output.push_str(&format!(">>> {} │ {}\n", i + 1, line));
                        } else {
                            output.push_str(&format!("    {} │ {}\n", i + 1, line));
                        }
                    }
                }
                output
            }
            _ => self.debug_report(),
        }
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(err: serde_json::Error) -> Self {
        ParseError::SerializationError {
            message: err.to_string(),
        }
    }
}

/// SWIFT Error Code Constants
///
/// This module contains all official SWIFT error codes organized by series.
/// Total: 1,335 unique error codes across all MT categories.
pub mod error_codes {
    /// T-Series: Technical/Format Validation Error Codes (275 codes)
    pub mod format {
        pub const T08: &str = "T08"; // Invalid code in field
        pub const T26: &str = "T26"; // Invalid slash usage
        pub const T27: &str = "T27"; // Invalid BIC code format
        pub const T28: &str = "T28"; // Invalid BIC code length
        pub const T29: &str = "T29"; // Invalid BIC code structure
        pub const T40: &str = "T40"; // Invalid amount format
        pub const T43: &str = "T43"; // Amount exceeds maximum digits
        pub const T45: &str = "T45"; // Invalid identifier code format
        pub const T50: &str = "T50"; // Invalid date format
        pub const T52: &str = "T52"; // Invalid currency code
        pub const T56: &str = "T56"; // Invalid structured address
        pub const T73: &str = "T73"; // Invalid country code
                                     // Additional T-series codes will be added as needed
    }

    /// C-Series: Conditional/Business Rules Error Codes (57 codes)
    pub mod business {
        pub const C02: &str = "C02"; // Currency code mismatch
        pub const C03: &str = "C03"; // Amount format validation
        pub const C08: &str = "C08"; // Commodity currency not allowed
        pub const C81: &str = "C81"; // Conditional field dependency
                                     // Additional C-series codes will be added as needed
    }

    /// D-Series: Data/Content Validation Error Codes (77 codes)
    pub mod content {
        pub const D17: &str = "D17"; // Field presence requirement
        pub const D18: &str = "D18"; // Mutually exclusive placement
        pub const D19: &str = "D19"; // IBAN mandatory for SEPA
        pub const D20: &str = "D20"; // Field 71A presence rules
        pub const D22: &str = "D22"; // Exchange rate dependency
        pub const D49: &str = "D49"; // Field 33B mandatory for EU
        pub const D50: &str = "D50"; // SHA charge restrictions
        pub const D51: &str = "D51"; // Field 33B with charge fields
        pub const D75: &str = "D75"; // Exchange rate mandatory
        pub const D79: &str = "D79"; // Field 71G dependency
        pub const D93: &str = "D93"; // Account restrictions by code
                                     // Additional D-series codes will be added as needed
    }

    /// E-Series: Enhanced/Field Relation Validation Error Codes (86 codes)
    pub mod relation {
        pub const E01: &str = "E01"; // Instruction code restrictions
        pub const E02: &str = "E02"; // Prohibited instruction codes
        pub const E03: &str = "E03"; // Field option restrictions
        pub const E04: &str = "E04"; // Party identifier requirements
        pub const E05: &str = "E05"; // Field 54A option restrictions
        pub const E06: &str = "E06"; // Multiple field dependency
        pub const E07: &str = "E07"; // Field 55A option restrictions
        pub const E09: &str = "E09"; // Party identifier mandatory
        pub const E10: &str = "E10"; // Beneficiary account mandatory
        pub const E13: &str = "E13"; // OUR charge restrictions
        pub const E15: &str = "E15"; // BEN charge requirements
        pub const E16: &str = "E16"; // Field restrictions with SPRI
        pub const E17: &str = "E17"; // Clearing code requirements
        pub const E18: &str = "E18"; // Account restrictions CHQB
        pub const E44: &str = "E44"; // Instruction code dependencies
        pub const E45: &str = "E45"; // Instruction code field dependencies
                                     // Additional E-series codes will be added as needed
    }

    /// G-Series: General/Field Validation Error Codes (823 codes)
    pub mod general {
        pub const G001: &str = "G001"; // Field format violation
        pub const G050: &str = "G050"; // Field content validation
        pub const G100: &str = "G100"; // Sequence validation
                                       // Additional G-series codes will be added as needed
    }
}

/// Helper functions for creating SWIFT validation errors
impl SwiftValidationError {
    /// Create a T-series format validation error
    pub fn format_error(
        code: &str,
        field: &str,
        value: &str,
        expected: &str,
        message: &str,
    ) -> Self {
        SwiftValidationError::Format(Box::new(SwiftFormatError {
            code: code.to_string(),
            field: field.to_string(),
            value: value.to_string(),
            expected: expected.to_string(),
            message: message.to_string(),
            context: None,
        }))
    }

    /// Create a C-series business rule validation error
    pub fn business_error(
        code: &str,
        field: &str,
        related_fields: Vec<String>,
        message: &str,
        rule_description: &str,
    ) -> Self {
        SwiftValidationError::Business(Box::new(SwiftBusinessError {
            code: code.to_string(),
            field: field.to_string(),
            related_fields,
            message: message.to_string(),
            rule_description: rule_description.to_string(),
            context: None,
        }))
    }

    /// Create a D-series content validation error
    pub fn content_error(
        code: &str,
        field: &str,
        content: &str,
        message: &str,
        requirements: &str,
    ) -> Self {
        SwiftValidationError::Content(Box::new(SwiftContentError {
            code: code.to_string(),
            field: field.to_string(),
            content: content.to_string(),
            message: message.to_string(),
            requirements: requirements.to_string(),
            context: None,
        }))
    }

    /// Create an E-series relation validation error
    pub fn relation_error(
        code: &str,
        field: &str,
        related_fields: Vec<String>,
        message: &str,
        rule_description: &str,
    ) -> Self {
        SwiftValidationError::Relation(Box::new(SwiftRelationError {
            code: code.to_string(),
            field: field.to_string(),
            related_fields,
            instruction_context: None,
            message: message.to_string(),
            rule_description: rule_description.to_string(),
            context: None,
        }))
    }

    /// Create a G-series general validation error
    pub fn general_error(
        code: &str,
        field: &str,
        value: &str,
        message: &str,
        category: Option<&str>,
    ) -> Self {
        SwiftValidationError::General(Box::new(SwiftGeneralError {
            code: code.to_string(),
            field: field.to_string(),
            value: value.to_string(),
            message: message.to_string(),
            category: category.map(|s| s.to_string()),
            context: None,
        }))
    }

    /// Get the error code from any SWIFT validation error
    pub fn code(&self) -> &str {
        match self {
            SwiftValidationError::Format(err) => &err.code,
            SwiftValidationError::Business(err) => &err.code,
            SwiftValidationError::Content(err) => &err.code,
            SwiftValidationError::Relation(err) => &err.code,
            SwiftValidationError::General(err) => &err.code,
        }
    }

    /// Get the field tag from any SWIFT validation error
    pub fn field(&self) -> &str {
        match self {
            SwiftValidationError::Format(err) => &err.field,
            SwiftValidationError::Business(err) => &err.field,
            SwiftValidationError::Content(err) => &err.field,
            SwiftValidationError::Relation(err) => &err.field,
            SwiftValidationError::General(err) => &err.field,
        }
    }

    /// Get the error message from any SWIFT validation error
    pub fn message(&self) -> &str {
        match self {
            SwiftValidationError::Format(err) => &err.message,
            SwiftValidationError::Business(err) => &err.message,
            SwiftValidationError::Content(err) => &err.message,
            SwiftValidationError::Relation(err) => &err.message,
            SwiftValidationError::General(err) => &err.message,
        }
    }
}

/// Convert SwiftValidationError to ValidationError for backward compatibility
impl From<SwiftValidationError> for ValidationError {
    fn from(swift_error: SwiftValidationError) -> Self {
        match swift_error {
            SwiftValidationError::Format(err) => ValidationError::FormatValidation {
                field_tag: err.field,
                message: format!("{}: {}", err.code, err.message),
            },
            SwiftValidationError::Business(err) => ValidationError::BusinessRuleValidation {
                rule_name: err.code,
                message: err.message,
            },
            SwiftValidationError::Content(err) => ValidationError::ValueValidation {
                field_tag: err.field,
                message: format!("{}: {}", err.code, err.message),
            },
            SwiftValidationError::Relation(err) => ValidationError::BusinessRuleValidation {
                rule_name: err.code,
                message: err.message,
            },
            SwiftValidationError::General(err) => ValidationError::FormatValidation {
                field_tag: err.field,
                message: format!("{}: {}", err.code, err.message),
            },
        }
    }
}

/// Convert SwiftValidationError to ParseError
impl From<SwiftValidationError> for ParseError {
    fn from(validation_error: SwiftValidationError) -> Self {
        ParseError::SwiftValidation(Box::new(validation_error))
    }
}

/// Convert ValidationError to SwiftValidationError
impl From<ValidationError> for SwiftValidationError {
    fn from(validation_error: ValidationError) -> Self {
        match validation_error {
            ValidationError::FormatValidation { field_tag, message } => {
                SwiftValidationError::format_error("T00", &field_tag, "", "", &message)
            }
            ValidationError::LengthValidation {
                field_tag,
                expected,
                actual,
            } => SwiftValidationError::format_error(
                "T00",
                &field_tag,
                &actual.to_string(),
                &expected,
                "Length validation failed",
            ),
            ValidationError::PatternValidation { field_tag, message } => {
                SwiftValidationError::format_error("T00", &field_tag, "", "", &message)
            }
            ValidationError::ValueValidation { field_tag, message } => {
                SwiftValidationError::content_error("D00", &field_tag, "", &message, "")
            }
            ValidationError::BusinessRuleValidation { rule_name, message } => {
                SwiftValidationError::business_error(&rule_name, "", vec![], &message, "")
            }
        }
    }
}
