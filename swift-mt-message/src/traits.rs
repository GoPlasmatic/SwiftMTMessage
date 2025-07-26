//! Core traits for SWIFT field and message types

use crate::{sample, Result, SwiftResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

/// Core trait for all Swift field types
pub trait SwiftField: Serialize + for<'de> Deserialize<'de> + Clone + std::fmt::Debug {
    /// Parse field value from string representation
    fn parse(value: &str) -> Result<Self>
    where
        Self: Sized;

    /// Parse field value with variant hint for enum fields
    /// Default implementation falls back to regular parse
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

    /// Convert field back to SWIFT string format
    fn to_swift_string(&self) -> String;

    /// Get field format specification
    fn format_spec() -> &'static str;

    /// Generate a random sample of this field
    fn sample() -> Self
    where
        Self: Sized;

    /// Generate a random sample with configuration
    fn sample_with_config(config: &sample::FieldConfig) -> Self
    where
        Self: Sized;

    /// Get valid variant letters for enum fields
    /// Returns None for non-enum fields, Some(vec) for enum fields
    fn valid_variants() -> Option<Vec<&'static str>> {
        None // Default implementation for non-enum fields
    }
}

/// Core trait for Swift message types
pub trait SwiftMessageBody: Debug + Clone + Send + Sync + Serialize + std::any::Any {
    /// Get the message type identifier (e.g., "103", "202")
    fn message_type() -> &'static str;

    /// Create from field map with sequential consumption tracking
    fn from_fields(fields: HashMap<String, Vec<(String, usize)>>) -> SwiftResult<Self>
    where
        Self: Sized;

    /// Convert to field map
    fn to_fields(&self) -> HashMap<String, Vec<String>>;

    /// Convert to ordered field list for MT serialization
    /// Returns fields in the correct sequence order for multi-sequence messages
    fn to_ordered_fields(&self) -> Vec<(String, String)> {
        // Default implementation: just flatten the HashMap in numeric order
        let field_map = self.to_fields();
        let mut ordered_fields = Vec::new();

        // Create ascending field order by sorting field tags numerically
        // Use stable sort and include the full tag as secondary sort key for deterministic ordering
        let mut field_tags: Vec<(&String, u32)> = field_map
            .keys()
            .map(|tag| {
                let num = tag
                    .chars()
                    .take_while(|c| c.is_ascii_digit())
                    .fold(0u32, |acc, c| acc * 10 + (c as u32 - '0' as u32));
                (tag, num)
            })
            .collect();
        // Sort by numeric value first, then by full tag string for stable ordering
        field_tags.sort_by(|(tag_a, num_a), (tag_b, num_b)| {
            num_a.cmp(num_b).then_with(|| tag_a.cmp(tag_b))
        });

        // Output fields in ascending numerical order
        for (field_tag, _) in field_tags {
            if let Some(field_values) = field_map.get(field_tag) {
                for field_value in field_values {
                    ordered_fields.push((field_tag.clone(), field_value.clone()));
                }
            }
        }

        ordered_fields
    }

    /// Get required field tags for this message type
    fn required_fields() -> Vec<&'static str>;

    /// Get optional field tags for this message type
    fn optional_fields() -> Vec<&'static str>;

    /// Generate a sample message with only mandatory fields
    fn sample() -> Self
    where
        Self: Sized;

    /// Generate a minimal sample (only mandatory fields)
    fn sample_minimal() -> Self
    where
        Self: Sized;

    /// Generate a full sample (all fields populated)
    fn sample_full() -> Self
    where
        Self: Sized;

    /// Generate a sample with configuration
    fn sample_with_config(config: &sample::MessageConfig) -> Self
    where
        Self: Sized;
}
