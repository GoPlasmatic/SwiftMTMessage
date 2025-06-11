//! Error types for SWIFT MT message parsing
//!
//! This module provides comprehensive error types with rich diagnostics for better debugging
//! and error reporting. All errors include context information when available.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, ParseError>;

/// Main error type for SWIFT MT parsing
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum ParseError {
    #[error("Missing required block: {block} (at line {line}, column {column}): {message}")]
    MissingRequiredBlock {
        block: String,
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Invalid block format (at line {line}, column {column}): {message}")]
    InvalidBlockFormat {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Unknown block number: {block_number} (at line {line}, column {column})")]
    UnknownBlockNumber {
        block_number: String,
        line: usize,
        column: usize,
    },

    #[error("No blocks found in message: {message}")]
    NoBlocksFound { message: String },

    #[error("Field parse error: {tag} - {message}")]
    FieldParseError { tag: String, message: String },

    #[error("Unknown field: {tag} in block {block}")]
    UnknownField { tag: String, block: String },

    #[error("Invalid field format: {tag} (expected {expected}, got {actual})")]
    InvalidFieldFormat {
        tag: String,
        expected: String,
        actual: String,
    },

    #[error("Missing required field: {tag} for message type {message_type}")]
    MissingRequiredField { tag: String, message_type: String },

    #[error("Invalid field length: {tag} (max {max_length}, got {actual_length})")]
    InvalidFieldLength {
        tag: String,
        max_length: usize,
        actual_length: usize,
    },

    #[error("Invalid currency code: {code}")]
    InvalidCurrencyCode { code: String },

    #[error("Amount parse error: {message}")]
    AmountParseError { message: String },

    #[error("Date parse error: {message}")]
    DateParseError { message: String },

    #[error("Time parse error: {message}")]
    TimeParseError { message: String },

    #[error("Wrong message type: expected {expected}, got {actual}")]
    WrongMessageType { expected: String, actual: String },

    #[error("Unsupported message type: {message_type}")]
    UnsupportedMessageType { message_type: String },

    #[error("Invalid message structure: {message}")]
    InvalidMessageStructure { message: String },

    #[error("Regex error: {message}")]
    RegexError { message: String },

    #[error("IO error: {message}")]
    IoError { message: String },

    #[error("Serialization error: {message}")]
    SerializationError { message: String },

    #[error("Validation error: {message}")]
    ValidationError { message: String },

    #[error("Format rule error: {message}")]
    FormatRuleError { message: String },

    #[error("JSON error: {message}")]
    JsonError { message: String },
}

/// Field-specific parse errors
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum FieldParseError {
    #[error("Invalid field usage: {0}")]
    InvalidUsage(String),

    #[error("Unknown field option: {option} for field {tag}")]
    UnknownOption { tag: String, option: String },

    #[error("Invalid field option: {option} for field {field}. Valid options: {valid_options:?}")]
    InvalidFieldOption {
        field: String,
        option: String,
        valid_options: Vec<String>,
    },

    #[error("Invalid field length: {field} (max {max_length}, got {actual_length})")]
    InvalidLength {
        field: String,
        max_length: usize,
        actual_length: usize,
    },

    #[error("Invalid field format: {field} - {message}")]
    InvalidFormat { field: String, message: String },

    #[error("Missing required data: {field} - {message}")]
    MissingData { field: String, message: String },

    #[error("Parse error: {message}")]
    ParseError { message: String },
}

/// Validation-specific errors
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum ValidationError {
    #[error("Validation failed for field {tag}: {message}")]
    FieldValidationFailed { tag: String, message: String },

    #[error("Format rule validation failed: {rule} - {message}")]
    FormatRuleValidationFailed { rule: String, message: String },

    #[error("Business rule validation failed: {rule} - {message}")]
    BusinessRuleValidationFailed { rule: String, message: String },

    #[error("Cross-field validation failed: fields {fields:?} - {message}")]
    CrossFieldValidationFailed {
        fields: Vec<String>,
        message: String,
    },

    #[error("Message structure validation failed: {message}")]
    MessageStructureValidationFailed { message: String },
}

/// Conversion from regex::Error
impl From<regex::Error> for ParseError {
    fn from(err: regex::Error) -> Self {
        ParseError::RegexError {
            message: err.to_string(),
        }
    }
}

/// Conversion from std::io::Error
impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IoError {
            message: err.to_string(),
        }
    }
}

/// Conversion from FieldParseError to ParseError
impl From<FieldParseError> for ParseError {
    fn from(err: FieldParseError) -> Self {
        ParseError::FieldParseError {
            tag: "unknown".to_string(),
            message: err.to_string(),
        }
    }
}

