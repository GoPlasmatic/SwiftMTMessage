//! Compile-time format validation for SWIFT field patterns
//!
//! This module provides compile-time validation of SWIFT format specifications
//! to catch invalid format patterns at compile time rather than runtime.

use crate::error::{MacroError, MacroResult};
use once_cell::sync::Lazy;
use proc_macro2::Span;
use std::collections::HashMap;

/// Compile-time validated format specification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValidatedFormatSpec {
    pub pattern: String,
    pub spec_type: FormatSpecType,
}

/// Type of format specification for compile-time validation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FormatSpecType {
    /// Fixed length alphabetic (e.g., "3!a")
    FixedAlphabetic { length: usize },
    /// Variable length alphabetic (e.g., "35a")
    VariableAlphabetic { max_length: usize },
    /// Fixed length numeric (e.g., "6!n")
    FixedNumeric { length: usize },
    /// Variable length numeric (e.g., "15n")
    VariableNumeric { max_length: usize },
    /// Fixed length character set (e.g., "4!c")
    FixedCharacterSet { length: usize },
    /// Variable length character set (e.g., "34c")
    VariableCharacterSet { max_length: usize },
    /// Variable length any character (e.g., "35x")
    VariableAny { max_length: usize },
    /// Decimal format (e.g., "15d")
    Decimal { max_digits: usize },
    /// Optional pattern (e.g., "[35x]")
    Optional { inner: Box<ValidatedFormatSpec> },
    /// Repetitive pattern (e.g., "4*35x")
    Repetitive {
        count: usize,
        inner: Box<ValidatedFormatSpec>,
    },
    /// Complex multi-component pattern
    MultiComponent {
        components: Vec<ValidatedFormatSpec>,
    },
}

