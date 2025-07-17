//! Error types for Swift MT message macro processing

use proc_macro2::{Span, TokenStream};
use std::fmt;

/// Comprehensive error type for macro processing failures
#[derive(Debug)]
pub enum MacroError {
    /// Syntax parsing error from syn
    Parse(syn::Error),
    
    /// Invalid format specification
    InvalidFormat {
        span: Span,
        format: String,
        reason: String,
    },
    
    /// Unsupported field type
    UnsupportedType {
        span: Span,
        type_name: String,
        context: String,
    },
    
    /// Missing required attribute
    MissingAttribute {
        span: Span,
        attribute: String,
        context: String,
    },
    
    /// Invalid attribute value
    InvalidAttribute {
        span: Span,
        attribute: String,
        value: String,
        expected: String,
    },
    
    /// Conflicting attributes or configurations
    Conflict {
        span: Span,
        message: String,
    },
    
    /// Internal macro processing error
    Internal {
        span: Span,
        message: String,
    },
}

impl MacroError {
    /// Convert error to compile error tokens
    pub fn to_compile_error(&self) -> TokenStream {
        match self {
            MacroError::Parse(syn_error) => syn_error.to_compile_error(),
            
            MacroError::InvalidFormat { span, format, reason } => {
                let message = format!("Invalid SWIFT format '{}': {}", format, reason);
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }
            
            MacroError::UnsupportedType { span, type_name, context } => {
                let message = format!("Unsupported type '{}' in {}", type_name, context);
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }
            
            MacroError::MissingAttribute { span, attribute, context } => {
                let message = format!("Missing required attribute '{}' in {}", attribute, context);
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }
            
            MacroError::InvalidAttribute { span, attribute, value, expected } => {
                let message = format!(
                    "Invalid value '{}' for attribute '{}', expected: {}",
                    value, attribute, expected
                );
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }
            
            MacroError::Conflict { span, message } => {
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }
            
            MacroError::Internal { span, message } => {
                let full_message = format!("Internal macro error: {}", message);
                quote::quote_spanned! { *span =>
                    compile_error!(#full_message);
                }
            }
        }
    }
    
    /// Create an invalid format error
    pub fn invalid_format(span: Span, format: &str, reason: &str) -> Self {
        MacroError::InvalidFormat {
            span,
            format: format.to_string(),
            reason: reason.to_string(),
        }
    }
    
    /// Create an unsupported type error
    pub fn unsupported_type(span: Span, type_name: &str, context: &str) -> Self {
        MacroError::UnsupportedType {
            span,
            type_name: type_name.to_string(),
            context: context.to_string(),
        }
    }
    
    /// Create a missing attribute error
    pub fn missing_attribute(span: Span, attribute: &str, context: &str) -> Self {
        MacroError::MissingAttribute {
            span,
            attribute: attribute.to_string(),
            context: context.to_string(),
        }
    }
    
    /// Create an invalid attribute error
    pub fn invalid_attribute(span: Span, attribute: &str, value: &str, expected: &str) -> Self {
        MacroError::InvalidAttribute {
            span,
            attribute: attribute.to_string(),
            value: value.to_string(),
            expected: expected.to_string(),
        }
    }
    
    /// Create a conflict error
    pub fn conflict(span: Span, message: &str) -> Self {
        MacroError::Conflict {
            span,
            message: message.to_string(),
        }
    }
    
    /// Create an internal error
    pub fn internal(span: Span, message: &str) -> Self {
        MacroError::Internal {
            span,
            message: message.to_string(),
        }
    }
}

impl From<syn::Error> for MacroError {
    fn from(error: syn::Error) -> Self {
        MacroError::Parse(error)
    }
}

impl fmt::Display for MacroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MacroError::Parse(syn_error) => write!(f, "Parse error: {}", syn_error),
            MacroError::InvalidFormat { format, reason, .. } => {
                write!(f, "Invalid SWIFT format '{}': {}", format, reason)
            }
            MacroError::UnsupportedType { type_name, context, .. } => {
                write!(f, "Unsupported type '{}' in {}", type_name, context)
            }
            MacroError::MissingAttribute { attribute, context, .. } => {
                write!(f, "Missing required attribute '{}' in {}", attribute, context)
            }
            MacroError::InvalidAttribute { attribute, value, expected, .. } => {
                write!(f, "Invalid value '{}' for attribute '{}', expected: {}", value, attribute, expected)
            }
            MacroError::Conflict { message, .. } => write!(f, "Conflict: {}", message),
            MacroError::Internal { message, .. } => write!(f, "Internal error: {}", message),
        }
    }
}

/// Result type for macro operations
pub type MacroResult<T> = Result<T, MacroError>;

/// Helper trait for adding context to errors
pub trait ErrorContext<T> {
    fn with_context(self, context: &str) -> MacroResult<T>;
}

impl<T, E> ErrorContext<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn with_context(self, context: &str) -> MacroResult<T> {
        self.map_err(|e| MacroError::internal(Span::call_site(), &format!("{}: {}", context, e)))
    }
}