/// Conversion from ValidationError to ParseError
impl From<ValidationError> for ParseError {
    fn from(err: ValidationError) -> Self {
        ParseError::ValidationError {
            message: err.to_string(),
        }
    }
}

/// Conversion from serde_json::Error
impl From<serde_json::Error> for ParseError {
    fn from(err: serde_json::Error) -> Self {
        ParseError::JsonError {
            message: err.to_string(),
        }
    }
}

/// Helper functions for creating common errors
impl ParseError {
    pub fn missing_required_field(tag: &str) -> Self {
        ParseError::MissingRequiredField {
            tag: tag.to_string(),
            message_type: "unknown".to_string(),
        }
    }

    pub fn missing_required_field_for_type(tag: &str, message_type: &str) -> Self {
        ParseError::MissingRequiredField {
            tag: tag.to_string(),
            message_type: message_type.to_string(),
        }
    }

    pub fn invalid_field_format(tag: &str, expected: &str, actual: &str) -> Self {
        ParseError::InvalidFieldFormat {
            tag: tag.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
        }
    }

    pub fn field_parse_error(tag: &str, message: &str) -> Self {
        ParseError::FieldParseError {
            tag: tag.to_string(),
            message: message.to_string(),
        }
    }
}

impl FieldParseError {
    pub fn invalid_length(field: &str, max_length: usize, actual_length: usize) -> Self {
        FieldParseError::InvalidLength {
            field: field.to_string(),
            max_length,
            actual_length,
        }
    }

    pub fn invalid_format(field: &str, message: &str) -> Self {
        FieldParseError::InvalidFormat {
            field: field.to_string(),
            message: message.to_string(),
        }
    }

    pub fn missing_data(field: &str, message: &str) -> Self {
        FieldParseError::MissingData {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}

impl ValidationError {
    pub fn field_validation_failed(tag: &str, message: &str) -> Self {
        ValidationError::FieldValidationFailed {
            tag: tag.to_string(),
            message: message.to_string(),
        }
    }

    pub fn format_rule_validation_failed(rule: &str, message: &str) -> Self {
        ValidationError::FormatRuleValidationFailed {
            rule: rule.to_string(),
            message: message.to_string(),
        }
    }

    pub fn business_rule_validation_failed(rule: &str, message: &str) -> Self {
        ValidationError::BusinessRuleValidationFailed {
            rule: rule.to_string(),
            message: message.to_string(),
        }
    }
}

/// Error result with context for better debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub error: ParseError,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub field_tag: Option<String>,
    pub message_type: Option<String>,
    pub raw_content: Option<String>,
}

impl ErrorContext {
    pub fn new(error: ParseError) -> Self {
        Self {
            error,
            line: None,
            column: None,
            field_tag: None,
            message_type: None,
            raw_content: None,
        }
    }

    pub fn with_location(mut self, line: usize, column: usize) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }

    pub fn with_field(mut self, field_tag: String) -> Self {
        self.field_tag = Some(field_tag);
        self
    }

    pub fn with_message_type(mut self, message_type: String) -> Self {
        self.message_type = Some(message_type);
        self
    }

    pub fn with_raw_content(mut self, raw_content: String) -> Self {
        self.raw_content = Some(raw_content);
        self
    }
}

/// Aggregate multiple errors for batch processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCollection {
    pub errors: Vec<ErrorContext>,
    pub warnings: Vec<String>,
}

impl ErrorCollection {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: ErrorContext) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}

impl Default for ErrorCollection {
    fn default() -> Self {
        Self::new()
    }
}

// For backward compatibility with existing error module
// Re-export common error types with old names
pub use ParseError as MTError;
pub type MTResult<T> = Result<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = ParseError::missing_required_field("20");
        assert!(matches!(error, ParseError::MissingRequiredField { .. }));
    }

    #[test]
    fn test_error_context() {
        let error = ParseError::FieldParseError {
            tag: "20".to_string(),
            message: "Invalid format".to_string(),
        };
        let context = ErrorContext::new(error)
            .with_location(5, 10)
            .with_field("20".to_string())
            .with_message_type("103".to_string());

        assert_eq!(context.line, Some(5));
        assert_eq!(context.column, Some(10));
        assert_eq!(context.field_tag, Some("20".to_string()));
        assert_eq!(context.message_type, Some("103".to_string()));
    }

    #[test]
    fn test_error_collection() {
        let mut collection = ErrorCollection::new();
        assert!(!collection.has_errors());

        let error = ErrorContext::new(ParseError::missing_required_field("20"));
        collection.add_error(error);
        collection.add_warning("This is a test warning".to_string());

        assert!(collection.has_errors());
        assert!(collection.has_warnings());
        assert_eq!(collection.error_count(), 1);
        assert_eq!(collection.warning_count(), 1);
    }
}