/// Registry of known valid format specifications
static KNOWN_VALID_SPECS: Lazy<HashMap<&'static str, ValidatedFormatSpec>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Common alphabetic patterns (both modern and legacy formats)
    map.insert(
        "3!a",
        ValidatedFormatSpec {
            pattern: "3!a".to_string(),
            spec_type: FormatSpecType::FixedAlphabetic { length: 3 },
        },
    );
    map.insert(
        "4!a",
        ValidatedFormatSpec {
            pattern: "4!a".to_string(),
            spec_type: FormatSpecType::FixedAlphabetic { length: 4 },
        },
    );
    map.insert(
        "8!a",
        ValidatedFormatSpec {
            pattern: "8!a".to_string(),
            spec_type: FormatSpecType::FixedAlphabetic { length: 8 },
        },
    );
    map.insert(
        "11!a",
        ValidatedFormatSpec {
            pattern: "11!a".to_string(),
            spec_type: FormatSpecType::FixedAlphabetic { length: 11 },
        },
    );
    // Legacy patterns without '!'
    map.insert(
        "1a",
        ValidatedFormatSpec {
            pattern: "1a".to_string(),
            spec_type: FormatSpecType::FixedAlphabetic { length: 1 },
        },
    );
    map.insert(
        "2a",
        ValidatedFormatSpec {
            pattern: "2a".to_string(),
            spec_type: FormatSpecType::FixedAlphabetic { length: 2 },
        },
    );
    map.insert(
        "3a",
        ValidatedFormatSpec {
            pattern: "3a".to_string(),
            spec_type: FormatSpecType::FixedAlphabetic { length: 3 },
        },
    );
    map.insert(
        "4a",
        ValidatedFormatSpec {
            pattern: "4a".to_string(),
            spec_type: FormatSpecType::FixedAlphabetic { length: 4 },
        },
    );
    map.insert(
        "35a",
        ValidatedFormatSpec {
            pattern: "35a".to_string(),
            spec_type: FormatSpecType::VariableAlphabetic { max_length: 35 },
        },
    );

    // Common numeric patterns (both modern and legacy formats)
    map.insert(
        "1!n",
        ValidatedFormatSpec {
            pattern: "1!n".to_string(),
            spec_type: FormatSpecType::FixedNumeric { length: 1 },
        },
    );
    map.insert(
        "2!n",
        ValidatedFormatSpec {
            pattern: "2!n".to_string(),
            spec_type: FormatSpecType::FixedNumeric { length: 2 },
        },
    );
    map.insert(
        "3!n",
        ValidatedFormatSpec {
            pattern: "3!n".to_string(),
            spec_type: FormatSpecType::FixedNumeric { length: 3 },
        },
    );
    map.insert(
        "4!n",
        ValidatedFormatSpec {
            pattern: "4!n".to_string(),
            spec_type: FormatSpecType::FixedNumeric { length: 4 },
        },
    );
    map.insert(
        "6!n",
        ValidatedFormatSpec {
            pattern: "6!n".to_string(),
            spec_type: FormatSpecType::FixedNumeric { length: 6 },
        },
    );
    map.insert(
        "8!n",
        ValidatedFormatSpec {
            pattern: "8!n".to_string(),
            spec_type: FormatSpecType::FixedNumeric { length: 8 },
        },
    );
    // Legacy patterns without '!'
    map.insert(
        "1n",
        ValidatedFormatSpec {
            pattern: "1n".to_string(),
            spec_type: FormatSpecType::FixedNumeric { length: 1 },
        },
    );
    map.insert(
        "2n",
        ValidatedFormatSpec {
            pattern: "2n".to_string(),
            spec_type: FormatSpecType::FixedNumeric { length: 2 },
        },
    );
    map.insert(
        "3n",
        ValidatedFormatSpec {
            pattern: "3n".to_string(),
            spec_type: FormatSpecType::FixedNumeric { length: 3 },
        },
    );
    map.insert(
        "4n",
        ValidatedFormatSpec {
            pattern: "4n".to_string(),
            spec_type: FormatSpecType::FixedNumeric { length: 4 },
        },
    );
    map.insert(
        "5n",
        ValidatedFormatSpec {
            pattern: "5n".to_string(),
            spec_type: FormatSpecType::FixedNumeric { length: 5 },
        },
    );
    map.insert(
        "6n",
        ValidatedFormatSpec {
            pattern: "6n".to_string(),
            spec_type: FormatSpecType::FixedNumeric { length: 6 },
        },
    );
    map.insert(
        "15n",
        ValidatedFormatSpec {
            pattern: "15n".to_string(),
            spec_type: FormatSpecType::VariableNumeric { max_length: 15 },
        },
    );
    map.insert(
        "16n",
        ValidatedFormatSpec {
            pattern: "16n".to_string(),
            spec_type: FormatSpecType::VariableNumeric { max_length: 16 },
        },
    );

    // Common character set patterns
    map.insert(
        "1!c",
        ValidatedFormatSpec {
            pattern: "1!c".to_string(),
            spec_type: FormatSpecType::FixedCharacterSet { length: 1 },
        },
    );
    map.insert(
        "3!c",
        ValidatedFormatSpec {
            pattern: "3!c".to_string(),
            spec_type: FormatSpecType::FixedCharacterSet { length: 3 },
        },
    );
    map.insert(
        "4!c",
        ValidatedFormatSpec {
            pattern: "4!c".to_string(),
            spec_type: FormatSpecType::FixedCharacterSet { length: 4 },
        },
    );
    map.insert(
        "8c",
        ValidatedFormatSpec {
            pattern: "8c".to_string(),
            spec_type: FormatSpecType::VariableCharacterSet { max_length: 8 },
        },
    );
    map.insert(
        "34c",
        ValidatedFormatSpec {
            pattern: "34c".to_string(),
            spec_type: FormatSpecType::VariableCharacterSet { max_length: 34 },
        },
    );

    // Common any character patterns
    map.insert(
        "16x",
        ValidatedFormatSpec {
            pattern: "16x".to_string(),
            spec_type: FormatSpecType::VariableAny { max_length: 16 },
        },
    );
    map.insert(
        "35x",
        ValidatedFormatSpec {
            pattern: "35x".to_string(),
            spec_type: FormatSpecType::VariableAny { max_length: 35 },
        },
    );
    map.insert(
        "50x",
        ValidatedFormatSpec {
            pattern: "50x".to_string(),
            spec_type: FormatSpecType::VariableAny { max_length: 50 },
        },
    );

    // Common decimal patterns
    map.insert(
        "15d",
        ValidatedFormatSpec {
            pattern: "15d".to_string(),
            spec_type: FormatSpecType::Decimal { max_digits: 15 },
        },
    );
    map.insert(
        "18d",
        ValidatedFormatSpec {
            pattern: "18d".to_string(),
            spec_type: FormatSpecType::Decimal { max_digits: 18 },
        },
    );

    // Common optional patterns
    map.insert(
        "[35x]",
        ValidatedFormatSpec {
            pattern: "[35x]".to_string(),
            spec_type: FormatSpecType::Optional {
                inner: Box::new(ValidatedFormatSpec {
                    pattern: "35x".to_string(),
                    spec_type: FormatSpecType::VariableAny { max_length: 35 },
                }),
            },
        },
    );
    map.insert(
        "[34x]",
        ValidatedFormatSpec {
            pattern: "[34x]".to_string(),
            spec_type: FormatSpecType::Optional {
                inner: Box::new(ValidatedFormatSpec {
                    pattern: "34x".to_string(),
                    spec_type: FormatSpecType::VariableAny { max_length: 34 },
                }),
            },
        },
    );
    map.insert(
        "[16x]",
        ValidatedFormatSpec {
            pattern: "[16x]".to_string(),
            spec_type: FormatSpecType::Optional {
                inner: Box::new(ValidatedFormatSpec {
                    pattern: "16x".to_string(),
                    spec_type: FormatSpecType::VariableAny { max_length: 16 },
                }),
            },
        },
    );
    map.insert(
        "[1!a]",
        ValidatedFormatSpec {
            pattern: "[1!a]".to_string(),
            spec_type: FormatSpecType::Optional {
                inner: Box::new(ValidatedFormatSpec {
                    pattern: "1!a".to_string(),
                    spec_type: FormatSpecType::FixedAlphabetic { length: 1 },
                }),
            },
        },
    );
    map.insert(
        "[4!n]",
        ValidatedFormatSpec {
            pattern: "[4!n]".to_string(),
            spec_type: FormatSpecType::Optional {
                inner: Box::new(ValidatedFormatSpec {
                    pattern: "4!n".to_string(),
                    spec_type: FormatSpecType::FixedNumeric { length: 4 },
                }),
            },
        },
    );
    map.insert(
        "[/34x]",
        ValidatedFormatSpec {
            pattern: "[/34x]".to_string(),
            spec_type: FormatSpecType::Optional {
                inner: Box::new(ValidatedFormatSpec {
                    pattern: "/34x".to_string(),
                    spec_type: FormatSpecType::VariableAny { max_length: 34 },
                }),
            },
        },
    );
    map.insert(
        "[//16x]",
        ValidatedFormatSpec {
            pattern: "[//16x]".to_string(),
            spec_type: FormatSpecType::Optional {
                inner: Box::new(ValidatedFormatSpec {
                    pattern: "//16x".to_string(),
                    spec_type: FormatSpecType::VariableAny { max_length: 16 },
                }),
            },
        },
    );
    map.insert(
        "[/2n]",
        ValidatedFormatSpec {
            pattern: "[/2n]".to_string(),
            spec_type: FormatSpecType::Optional {
                inner: Box::new(ValidatedFormatSpec {
                    pattern: "/2n".to_string(),
                    spec_type: FormatSpecType::FixedNumeric { length: 2 },
                }),
            },
        },
    );
    map.insert(
        "[/5n]",
        ValidatedFormatSpec {
            pattern: "[/5n]".to_string(),
            spec_type: FormatSpecType::Optional {
                inner: Box::new(ValidatedFormatSpec {
                    pattern: "/5n".to_string(),
                    spec_type: FormatSpecType::FixedNumeric { length: 5 },
                }),
            },
        },
    );

    // Common repetitive patterns
    map.insert(
        "4*35x",
        ValidatedFormatSpec {
            pattern: "4*35x".to_string(),
            spec_type: FormatSpecType::Repetitive {
                count: 4,
                inner: Box::new(ValidatedFormatSpec {
                    pattern: "35x".to_string(),
                    spec_type: FormatSpecType::VariableAny { max_length: 35 },
                }),
            },
        },
    );
    map.insert(
        "4*(1!n/33x)",
        ValidatedFormatSpec {
            pattern: "4*(1!n/33x)".to_string(),
            spec_type: FormatSpecType::Repetitive {
                count: 4,
                inner: Box::new(ValidatedFormatSpec {
                    pattern: "1!n/33x".to_string(),
                    spec_type: FormatSpecType::MultiComponent {
                        components: vec![
                            ValidatedFormatSpec {
                                pattern: "1!n".to_string(),
                                spec_type: FormatSpecType::FixedNumeric { length: 1 },
                            },
                            ValidatedFormatSpec {
                                pattern: "33x".to_string(),
                                spec_type: FormatSpecType::VariableAny { max_length: 33 },
                            },
                        ],
                    },
                }),
            },
        },
    );

    // Multi-component patterns
    map.insert(
        "1!a3!c",
        ValidatedFormatSpec {
            pattern: "1!a3!c".to_string(),
            spec_type: FormatSpecType::MultiComponent {
                components: vec![
                    ValidatedFormatSpec {
                        pattern: "1!a".to_string(),
                        spec_type: FormatSpecType::FixedAlphabetic { length: 1 },
                    },
                    ValidatedFormatSpec {
                        pattern: "3!c".to_string(),
                        spec_type: FormatSpecType::FixedCharacterSet { length: 3 },
                    },
                ],
            },
        },
    );

    // Complex BIC pattern
    map.insert(
        "4!a2!a2!c[3!c]",
        ValidatedFormatSpec {
            pattern: "4!a2!a2!c[3!c]".to_string(),
            spec_type: FormatSpecType::MultiComponent {
                components: vec![
                    ValidatedFormatSpec {
                        pattern: "4!a".to_string(),
                        spec_type: FormatSpecType::FixedAlphabetic { length: 4 },
                    },
                    ValidatedFormatSpec {
                        pattern: "2!a".to_string(),
                        spec_type: FormatSpecType::FixedAlphabetic { length: 2 },
                    },
                    ValidatedFormatSpec {
                        pattern: "2!c".to_string(),
                        spec_type: FormatSpecType::FixedCharacterSet { length: 2 },
                    },
                    ValidatedFormatSpec {
                        pattern: "[3!c]".to_string(),
                        spec_type: FormatSpecType::Optional {
                            inner: Box::new(ValidatedFormatSpec {
                                pattern: "3!c".to_string(),
                                spec_type: FormatSpecType::FixedCharacterSet { length: 3 },
                            }),
                        },
                    },
                ],
            },
        },
    );

    map
});

