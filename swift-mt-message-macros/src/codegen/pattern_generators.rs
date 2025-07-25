//! Pattern-specific field generators using the strategy pattern
//!
//! This module implements different field pattern generators to break down
//! the complex `generate_regex_parse_impl` function into manageable pieces.

use crate::ast::StructField;
use crate::error::MacroResult;
use crate::format::{generate_type_conversion_expr, swift_format_to_regex};
use crate::utils::types::extract_inner_type;
use proc_macro2::TokenStream;
use quote::quote;

/// Trait for field pattern generators
pub trait FieldPatternGenerator: std::fmt::Debug {
    /// Check if this generator can handle the given field
    fn can_handle(&self, name: &syn::Ident, field: &StructField) -> bool;

    /// Generate parser code for the field
    fn generate_parser(&self, name: &syn::Ident, field: &StructField) -> MacroResult<TokenStream>;
}

/// Generator for simple single-component patterns
#[derive(Debug)]
pub struct SimplePatternGenerator;

impl FieldPatternGenerator for SimplePatternGenerator {
    fn can_handle(&self, _name: &syn::Ident, field: &StructField) -> bool {
        // Handle fields with a single component and no special processing
        field.components.len() == 1
            && !field.components[0].is_repetitive
            && !field.components[0].format.pattern.contains('*')
            && !field.components[0].format.pattern.contains('[')
    }

