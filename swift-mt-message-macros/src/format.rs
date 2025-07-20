//! SWIFT field formatting utilities for macro code generation
//!
//! This module provides utilities for formatting and parsing SWIFT field components
//! according to SWIFT format specifications like "16x", "5n", "4!c", etc.
//! Used by the derive macros to generate formatting code.
//!
//! ## SWIFT Format Specifications
//!
//! The format system supports all standard SWIFT format types:
//!
//! ### Basic Format Types
//! - `n` - Numeric (0-9)
//! - `a` - Alphabetic (A-Z, uppercase only)
//! - `c` - Character set (A-Z, 0-9, and certain symbols)
//! - `x` - Any printable character
//! - `d` - Decimal number with optional fractional part
//!
//! ### Length Specifications  
//! - `3!a` - Fixed length (exactly 3 alphabetic characters)
//! - `16x` - Variable length (up to 16 any characters)
//! - `15d` - Decimal number (up to 15 digits with optional decimal point)
//!
//! ### Optional and Repetitive Patterns
//! - `[35x]` - Optional field (can be empty)
//! - `4*35x` - Repetitive field (up to 4 lines of 35 characters each)
//! - `[/34x]` - Optional with literal prefix (starts with "/" if present)
//!
//! ### Complex Patterns
//! - `4!a2!a2!c[3!c]` - Multi-component patterns (BIC codes)
//! - `[/1!a][/34x]` - Compound optional patterns
//! - `4*(1!n/33x)` - Numbered line patterns

use crate::ast::StructField;
use crate::error::MacroResult;
use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use syn::Type;

/// Efficient pattern building functions to avoid string allocations
/// Build decimal pattern without format! macro
fn build_decimal_pattern(length: usize) -> String {
    // Use string literals for common cases
    match length {
        1 => "(\\d{1}(?:[.,]\\d+)?)".to_string(),
        2 => "(\\d{1,2}(?:[.,]\\d+)?)".to_string(),
        3 => "(\\d{1,3}(?:[.,]\\d+)?)".to_string(),
        6 => "(\\d{1,6}(?:[.,]\\d+)?)".to_string(),
        12 => "(\\d{1,12}(?:[.,]\\d+)?)".to_string(),
        15 => "(\\d{1,15}(?:[.,]\\d+)?)".to_string(),
        18 => "(\\d{1,18}(?:[.,]\\d+)?)".to_string(),
        _ => {
            // Only fallback to format! for uncommon lengths
            format!(r"(\d{{1,{length}}}(?:[.,]\d+)?)")
        }
    }
}

/// Build fixed-length pattern efficiently
fn build_fixed_pattern(char_class: &str, length: usize) -> String {
    // Pre-built patterns for common combinations
    match (char_class, length) {
        ("[A-Z]", 3) => "([A-Z]{3})".to_string(),
        ("[A-Z]", 4) => "([A-Z]{4})".to_string(),
        ("[A-Z]", 8) => "([A-Z]{8})".to_string(),
        ("[A-Z]", 11) => "([A-Z]{11})".to_string(),
        ("\\d", 1) => "(\\d{1})".to_string(),
        ("\\d", 2) => "(\\d{2})".to_string(),
        ("\\d", 3) => "(\\d{3})".to_string(),
        ("\\d", 4) => "(\\d{4})".to_string(),
        ("\\d", 6) => "(\\d{6})".to_string(),
        ("\\d", 8) => "(\\d{8})".to_string(),
        ("[A-Z0-9]", 1) => "([A-Z0-9]{1})".to_string(),
        ("[A-Z0-9]", 3) => "([A-Z0-9]{3})".to_string(),
        ("[A-Z0-9]", 4) => "([A-Z0-9]{4})".to_string(),
        (".", 35) => "(.{35})".to_string(),
        _ => format!("({char_class}{{{length}}})"),
    }
}

