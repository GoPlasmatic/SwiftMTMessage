//! SWIFT field formatting utilities for macro code generation
//!
//! This module provides utilities for formatting and parsing SWIFT field components
//! according to SWIFT format specifications like "16x", "5n", "4!c", etc.
//! Used by the derive macros to generate formatting code.

use crate::ast::StructField;
use crate::error::MacroResult;
use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use syn::Type;

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

    Ok(format!("^{}$", regex_parts.join("")))
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
                    let combined_pattern = format!("{}{}", pattern, second_pattern);
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
        return Ok(format!("(?:{inner_regex})?"));
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
            return Ok(format!("(?:({first_clean}{second_clean}))?"));
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
            return Ok(format!("/({}))/", inner_regex_no_parens));
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
            return Ok(format!("(/{}{})", first_no_parens, second_no_parens));
        }
    }

    // Handle simple slash prefix patterns like /34x
    if pattern.starts_with('/') && pattern.len() > 1 {
        let after_slash = &pattern[1..];
        let inner_regex = pattern_to_regex(after_slash)?;
        // Remove the capturing group from inner regex since we'll add our own
        let inner_regex_no_parens = inner_regex.trim_start_matches('(').trim_end_matches(')');
        // Match the slash but don't capture it, only capture the content after
        return Ok(format!("/({inner_regex_no_parens})"));
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
                return Ok(format!(r"(\d{{1,{length}}}(?:[.,]\d+)?)"));
            }

            if !length_str.is_empty() {
                let length: usize = length_str.parse().unwrap_or(1);
                if is_fixed {
                    return Ok(format!("({char_class}{{{length}}})"));
                } else {
                    return Ok(format!("({char_class}{{1,{length}}})"));
                }
            } else {
                return Ok(format!("({char_class}+)"));
            }
        }
    }

    Ok(format!("({})", regex::escape(pattern)))
}

