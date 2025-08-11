//! Utility functions for SWIFT message processing

use crate::parser::extract_base_tag;

/// Helper function to get field tag with variant for enum fields
pub fn get_field_tag_with_variant<T>(base_tag: &str, field_value: &T) -> String
where
    T: std::fmt::Debug,
{
    let debug_string = format!("{field_value:?}");

    // Extract variant from debug string (e.g., "K(...)" -> "K")
    if let Some(variant_end) = debug_string.find('(') {
        let variant = &debug_string[..variant_end];

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
