//! Error types for Swift MT message macro processing

use proc_macro2::{Span, TokenStream};
use std::fmt;

/// Comprehensive error type for macro processing failures
#[derive(Debug)]
pub enum MacroError {
    /// Syntax parsing error from syn
    Parse(syn::Error),

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

    /// Internal macro processing error
    Internal { span: Span, message: String },
}

impl MacroError {
    /// Convert error to compile error tokens
    pub fn to_compile_error(&self) -> TokenStream {
        match self {
            MacroError::Parse(syn_error) => syn_error.to_compile_error(),

            MacroError::UnsupportedType {
                span,
                type_name,
                context,
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
            } => {
                let message = format!("Missing required attribute '{attribute}' in {context}");
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }

            MacroError::InvalidAttribute {
                span,
                attribute,
                value,
                expected,
            } => {
                let message = format!(
                    "Invalid value '{value}' for attribute '{attribute}', expected: {expected}"
                );
                quote::quote_spanned! { *span =>
                    compile_error!(#message);
                }
            }

            MacroError::Internal { span, message } => {
                let full_message = format!("Internal macro error: {message}");
                quote::quote_spanned! { *span =>
                    compile_error!(#full_message);
                }
            }
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
            MacroError::Parse(syn_error) => write!(f, "Parse error: {syn_error}"),
            MacroError::UnsupportedType {
                type_name, context, ..
            } => {
                write!(f, "Unsupported type '{type_name}' in {context}")
            }
            MacroError::MissingAttribute {
                attribute, context, ..
            } => {
                write!(f, "Missing required attribute '{attribute}' in {context}")
            }
            MacroError::InvalidAttribute {
                attribute,
                value,
                expected,
                ..
            } => {
                write!(
                    f,
                    "Invalid value '{value}' for attribute '{attribute}', expected: {expected}"
                )
            }
            MacroError::Internal { message, .. } => write!(f, "Internal error: {message}"),
        }
    }
}

/// Result type for macro operations
pub type MacroResult<T> = Result<T, MacroError>;
