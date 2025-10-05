//! # Core Traits
//!
//! Fundamental traits for SWIFT message parsing and serialization.
//!
//! - **SwiftField**: Field-level parsing and serialization
//! - **SwiftMessageBody**: Message-level operations and validation

use crate::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Trait for SWIFT field types
///
/// Implemented by all field types for parsing and serialization.
/// Enum fields (Field50, Field59) support variant-based parsing.
pub trait SwiftField: Serialize + for<'de> Deserialize<'de> + Clone + std::fmt::Debug {
    /// Parse field from SWIFT format (without `:TAG:` prefix)
    fn parse(value: &str) -> Result<Self>
    where
        Self: Sized;

    /// Parse field with variant (e.g., 50A, 50K) for enum fields
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

    /// Convert to SWIFT format (includes `:TAG:` prefix)
    fn to_swift_string(&self) -> String;

    /// Get variant tag (e.g., "A", "K") for enum fields, None for simple fields
    fn get_variant_tag(&self) -> Option<&'static str> {
        None
    }
}

/// Trait for SWIFT message types (MT103, MT202, etc.)
///
/// Provides parsing, serialization, and validation for complete messages.
pub trait SwiftMessageBody: Debug + Clone + Send + Sync + Serialize + std::any::Any {
    /// Get the message type identifier (e.g., "103", "202", "940")
    fn message_type() -> &'static str;

    /// Parse message from Block 4 content (fields only)
    fn parse_from_block4(_block4: &str) -> Result<Self>
    where
        Self: Sized,
    {
        panic!("parse_from_block4 not implemented for message type")
    }

    /// Convert to SWIFT MT format (Block 4 content, no wrapper braces)
    fn to_mt_string(&self) -> String;

    /// Validate SWIFT network rules (C/D/E series) for this message
    fn validate_network_rules(
        &self,
        _stop_on_first_error: bool,
    ) -> Vec<crate::errors::SwiftValidationError> {
        Vec::new()
    }
}