/// Validate a format specification at compile time
pub fn validate_format_spec(
    pattern: &str,
    span: Span,
) -> Result<ValidatedFormatSpec, crate::error::MacroError> {
    // First check if it's a known valid specification
    if let Some(spec) = KNOWN_VALID_SPECS.get(pattern) {
        return Ok(spec.clone());
    }

    // Try to parse and validate the pattern
    parse_format_pattern(pattern, span)
}

/// Parse and validate a format pattern
fn parse_format_pattern(
    pattern: &str,
    span: Span,
) -> Result<ValidatedFormatSpec, crate::error::MacroError> {
    // Handle optional patterns
    if pattern.starts_with('[') && pattern.ends_with(']') {
        let inner_pattern = &pattern[1..pattern.len() - 1];
        let inner_spec = parse_format_pattern(inner_pattern, span)?;
        return Ok(ValidatedFormatSpec {
            pattern: pattern.to_string(),
            spec_type: FormatSpecType::Optional {
                inner: Box::new(inner_spec),
            },
        });
    }

    // Handle repetitive patterns (e.g., "4*35x")
    if let Some(star_pos) = pattern.find('*') {
        let count_str = &pattern[..star_pos];
        let inner_pattern = &pattern[star_pos + 1..];

        let count = count_str.parse::<usize>().map_err(|_| {
            crate::error::MacroError::invalid_attribute(
                span,
                "component",
                pattern,
                "valid repetitive pattern like '4*35x'",
            )
        })?;

        let inner_spec = parse_format_pattern(inner_pattern, span)?;
        return Ok(ValidatedFormatSpec {
            pattern: pattern.to_string(),
            spec_type: FormatSpecType::Repetitive {
                count,
                inner: Box::new(inner_spec),
            },
        });
    }

    // Handle basic patterns
    if let Some(spec_type) = parse_basic_pattern(pattern) {
        return Ok(ValidatedFormatSpec {
            pattern: pattern.to_string(),
            spec_type,
        });
    }

    // If we can't parse it, it's an invalid format
    Err(crate::error::MacroError::invalid_attribute(
        span,
        "component",
        pattern,
        "valid SWIFT format specification (e.g., '3!a', '35x', '[35x]', '4*35x')",
    ))
}