    fn generate_parser(&self, name: &syn::Ident, field: &StructField) -> MacroResult<TokenStream> {
        let component = &field.components[0];
        let field_name = &component.name;
        let pattern = &component.format.pattern;

        // Generate regex
        let regex_pattern = swift_format_to_regex(pattern)?;

        // Generate conversion logic
        let conversion_expr =
            generate_type_conversion_expr(&component.field_type, quote! { raw_value })?;

        let name_str = name.to_string();

        Ok(quote! {
            use once_cell::sync::Lazy;
            use regex::Regex;

            static PATTERN_REGEX: Lazy<Regex> = Lazy::new(|| {
                Regex::new(#regex_pattern).unwrap()
            });

            let captures = PATTERN_REGEX.captures(value.trim())
                .ok_or_else(|| crate::errors::ParseError::ComponentParseError {
                    field_tag: #name_str.to_string(),
                    component_index: 0,
                    component_name: stringify!(#field_name).to_string(),
                    expected_format: #pattern.to_string(),
                    actual_value: value.to_string(),
                })?;

            let raw_value = captures.get(1)
                .ok_or_else(|| crate::errors::ParseError::ComponentParseError {
                    field_tag: #name_str.to_string(),
                    component_index: 0,
                    component_name: stringify!(#field_name).to_string(),
                    expected_format: #pattern.to_string(),
                    actual_value: value.to_string(),
                })?
                .as_str();

            let #field_name = #conversion_expr;

            Ok(Self { #field_name })
        })
    }
}

/// Generator for optional patterns like [35x]
#[derive(Debug)]
pub struct OptionalPatternGenerator;

impl FieldPatternGenerator for OptionalPatternGenerator {
    fn can_handle(&self, _name: &syn::Ident, field: &StructField) -> bool {
        field.components.len() == 1
            && field.components[0].is_optional
            && field.components[0].format.pattern.starts_with('[')
            && field.components[0].format.pattern.ends_with(']')
    }

    fn generate_parser(&self, name: &syn::Ident, field: &StructField) -> MacroResult<TokenStream> {
        let component = &field.components[0];
        let field_name = &component.name;
        let pattern = &component.format.pattern;

        // Generate regex
        let regex_pattern = swift_format_to_regex(pattern)?;

        // Generate human-readable format description
        let format_desc = crate::format::format_to_description(pattern);

        // Extract inner type from Option<T>
        let inner_type = extract_inner_type(&component.field_type, true, false);

        // Special handling for is_negative field in Field37H
        let conversion_expr = if *field_name == "is_negative" && pattern == "[1!a]" {
            // For is_negative field, 'N' means true (the rate is negative)
            quote! {
                match raw_value {
                    "N" => true,
                    _ => return Err(crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                        field_tag: stringify!(#name).to_string(),
                        component_name: stringify!(#field_name).to_string(),
                        value: raw_value.to_string(),
                        format_spec: "N for negative rate".to_string(),
                        position: None,
                        inner_error: "Expected 'N' for negative rate indicator".to_string(),
                    })))
                }
            }
        } else {
            generate_type_conversion_expr(&inner_type, quote! { raw_value })?
        };

        Ok(quote! {
            use once_cell::sync::Lazy;
            use regex::Regex;

            static PATTERN_REGEX: Lazy<Regex> = Lazy::new(|| {
                Regex::new(#regex_pattern).unwrap()
            });

            let captures = PATTERN_REGEX.captures(value.trim())
                .ok_or_else(|| crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                    field_tag: stringify!(#name).to_string(),
                    component_name: stringify!(#field_name).to_string(),
                    value: value.to_string(),
                    format_spec: #format_desc.to_string(),
                    position: None,
                    inner_error: "Value does not match expected pattern".to_string(),
                })))?;

            let #field_name = if let Some(captured) = captures.get(1) {
                let raw_value = captured.as_str();
                if raw_value.is_empty() {
                    None
                } else {
                    Some(#conversion_expr)
                }
            } else {
                None
            };

            Ok(Self { #field_name })
        })
    }
}

/// Generator for repetitive patterns like 4*35x
#[derive(Debug)]
pub struct RepetitivePatternGenerator;

impl FieldPatternGenerator for RepetitivePatternGenerator {
    fn can_handle(&self, _name: &syn::Ident, field: &StructField) -> bool {
        field.components.len() == 1
            && (field.components[0].is_repetitive
                || field.components[0].format.pattern.contains('*'))
    }

    fn generate_parser(&self, name: &syn::Ident, field: &StructField) -> MacroResult<TokenStream> {
        let component = &field.components[0];
        let field_name = &component.name;
        let pattern = &component.format.pattern;

        // Generate regex
        let regex_pattern = swift_format_to_regex(pattern)?;

        // Generate human-readable format description
        let format_desc = match pattern.as_str() {
            "4*35x" => "Up to 4 lines of 35 characters each".to_string(),
            "3*35x" => "Up to 3 lines of 35 characters each".to_string(),
            "6*35x" => "Up to 6 lines of 35 characters each".to_string(),
            _ if pattern.contains('*') => {
                let parts: Vec<&str> = pattern.split('*').collect();
                if parts.len() == 2 {
                    format!(
                        "Up to {} lines of {} characters each",
                        parts[0],
                        parts[1].trim_end_matches(char::is_alphabetic)
                    )
                } else {
                    pattern.to_string()
                }
            }
            _ => pattern.to_string(),
        };

        Ok(quote! {
            use once_cell::sync::Lazy;
            use regex::Regex;

            static PATTERN_REGEX: Lazy<Regex> = Lazy::new(|| {
                Regex::new(#regex_pattern).unwrap()
            });

            let captures = PATTERN_REGEX.captures(value.trim())
                .ok_or_else(|| crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                    field_tag: stringify!(#name).to_string(),
                    component_name: stringify!(#field_name).to_string(),
                    value: value.to_string(),
                    format_spec: #format_desc.to_string(),
                    position: None,
                    inner_error: "Value does not match expected pattern".to_string(),
                })))?;

            let #field_name = if let Some(captured) = captures.get(1) {
                captured.as_str().lines()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect()
            } else {
                Vec::new()
            };

            Ok(Self { #field_name })
        })
    }
}

/// Generator for special Field53B/Field57B patterns
#[derive(Debug)]
pub struct Field53B57BPatternGenerator;

impl FieldPatternGenerator for Field53B57BPatternGenerator {
    fn can_handle(&self, name: &syn::Ident, field: &StructField) -> bool {
        (name == "Field53B" || name == "Field57B")
            && field.components.len() == 2
            && field.components[0].format.pattern == "[/1!a][/34x]"
            && field.components[1].format.pattern == "[35x]"
    }

    fn generate_parser(&self, _name: &syn::Ident, field: &StructField) -> MacroResult<TokenStream> {
        let first_field_name = &field.components[0].name;
        let second_field_name = &field.components[1].name;

        Ok(quote! {
            // Handle Field53B/Field57B parsing: optional party identifier + optional location
            let lines: Vec<&str> = value.trim().lines().collect();

            let (#first_field_name, #second_field_name) = if lines.len() == 2 {
                // Two lines: first is party identifier, second is location
                let first_line = lines[0];
                let second_line = lines[1];

                // Parse party identifier (remove leading slash)
                let party_id = if first_line.starts_with('/') {
                    Some(first_line.trim_start_matches('/').to_string())
                } else {
                    None
                };

                // Parse location
                let location = if !second_line.is_empty() {
                    Some(second_line.to_string())
                } else {
                    None
                };

                (party_id, location)
            } else if lines.len() == 1 {
                // One line: could be either party identifier or location
                let line = lines[0];

                if line.starts_with('/') {
                    // This is a party identifier only
                    (Some(line.trim_start_matches('/').to_string()), None)
                } else {
                    // This is a location only
                    (None, Some(line.to_string()))
                }
            } else {
                // Empty or unexpected format
                (None, None)
            };

            Ok(Self {
                #first_field_name,
                #second_field_name,
            })
        })
    }
}

/// Generator for fields with optional party ID + multiline address (Field50K, Field59, etc.)
#[derive(Debug)]
pub struct PartyAddressPatternGenerator;

impl FieldPatternGenerator for PartyAddressPatternGenerator {
    fn can_handle(&self, name: &syn::Ident, field: &StructField) -> bool {
        let party_fields = [
            "Field50K",
            "Field59NoOption",
            "Field53D",
            "Field52D",
            "Field56D",
            "Field57D",
            "Field59F",
        ];
        party_fields.contains(&name.to_string().as_str())
            && field.components.len() == 2
            && (field.components[0].format.pattern == "[/34x]"
                || field.components[0].format.pattern == "[/1!a][/34x]")
            && (field.components[1].format.pattern == "4*35x"
                || field.components[1].format.pattern == "4*(1!n/33x)")
    }

    fn generate_parser(&self, _name: &syn::Ident, field: &StructField) -> MacroResult<TokenStream> {
        let first_field_name = &field.components[0].name;
        let second_field_name = &field.components[1].name;

        Ok(quote! {
            // Account field (optional) - extract from the beginning of the input if present
            let #first_field_name = if value.trim().starts_with('/') {
                // Extract account from the first line if it starts with '/'
                let lines: Vec<&str> = value.trim().lines().collect();
                if let Some(first_line) = lines.first() {
                    if first_line.starts_with('/') {
                        // Account doesn't include the slash
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

            // Name and address field (required) - skip first line if it's an account
            let #second_field_name = {
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

            Ok(Self {
                #first_field_name,
                #second_field_name,
            })
        })
    }
}

/// Generator for multiline fields with optional first component
#[derive(Debug)]
pub struct OptionalMultilinePatternGenerator;

impl FieldPatternGenerator for OptionalMultilinePatternGenerator {
    fn can_handle(&self, _name: &syn::Ident, field: &StructField) -> bool {
        field.components.len() == 2
            && field.components[0].is_optional
            && field.components[0].format.pattern.starts_with('[')
            && field.components[0].format.pattern.ends_with(']')
            && !field.components[1].is_repetitive
    }

    fn generate_parser(&self, name: &syn::Ident, field: &StructField) -> MacroResult<TokenStream> {
        let first_field_name = &field.components[0].name;
        let second_field_name = &field.components[1].name;
        let second_is_optional = field.components[1].is_optional;

        let mut field_assignments = vec![quote! {
            let lines: Vec<&str> = value.trim().lines().collect();

            let (#first_field_name, #second_field_name) = if lines.len() == 2 {
                // Two lines: first is optional component, second is required component
                let first_line = lines[0];
                let second_line = lines[1];

                // Parse first component
                let first_val = if first_line.starts_with('/') {
                    // Remove leading slash for party identifiers
                    Some(first_line.trim_start_matches('/').to_string())
                } else {
                    Some(first_line.to_string())
                };

                // Parse second component
                #[allow(clippy::redundant_clone)]
                let second_val = second_line.to_string();

                (first_val, second_val)
            } else if lines.len() == 1 {
                // One line: only second component is present
                let line = lines[0];

                // Check if this line could be the optional first component
                if line.starts_with('/') && line.len() <= 35 {
                    // Looks like a party identifier - but where's the second component?
                    (None, line.to_string())
                } else {
                    // This is the second component
                    (None, line.to_string())
                }
            } else {
                // Unexpected format
                return Err(crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                    field_tag: stringify!(#name).to_string(),
                    component_name: "multiline".to_string(),
                    value: value.to_string(),
                    format_spec: "1 or 2 lines".to_string(),
                    position: None,
                    inner_error: format!("Expected 1 or 2 lines, got {}", lines.len()),
                })));
            };
        }];

        // For optional second components, wrap in Option
        if second_is_optional {
            field_assignments.push(quote! {
                let #second_field_name = if #second_field_name.is_empty() {
                    None
                } else {
                    Some(#second_field_name)
                };
            });
        }

        Ok(quote! {
            #(#field_assignments)*

            Ok(Self {
                #first_field_name,
                #second_field_name,
            })
        })
    }
}

/// Main field parser generator that uses the strategy pattern
#[derive(Debug)]
pub struct FieldParserGenerator {
    generators: Vec<Box<dyn FieldPatternGenerator>>,
}

impl FieldParserGenerator {
    pub fn new() -> Self {
        Self {
            generators: vec![
                Box::new(Field53B57BPatternGenerator),
                Box::new(PartyAddressPatternGenerator),
                Box::new(OptionalMultilinePatternGenerator),
                Box::new(RepetitivePatternGenerator),
                Box::new(OptionalPatternGenerator),
                Box::new(SimplePatternGenerator),
            ],
        }
    }

    pub fn generate_parser(
        &self,
        name: &syn::Ident,
        field: &StructField,
    ) -> MacroResult<TokenStream> {
        // Find the first generator that can handle this field
        for generator in &self.generators {
            if generator.can_handle(name, field) {
                return generator.generate_parser(name, field);
            }
        }

        // Fall back to generic multi-component generator
        self.generate_generic_parser(name, field)
    }

    /// Generate parser for generic multi-component fields
    fn generate_generic_parser(
        &self,
        name: &syn::Ident,
        field: &StructField,
    ) -> MacroResult<TokenStream> {
        use crate::format::{
            build_capturing_pattern, build_separated_pattern, convert_to_non_capturing_groups,
        };

        if field.components.is_empty() {
            return Ok(quote! {
                return Err(crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                    field_tag: stringify!(#name).to_string(),
                    component_name: "components".to_string(),
                    value: value.to_string(),
                    format_spec: "at least one component".to_string(),
                    position: None,
                    inner_error: "No components defined".to_string(),
                })));
            });
        }

        // Generate regex for each component
        let component_names: Vec<_> = field.components.iter().map(|c| &c.name).collect();
        let mut regex_parts = Vec::new();

        for component in &field.components {
            let component_regex = swift_format_to_regex(&component.format.pattern)?;
            let component_regex = component_regex
                .trim_start_matches('^')
                .trim_end_matches('$');

            let non_capturing_regex = convert_to_non_capturing_groups(component_regex);

            if (non_capturing_regex.starts_with('/')
                && non_capturing_regex.ends_with('/')
                && non_capturing_regex.contains('('))
                || (non_capturing_regex.starts_with("(?:/") && non_capturing_regex.contains('('))
            {
                regex_parts.push(non_capturing_regex);
            } else {
                regex_parts.push(build_capturing_pattern(&non_capturing_regex));
            }
        }

        // Determine if this is a multiline field
        let is_multiline = self.is_multiline_field(name, field);
        let separator = if is_multiline { r"\n" } else { "" };
        let regex_pattern = build_separated_pattern(&regex_parts, separator);

        // Generate field assignments
        let mut field_assignments = Vec::new();
        for (i, component) in field.components.iter().enumerate() {
            let field_name = &component.name;
            let capture_index = i + 1;

            if component.is_repetitive {
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
                let inner_type = extract_inner_type(&component.field_type, true, false);

                // Special handling for is_negative field in Field37H
                let conversion_expr = if *field_name == "is_negative"
                    && component.format.pattern == "[1!a]"
                {
                    // For is_negative field, 'N' means true (the rate is negative)
                    quote! {
                        match raw_value {
                            "N" => true,
                            _ => return Err(crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                                field_tag: stringify!(#name).to_string(),
                                component_name: stringify!(#field_name).to_string(),
                                value: raw_value.to_string(),
                                format_spec: "N for negative rate".to_string(),
                                position: None,
                                inner_error: "Expected 'N' for negative rate indicator".to_string(),
                            })))
                        }
                    }
                } else {
                    generate_type_conversion_expr(&inner_type, quote! { raw_value })?
                };

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
                let component_pattern = &component.format.pattern;
                let conversion_expr =
                    generate_type_conversion_expr(&component.field_type, quote! { raw_value })?;
                field_assignments.push(quote! {
                    let raw_value = captures.get(#capture_index)
                        .ok_or_else(|| crate::errors::ParseError::ComponentParseError {
                            field_tag: stringify!(#name).to_string(),
                            component_index: #i,
                            component_name: stringify!(#field_name).to_string(),
                            expected_format: #component_pattern.to_string(),
                            actual_value: value.to_string(),
                        })?
                        .as_str();
                    let #field_name = #conversion_expr;
                });
            }
        }

        let pattern_without_anchors = regex_pattern.trim_start_matches('^').trim_end_matches('$');

        // Check if this is a known pattern that should use the cache
        let cache_key = if field.components.len() == 2
            && field.components[0].format.pattern == "5n"
            && field.components[1].format.pattern == "/5n"
        {
            // This is the Field28D pattern "5n/5n"
            "5n/5n"
        } else {
            pattern_without_anchors
        };

        // Generate the friendly format at compile time
        let friendly_format_desc = crate::format::format_to_description(&regex_pattern);

        Ok(quote! {
            use once_cell::sync::Lazy;
            use regex::Regex;
            use std::collections::HashMap;

            // Pre-compiled regex cache for common SWIFT patterns
            static REGEX_CACHE: Lazy<HashMap<&'static str, Regex>> = Lazy::new(|| {
                let mut cache = HashMap::new();

                // Common patterns
                cache.insert("3!a", Regex::new(r"^([A-Z]{3})$").unwrap());
                cache.insert("4!a", Regex::new(r"^([A-Z]{4})$").unwrap());
                cache.insert("6!n", Regex::new(r"^(\d{6})$").unwrap());
                cache.insert("15d", Regex::new(r"^(\d{1,15}(?:[.,]\d+)?)$").unwrap());
                cache.insert("16x", Regex::new(r"^(.{1,16})$").unwrap());
                cache.insert("35x", Regex::new(r"^(.{1,35})$").unwrap());
                cache.insert("5n/5n", Regex::new(r"^(\d{1,5})/(\d{1,5})$").unwrap());

                cache
            });

            // Try to get regex from cache first
            let regex = if let Some(cached_regex) = REGEX_CACHE.get(#cache_key) {
                cached_regex
            } else {
                static FALLBACK_REGEX: Lazy<Regex> = Lazy::new(|| {
                    Regex::new(#regex_pattern).unwrap()
                });
                &*FALLBACK_REGEX
            };

            let captures = regex.captures(value.trim()).ok_or_else(|| {
                crate::errors::ParseError::InvalidFieldFormat(Box::new(crate::errors::InvalidFieldFormatError {
                    field_tag: stringify!(#name).to_string(),
                    component_name: "value".to_string(),
                    value: value.to_string(),
                    format_spec: #friendly_format_desc.to_string(),
                    position: None,
                    inner_error: "Format validation failed".to_string(),
                }))
            })?;

            #(#field_assignments)*

            Ok(Self {
                #(#component_names),*
            })
        })
    }

    /// Determine if a field should use multiline parsing
    fn is_multiline_field(&self, _name: &syn::Ident, field: &StructField) -> bool {
        if field.components.len() <= 1 {
            return false;
        }

        // Check for patterns that indicate multiline fields
        for component in &field.components {
            let pattern = &component.format.pattern;
            if pattern.contains("*") || pattern.contains("4*") || pattern.contains("4!(") {
                return true;
            }
        }

        false
    }
}

impl Default for FieldParserGenerator {
    fn default() -> Self {
        Self::new()
    }
}
