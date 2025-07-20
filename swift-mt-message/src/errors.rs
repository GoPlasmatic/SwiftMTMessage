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
//!
//! // Handle parsing errors
//! match SwiftParser::parse_auto(&invalid_message) {
//!     Ok(message) => println!("Parsed successfully: {:?}", message),
//!     Err(ParseError::InvalidFormat { message }) => {
//!         eprintln!("Format error: {}", message);
//!     },
//!     Err(ParseError::MissingRequiredField { field_tag }) => {
//!         eprintln!("Missing required field: {}", field_tag);
//!     },
//!     Err(other) => eprintln!("Other error: {}", other),
//! }
//!
//! // Handle validation errors
//! match message.validate_business_rules() {
//!     validation_result if !validation_result.is_valid => {
//!         for error in validation_result.errors {
//!             match error {
//!                 ValidationError::FormatValidation { field_tag, message } => {
//!                     eprintln!("Field {} format error: {}", field_tag, message);
//!                 },
//!                 ValidationError::BusinessRuleValidation { rule_name, message } => {
//!                     eprintln!("Business rule {} failed: {}", rule_name, message);
//!                 },
//!                 _ => eprintln!("Validation error: {}", error),
//!             }
//!         }
//!     },
//!     _ => println!("Validation passed"),
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

/// Main error type for parsing operations
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ParseError {
    #[error("Invalid message format: {message}")]
    InvalidFormat { message: String },

    #[error("Missing required field: {field_tag}")]
    MissingRequiredField { field_tag: String },

    #[error("Invalid field format for {field_tag}: {message}")]
    InvalidFieldFormat { field_tag: String, message: String },

    #[error("Wrong message type: expected {expected}, got {actual}")]
    WrongMessageType { expected: String, actual: String },

    #[error("Invalid block structure: {message}")]
    InvalidBlockStructure { message: String },

    #[error("Unsupported message type: {message_type}")]
    UnsupportedMessageType { message_type: String },

    #[error("Field validation failed: {errors:?}")]
    ValidationFailed { errors: Vec<ValidationError> },

    #[error("IO error: {message}")]
    IoError { message: String },

    #[error("Serialization error: {message}")]
    SerializationError { message: String },
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

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IoError {
            message: err.to_string(),
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