/// Parse basic format patterns (non-optional, non-repetitive)
fn parse_basic_pattern(pattern: &str) -> Option<FormatSpecType> {
    // Handle decimal patterns (e.g., "15d")
    if pattern.ends_with('d') {
        if let Ok(digits) = pattern[..pattern.len() - 1].parse::<usize>() {
            return Some(FormatSpecType::Decimal { max_digits: digits });
        }
    }

    // Handle fixed-length patterns (e.g., "3!a", "6!n", "4!c")
    if let Some(exclamation_pos) = pattern.find('!') {
        if let Ok(length) = pattern[..exclamation_pos].parse::<usize>() {
            let type_char = pattern.chars().nth(exclamation_pos + 1)?;
            return match type_char {
                'a' => Some(FormatSpecType::FixedAlphabetic { length }),
                'n' => Some(FormatSpecType::FixedNumeric { length }),
                'c' => Some(FormatSpecType::FixedCharacterSet { length }),
                'x' => Some(FormatSpecType::VariableAny { max_length: length }),
                _ => None,
            };
        }
    }

    // Handle variable-length patterns (e.g., "35a", "15n", "34c", "35x")
    // Also handle legacy patterns without '!' for fixed length (e.g., "2a", "4n")
    if let Some(type_char) = pattern.chars().last() {
        if let Ok(length) = pattern[..pattern.len() - 1].parse::<usize>() {
            return match type_char {
                'a' => {
                    // For legacy compatibility, treat short patterns as fixed length
                    if length <= 4 {
                        Some(FormatSpecType::FixedAlphabetic { length })
                    } else {
                        Some(FormatSpecType::VariableAlphabetic { max_length: length })
                    }
                }
                'n' => {
                    // For legacy compatibility, treat short patterns as fixed length
                    if length <= 8 {
                        Some(FormatSpecType::FixedNumeric { length })
                    } else {
                        Some(FormatSpecType::VariableNumeric { max_length: length })
                    }
                }
                'c' => {
                    // For legacy compatibility, treat short patterns as fixed length
                    if length <= 4 {
                        Some(FormatSpecType::FixedCharacterSet { length })
                    } else {
                        Some(FormatSpecType::VariableCharacterSet { max_length: length })
                    }
                }
                'x' => Some(FormatSpecType::VariableAny { max_length: length }),
                _ => None,
            };
        }
    }

    None
}

