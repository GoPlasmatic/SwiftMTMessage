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

/// Convert SWIFT format pattern or regex to human-readable description
pub fn format_to_description(pattern: &str) -> String {
    // First check for common regex patterns
    match pattern {
        // Currency and amount patterns
        r"^((?:[A-Z]{3}))((?:\d{1,15}(?:[.,]\d+)?))$" |
        "((?:[A-Z]{3}))((?:\\d{1,15}(?:[.,]\\d+)?))" => {
            "3-letter currency code followed by amount (e.g., USD1234.56 or EUR1234,56)".to_string()
        }

        // Complex multi-component patterns (like Field32A: date + currency + amount)
        r"^(\d{6})([A-Z]{3})(\d{1,15}(?:[.,]\d+)?)$" |
        "(\\d{6})([A-Z]{3})(\\d{1,15}(?:[.,]\\d+)?)" => {
            "Date (YYMMDD), currency code (3 letters), and amount (e.g., 241231USD1500000.00)".to_string()
        }
        r"^(\d{8})([A-Z]{3})(\d{1,15}(?:[.,]\d+)?)$" |
        "(\\d{8})([A-Z]{3})(\\d{1,15}(?:[.,]\\d+)?)" => {
            "Date (YYYYMMDD), currency code (3 letters), and amount (e.g., 20241231USD1500000.00)".to_string()
        }

        // Field23E pattern: instruction code + optional additional info
        r"^([A-Z]{4})(?:/(.{1,30}))?$" |
        "([A-Z]{4})(?:/(.{1,30}))?" => {
            "4-letter instruction code with optional additional info after / (e.g., INTC or INTC/COMPLIANCE)".to_string()
        }

        // Code patterns with optional additional information
        r"^([A-Z0-9]{1,4})(?:/(.{1,30}))?$" |
        "([A-Z0-9]{1,4})(?:/(.{1,30}))?" => {
            "1-4 character code with optional additional info after / (e.g., A01 or SNDTIME/1200+0100)".to_string()
        }
        r"^([A-Z0-9]{4})(?:/(.{1,30}))?$" |
        "([A-Z0-9]{4})(?:/(.{1,30}))?" => {
            "4-character code with optional additional info after / (e.g., INTC/COMPLIANCE)".to_string()
        }

        // Multiline patterns
        r"^(.{0,35}(?:\n.{0,35}){0,3})$" |
        "(.{0,35}(?:\\n.{0,35}){0,3})" => {
            "Up to 4 lines of 35 characters each".to_string()
        }
        r"^(.{0,35}(?:\n.{0,35}){0,2})$" |
        "(.{0,35}(?:\\n.{0,35}){0,2})" => {
            "Up to 3 lines of 35 characters each".to_string()
        }
        r"^(.{0,35}(?:\n.{0,35}){0,5})$" |
        "(.{0,35}(?:\\n.{0,35}){0,5})" => {
            "Up to 6 lines of 35 characters each".to_string()
        }

        // BIC patterns
        r"^([A-Z]{4}[A-Z]{2}[A-Z0-9]{2}(?:[A-Z0-9]{3})?)$" |
        "([A-Z]{4}[A-Z]{2}[A-Z0-9]{2}(?:[A-Z0-9]{3})?)" => {
            "BIC code (8 or 11 characters, e.g., DEUTDEFF or DEUTDEFFXXX)".to_string()
        }

        // Date patterns
        r"^(\d{6})$" | "(\\d{6})" => {
            "Date in YYMMDD format (e.g., 241231 for Dec 31, 2024)".to_string()
        }
        r"^(\d{8})$" | "(\\d{8})" => {
            "Date in YYYYMMDD format (e.g., 20241231)".to_string()
        }

        // Time patterns
        r"^(\d{4})$" if pattern.len() <= 10 => {
            "Time in HHMM format (e.g., 1430 for 2:30 PM)".to_string()
        }

        // Time with timezone patterns
        r"^(\d{4})([+-])(\d{4})$" |
        "(\\d{4})([+-])(\\d{4})" => {
            "Time (HHMM) with timezone offset (e.g., 1200+0100)".to_string()
        }

        // Amount patterns
        r"^(\d{1,15}(?:[.,]\d+)?)$" | "(\\d{1,15}(?:[.,]\\d+)?)" => {
            "Decimal amount up to 15 digits (e.g., 1234.56 or 1234,56)".to_string()
        }
        r"^(\d{1,18}(?:[.,]\d+)?)$" | "(\\d{1,18}(?:[.,]\\d+)?)" => {
            "Decimal amount up to 18 digits (e.g., 123456789012.34)".to_string()
        }

        // Rate/percentage patterns
        r"^(\d{1,3}(?:[.,]\d{1,10})?)$" |
        "(\\d{1,3}(?:[.,]\\d{1,10})?)" => {
            "Rate or percentage (e.g., 1.0909 or 12.5)".to_string()
        }

        // Account patterns
        r"^(?:/(.{1,34}))?$" | "(?:/(.{1,34}))?" => {
            "Optional account number with / prefix (e.g., /1234567890)".to_string()
        }
        r"^/(.{1,34})$" | "/(.{1,34})" => {
            "Account number with / prefix (e.g., /1234567890)".to_string()
        }

        // Party identifier patterns
        r"^(?:/([A-Z]{1}))?(?:/(.{1,34}))?$" | "(?:/([A-Z]{1}))?(?:/(.{1,34}))?" => {
            "Optional party identifier (e.g., /C/12345 or /12345)".to_string()
        }

        // Code patterns
        r"^([A-Z]{3})$" | "([A-Z]{3})" => {
            "Exactly 3 uppercase letters (e.g., SHA, BEN, OUR)".to_string()
        }
        r"^([A-Z]{4})$" | "([A-Z]{4})" => {
            "Exactly 4 uppercase letters (e.g., CRED, SPAY, SSTD)".to_string()
        }
        r"^([A-Z0-9]{3})$" | "([A-Z0-9]{3})" => {
            "Exactly 3 letters or digits".to_string()
        }
        r"^([A-Z0-9]{4})$" | "([A-Z0-9]{4})" => {
            "Exactly 4 letters or digits".to_string()
        }

        // Variable length patterns
        r"^(.{1,16})$" | "(.{1,16})" => {
            "Up to 16 characters".to_string()
        }
        r"^(.{1,35})$" | "(.{1,35})" => {
            "Up to 35 characters".to_string()
        }
        r"^(.{1,34})$" | "(.{1,34})" => {
            "Up to 34 characters".to_string()
        }
        r"^(.{1,50})$" | "(.{1,50})" => {
            "Up to 50 characters".to_string()
        }
        r"^(.{1,65})$" | "(.{1,65})" => {
            "Up to 65 characters".to_string()
        }
        r"^(.{1,100})$" | "(.{1,100})" => {
            "Up to 100 characters".to_string()
        }

        // Fixed length patterns
        r"^(.{16})$" | "(.{16})" => {
            "Exactly 16 characters".to_string()
        }
        r"^(.{35})$" | "(.{35})" => {
            "Exactly 35 characters".to_string()
        }

        // Alphanumeric patterns
        r"^([A-Z0-9]{1,16})$" | "([A-Z0-9]{1,16})" => {
            "Up to 16 letters or digits".to_string()
        }
        r"^([A-Z0-9]{1,34})$" | "([A-Z0-9]{1,34})" => {
            "Up to 34 letters or digits".to_string()
        }
        r"^([A-Z0-9]{1,35})$" | "([A-Z0-9]{1,35})" => {
            "Up to 35 letters or digits".to_string()
        }

        // Numeric patterns
        r"^(\d{1})$" | "(\\d{1})" => {
            "Single digit (0-9)".to_string()
        }
        r"^(\d{2})$" | "(\\d{2})" => {
            "Exactly 2 digits".to_string()
        }
        r"^(\d{3})$" | "(\\d{3})" => {
            "Exactly 3 digits".to_string()
        }
        r"^(\d{4})$" | "(\\d{4})" => {
            "Exactly 4 digits".to_string()
        }
        r"^(\d{5})$" | "(\\d{5})" => {
            "Exactly 5 digits".to_string()
        }
        r"^(\d{1,6})$" | "(\\d{1,6})" => {
            "Up to 6 digits".to_string()
        }
        r"^(\d{1,10})$" | "(\\d{1,10})" => {
            "Up to 10 digits".to_string()
        }
        r"^(\d{1,15})$" | "(\\d{1,15})" => {
            "Up to 15 digits".to_string()
        }

        // Letter patterns
        r"^([A-Z]{1})$" | "([A-Z]{1})" => {
            "Single uppercase letter".to_string()
        }
        r"^([A-Z]{2})$" | "([A-Z]{2})" => {
            "Exactly 2 uppercase letters".to_string()
        }
        r"^([A-Z]{1,35})$" | "([A-Z]{1,35})" => {
            "Up to 35 uppercase letters".to_string()
        }

        _ => {
            // Try to parse SWIFT format notation
            parse_swift_format_to_description(pattern)
        }
    }
}

