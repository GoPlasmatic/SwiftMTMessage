//! Error types for SWIFT MT message parsing

use thiserror::Error;

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, MTError>;

/// Main error type for SWIFT MT message parsing
#[derive(Debug, Error)]
pub enum MTError {
    #[error("Parse error at line {line}, column {column}: {message}")]
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Validation error in field {field}: {message}")]
    ValidationError {
        field: String,
        message: String,
    },

    #[error("Unsupported message type: {message_type}")]
    UnsupportedMessageType {
        message_type: String,
    },

    #[error("Field not found: {field_tag}")]
    FieldNotFound {
        field_tag: String,
    },

    #[error("Invalid field format in field {field}: {message}")]
    InvalidFieldFormat {
        field: String,
        message: String,
    },

    #[error("Missing required field: {field_tag}")]
    MissingRequiredField {
        field_tag: String,
    },

    #[error("Invalid message structure: {message}")]
    InvalidMessageStructure {
        message: String,
    },

    #[error("Date parsing error: {message}")]
    DateParseError {
        message: String,
    },

    #[error("Amount parsing error: {message}")]
    AmountParseError {
        message: String,
    },

    #[error("Currency code error: {message}")]
    CurrencyError {
        message: String,
    },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
}

impl MTError {
    /// Create a new parse error
    pub fn parse_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::ParseError {
            line,
            column,
            message: message.into(),
        }
    }

    /// Create a new validation error
    pub fn validation_error(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ValidationError {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a new field not found error
    pub fn field_not_found(field_tag: impl Into<String>) -> Self {
        Self::FieldNotFound {
            field_tag: field_tag.into(),
        }
    }

    /// Create a new invalid field format error
    pub fn invalid_field_format(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidFieldFormat {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a new missing required field error
    pub fn missing_required_field(field_tag: impl Into<String>) -> Self {
        Self::MissingRequiredField {
            field_tag: field_tag.into(),
        }
    }
} 