/// Convert all capturing groups in a regex to non-capturing groups
/// This ensures that we can wrap the entire pattern in a single capturing group
fn convert_to_non_capturing_groups(regex: &str) -> String {
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
    if struct_field.components.is_empty() {
        return Ok(quote! {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "No components defined".to_string(),
            });
        });
    }

    // Generate regex for each component separately to ensure one capture group per component
    let component_names: Vec<_> = struct_field.components.iter().map(|c| &c.name).collect();
    let mut regex_parts = Vec::new();

    for component in &struct_field.components {
        let component_regex = swift_format_to_regex(&component.format.pattern)?;
        // Remove the ^ and $ anchors since we'll add them to the final pattern
        let component_regex = component_regex
            .trim_start_matches('^')
            .trim_end_matches('$');

        // Convert all inner capturing groups to non-capturing groups to ensure exactly one capture per component
        let non_capturing_regex = convert_to_non_capturing_groups(component_regex);

        // Special case: if the regex already has proper capture groups, don't wrap it
        if (non_capturing_regex.starts_with('/')
            && non_capturing_regex.ends_with('/')
            && non_capturing_regex.contains('('))
            || (non_capturing_regex.starts_with("(?:/") && non_capturing_regex.contains('('))
        {
            regex_parts.push(non_capturing_regex);
        } else {
            // Wrap each component in exactly one capture group
            regex_parts.push(format!("({non_capturing_regex})"));
        }
    }

    // Determine if this is a multiline field that needs newline separators
    let is_multiline_field = is_multiline_field_type(name, struct_field);

    // For Field50K and Field59NoOption specifically, handle the optional account pattern followed by multiline address
    if (name.to_string() == "Field50K" || name.to_string() == "Field59NoOption")
        && struct_field.components.len() == 2
    {
        // Verify the field has the expected pattern: [/34x] followed by 4*35x
        let first_component = &struct_field.components[0];
        let second_component = &struct_field.components[1];

        if first_component.format.pattern == "[/34x]" && second_component.format.pattern == "4*35x"
        {
            // These fields have optional account [/34x] followed by multiline address 4*35x
            // We need to handle both cases: with and without account
            let account_pattern = &regex_parts[0];
            let address_pattern = &regex_parts[1];

            // Remove the outer capture groups temporarily
            let account_inner = account_pattern
                .trim_start_matches("(?:")
                .trim_end_matches(")?");
            let address_inner = address_pattern
                .trim_start_matches("(")
                .trim_end_matches(")");

            // Create a pattern that matches either:
            // 1. /account followed by newline and addresses, or
            // 2. Just the addresses without account
            let regex_pattern = format!("^(?:{}\\n)?({})$", account_inner, address_inner);

            // We need to adjust the field assignments for this special case
            let mut field_assignments = Vec::new();

            // Account field (optional) - extract from the beginning of the input if present
            field_assignments.push(quote! {
                let account = if value.trim().starts_with('/') {
                    // Extract account from the first line if it starts with '/'
                    let lines: Vec<&str> = value.trim().lines().collect();
                    if let Some(first_line) = lines.first() {
                        if first_line.starts_with('/') {
                            // For Field50K/Field59NoOption, account is optional and doesn't include the slash
                            let account_value = first_line.trim_start_matches('/');
                            Some(account_value.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };
            });

            // Name and address field (required) - skip first line if it's an account
            field_assignments.push(quote! {
                let name_and_address = {
                    // Split the input into lines
                    let lines: Vec<&str> = value.trim().lines().collect();

                    // If the first line starts with '/', skip it (it's the account)
                    let start_index = if lines.first().map_or(false, |line| line.starts_with('/')) {
                        1
                    } else {
                        0
                    };

                    // Collect the name and address lines
                    lines.iter()
                        .skip(start_index)
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect()
                };
            });

            return Ok(quote! {
                use regex::Regex;

                let pattern = #regex_pattern;
                let re = Regex::new(pattern).map_err(|e| crate::errors::ParseError::InvalidFormat {
                    message: format!("Invalid regex pattern: {}", e),
                })?;

                let captures = re.captures(value.trim()).ok_or_else(|| crate::errors::ParseError::InvalidFormat {
                    message: format!("Value does not match expected pattern: {}", pattern),
                })?;

                #(#field_assignments)*

                Ok(Self {
                    account,
                    name_and_address,
                })
            });
        }
    }

    let separator = if is_multiline_field { r"\n" } else { "" };
    let regex_pattern = format!("^{}$", regex_parts.join(separator));

    // Generate field assignments with type conversion
    let mut field_assignments = Vec::new();
    for (i, component) in struct_field.components.iter().enumerate() {
        let field_name = &component.name;
        let capture_index = i + 1; // Regex captures are 1-indexed

        if component.is_repetitive {
            // For Vec<String> fields, split by newlines
            field_assignments.push(quote! {
                let #field_name = if let Some(captured) = captures.get(#capture_index) {
                    captured.as_str().lines()
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect()
                } else {
                    Vec::new()
                };
            });
        } else if component.is_optional {
            // For Option types, convert the inner value and wrap in Some/None
            let inner_type = extract_option_inner_type(&component.field_type)?;
            let conversion_expr = generate_type_conversion_expr(&inner_type, quote! { raw_value })?;
            field_assignments.push(quote! {
                let #field_name = if let Some(captured) = captures.get(#capture_index) {
                    let raw_value = captured.as_str();
                    if raw_value.is_empty() {
                        None
                    } else {
                        Some(#conversion_expr)
                    }
                } else {
                    None
                };
            });
        } else {
            let conversion_expr =
                generate_type_conversion_expr(&component.field_type, quote! { raw_value })?;
            field_assignments.push(quote! {
                let raw_value = captures.get(#capture_index)
                    .ok_or_else(|| crate::errors::ParseError::InvalidFormat {
                        message: format!("Missing component {}", stringify!(#field_name)),
                    })?
                    .as_str();
                let #field_name = #conversion_expr;
            });
        }
    }

    Ok(quote! {
        use regex::Regex;

        let pattern = #regex_pattern;
        let re = Regex::new(pattern).map_err(|e| crate::errors::ParseError::InvalidFormat {
            message: format!("Invalid regex pattern: {}", e),
        })?;

        let captures = re.captures(value.trim()).ok_or_else(|| crate::errors::ParseError::InvalidFormat {
            message: format!("Value does not match expected pattern: {}", pattern),
        })?;

        #(#field_assignments)*

        Ok(Self {
            #(#component_names),*
        })
    })
}

/// Determine if a field should use multiline parsing (components separated by newlines)
fn is_multiline_field_type(_name: &syn::Ident, struct_field: &StructField) -> bool {
    if struct_field.components.len() <= 1 {
        return false;
    }

    // Also check for specific patterns that indicate multiline fields
    for component in &struct_field.components {
        let pattern = &component.format.pattern;

        // Repetitive patterns with * that indicate multiple lines of text
        // Examples: 4*35x, 6*65x, 35*50x, 20*35x (multiple address lines)
        // 4*(1!n/33x) (numbered address lines)
        if pattern.contains("*") || pattern.contains("4*") || pattern.contains("4!(") {
            return true;
        }
    }

    false
}

/// Extract inner type from Option<T>
fn extract_option_inner_type(ty: &Type) -> MacroResult<Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        return Ok(inner_type.clone());
                    }
                }
            }
        }
    }
    Err(crate::error::MacroError::internal(
        proc_macro2::Span::call_site(),
        "Expected Option<T> type",
    ))
}

/// Generate type conversion expression from string to target type
fn generate_type_conversion_expr(
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