/// Build variable-length pattern efficiently
fn build_variable_pattern(char_class: &str, length: usize) -> String {
    // Pre-built patterns for common combinations
    match (char_class, length) {
        ("[A-Z]", 35) => "([A-Z]{1,35})".to_string(),
        ("\\d", 15) => "(\\d{1,15})".to_string(),
        ("\\d", 16) => "(\\d{1,16})".to_string(),
        (".", 16) => "(.{1,16})".to_string(),
        (".", 35) => "(.{1,35})".to_string(),
        (".", 50) => "(.{1,50})".to_string(),
        ("[A-Z0-9]", 8) => "([A-Z0-9]{1,8})".to_string(),
        ("[A-Z0-9]", 34) => "([A-Z0-9]{1,34})".to_string(),
        _ => format!("({char_class}{{1,{length}}})"),
    }
}

/// Build unlimited pattern efficiently
fn build_unlimited_pattern(char_class: &str) -> String {
    match char_class {
        "[A-Z]" => "([A-Z]+)".to_string(),
        "\\d" => "(\\d+)".to_string(),
        "[A-Z0-9]" => "([A-Z0-9]+)".to_string(),
        "." => "(.+)".to_string(),
        _ => format!("({char_class}+)"),
    }
}

/// Build escaped pattern efficiently
fn build_escaped_pattern(pattern: &str) -> String {
    // Cache escaped versions of common patterns
    match pattern {
        "/" => "(/)".to_string(),
        "//" => "(//)".to_string(),
        ":" => "(:)".to_string(),
        "," => "(,)".to_string(),
        "." => "(.)".to_string(),
        _ => format!("({})", regex::escape(pattern)),
    }
}

/// Build anchored pattern efficiently
fn build_anchored_pattern(pattern: &str) -> String {
    let mut result = String::with_capacity(pattern.len() + 3);
    result.push('^');
    result.push_str(pattern);
    result.push('$');
    result
}

/// Build combined optional pattern efficiently
fn build_combined_optional_pattern(first: &str, second: &str) -> String {
    let mut result = String::with_capacity(first.len() + second.len());
    result.push_str(first);
    result.push_str(second);
    result
}

/// Build optional pattern efficiently
fn build_optional_pattern(inner_regex: &str) -> String {
    let mut result = String::with_capacity(inner_regex.len() + 5);
    result.push_str("(?:");
    result.push_str(inner_regex);
    result.push_str(")?");
    result
}

/// Build compound optional pattern efficiently
fn build_compound_optional_pattern(first_clean: &str, second_clean: &str) -> String {
    let mut result = String::with_capacity(first_clean.len() + second_clean.len() + 8);
    result.push_str("(?:(");
    result.push_str(first_clean);
    result.push_str(second_clean);
    result.push_str("))?");
    result
}

/// Build slash wrapped pattern efficiently
fn build_slash_wrapped_pattern(inner_regex_no_parens: &str) -> String {
    let mut result = String::with_capacity(inner_regex_no_parens.len() + 5);
    result.push_str("/(");
    result.push_str(inner_regex_no_parens);
    result.push_str(")/");
    result
}

/// Build slash compound pattern efficiently
fn build_slash_compound_pattern(first_no_parens: &str, second_no_parens: &str) -> String {
    let mut result = String::with_capacity(first_no_parens.len() + second_no_parens.len() + 4);
    result.push_str("(/");
    result.push_str(first_no_parens);
    result.push_str(second_no_parens);
    result.push(')');
    result
}

/// Build slash pattern efficiently
fn build_slash_pattern(inner_regex_no_parens: &str) -> String {
    let mut result = String::with_capacity(inner_regex_no_parens.len() + 4);
    result.push_str("/(");
    result.push_str(inner_regex_no_parens);
    result.push(')');
    result
}