/// Enhanced format validation with error recovery
#[allow(dead_code)] // Phase 3 infrastructure - will be integrated into main validation pipeline
pub fn validate_format_with_recovery(
    pattern: &str,
    field_name: &str,
    span: Span,
) -> MacroResult<ValidatedFormatSpec> {
    // First try standard validation
    if let Ok(spec) = validate_format_spec(pattern, span) {
        return Ok(spec);
    }

    // If validation fails, try error recovery
    if let Some(suggested_pattern) = suggest_format_correction(pattern) {
        return Err(MacroError::invalid_format(
            span,
            pattern,
            field_name,
            &format!("Invalid format. Did you mean '{suggested_pattern}'?"),
            None,
        ));
    }

    // If no specific suggestion, provide general guidance
    let common_patterns = [
        "3!a (fixed 3 alphabetic)",
        "6!n (fixed 6 numeric)",
        "35x (variable up to 35 characters)",
        "15d (decimal up to 15 digits)",
        "[35x] (optional)",
        "4*35x (repetitive)",
    ];

    Err(MacroError::invalid_format(
        span,
        pattern,
        field_name,
        &format!(
            "Invalid SWIFT format specification. Common patterns: {}",
            common_patterns.join(", ")
        ),
        None,
    ))
}

/// Suggest format corrections for common mistakes
#[allow(dead_code)] // Phase 3 infrastructure - helper function for error recovery
fn suggest_format_correction(pattern: &str) -> Option<String> {
    // Remove whitespace for analysis
    let clean_pattern = pattern.trim();

    // Common mistakes and corrections
    if clean_pattern.is_empty() {
        return Some("3!a".to_string());
    }

    // Missing exclamation for fixed length
    if clean_pattern.chars().all(|c| c.is_ascii_digit()) {
        if let Ok(num) = clean_pattern.parse::<usize>() {
            if num <= 35 {
                return Some(format!("{num}!a"));
            }
        }
    }

    // Wrong type indicator
    if clean_pattern.ends_with('s') && clean_pattern.len() > 1 {
        let without_s = &clean_pattern[..clean_pattern.len() - 1];
        if without_s.chars().all(|c| c.is_ascii_digit()) {
            return Some(format!("{without_s}x"));
        }
    }

    // Missing type indicator
    if clean_pattern.contains('!') && clean_pattern.len() >= 3 {
        let parts: Vec<&str> = clean_pattern.split('!').collect();
        if parts.len() == 2 && parts[0].chars().all(|c| c.is_ascii_digit())
            && parts[1].is_empty() {
                return Some(format!("{}!a", parts[0]));
            }
    }

    // Lowercase to uppercase correction
    if clean_pattern.contains(char::is_lowercase) {
        return Some(clean_pattern.to_uppercase());
    }

    None
}

