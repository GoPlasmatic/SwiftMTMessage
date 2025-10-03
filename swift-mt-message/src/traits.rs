//! Core traits for SWIFT field and message types
//!
//! This module defines the fundamental traits used throughout the SWIFT message parser:
//! - `SwiftField`: For individual field types (Field20, Field50, etc.)
//! - `SwiftMessageBody`: For complete message types (MT103, MT202, etc.)

use crate::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Core trait for all SWIFT field types
///
/// This trait is implemented by all field types (Field20, Field50, etc.) to provide
/// parsing, serialization, and variant handling capabilities.
///
/// ## Implementation Notes
///
/// - Simple fields (e.g., Field20) implement `parse()` and `to_swift_string()`
/// - Enum fields (e.g., Field50) also implement `parse_with_variant()` and `get_variant_tag()`
/// - All fields automatically get `to_swift_value()` via the default implementation
pub trait SwiftField: Serialize + for<'de> Deserialize<'de> + Clone + std::fmt::Debug {
    /// Parse field value from string representation
    ///
    /// This is the primary parsing method for simple (non-enum) fields.
    /// Input should be the field content without the `:TAG:` prefix.
    fn parse(value: &str) -> Result<Self>
    where
        Self: Sized;

    /// Parse field value with variant hint for enum fields
    ///
    /// Used by the MessageParser to parse enum fields like Field50A, Field50K, etc.
    /// Default implementation delegates to `parse()` for simple fields.
    ///
    /// ## Parameters
    /// - `value`: Field content without tag prefix
    /// - `variant`: Variant letter (e.g., "A", "K", "F") or empty string for no-option variant
    /// - `field_tag`: Base field tag (e.g., "50", "52") for error reporting
    fn parse_with_variant(
        value: &str,
        _variant: Option<&str>,
        _field_tag: Option<&str>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Self::parse(value)
    }

    /// Convert field to SWIFT format string (includes `:TAG:` prefix)
    ///
    /// Returns the complete SWIFT field representation, e.g., `:20:PAYMENT123` or `:50K:/ACC\nNAME`
    fn to_swift_string(&self) -> String;

    /// Get the field value without `:TAG:` prefix
    ///
    /// Returns only the field content, without the SWIFT tag prefix.
    /// Used by `to_ordered_fields()` implementations.
    ///
    /// ## Example
    /// - Input: `:20:PAYMENT123` → Output: `PAYMENT123`
    /// - Input: `:50K:/ACCOUNT\nNAME` → Output: `/ACCOUNT\nNAME`
    ///
    /// Default implementation automatically strips the prefix from `to_swift_string()`.
    fn to_swift_value(&self) -> String {
        let swift_str = self.to_swift_string();
        // Format is :TAG:VALUE or :TAGX:VALUE (where X is variant like A, K, etc.)
        // Find first : then find second : and return everything after it
        if let Some(first_colon) = swift_str.find(':')
            && let Some(second_colon) = swift_str[first_colon + 1..].find(':')
        {
            return swift_str[first_colon + second_colon + 2..].to_string();
        }
        swift_str
    }

    /// Get the variant tag for enum field values
    ///
    /// Returns the variant letter (e.g., "A", "K", "F") for enum fields like Field50OrderingCustomerAFK.
    /// Returns `None` for simple (non-enum) fields.
    ///
    /// This is used during serialization to determine which variant is active.
    fn get_variant_tag(&self) -> Option<&'static str> {
        None
    }
}

/// Core trait for Swift message types
///
/// This trait defines the interface for all SWIFT MT message types (MT103, MT202, etc.).
/// It provides methods for parsing, serialization, and metadata about message structure.
pub trait SwiftMessageBody: Debug + Clone + Send + Sync + Serialize + std::any::Any {
    /// Get the message type identifier (e.g., "103", "202", "940")
    fn message_type() -> &'static str;

    /// Parse message from Block 4 content
    ///
    /// Block 4 contains the actual message fields in SWIFT format.
    /// Each message type implements this to parse its specific field structure.
    fn parse_from_block4(_block4: &str) -> Result<Self>
    where
        Self: Sized,
    {
        panic!("parse_from_block4 not implemented for message type")
    }

    /// Convert message to SWIFT MT format string (Block 4 content only)
    ///
    /// Returns the message fields in SWIFT MT format, ready for serialization.
    /// The output does not include Block 4 wrapper braces `{4:...}`.
    ///
    /// Each message type must implement custom serialization logic that matches
    /// its custom parsing logic in `parse_from_block4()`.
    ///
    /// ## Format
    /// - Fields are formatted as `:TAG:VALUE\r\n`
    /// - Enum fields include variant letter: `:50K:value` or `:59A:value`
    /// - No trailing `\r\n` at the end
    ///
    /// ## Example Implementation
    /// ```ignore
    /// fn to_mt_string(&self) -> String {
    ///     use crate::traits::SwiftField;
    ///     let mut result = String::new();
    ///     result.push_str(&self.field_20.to_swift_string());
    ///     result.push_str("\r\n");
    ///     // ... add other fields in correct order
    ///     result.truncate(result.len() - 2); // Remove trailing \r\n
    ///     result
    /// }
    /// ```
    fn to_mt_string(&self) -> String;
}
