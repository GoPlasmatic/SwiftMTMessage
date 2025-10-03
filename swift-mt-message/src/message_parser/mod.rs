//! # Message Parser Module
//!
//! Pointer-based parser for SWIFT MT messages that tracks position while parsing fields sequentially.
//! This replaces the HashMap-based approach with a more efficient single-pass parser.

use crate::errors::{InvalidFieldFormatError, ParseError};
use crate::traits::SwiftField;
use std::collections::HashSet;

pub mod field_extractor;

pub use field_extractor::extract_field_content;

/// Message parser that tracks position while parsing SWIFT messages
#[derive(Debug)]
pub struct MessageParser<'a> {
    /// The input message text
    input: &'a str,
    /// Current position in the input
    position: usize,
    /// Fields that have been parsed (for duplicate detection)
    fields_seen: HashSet<String>,
    /// Message type being parsed
    message_type: String,
    /// Whether to allow duplicate fields
    allow_duplicates: bool,
}

impl<'a> MessageParser<'a> {
    /// Create a new message parser
    pub fn new(input: &'a str, message_type: &str) -> Self {
        Self {
            input,
            position: 0,
            fields_seen: HashSet::new(),
            message_type: message_type.to_string(),
            allow_duplicates: false,
        }
    }

    /// Enable or disable duplicate field handling
    pub fn with_duplicates(mut self, allow: bool) -> Self {
        self.allow_duplicates = allow;
        self
    }

    /// Parse a required field
    pub fn parse_field<T: SwiftField>(&mut self, tag: &str) -> Result<T, ParseError> {
        let field_content = self.extract_field(tag, false)?;

        // Try to parse the field
        T::parse(&field_content).map_err(|e| {
            ParseError::InvalidFieldFormat(Box::new(InvalidFieldFormatError {
                field_tag: tag.to_string(),
                component_name: "field".to_string(),
                value: field_content,
                format_spec: "field format".to_string(),
                position: Some(self.position),
                inner_error: e.to_string(),
            }))
        })
    }

    /// Parse an optional field (only checks immediate next field, not searching ahead)
    pub fn parse_optional_field<T: SwiftField>(
        &mut self,
        tag: &str,
    ) -> Result<Option<T>, ParseError> {
        // For optional fields, only check if the immediate next field matches
        // Don't search ahead in the input to avoid consuming fields from later sections
        if !self.detect_field(tag) {
            return Ok(None);
        }

        // If immediate next field matches, extract and parse it
        match self.extract_field(tag, true) {
            Ok(content) => {
                let parsed = T::parse(&content).map_err(|e| {
                    ParseError::InvalidFieldFormat(Box::new(InvalidFieldFormatError {
                        field_tag: tag.to_string(),
                        component_name: "field".to_string(),
                        value: content,
                        format_spec: "field format".to_string(),
                        position: Some(self.position),
                        inner_error: e.to_string(),
                    }))
                })?;
                Ok(Some(parsed))
            }
            Err(_) => Ok(None), // Field not found, return None for optional
        }
    }

    /// Parse a repeated field (returns Vec)
    pub fn parse_repeated_field<T: SwiftField>(&mut self, tag: &str) -> Result<Vec<T>, ParseError> {
        let mut results = Vec::new();

        // Keep parsing until no more instances found
        while let Ok(content) = self.extract_field(tag, true) {
            let parsed = T::parse(&content).map_err(|e| {
                ParseError::InvalidFieldFormat(Box::new(InvalidFieldFormatError {
                    field_tag: tag.to_string(),
                    component_name: "field".to_string(),
                    value: content,
                    format_spec: "field format".to_string(),
                    position: Some(self.position),
                    inner_error: e.to_string(),
                }))
            })?;
            results.push(parsed);
        }

        Ok(results)
    }

    /// Parse a field with variant detection (for enum fields)
    pub fn parse_variant_field<T: SwiftField>(&mut self, base_tag: &str) -> Result<T, ParseError> {
        // Look ahead to find which variant is present
        let variant = self.detect_variant(base_tag)?;
        let full_tag = format!("{}{}", base_tag, variant);
        let field_content = self.extract_field(&full_tag, false)?;

        // Use parse_with_variant for enum fields
        T::parse_with_variant(&field_content, Some(&variant), Some(base_tag)).map_err(|e| {
            ParseError::InvalidFieldFormat(Box::new(InvalidFieldFormatError {
                field_tag: full_tag,
                component_name: "field".to_string(),
                value: field_content,
                format_spec: "field format".to_string(),
                position: Some(self.position),
                inner_error: e.to_string(),
            }))
        })
    }