/// Validate format at compile time and return suggestions if invalid
#[allow(dead_code)] // Phase 3 infrastructure - compile-time validation for future use
pub fn compile_time_validate(pattern: &str) -> Result<ValidatedFormatSpec, Vec<String>> {
    let span = proc_macro2::Span::call_site();

    match validate_format_spec(pattern, span) {
        Ok(spec) => Ok(spec),
        Err(_) => {
            let mut suggestions = Vec::new();

            if let Some(correction) = suggest_format_correction(pattern) {
                suggestions.push(format!("Did you mean '{correction}'?"));
            }

            suggestions.extend(vec![
                "Use valid SWIFT formats like:".to_string(),
                "- 3!a (fixed 3 alphabetic characters)".to_string(),
                "- 6!n (fixed 6 numeric digits)".to_string(),
                "- 35x (variable up to 35 characters)".to_string(),
                "- 15d (decimal number)".to_string(),
                "- [35x] (optional field)".to_string(),
                "- 4*35x (repetitive field)".to_string(),
            ]);

            Err(suggestions)
        }
    }
}

/// Generate compile-time checked regex patterns for validated specs
impl ValidatedFormatSpec {
    /// Get the regex pattern for this format specification
    #[allow(dead_code)]
    pub fn regex_pattern(&self) -> String {
        match &self.spec_type {
            FormatSpecType::FixedAlphabetic { length } => {
                format!("([A-Z]{{{length}}})")
            }
            FormatSpecType::VariableAlphabetic { max_length } => {
                format!("([A-Z]{{1,{max_length}}})")
            }
            FormatSpecType::FixedNumeric { length } => {
                format!("(\\d{{{length}}})")
            }
            FormatSpecType::VariableNumeric { max_length } => {
                format!("(\\d{{1,{max_length}}})")
            }
            FormatSpecType::FixedCharacterSet { length } => {
                format!("([A-Z0-9]{{{length}}})")
            }
            FormatSpecType::VariableCharacterSet { max_length } => {
                format!("([A-Z0-9]{{1,{max_length}}})")
            }
            FormatSpecType::VariableAny { max_length } => {
                format!("(.{{1,{max_length}}})")
            }
            FormatSpecType::Decimal { max_digits } => {
                format!("(\\d{{1,{max_digits}}}(?:[.,]\\d+)?)")
            }
            FormatSpecType::Optional { inner } => {
                let inner_pattern = inner.regex_pattern();
                format!(
                    "(?:{})?",
                    inner_pattern.trim_start_matches('(').trim_end_matches(')')
                )
            }
            FormatSpecType::Repetitive { count: _, inner } => {
                // Repetitive patterns need special handling in the parser
                inner.regex_pattern()
            }
            FormatSpecType::MultiComponent { components } => {
                let patterns: Vec<String> = components.iter().map(|c| c.regex_pattern()).collect();
                format!("({})", patterns.join(""))
            }
        }
    }