/// Parse SWIFT format notation to human-readable description
fn parse_swift_format_to_description(pattern: &str) -> String {
    match pattern {
        // Fixed length patterns
        "1!a" => "Single uppercase letter".to_string(),
        "2!a" => "Exactly 2 uppercase letters".to_string(),
        "3!a" => "Exactly 3 uppercase letters (e.g., USD, EUR, SHA)".to_string(),
        "4!a" => "Exactly 4 uppercase letters (e.g., CRED, SPAY)".to_string(),
        "5!a" => "Exactly 5 uppercase letters".to_string(),
        "6!a" => "Exactly 6 uppercase letters".to_string(),
        "8!a" => "Exactly 8 uppercase letters".to_string(),
        "1!c" => "Single character (letter or digit)".to_string(),
        "2!c" => "Exactly 2 characters (letters/digits)".to_string(),
        "3!c" => "Exactly 3 characters (letters/digits)".to_string(),
        "4!c" => "Exactly 4 characters (letters/digits)".to_string(),
        "1!n" => "Single digit".to_string(),
        "2!n" => "Exactly 2 digits".to_string(),
        "3!n" => "Exactly 3 digits".to_string(),
        "4!n" => "Exactly 4 digits".to_string(),
        "5!n" => "Exactly 5 digits".to_string(),
        "6!n" => "Exactly 6 digits (e.g., date YYMMDD)".to_string(),
        "8!n" => "Exactly 8 digits (e.g., date YYYYMMDD)".to_string(),
        "10!n" => "Exactly 10 digits".to_string(),
        "12!n" => "Exactly 12 digits".to_string(),

        // Variable length patterns
        "16x" => "Up to 16 characters".to_string(),
        "20x" => "Up to 20 characters".to_string(),
        "25x" => "Up to 25 characters".to_string(),
        "30x" => "Up to 30 characters".to_string(),
        "33x" => "Up to 33 characters".to_string(),
        "34x" => "Up to 34 characters".to_string(),
        "35x" => "Up to 35 characters".to_string(),
        "50x" => "Up to 50 characters".to_string(),
        "65x" => "Up to 65 characters".to_string(),
        "100x" => "Up to 100 characters".to_string(),
        "12d" => "Decimal amount up to 12 digits".to_string(),
        "15d" => "Decimal amount up to 15 digits".to_string(),
        "18d" => "Decimal amount up to 18 digits".to_string(),
        "2n" => "Up to 2 digits".to_string(),
        "3n" => "Up to 3 digits".to_string(),
        "5n" => "Up to 5 digits".to_string(),
        "10n" => "Up to 10 digits".to_string(),
        "15n" => "Up to 15 digits".to_string(),
        "16n" => "Up to 16 digits".to_string(),
        "1a" => "Single uppercase letter".to_string(),
        "3a" => "Up to 3 uppercase letters".to_string(),
        "35a" => "Up to 35 uppercase letters".to_string(),
        "8c" => "Up to 8 characters (letters/digits)".to_string(),
        "16c" => "Up to 16 characters (letters/digits)".to_string(),
        "34c" => "Up to 34 characters (letters/digits)".to_string(),

        // Combined patterns
        "3!a15d" => "3-letter currency code followed by amount (e.g., USD1234.56)".to_string(),
        "3!a18d" => "3-letter currency code followed by amount up to 18 digits".to_string(),
        "6!n3!a15d" => {
            "Date (YYMMDD), currency code, and amount (e.g., 241231USD1500000.00)".to_string()
        }
        "8!n3!a15d" => "Date (YYYYMMDD), currency code, and amount".to_string(),
        "4!c[/30x]" => {
            "4-character code with optional additional info after / (e.g., INTC/COMPLIANCE)"
                .to_string()
        }
        "3!a[/30x]" => "3-letter code with optional additional info after /".to_string(),
        "1!a3!c" => "Single letter followed by 3 characters".to_string(),

        // BIC patterns
        "4!a2!a2!c[3!c]" => {
            "BIC code (8 or 11 characters, e.g., DEUTDEFF or DEUTDEFFXXX)".to_string()
        }
        "4!a2!a2!c" => "8-character BIC code (e.g., DEUTDEFF)".to_string(),
        "4!a2!a2!c3!c" => "11-character BIC code (e.g., DEUTDEFFXXX)".to_string(),

        // Multiline patterns
        "4*35x" => "Up to 4 lines of 35 characters each".to_string(),
        "3*35x" => "Up to 3 lines of 35 characters each".to_string(),
        "5*35x" => "Up to 5 lines of 35 characters each".to_string(),
        "6*35x" => "Up to 6 lines of 35 characters each".to_string(),
        "10*35x" => "Up to 10 lines of 35 characters each".to_string(),
        "4*(1!n/33x)" => "Up to 4 numbered lines (e.g., 1/text, 2/text)".to_string(),
        "6*50x" => "Up to 6 lines of 50 characters each".to_string(),
        "35*50x" => "Up to 35 lines of 50 characters each".to_string(),

        // Optional patterns
        "[/34x]" => "Optional value with / prefix (up to 34 characters)".to_string(),
        "[/1!a][/34x]" => "Optional party code and identifier (e.g., /C/12345)".to_string(),
        "[35x]" => "Optional text up to 35 characters".to_string(),
        "[34x]" => "Optional text up to 34 characters".to_string(),
        "[30x]" => "Optional text up to 30 characters".to_string(),
        "[16x]" => "Optional text up to 16 characters".to_string(),
        "[/16x]" => "Optional value with / prefix (up to 16 characters)".to_string(),
        "[//16x]" => "Optional value with // prefix (up to 16 characters)".to_string(),
        "[/30x]" => "Optional value with / prefix (up to 30 characters)".to_string(),
        "[1!a]" => "Optional single letter".to_string(),
        "[4!n]" => "Optional 4 digits".to_string(),
        "[/2n]" => "Optional 2 digits with / prefix".to_string(),
        "[/5n]" => "Optional 5 digits with / prefix".to_string(),

        // Special patterns
        "/8c/" => "8 characters between slashes (e.g., /ABCD1234/)".to_string(),
        "/16x" => "Up to 16 characters with / prefix".to_string(),
        "/30x" => "Up to 30 characters with / prefix".to_string(),
        "/34x" => "Up to 34 characters with / prefix".to_string(),
        "//16x" => "Up to 16 characters with // prefix".to_string(),
        "/1!a/34x" => {
            "Single letter and up to 34 characters with / prefixes (e.g., /C/12345)".to_string()
        }

        // Account and reference patterns
        "16!x" => "Exactly 16 characters (reference number)".to_string(),
        "35!x" => "Exactly 35 characters".to_string(),

        _ => {
            // Try to parse dynamic patterns
            if pattern.contains('*')
                && let Some(captures) = regex::Regex::new(r"(\d+)\*(\d+)([a-zA-Z])")
                    .unwrap()
                    .captures(pattern)
            {
                let lines = captures.get(1).map_or("", |m| m.as_str());
                let chars = captures.get(2).map_or("", |m| m.as_str());
                let format_type = captures.get(3).map_or("", |m| m.as_str());

                let char_type = match format_type {
                    "a" => "letters",
                    "n" => "digits",
                    "c" => "alphanumeric characters",
                    "x" => "characters",
                    _ => "characters",
                };

                return format!("Up to {lines} lines of {chars} {char_type} each");
            }

            // Try to parse simple patterns with regex
            if let Ok(re) = regex::Regex::new(r"^(\d+)(!?)([a-zA-Z])$")
                && let Some(captures) = re.captures(pattern)
            {
                let length = captures.get(1).map_or("", |m| m.as_str());
                let is_fixed = captures.get(2).map_or("", |m| m.as_str()) == "!";
                let format_type = captures.get(3).map_or("", |m| m.as_str());

                let type_desc = match format_type {
                    "a" => "uppercase letters",
                    "n" => "digits",
                    "c" => "characters (letters/digits)",
                    "x" => "characters",
                    "d" => "decimal digits",
                    _ => "characters",
                };

                if is_fixed {
                    return format!("Exactly {length} {type_desc}");
                } else if format_type == "d" {
                    return format!("Decimal amount up to {length} digits");
                } else {
                    return format!("Up to {length} {type_desc}");
                }
            }

            // If pattern contains regex special characters, provide a more specific description
            if pattern.contains('^') || pattern.contains('$') || pattern.contains('(') {
                // Try to analyze complex regex patterns for better descriptions

                // Check for specific multi-component patterns
                if pattern.contains("\\d{6}")
                    && pattern.contains("[A-Z]{3}")
                    && pattern.contains("\\d{1,15}")
                {
                    "Date (YYMMDD), currency code (3 letters), and amount".to_string()
                } else if pattern.contains("\\d{8}")
                    && pattern.contains("[A-Z]{3}")
                    && pattern.contains("\\d{1,15}")
                {
                    "Date (YYYYMMDD), currency code (3 letters), and amount".to_string()
                } else if pattern.contains("[A-Z]{4}") && pattern.contains("(?:/") {
                    "4-letter code with optional additional information".to_string()
                } else if pattern.contains("[A-Z]{3}") && pattern.contains("\\d{1,15}") {
                    "Currency code and amount".to_string()
                } else if pattern.contains("\\d{4}[+-]\\d{4}") {
                    "Time with timezone offset".to_string()
                } else if pattern.contains("/[A-Z0-9]{1,8}/") {
                    "Code between slashes".to_string()
                } else if pattern.contains("\\d") && pattern.contains("[A-Z]") {
                    // More specific descriptions based on pattern structure
                    if pattern.contains("{6}") || pattern.contains("{8}") {
                        "Date followed by other components".to_string()
                    } else if pattern.contains("{3}") && pattern.contains("{1,15}") {
                        "Code followed by value".to_string()
                    } else {
                        "Combination of letters and digits".to_string()
                    }
                } else if pattern.contains("\\d") {
                    "Numeric value".to_string()
                } else if pattern.contains("[A-Z]") {
                    "Uppercase letters".to_string()
                } else if pattern.contains("(?:") && pattern.contains(")?") {
                    // Try to provide more specific descriptions for common optional patterns
                    if pattern.contains("[A-Z0-9]{4}") && pattern.contains("/(.{1,30})") {
                        "4-character code with optional additional info after /".to_string()
                    } else if pattern.contains("[A-Z0-9]{1,4}") && pattern.contains("/(.{1,30})") {
                        "1-4 character code with optional additional info after /".to_string()
                    } else {
                        "Format with optional components".to_string()
                    }
                } else {
                    "Valid SWIFT format".to_string()
                }
            } else {
                // Return a generic description for unknown patterns
                format!("Format: {pattern}")
            }
        }
    }
}

