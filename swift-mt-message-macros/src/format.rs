//! SWIFT field formatting utilities for macro code generation
//!
//! This module provides utilities for formatting and parsing SWIFT field components
//! according to SWIFT format specifications like "16x", "5n", "4!c", etc.
//! Used by the derive macros to generate formatting code.

use crate::error::{MacroError, MacroResult};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use std::fmt;

/// SWIFT format specification parser and formatter
#[derive(Debug, Clone, PartialEq)]
pub struct FormatSpec {
    pub pattern: String,
    pub length: Option<usize>,
    pub max_length: Option<usize>,
    pub format_type: FormatType,
    pub is_fixed: bool,
    pub is_optional: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormatType {
    /// Alphabetic characters only
    Alpha,
    /// Numeric characters only
    Numeric,
    /// Alphanumeric characters
    Alphanumeric,
    /// Character set (uppercase letters, digits, and specific symbols)
    CharacterSet,
    /// Any character except CR, LF, and certain control characters
    AnyCharacter,
    /// Decimal number with fractional part
    Decimal,
    /// Date format (YYMMDD)
    Date,
    /// Time format (HHMM or HHMMSS)
    Time,
}

impl FormatSpec {
    /// Parse a SWIFT format specification string
    /// Examples: "16x", "5n", "4!c", "[/2n]", "15d"
    pub fn parse(format_str: &str) -> MacroResult<Self> {
        let mut spec = FormatSpec {
            pattern: format_str.to_string(),
            length: None,
            max_length: None,
            format_type: FormatType::AnyCharacter,
            is_fixed: false,
            is_optional: false,
        };

        let mut working_str = format_str;

        // Check for optional format [content]
        if working_str.starts_with('[') && working_str.ends_with(']') {
            spec.is_optional = true;
            working_str = &working_str[1..working_str.len()-1];
        }

        // Handle prefix characters like '/'
        if working_str.starts_with('/') {
            working_str = &working_str[1..];
        }

        // Try to parse simple format specifications first
        if let Ok(re) = Regex::new(r"^(\d*)(!?)([a-zA-Z])$") {
            if let Some(captures) = re.captures(working_str) {
                // Extract length
                let length_str = captures.get(1).map_or("", |m| m.as_str());
                if !length_str.is_empty() {
                    if let Ok(length) = length_str.parse() {
                        spec.length = Some(length);
                        spec.max_length = spec.length;
                    }
                }

                // Check if fixed length (!)
                spec.is_fixed = captures.get(2).map_or(false, |m| m.as_str() == "!");

                // Extract format type
                let format_char = captures.get(3).map_or("", |m| m.as_str());
                spec.format_type = match format_char {
                    "a" => FormatType::Alpha,
                    "n" => FormatType::Numeric,
                    "c" => FormatType::CharacterSet,
                    "x" => FormatType::AnyCharacter,
                    "d" => FormatType::Decimal,
                    "y" => FormatType::Date,
                    "t" => FormatType::Time,
                    _ => FormatType::AnyCharacter,
                };
            } else {
                // For complex patterns that we can't parse, just default to AnyCharacter
                spec.format_type = FormatType::AnyCharacter;
                spec.length = Some(16); // Default length
            }
        } else {
            // Fallback for any other format
            spec.format_type = FormatType::AnyCharacter;
            spec.length = Some(16);
        }

        Ok(spec)
    }

    /// Generate validation code for this format specification
    pub fn generate_validation_code(&self, value_expr: &TokenStream) -> TokenStream {
        let mut validations = Vec::new();

        // Length validations
        if let Some(length) = self.length {
            if self.is_fixed {
                validations.push(quote! {
                    if #value_expr.len() != #length {
                        return Err(crate::errors::ParseError::InvalidFormat {
                            message: format!(
                                "Fixed length {} required, got {} characters",
                                #length,
                                #value_expr.len()
                            ),
                        });
                    }
                });
            } else {
                validations.push(quote! {
                    if #value_expr.len() > #length {
                        return Err(crate::errors::ParseError::InvalidFormat {
                            message: format!(
                                "Maximum length {} exceeded, got {} characters",
                                #length,
                                #value_expr.len()
                            ),
                        });
                    }
                });
            }
        }

