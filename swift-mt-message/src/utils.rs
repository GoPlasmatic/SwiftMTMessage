//! Utility functions for SWIFT message processing

use crate::parser::extract_base_tag;

/// Helper function to get field tag with variant for enum fields
pub fn get_field_tag_with_variant<T>(base_tag: &str, field_value: &T) -> String
where
    T: crate::SwiftField,
{
    // Use the get_variant_tag method if available
    if let Some(variant) = field_value.get_variant_tag() {
        // Special handling for "NoOption" variant - use base tag without suffix
        if variant == "NoOption" {
            base_tag.to_string()
        } else {
            format!("{base_tag}{variant}")
        }
    } else {
        base_tag.to_string()
    }
}

/// Get field tag for MT serialization by stripping index suffix
pub fn get_field_tag_for_mt(tag: &str) -> String {
    extract_base_tag(tag).to_string()
}

/// Check if a field tag is a numbered field (contains #)
pub fn is_numbered_field(tag: &str) -> bool {
    tag.contains('#')
}

/// Map a variant-based field tag back to a numbered field tag based on context
/// This is used during MT parsing to restore numbered field distinction
pub fn map_variant_to_numbered(
    base_tag: &str,
    variant: Option<&str>,
    field_index: usize,
) -> String {
    // For now, use a simple mapping based on field order
    // In a more complete implementation, this would use message-specific rules
    match (base_tag, variant, field_index) {
        ("50", _, 0) => "50#1".to_string(),
        ("50", _, 1) => "50#2".to_string(),
        _ => {
            if let Some(v) = variant {
                format!("{}{}", base_tag, v)
            } else {
                base_tag.to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_field_tag_for_mt() {
        // Test conversion for MT serialization
        assert_eq!(get_field_tag_for_mt("50"), "50");
        assert_eq!(get_field_tag_for_mt("50#1"), "50");
        assert_eq!(get_field_tag_for_mt("50#2"), "50");
        assert_eq!(get_field_tag_for_mt("32A#1"), "32A");
        assert_eq!(get_field_tag_for_mt("34F#10"), "34F");

        // Test tags without index
        assert_eq!(get_field_tag_for_mt("20"), "20");
        assert_eq!(get_field_tag_for_mt("32A"), "32A");
        assert_eq!(get_field_tag_for_mt("59F"), "59F");
    }
}