/// Efficient pattern building functions to avoid string allocations
/// Build decimal pattern without format! macro
fn build_decimal_pattern(length: usize) -> String {
    // Use string literals for common cases
    // Pattern allows: digits, optionally followed by comma/period and optionally followed by more digits
    // This generic pattern supports: "5000", "5000,", "5000,00", "5000.50"
    //
    // IMPORTANT: This is a GENERIC pattern used by multiple decimal fields (Field32, Field33, Field36, etc.)
    // Some fields (like Field32A) require mandatory decimal separators per SWIFT specification,
    // but this is enforced at the field level using custom validation, not in this generic pattern
    match length {
        1 => "(\\d{1}(?:[.,]\\d*)?)".to_string(),
        2 => "(\\d{1,2}(?:[.,]\\d*)?)".to_string(),
        3 => "(\\d{1,3}(?:[.,]\\d*)?)".to_string(),
        6 => "(\\d{1,6}(?:[.,]\\d*)?)".to_string(),
        12 => "(\\d{1,12}(?:[.,]\\d*)?)".to_string(),
        15 => "(\\d{1,15}(?:[.,]\\d*)?)".to_string(),
        18 => "(\\d{1,18}(?:[.,]\\d*)?)".to_string(),
        _ => {
            // Only fallback to format! for uncommon lengths
            format!(r"(\d{{1,{length}}}(?:[.,]\d*)?)")
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
    // Match slash prefix but only capture the content after it
    let mut result = String::with_capacity(inner_regex_no_parens.len() + 4);
    result.push('/');
    result.push('(');
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
                if !length_str.is_empty()
                    && let Ok(length) = length_str.parse()
                {
                    spec.length = Some(length);
                    spec.max_length = spec.length;
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
    if start.starts_with('[')
        && let Some(end_pos) = start.find(']')
    {
        let pattern = start[..=end_pos].to_string();
        *remaining = &start[end_pos + 1..];

        // Check if the next pattern is also optional and should be combined
        if remaining.starts_with('[') {
            // This is a compound optional pattern like [/1!a][/34x]
            // We need to combine them into a single pattern
            if let Some(second_end) = remaining.find(']') {
                let second_pattern = &remaining[..=second_end];
                let combined_pattern = build_combined_optional_pattern(&pattern, second_pattern);
                *remaining = &remaining[second_end + 1..];
                return Ok(Some(combined_pattern));
            }
        }

        return Ok(Some(pattern));
    }

    // Handle repetitive patterns like 4*35x
    if let Ok(re) = Regex::new(r"^(\d+)\*(\d+)([a-zA-Z])")
        && let Some(captures) = re.captures(start)
    {
        let full_match = captures.get(0).unwrap();
        let pattern = full_match.as_str().to_string();
        *remaining = &start[full_match.end()..];
        return Ok(Some(pattern));
    }

    // Handle simple patterns like 6!n, 3!a, 15d
    if let Ok(re) = Regex::new(r"^(\d*)(!?)([a-zA-Z])")
        && let Some(captures) = re.captures(start)
    {
        let full_match = captures.get(0).unwrap();
        let pattern = full_match.as_str().to_string();
        *remaining = &start[full_match.end()..];
        return Ok(Some(pattern));
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

    // Handle double slash prefix patterns like //16x first
    if pattern.starts_with("//") && pattern.len() > 2 {
        let after_double_slash = &pattern[2..];
        let inner_regex = pattern_to_regex(after_double_slash)?;
        // Remove the capturing group from inner regex since we'll add our own
        let inner_regex_no_parens = inner_regex.trim_start_matches('(').trim_end_matches(')');
        // For //16x patterns, make them non-greedy to avoid capturing content from following optional fields
        // This is especially important for Field 61 where [//16x] is followed by [34x]
        let inner_regex_non_greedy = if inner_regex_no_parens == ".{1,16}" {
            ".{1,16}?" // Make it non-greedy
        } else {
            inner_regex_no_parens
        };
        // Capture the double slash and content together
        return Ok(format!("(//{inner_regex_non_greedy})"));
    }

    // Handle simple slash prefix patterns like /34x
    if pattern.starts_with('/') && pattern.len() > 1 {
        let after_slash = &pattern[1..];
        let inner_regex = pattern_to_regex(after_slash)?;
        // Remove the capturing group from inner regex since we'll add our own
        let inner_regex_no_parens = inner_regex.trim_start_matches('(').trim_end_matches(')');
        // Capture the slash and content together
        return Ok(build_slash_pattern(inner_regex_no_parens));
    }

    // Handle repetitive patterns like 4*35x
    if let Ok(re) = Regex::new(r"^(\d+)\*(\d+)([a-zA-Z])")
        && let Some(captures) = re.captures(pattern)
    {
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

    // Handle simple patterns like 6!n, 3!a, 15d
    if let Ok(re) = Regex::new(r"^(\d*)(!?)([a-zA-Z])")
        && let Some(captures) = re.captures(pattern)
    {
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
            {
                // Handle trailing comma without digits (e.g., "5000," -> "5000.00")
                let normalized = if #value_expr.ends_with(',') {
                    format!("{}00", #value_expr.replace(',', "."))
                } else {
                    #value_expr.replace(',', ".")
                };
                normalized.parse::<f64>()
                    .map_err(|e| crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                        field_tag: "type_conversion".to_string(),
                        component_name: "decimal".to_string(),
                        value: #value_expr.to_string(),
                        format_spec: "decimal number".to_string(),
                        position: None,
                        inner_error: e.to_string(),
                    })))?
            }
        }),
        "f32" => Ok(quote! {
            {
                // Handle trailing comma without digits (e.g., "5000," -> "5000.00")
                let normalized = if #value_expr.ends_with(',') {
                    format!("{}00", #value_expr.replace(',', "."))
                } else {
                    #value_expr.replace(',', ".")
                };
                normalized.parse::<f32>()
                    .map_err(|e| crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                        field_tag: "type_conversion".to_string(),
                        component_name: "decimal".to_string(),
                        value: #value_expr.to_string(),
                        format_spec: "decimal number".to_string(),
                        position: None,
                        inner_error: e.to_string(),
                    })))?
            }
        }),
        "u32" => Ok(quote! {
            #value_expr.parse::<u32>()
                .map_err(|e| crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                    field_tag: "type_conversion".to_string(),
                    component_name: "unsigned_integer".to_string(),
                    value: #value_expr.to_string(),
                    format_spec: "unsigned integer".to_string(),
                    position: None,
                    inner_error: e.to_string(),
                })))?
        }),
        "u8" => Ok(quote! {
            #value_expr.parse::<u8>()
                .map_err(|e| crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                    field_tag: "type_conversion".to_string(),
                    component_name: "byte".to_string(),
                    value: #value_expr.to_string(),
                    format_spec: "byte value (0-255)".to_string(),
                    position: None,
                    inner_error: e.to_string(),
                })))?
        }),
        "i32" => Ok(quote! {
            #value_expr.parse::<i32>()
                .map_err(|e| crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                    field_tag: "type_conversion".to_string(),
                    component_name: "integer".to_string(),
                    value: #value_expr.to_string(),
                    format_spec: "integer".to_string(),
                    position: None,
                    inner_error: e.to_string(),
                })))?
        }),
        "char" => Ok(quote! {
            #value_expr.chars().next()
                .ok_or_else(|| crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                    field_tag: "type_conversion".to_string(),
                    component_name: "character".to_string(),
                    value: #value_expr.to_string(),
                    format_spec: "single character".to_string(),
                    position: None,
                    inner_error: "Expected single character".to_string(),
                })))?
        }),
        "bool" => Ok(quote! {
            match #value_expr {
                "Y" | "1" | "true" => true,
                "N" | "0" | "false" => false,
                _ => return Err(crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                    field_tag: "type_conversion".to_string(),
                    component_name: "boolean".to_string(),
                    value: #value_expr.to_string(),
                    format_spec: "Y/N, 1/0, or true/false".to_string(),
                    position: None,
                    inner_error: "Invalid boolean value".to_string(),
                })))
            }
        }),
        "NaiveDate" => Ok(quote! {
            chrono::NaiveDate::parse_from_str(#value_expr, "%y%m%d")
                .map_err(|e| crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                    field_tag: "type_conversion".to_string(),
                    component_name: "date".to_string(),
                    value: #value_expr.to_string(),
                    format_spec: "YYMMDD".to_string(),
                    position: None,
                    inner_error: e.to_string(),
                })))?
        }),
        "NaiveTime" => Ok(quote! {
            chrono::NaiveTime::parse_from_str(#value_expr, "%H%M")
                .or_else(|_| chrono::NaiveTime::parse_from_str(#value_expr, "%H%M%S"))
                .map_err(|e| crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                    field_tag: "type_conversion".to_string(),
                    component_name: "time".to_string(),
                    value: #value_expr.to_string(),
                    format_spec: "HHMM or HHMMSS".to_string(),
                    position: None,
                    inner_error: e.to_string(),
                })))?
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
