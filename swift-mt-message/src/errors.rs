use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type alias for the library
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
