/// Utility functions for SWIFT message formatting
// Legacy functions for backward compatibility
pub fn get_field_tag_for_mt(_message_type: &str, field_name: &str) -> String {
    // Extract the numeric part of the field name
    field_name.trim_start_matches("field_").to_string()
}

pub fn get_field_tag_with_variant(base_tag: &str, variant: Option<&str>) -> String {
    match variant {
        Some(v) => format!("{}{}", base_tag, v),
        None => base_tag.to_string(),
    }
}

pub fn is_numbered_field(field_name: &str) -> bool {
    field_name.starts_with("field_") && field_name[6..].chars().all(|c| c.is_ascii_digit())
}

pub fn map_variant_to_numbered(_variant: &str) -> Option<String> {
    None
}