        // Character type validations
        match self.format_type {
            FormatType::Alpha => {
                validations.push(quote! {
                    if !#value_expr.chars().all(|c| c.is_alphabetic()) {
                        return Err(crate::errors::ParseError::InvalidFormat {
                            message: "Value must contain only alphabetic characters".to_string(),
                        });
                    }
                });
            }
            FormatType::Numeric => {
                validations.push(quote! {
                    if !#value_expr.chars().all(|c| c.is_ascii_digit()) {
                        return Err(crate::errors::ParseError::InvalidFormat {
                            message: "Value must contain only numeric characters".to_string(),
                        });
                    }
                });
            }
            FormatType::CharacterSet => {
                validations.push(quote! {
                    if !#value_expr.chars().all(|c| c.is_uppercase() || c.is_ascii_digit() || " /.,()-".contains(c)) {
                        return Err(crate::errors::ParseError::InvalidFormat {
                            message: "Value contains invalid characters for character set format".to_string(),
                        });
                    }
                });
            }
            FormatType::Decimal => {
                validations.push(quote! {
                    if #value_expr.parse::<f64>().is_err() {
                        return Err(crate::errors::ParseError::InvalidFormat {
                            message: "Value must be a valid decimal number".to_string(),
                        });
                    }
                });
            }
            FormatType::Date => {
                validations.push(quote! {
                    if #value_expr.len() != 6 || !#value_expr.chars().all(|c| c.is_ascii_digit()) {
                        return Err(crate::errors::ParseError::InvalidFormat {
                            message: "Date must be in YYMMDD format".to_string(),
                        });
                    }
                });
            }
            FormatType::Time => {
                validations.push(quote! {
                    if (#value_expr.len() != 4 && #value_expr.len() != 6) || !#value_expr.chars().all(|c| c.is_ascii_digit()) {
                        return Err(crate::errors::ParseError::InvalidFormat {
                            message: "Time must be in HHMM or HHMMSS format".to_string(),
                        });
                    }
                });
            }
            FormatType::AnyCharacter | FormatType::Alphanumeric => {
                validations.push(quote! {
                    if #value_expr.chars().any(|c| c == '\r' || c == '\n') {
                        return Err(crate::errors::ParseError::InvalidFormat {
                            message: "Value cannot contain carriage return or line feed".to_string(),
                        });
                    }
                });
            }
        }

        quote! {
            #(#validations)*
        }
    }
}

/// Generate formatting code for a component value according to a SWIFT format pattern
/// This function generates TokenStream for use in derive macros
pub fn generate_format_code(value_expr: &TokenStream, pattern: &str) -> MacroResult<TokenStream> {
    let spec = FormatSpec::parse(pattern)?;
    
    let format_code = if spec.is_fixed && spec.length.is_some() {
        let length = spec.length.unwrap();
        match spec.format_type {
            FormatType::Numeric => {
                quote! {
                    format!("{:0width$}", #value_expr, width = #length)
                }
            }
            _ => {
                quote! {
                    format!("{:width$}", #value_expr, width = #length)
                }
            }
        }
    } else if let Some(max_len) = spec.max_length {
        quote! {
            {
                let mut result = #value_expr.to_string();
                if result.len() > #max_len {
                    result.truncate(#max_len);
                }
                result
            }
        }
    } else {
        quote! {
            #value_expr.to_string()
        }
    };

    Ok(format_code)
}

/// Generate sample value code for a given format pattern
/// This is used by the derive macros for sample generation
pub fn generate_sample_code(pattern: &str) -> MacroResult<TokenStream> {
    let spec = FormatSpec::parse(pattern)?;
    let length = spec.length.unwrap_or(10).min(35); // Reasonable default/max
    
    let sample_value = match spec.format_type {
        FormatType::Alpha => {
            let sample = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .chars()
                .cycle()
                .take(length)
                .collect::<String>();
            quote! { #sample.to_string() }
        }
        FormatType::Numeric => {
            let sample = "1234567890"
                .chars()
                .cycle()
                .take(length)
                .collect::<String>();
            quote! { #sample.to_string() }
        }
        FormatType::CharacterSet => {
            let sample = "ABCD1234"
                .chars()
                .cycle()
                .take(length)
                .collect::<String>();
            quote! { #sample.to_string() }
        }
        FormatType::Decimal => {
            if length <= 5 {
                quote! { "12.34".to_string() }
            } else {
                quote! { "1234.56".to_string() }
            }
        }
        FormatType::Date => quote! { "231215".to_string() }, // December 15, 2023
        FormatType::Time => quote! { "1430".to_string() },   // 2:30 PM
        FormatType::AnyCharacter | FormatType::Alphanumeric => {
            let sample = "ABC123XYZ"
                .chars()
                .cycle()
                .take(length)
                .collect::<String>();
            quote! { #sample.to_string() }
        }
    };

    Ok(sample_value)
}

/// Runtime format component function - generates code that will format at runtime
pub fn generate_runtime_format_code(value_expr: &TokenStream, pattern: &str) -> TokenStream {
    quote! {
        {
            let value_str = #value_expr.to_string();
            // Apply basic formatting - could be enhanced with more sophisticated logic
            value_str
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_spec_parsing() {
        let spec = FormatSpec::parse("16x").unwrap();
        assert_eq!(spec.length, Some(16));
        assert_eq!(spec.format_type, FormatType::AnyCharacter);
        assert!(!spec.is_fixed);

        let spec = FormatSpec::parse("4!c").unwrap();
        assert_eq!(spec.length, Some(4));
        assert_eq!(spec.format_type, FormatType::CharacterSet);
        assert!(spec.is_fixed);

        let spec = FormatSpec::parse("[/2n]").unwrap();
        assert_eq!(spec.length, Some(2));
        assert_eq!(spec.format_type, FormatType::Numeric);
        assert!(spec.is_optional);
    }

    #[test]
    fn test_sample_generation() {
        let sample = generate_sample_code("4!c").unwrap();
        // Should generate some valid TokenStream
        assert!(!sample.is_empty());
    }
}