/// Build capturing pattern efficiently
pub fn build_capturing_pattern(non_capturing_regex: &str) -> String {
    let mut result = String::with_capacity(non_capturing_regex.len() + 2);
    result.push('(');
    result.push_str(non_capturing_regex);
    result.push(')');
    result
}

/// Build separated pattern efficiently
pub fn build_separated_pattern(regex_parts: &[String], separator: &str) -> String {
    let total_len = regex_parts.iter().map(|s| s.len()).sum::<usize>()
        + (regex_parts.len() - 1) * separator.len()
        + 2;
    let mut result = String::with_capacity(total_len);
    result.push('^');
    for (i, part) in regex_parts.iter().enumerate() {
        if i > 0 {
            result.push_str(separator);
        }
        result.push_str(part);
    }
    result.push('$');
    result
}

/// SWIFT format specification parser and formatter
///
/// Represents a parsed SWIFT format specification that can be used to generate
/// parsing and validation code. This structure captures all the important
/// aspects of a SWIFT format pattern.
///
/// ## Examples
/// - Pattern `"3!a"` → `{ length: Some(3), format_type: Alpha, is_fixed: true }`
/// - Pattern `"[35x]"` → `{ max_length: Some(35), format_type: AnyCharacter, is_optional: true }`
/// - Pattern `"15d"` → `{ max_length: Some(15), format_type: Decimal, is_fixed: false }`
#[derive(Debug, Clone, PartialEq)]
pub struct FormatSpec {
    /// Original pattern string (e.g., "3!a", "[35x]", "15d")
    pub pattern: String,
    /// Exact length for fixed-length patterns
    pub length: Option<usize>,
    /// Maximum length for variable-length patterns
    pub max_length: Option<usize>,
    /// Character type and validation rules
    pub format_type: FormatType,
    /// Whether the length is fixed (! modifier present)
    pub is_fixed: bool,
    /// Whether the field is optional (wrapped in [...])
    pub is_optional: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormatType {
    /// Alphabetic characters only
    Alpha,
    /// Numeric characters only
    Numeric,
    /// Alphanumeric characters
    _Alphanumeric,
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
            working_str = &working_str[1..working_str.len() - 1];
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
                spec.is_fixed = captures.get(2).is_some_and(|m| m.as_str() == "!");

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
}

/// Convert SWIFT format specification to regex pattern
/// Examples:
/// - "6!n" -> r"(\d{6})"
/// - "3!a" -> r"([A-Z]{3})"  
/// - "15d" -> r"(\d{1,15}(?:[.,]\d+)?)"
/// - "[/34x]" -> r"(?:/(.{0,34}))?"
/// - "4*35x" -> r"(.{0,35}(?:\n.{0,35}){0,3})"
pub fn swift_format_to_regex(format_str: &str) -> MacroResult<String> {
    let mut regex_parts = Vec::new();
    let mut remaining = format_str;

    while !remaining.is_empty() {
        if let Some(pattern) = extract_next_pattern(&mut remaining)? {
            let regex_part = pattern_to_regex(&pattern)?;
            regex_parts.push(regex_part);
        } else {
            break;
        }
    }

    Ok(build_anchored_pattern(&regex_parts.join("")))
}

/// Extract the next format pattern from the string
fn extract_next_pattern(remaining: &mut &str) -> MacroResult<Option<String>> {
    if remaining.is_empty() {
        return Ok(None);
    }

    let start = *remaining;

    // Handle optional patterns [...]
    if start.starts_with('[') {
        if let Some(end_pos) = start.find(']') {
            let pattern = start[..=end_pos].to_string();
            *remaining = &start[end_pos + 1..];

            // Check if the next pattern is also optional and should be combined
            if remaining.starts_with('[') {
                // This is a compound optional pattern like [/1!a][/34x]
                // We need to combine them into a single pattern
                if let Some(second_end) = remaining.find(']') {
                    let second_pattern = &remaining[..=second_end];
                    let combined_pattern =
                        build_combined_optional_pattern(&pattern, second_pattern);
                    *remaining = &remaining[second_end + 1..];
                    return Ok(Some(combined_pattern));
                }
            }

            return Ok(Some(pattern));
        }
    }

    // Handle repetitive patterns like 4*35x
    if let Ok(re) = Regex::new(r"^(\d+)\*(\d+)([a-zA-Z])") {
        if let Some(captures) = re.captures(start) {
            let full_match = captures.get(0).unwrap();
            let pattern = full_match.as_str().to_string();
            *remaining = &start[full_match.end()..];
            return Ok(Some(pattern));
        }
    }

    // Handle simple patterns like 6!n, 3!a, 15d
    if let Ok(re) = Regex::new(r"^(\d*)(!?)([a-zA-Z])") {
        if let Some(captures) = re.captures(start) {
            let full_match = captures.get(0).unwrap();
            let pattern = full_match.as_str().to_string();
            *remaining = &start[full_match.end()..];
            return Ok(Some(pattern));
        }
    }

    // Handle prefix characters like '/'
    if let Some(stripped) = start.strip_prefix('/') {
        *remaining = stripped;
        return Ok(Some("/".to_string()));
    }

    // Skip unknown characters
    *remaining = &start[1..];
    Ok(None)
}

/// Convert a single format pattern to regex
fn pattern_to_regex(pattern: &str) -> MacroResult<String> {
    // Handle optional patterns [...]
    if pattern.starts_with('[') && pattern.ends_with(']') {
        let inner = &pattern[1..pattern.len() - 1];
        let inner_regex = pattern_to_regex(inner)?;
        return Ok(build_optional_pattern(&inner_regex));
    }

    // Handle compound optional patterns like [/1!a][/34x]
    if pattern.starts_with('[') && pattern.contains("][") {
        // Split compound pattern and process each part
        let parts: Vec<&str> = pattern.split("][").collect();
        if parts.len() == 2 {
            let first_part = parts[0].trim_start_matches('[');
            let second_part = parts[1].trim_end_matches(']');

            let first_regex = pattern_to_regex(first_part)?;
            let second_regex = pattern_to_regex(second_part)?;

            // Remove outer capture groups to avoid nested captures
            let first_clean = first_regex.trim_start_matches('(').trim_end_matches(')');
            let second_clean = second_regex.trim_start_matches('(').trim_end_matches(')');

            // Create a single optional group that captures the entire compound pattern
            return Ok(build_compound_optional_pattern(first_clean, second_clean));
        }
    }

    // Handle prefix slash
    if pattern == "/" {
        return Ok("/".to_string());
    }

    // Handle compound slash patterns like /1!a/34x and /8c/
    if pattern.starts_with('/') && pattern.contains('/') && pattern.len() > 1 {
        let parts: Vec<&str> = pattern[1..].split('/').collect();

        // Handle patterns like /8c/ (delimited with slashes on both sides)
        if parts.len() == 2 && parts[1].is_empty() {
            // Pattern like /8c/ - match /content/ but capture only content
            let inner_regex = pattern_to_regex(parts[0])?;
            let inner_regex_no_parens = inner_regex.trim_start_matches('(').trim_end_matches(')');
            // Return the pattern that captures only the content between slashes
            return Ok(build_slash_wrapped_pattern(inner_regex_no_parens));
        }

        // Handle compound patterns like /1!a/34x
        if parts.len() == 2 && !parts[1].is_empty() {
            let first_part_regex = pattern_to_regex(parts[0])?;
            let second_part_regex = pattern_to_regex(parts[1])?;
            // Remove capturing groups to avoid nested captures
            let first_no_parens = first_part_regex
                .trim_start_matches('(')
                .trim_end_matches(')');
            let second_no_parens = second_part_regex
                .trim_start_matches('(')
                .trim_end_matches(')');
            // Create a single capturing group that captures both parts including the leading slash
            return Ok(build_slash_compound_pattern(
                first_no_parens,
                second_no_parens,
            ));
        }
    }

    // Handle simple slash prefix patterns like /34x
    if pattern.starts_with('/') && pattern.len() > 1 {
        let after_slash = &pattern[1..];
        let inner_regex = pattern_to_regex(after_slash)?;
        // Remove the capturing group from inner regex since we'll add our own
        let inner_regex_no_parens = inner_regex.trim_start_matches('(').trim_end_matches(')');
        // Match the slash but don't capture it, only capture the content after
        return Ok(build_slash_pattern(inner_regex_no_parens));
    }

    // Handle repetitive patterns like 4*35x
    if let Ok(re) = Regex::new(r"^(\d+)\*(\d+)([a-zA-Z])") {
        if let Some(captures) = re.captures(pattern) {
            let count: usize = captures.get(1).unwrap().as_str().parse().unwrap_or(1);
            let length: usize = captures.get(2).unwrap().as_str().parse().unwrap_or(35);
            let format_char = captures.get(3).unwrap().as_str();

            let char_class = match format_char {
                "a" => "[A-Z]",
                "n" => "\\d",
                "c" => "[A-Z0-9 /.,()-]",
                "x" => ".",
                _ => ".",
            };

            // Pattern for multiple lines: first line + optional additional lines
            return Ok(format!(
                "({}{{{},{}}}(?:\\n{}{{{},{}}}){{{},{}}})",
                char_class,
                0,
                length,
                char_class,
                0,
                length,
                0,
                count.saturating_sub(1)
            ));
        }
    }

    // Handle simple patterns like 6!n, 3!a, 15d
    if let Ok(re) = Regex::new(r"^(\d*)(!?)([a-zA-Z])") {
        if let Some(captures) = re.captures(pattern) {
            let length_str = captures.get(1).map_or("", |m| m.as_str());
            let is_fixed = captures.get(2).is_some_and(|m| m.as_str() == "!");
            let format_char = captures.get(3).map_or("", |m| m.as_str());

            let char_class = match format_char {
                "a" => "[A-Z]",
                "n" => "\\d",
                "c" => "[A-Z0-9]", // BIC codes should only contain letters and numbers
                "x" => ".",
                "d" => "\\d", // Will be handled specially for decimals
                _ => ".",
            };

            if format_char == "d" {
                // Decimal number with optional fractional part
                let length = if !length_str.is_empty() {
                    length_str.parse().unwrap_or(15)
                } else {
                    15
                };
                return Ok(build_decimal_pattern(length));
            }

            if !length_str.is_empty() {
                let length: usize = length_str.parse().unwrap_or(1);
                if is_fixed {
                    return Ok(build_fixed_pattern(char_class, length));
                } else {
                    return Ok(build_variable_pattern(char_class, length));
                }
            } else {
                return Ok(build_unlimited_pattern(char_class));
            }
        }
    }

    Ok(build_escaped_pattern(pattern))
}

/// Convert all capturing groups in a regex to non-capturing groups
/// This ensures that we can wrap the entire pattern in a single capturing group
pub fn convert_to_non_capturing_groups(regex: &str) -> String {
    // Special handling for slash patterns: if pattern is like /([A-Z0-9]{1,8})/
    // we want to preserve the inner capture group
    if regex.starts_with('/') && regex.ends_with('/') {
        // For patterns like /([A-Z0-9]{1,8})/, keep the inner capture group
        return regex.to_string();
    }

    // Special handling for optional slash patterns: if pattern is like (?:/(.{1,34}))? or (?:/([A-Z]{1}/.{1,34}))?
    // we want to preserve the inner capture group
    if regex.starts_with("(?:/") && regex.contains('(') {
        // For patterns like (?:/(.{1,34}))? or (?:/([A-Z]{1}/.{1,34}))?, keep the inner capture group
        return regex.to_string();
    }

    // Simple approach: replace all "(" with "(?:" except for the outer wrapping groups
    // This is a bit naive but should work for our generated patterns
    let mut result = String::new();
    let mut chars = regex.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '(' {
            // Check if this is already a non-capturing group
            if chars.peek() == Some(&'?') {
                result.push(ch); // Keep as is
            } else {
                result.push_str("(?:"); // Convert to non-capturing
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Generate regex-based parsing code for struct fields
pub fn generate_regex_parse_impl(
    name: &syn::Ident,
    struct_field: &StructField,
) -> MacroResult<TokenStream> {
    // Use the new pattern generator system
    use crate::codegen::pattern_generators::FieldParserGenerator;

    let generator = FieldParserGenerator::new();
    generator.generate_parser(name, struct_field)
}

/// Generate type conversion expression from string to target type
pub fn generate_type_conversion_expr(
    target_type: &Type,
    value_expr: TokenStream,
) -> MacroResult<TokenStream> {
    let type_str = quote! { #target_type }.to_string();

    // Remove whitespace for easier matching
    let type_str = type_str.replace(' ', "");

    match type_str.as_str() {
        "String" => Ok(quote! { #value_expr.to_string() }),
        "f64" => Ok(quote! {
            #value_expr.replace(',', ".").parse::<f64>()
                .map_err(|_| crate::errors::ParseError::InvalidFormat {
                    message: format!("Invalid decimal number: {}", #value_expr),
                })?
        }),
        "f32" => Ok(quote! {
            #value_expr.replace(',', ".").parse::<f32>()
                .map_err(|_| crate::errors::ParseError::InvalidFormat {
                    message: format!("Invalid decimal number: {}", #value_expr),
                })?
        }),
        "u32" => Ok(quote! {
            #value_expr.parse::<u32>()
                .map_err(|_| crate::errors::ParseError::InvalidFormat {
                    message: format!("Invalid number: {}", #value_expr),
                })?
        }),
        "u8" => Ok(quote! {
            #value_expr.parse::<u8>()
                .map_err(|_| crate::errors::ParseError::InvalidFormat {
                    message: format!("Invalid number: {}", #value_expr),
                })?
        }),
        "i32" => Ok(quote! {
            #value_expr.parse::<i32>()
                .map_err(|_| crate::errors::ParseError::InvalidFormat {
                    message: format!("Invalid number: {}", #value_expr),
                })?
        }),
        "char" => Ok(quote! {
            #value_expr.chars().next()
                .ok_or_else(|| crate::errors::ParseError::InvalidFormat {
                    message: format!("Expected single character, got: {}", #value_expr),
                })?
        }),
        "bool" => Ok(quote! {
            match #value_expr {
                "Y" | "1" | "true" => true,
                "N" | "0" | "false" => false,
                _ => return Err(crate::errors::ParseError::InvalidFormat {
                    message: format!("Invalid boolean value: {}", #value_expr),
                })
            }
        }),
        "NaiveDate" => Ok(quote! {
            chrono::NaiveDate::parse_from_str(#value_expr, "%y%m%d")
                .map_err(|_| crate::errors::ParseError::InvalidFormat {
                    message: format!("Invalid date format (expected YYMMDD): {}", #value_expr),
                })?
        }),
        "NaiveTime" => Ok(quote! {
            chrono::NaiveTime::parse_from_str(#value_expr, "%H%M")
                .or_else(|_| chrono::NaiveTime::parse_from_str(#value_expr, "%H%M%S"))
                .map_err(|_| crate::errors::ParseError::InvalidFormat {
                    message: format!("Invalid time format (expected HHMM or HHMMSS): {}", #value_expr),
                })?
        }),
        "Vec<String>" => Ok(quote! {
            #value_expr.lines().map(|s| s.to_string()).collect()
        }),
        _ => {
            // For unknown types, try to parse as String and let the field handle conversion
            Ok(quote! { #value_expr.to_string() })
        }
    }
}
