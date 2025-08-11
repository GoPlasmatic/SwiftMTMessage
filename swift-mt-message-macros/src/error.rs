//! Error types for Swift MT message macro processing

use proc_macro2::{Span, TokenStream};
use thiserror::Error;

/// Comprehensive error type for macro processing failures with enhanced context
#[derive(Debug, Error)]
#[allow(dead_code)] // Phase 3 infrastructure - variants will be used as the codebase grows
pub enum MacroError {
    /// Syntax parsing error from syn
    #[error("Parse error: {0}")]
    Parse(#[from] syn::Error),

    /// Invalid format specification
    #[error("Invalid format specification '{spec}' for field '{field}': {reason}")]
    InvalidFormat {
        spec: String,
        field: String,
        reason: String,
        span: Span,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Unsupported field type
    #[error("Unsupported type '{type_name}' in {context}")]
    UnsupportedType {
        span: Span,
        type_name: String,
        context: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Missing required attribute
    #[error("Missing required attribute '{attribute}' in {context}")]
    MissingAttribute {
        span: Span,
        attribute: String,
        context: String,
        suggestion: Option<String>,
    },

    /// Invalid attribute value
    #[error("Invalid value '{value}' for attribute '{attribute}', expected: {expected}")]
    InvalidAttribute {
        span: Span,
        attribute: String,
        value: String,
        expected: String,
        suggestion: Option<String>,
    },

    /// Regex compilation error
    #[error("Failed to compile regex pattern '{pattern}': {reason}")]
    RegexError {
        pattern: String,
        reason: String,
        span: Span,
        #[source]
        source: Option<regex::Error>,
    },

    /// Type conversion error
    #[error("Cannot convert from type '{from_type}' to '{to_type}' in {context}")]
    TypeConversion {
        from_type: String,
        to_type: String,
        context: String,
        span: Span,
        suggestion: Option<String>,
    },

    /// Field validation error
    #[error("Field validation failed for '{field_name}': {reason}")]
    FieldValidation {
        field_name: String,
        reason: String,
        span: Span,
        suggestion: Option<String>,
    },

    /// Internal macro processing error
    #[error("Internal macro error: {message}")]
    Internal {
        span: Span,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

#[allow(dead_code)] // Phase 3 infrastructure - methods will be used as error handling is enhanced
impl MacroError {
    /// Convert error to compile error tokens with enhanced context
    pub fn to_compile_error(&self) -> TokenStream {
        match self {
            MacroError::Parse(syn_error) => syn_error.to_compile_error(),

            MacroError::InvalidFormat {
                spec,
                field,
                reason,
                span,
                ..
            } => {
                let message =
                    format!("Invalid format specification '{spec}' for field '{field}': {reason}");
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }

            MacroError::UnsupportedType {
                span,
                type_name,
                context,
                ..
            } => {
                let message = format!("Unsupported type '{type_name}' in {context}");
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }

            MacroError::MissingAttribute {
                span,
                attribute,
                context,
                suggestion,
            } => {
                let mut message = format!("Missing required attribute '{attribute}' in {context}");
                if let Some(suggestion) = suggestion {
                    message.push_str(&format!(". Suggestion: {suggestion}"));
                }
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }

            MacroError::InvalidAttribute {
                span,
                attribute,
                value,
                expected,
                suggestion,
            } => {
                let mut message = format!(
                    "Invalid value '{value}' for attribute '{attribute}', expected: {expected}"
                );
                if let Some(suggestion) = suggestion {
                    message.push_str(&format!(". Suggestion: {suggestion}"));
                }
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }

            MacroError::RegexError {
                pattern,
                reason,
                span,
                ..
            } => {
                let message = format!("Failed to compile regex pattern '{pattern}': {reason}");
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }

            MacroError::TypeConversion {
                from_type,
                to_type,
                context,
                span,
                suggestion,
            } => {
                let mut message =
                    format!("Cannot convert from type '{from_type}' to '{to_type}' in {context}");
                if let Some(suggestion) = suggestion {
                    message.push_str(&format!(". Suggestion: {suggestion}"));
                }
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }

            MacroError::FieldValidation {
                field_name,
                reason,
                span,
                suggestion,
            } => {
                let mut message = format!("Field validation failed for '{field_name}': {reason}");
                if let Some(suggestion) = suggestion {
                    message.push_str(&format!(". Suggestion: {suggestion}"));
                }
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }

            MacroError::Internal { span, message, .. } => {
                let full_message = format!("Internal macro error: {message}");
                quote::quote_spanned! { *span =>
                    compile_error!(#full_message);
                }
            }
        }
    }

    /// Add span information to any error
    pub fn with_span(mut self, span: Span) -> Self {
        match &mut self {
            MacroError::Parse(_syn_error) => {
                // Cannot modify syn::Error span, return as-is
                self
            }
            MacroError::InvalidFormat {
                span: error_span, ..
            } => {
                *error_span = span;
                self
            }
            MacroError::UnsupportedType {
                span: error_span, ..
            } => {
                *error_span = span;
                self
            }
            MacroError::MissingAttribute {
                span: error_span, ..
            } => {
                *error_span = span;
                self
            }
            MacroError::InvalidAttribute {
                span: error_span, ..
            } => {
                *error_span = span;
                self
            }
            MacroError::RegexError {
                span: error_span, ..
            } => {
                *error_span = span;
                self
            }
            MacroError::TypeConversion {
                span: error_span, ..
            } => {
                *error_span = span;
                self
            }
            MacroError::FieldValidation {
                span: error_span, ..
            } => {
                *error_span = span;
                self
            }
            MacroError::Internal {
                span: error_span, ..
            } => {
                *error_span = span;
                self
            }
        }
    }

    /// Create an invalid format error with context
    pub fn invalid_format(
        span: Span,
        spec: &str,
        field: &str,
        reason: &str,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        MacroError::InvalidFormat {
            spec: spec.to_string(),
            field: field.to_string(),
            reason: reason.to_string(),
            span,
            source,
        }
    }

    /// Create an unsupported type error
    pub fn unsupported_type(span: Span, type_name: &str, context: &str) -> Self {
        MacroError::UnsupportedType {
            span,
            type_name: type_name.to_string(),
            context: context.to_string(),
            source: None,
        }
    }

    /// Create a missing attribute error with suggestion
    pub fn missing_attribute(span: Span, attribute: &str, context: &str) -> Self {
        let suggestion = match attribute {
            "format" => Some("Add #[format(\"pattern\")] attribute".to_string()),
            "field" => Some("Add #[field(\"tag\")] attribute".to_string()),
            _ => None,
        };

        MacroError::MissingAttribute {
            span,
            attribute: attribute.to_string(),
            context: context.to_string(),
            suggestion,
        }
    }

    /// Create an invalid attribute error with suggestion
    pub fn invalid_attribute(span: Span, attribute: &str, value: &str, expected: &str) -> Self {
        let suggestion = match attribute {
            "format" => Some("Use valid SWIFT format like \"3!a\", \"6!n\", \"15d\"".to_string()),
            "field" => Some("Use valid field tag like \"20\", \"23B\", \"50K\"".to_string()),
            _ => None,
        };

        MacroError::InvalidAttribute {
            span,
            attribute: attribute.to_string(),
            value: value.to_string(),
            expected: expected.to_string(),
            suggestion,
        }
    }

    /// Create a regex compilation error
    pub fn regex_error(span: Span, pattern: &str, source: regex::Error) -> Self {
        MacroError::RegexError {
            pattern: pattern.to_string(),
            reason: source.to_string(),
            span,
            source: Some(source),
        }
    }

    /// Create a type conversion error
    pub fn type_conversion(
        span: Span,
        from_type: &str,
        to_type: &str,
        context: &str,
        suggestion: Option<String>,
    ) -> Self {
        MacroError::TypeConversion {
            from_type: from_type.to_string(),
            to_type: to_type.to_string(),
            context: context.to_string(),
            span,
            suggestion,
        }
    }

    /// Create a field validation error
    pub fn field_validation(
        span: Span,
        field_name: &str,
        reason: &str,
        suggestion: Option<String>,
    ) -> Self {
        MacroError::FieldValidation {
            field_name: field_name.to_string(),
            reason: reason.to_string(),
            span,
            suggestion,
        }
    }

    /// Create an internal error with source
    pub fn internal(span: Span, message: &str) -> Self {
        MacroError::Internal {
            span,
            message: message.to_string(),
            source: None,
        }
    }

    /// Create an internal error with source
    pub fn internal_with_source(
        span: Span,
        message: &str,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        MacroError::Internal {
            span,
            message: message.to_string(),
            source: Some(source),
        }
    }
}

/// Error recovery strategies for common macro issues
#[allow(dead_code)] // Phase 3 infrastructure - recovery functions will be used as error handling is enhanced
pub mod recovery {
    use super::{MacroError, MacroResult};
    use proc_macro2::Span;

    /// Attempt to recover from invalid format specification by suggesting valid alternatives
    pub fn suggest_valid_format(invalid_spec: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Common format pattern corrections
        if invalid_spec.contains('!')
            && !invalid_spec.ends_with('a')
            && !invalid_spec.ends_with('n')
            && !invalid_spec.ends_with('c')
        {
            suggestions.push(
                "Try using 'a' for alphabetic, 'n' for numeric, or 'c' for character set"
                    .to_string(),
            );
        }

        if invalid_spec.chars().any(|c| c.is_ascii_digit())
            && !invalid_spec.contains('!')
            && !invalid_spec.ends_with('x')
            && invalid_spec.len() <= 3
        {
            suggestions.push(format!(
                    "Did you mean '{invalid_spec}!a' for fixed alphabetic or '{invalid_spec}x' for variable?"
                ));
        }

        // Common valid patterns
        if suggestions.is_empty() {
            suggestions.extend([
                "3!a (fixed 3 alphabetic)".to_string(),
                "6!n (fixed 6 numeric)".to_string(),
                "35x (variable up to 35 characters)".to_string(),
                "15d (decimal up to 15 digits)".to_string(),
            ]);
        }

        suggestions
    }

    /// Attempt to recover from missing attribute by providing helpful context
    pub fn recover_missing_attribute(
        attribute: &str,
        context: &str,
        span: Span,
    ) -> MacroResult<String> {
        match (attribute, context) {
            ("format", _) => Ok(
                "Consider adding #[format(\"3!a\")] or similar SWIFT format specification"
                    .to_string(),
            ),
            ("field", "SwiftMessage") => {
                Ok("Consider adding #[field(\"20\")] with the appropriate field tag".to_string())
            }
            _ => Err(MacroError::missing_attribute(span, attribute, context)),
        }
    }
}

/// Result type for macro operations
pub type MacroResult<T> = Result<T, MacroError>;