    /// Check if this pattern is optional
    #[allow(dead_code)]
    pub fn is_optional(&self) -> bool {
        matches!(self.spec_type, FormatSpecType::Optional { .. })
    }

    /// Check if this pattern is repetitive
    #[allow(dead_code)]
    pub fn is_repetitive(&self) -> bool {
        matches!(self.spec_type, FormatSpecType::Repetitive { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;

    #[test]
    fn test_validate_known_specs() {
        let span = Span::call_site();

        // Test known valid specs
        assert!(validate_format_spec("3!a", span).is_ok());
        assert!(validate_format_spec("6!n", span).is_ok());
        assert!(validate_format_spec("35x", span).is_ok());
        assert!(validate_format_spec("15d", span).is_ok());
        assert!(validate_format_spec("[35x]", span).is_ok());
        assert!(validate_format_spec("4*35x", span).is_ok());
    }

    #[test]
    fn test_validate_invalid_specs() {
        let span = Span::call_site();

        // Test invalid specs
        assert!(validate_format_spec("invalid", span).is_err());
        assert!(validate_format_spec("3!z", span).is_err());
        assert!(validate_format_spec("abc!n", span).is_err());
    }

    #[test]
    fn test_regex_patterns() {
        let span = Span::call_site();

        let spec = validate_format_spec("3!a", span).unwrap();
        assert_eq!(spec.regex_pattern(), "([A-Z]{3})");

        let spec = validate_format_spec("35x", span).unwrap();
        assert_eq!(spec.regex_pattern(), "(.{1,35})");

        let spec = validate_format_spec("15d", span).unwrap();
        assert_eq!(spec.regex_pattern(), "(\\d{1,15}(?:[.,]\\d+)?)");
    }
}