    /// Parse an optional field with variant detection
    pub fn parse_optional_variant_field<T: SwiftField>(
        &mut self,
        base_tag: &str,
    ) -> Result<Option<T>, ParseError> {
        match self.detect_variant_optional(base_tag) {
            Some(variant) => {
                let full_tag = format!("{}{}", base_tag, variant);
                if let Ok(content) = self.extract_field(&full_tag, true) {
                    let parsed = T::parse_with_variant(&content, Some(&variant), Some(base_tag))
                        .map_err(|e| {
                            ParseError::InvalidFieldFormat(Box::new(InvalidFieldFormatError {
                                field_tag: full_tag,
                                component_name: "field".to_string(),
                                value: content,
                                format_spec: "field format".to_string(),
                                position: Some(self.position),
                                inner_error: e.to_string(),
                            }))
                        })?;
                    Ok(Some(parsed))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None), // No variant found
        }
    }

    /// Extract field content from the message
    fn extract_field(&mut self, tag: &str, optional: bool) -> Result<String, ParseError> {
        // Check for duplicates if not allowed
        if !self.allow_duplicates && self.fields_seen.contains(tag) && !optional {
            return Err(ParseError::InvalidFormat {
                message: format!("Duplicate field: {}", tag),
            });
        }

        #[cfg(debug_assertions)]
        {
            if tag.starts_with("21")
                || tag.starts_with("32")
                || tag.starts_with("57")
                || tag.starts_with("59")
            {
                eprintln!(
                    "DEBUG extract_field('{}') at position {} (allow_duplicates={})",
                    tag, self.position, self.allow_duplicates
                );
                eprintln!(
                    "  -> input slice (first 100 chars): {:?}",
                    &self.input[self.position..]
                        .chars()
                        .take(100)
                        .collect::<String>()
                );
            }
        }

        // Extract field content using the field_extractor module
        let extract_result = extract_field_content(&self.input[self.position..], tag);

        #[cfg(debug_assertions)]
        {
            if tag.starts_with("21")
                || tag.starts_with("32")
                || tag.starts_with("57")
                || tag.starts_with("59")
            {
                eprintln!(
                    "  -> extract_field_content returned: {:?}",
                    extract_result
                        .as_ref()
                        .map(|(c, consumed)| (c.len(), *consumed))
                );
            }
        }

        match extract_result {
            Some((content, consumed)) => {
                #[cfg(debug_assertions)]
                {
                    if tag.starts_with("21")
                        || tag.starts_with("32")
                        || tag.starts_with("57")
                        || tag.starts_with("59")
                    {
                        eprintln!(
                            "  -> extracted content length={}, consumed={}, new position={}",
                            content.len(),
                            consumed,
                            self.position + consumed
                        );
                        eprintln!(
                            "  -> content (first 40 chars): {:?}",
                            &content.chars().take(40).collect::<String>()
                        );
                    }
                }
                self.position += consumed;
                // Only track fields if duplicates are not allowed
                if !self.allow_duplicates {
                    self.fields_seen.insert(tag.to_string());
                }
                Ok(content)
            }
            None => {
                #[cfg(debug_assertions)]
                {
                    if tag.starts_with("21")
                        || tag.starts_with("32")
                        || tag.starts_with("57")
                        || tag.starts_with("59")
                    {
                        eprintln!("  -> NOT FOUND");
                    }
                }
                if optional {
                    // For optional fields, just return a format error that will be caught
                    Err(ParseError::InvalidFormat {
                        message: format!("Optional field {} not found", tag),
                    })
                } else {
                    Err(ParseError::MissingRequiredField {
                        field_tag: tag.to_string(),
                        field_name: tag.to_string(),
                        message_type: self.message_type.clone(),
                        position_in_block4: Some(self.position),
                    })
                }
            }
        }
    }

    /// Detect which variant is present for an enum field
    fn detect_variant(&self, base_tag: &str) -> Result<String, ParseError> {
        // Look for common variants in order of preference
        let common_variants = vec!["A", "B", "C", "D", "F", "K", "L"];

        // Get the remaining input
        let remaining = &self.input[self.position..];

        // For required fields, we should find it immediately (possibly after whitespace)
        let trimmed = remaining.trim_start_matches(|c: char| c.is_whitespace());

        #[cfg(debug_assertions)]
        {
            if base_tag == "59" {
                eprintln!(
                    "DEBUG detect_variant('{}') at position {}",
                    base_tag, self.position
                );
                eprintln!(
                    "  remaining (first 80 chars): {:?}",
                    &remaining.chars().take(80).collect::<String>()
                );
                eprintln!(
                    "  trimmed (first 80 chars): {:?}",
                    &trimmed.chars().take(80).collect::<String>()
                );
            }
        }

        for variant in common_variants {
            let full_tag = format!("{}{}", base_tag, variant);
            if trimmed.starts_with(&format!(":{}:", full_tag)) {
                #[cfg(debug_assertions)]
                {
                    if base_tag == "59" {
                        eprintln!("  -> Found variant '{}'", variant);
                    }
                }
                return Ok(variant.to_string());
            }
        }

        // Also check for no-variant version (just the base tag)
        if trimmed.starts_with(&format!(":{}:", base_tag)) {
            #[cfg(debug_assertions)]
            {
                if base_tag == "59" {
                    eprintln!("  -> Found no-variant (base tag only)");
                }
            }
            return Ok(String::new());
        }

        #[cfg(debug_assertions)]
        {
            if base_tag == "59" {
                eprintln!("  -> NOT FOUND - returning error");
            }
        }

        Err(ParseError::MissingRequiredField {
            field_tag: base_tag.to_string(),
            field_name: base_tag.to_string(),
            message_type: self.message_type.clone(),
            position_in_block4: Some(self.position),
        })
    }

    /// Detect variant for optional fields
    fn detect_variant_optional(&self, base_tag: &str) -> Option<String> {
        // Look for common variants
        let common_variants = vec!["A", "B", "C", "D", "F", "K", "L"];

        // Get the remaining input
        let remaining = &self.input[self.position..];

        // Skip any leading whitespace
        let trimmed = remaining.trim_start_matches(|c: char| c.is_whitespace());

        // Check if the immediate next field is one of our variants
        for variant in common_variants {
            let full_tag = format!("{}{}", base_tag, variant);
            if trimmed.starts_with(&format!(":{}:", full_tag)) {
                return Some(variant.to_string());
            }
        }

        // Check for no-variant version
        if trimmed.starts_with(&format!(":{}:", base_tag)) {
            return Some(String::new());
        }

        None
    }

    /// Get current position in input
    pub fn position(&self) -> usize {
        self.position
    }

    /// Get remaining unparsed content (useful for debugging)
    pub fn remaining(&self) -> &str {
        &self.input[self.position..]
    }

    /// Check if we've reached the end of input
    pub fn is_complete(&self) -> bool {
        self.position >= self.input.len()
            || self.remaining().trim().is_empty()
            || self.remaining().trim() == "-"
    }

    /// Check if a field exists in the remaining content
    pub fn detect_field(&self, tag: &str) -> bool {
        let remaining = self.remaining();
        let trimmed = remaining.trim_start_matches(|c: char| c.is_whitespace());
        trimmed.starts_with(&format!(":{}:", tag))
    }

    /// Peek at the variant of a field without consuming it
    /// Returns the variant letter (e.g., "A", "K", "C", "L") if the field exists
    pub fn peek_field_variant(&self, base_tag: &str) -> Option<String> {
        let remaining = self.remaining();
        let trimmed = remaining.trim_start_matches(|c: char| c.is_whitespace());

        // Try to find field with any variant (e.g., :50A:, :50K:, etc.)
        for variant in [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ] {
            let search_pattern = format!(":{}{}:", base_tag, variant);
            if trimmed.starts_with(&search_pattern) {
                return Some(variant.to_string());
            }
        }

        // Check for field without variant (e.g., :50:)
        let search_pattern = format!(":{}:", base_tag);
        if trimmed.starts_with(&search_pattern) {
            return Some("".to_string()); // No variant
        }

        None
    }
}
