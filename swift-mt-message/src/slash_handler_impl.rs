// Generic slash prefix handling for SWIFT components
//
// This module provides a unified approach to handling slash prefixes in SWIFT field patterns,
// ensuring consistent parsing and serialization across all field types.


/// Types of slash prefixes in SWIFT patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlashPrefixType {
    /// No slash prefix
    None,
    /// Optional slash prefix: [/34x] - slash required when value present
    Optional,
    /// Required slash prefix: /34x - always has slash
    Required,
    /// Double slash prefix: //16x - double slash prefix
    Double,
    /// Wrapped in slashes: /8c/ - both leading and trailing
    Wrapper,
    /// Optional with numeric constraint: [/5n]
    OptionalNumeric(usize),
}

/// Trait for handling slash prefixes in SWIFT components
pub trait SlashPrefixHandler {
    /// Extract the slash prefix type from a pattern
    fn get_slash_type(pattern: &str) -> SlashPrefixType;

    /// Parse value removing slash prefix
    fn parse_with_slash(value: &str, slash_type: SlashPrefixType) -> Result<String, String>;

    /// Serialize value adding slash prefix
    fn serialize_with_slash(value: &str, slash_type: SlashPrefixType) -> String;

    /// Check if a pattern contains any slash prefix
    fn has_slash_prefix(pattern: &str) -> bool;

    /// Get regex pattern that handles the slash prefix correctly
    fn get_slash_aware_regex(pattern: &str, base_regex: &str) -> String;
}

/// Default implementation of slash prefix handling
pub struct SwiftSlashHandler;

impl SlashPrefixHandler for SwiftSlashHandler {
    fn get_slash_type(pattern: &str) -> SlashPrefixType {
        // Handle wrapped pattern (e.g., /8c/)
        if pattern.starts_with('/') && pattern.ends_with('/') && pattern.len() > 2 {
            return SlashPrefixType::Wrapper;
        }

        // Handle optional patterns [...]
        if pattern.starts_with('[') && pattern.ends_with(']') {
            let inner = &pattern[1..pattern.len()-1];

            // Double slash optional [//16x]
            if inner.starts_with("//") {
                return SlashPrefixType::Double;
            }

            // Single slash optional [/34x]
            if let Some(stripped) = inner.strip_prefix('/') {
                // Check if numeric pattern like [/5n]
                if let Some(n) = extract_numeric_length(stripped) {
                    return SlashPrefixType::OptionalNumeric(n);
                }
                return SlashPrefixType::Optional;
            }
        }

        // Handle required slash patterns
        if pattern.starts_with('/') {
            if pattern.starts_with("//") {
                return SlashPrefixType::Double;
            }
            return SlashPrefixType::Required;
        }

        SlashPrefixType::None
    }

    fn parse_with_slash(value: &str, slash_type: SlashPrefixType) -> Result<String, String> {
        match slash_type {
            SlashPrefixType::None => Ok(value.to_string()),

            SlashPrefixType::Optional | SlashPrefixType::Required => {
                // Remove single leading slash if present
                Ok(value.strip_prefix('/').unwrap_or(value).to_string())
            }

            SlashPrefixType::Double => {
                // Remove double slash prefix
                Ok(value.strip_prefix("//").unwrap_or(value).to_string())
            }

            SlashPrefixType::Wrapper => {
                // Remove both leading and trailing slash
                let trimmed = value.strip_prefix('/').unwrap_or(value);
                Ok(trimmed.strip_suffix('/').unwrap_or(trimmed).to_string())
            }

            SlashPrefixType::OptionalNumeric(n) => {
                // Remove slash and validate numeric
                let without_slash = value.strip_prefix('/').unwrap_or(value);
                // Validate that it's numeric and within length
                if without_slash.chars().all(|c| c.is_ascii_digit()) && without_slash.len() <= n {
                    Ok(without_slash.to_string())
                } else {
                    Err(format!("Invalid numeric value: expected up to {} digits", n))
                }
            }
        }
    }

    fn serialize_with_slash(value: &str, slash_type: SlashPrefixType) -> String {
        // Empty values for optional fields should remain empty
        if value.is_empty() && matches!(slash_type, SlashPrefixType::Optional | SlashPrefixType::OptionalNumeric(_)) {
            return String::new();
        }

        match slash_type {
            SlashPrefixType::None => value.to_string(),

            SlashPrefixType::Optional | SlashPrefixType::Required => {
                // Add slash if not present
                if !value.starts_with('/') {
                    format!("/{}", value)
                } else {
                    value.to_string()
                }
            }

            SlashPrefixType::Double => {
                // Ensure double slash prefix
                if value.starts_with("//") {
                    value.to_string()
                } else if value.starts_with('/') {
                    format!("/{}", value)
                } else {
                    format!("//{}", value)
                }
            }

            SlashPrefixType::Wrapper => {
                // Wrap in slashes, removing any existing ones first
                let trimmed = value.strip_prefix('/').unwrap_or(value);
                let trimmed = trimmed.strip_suffix('/').unwrap_or(trimmed);
                format!("/{}/", trimmed)
            }

            SlashPrefixType::OptionalNumeric(_) => {
                // Add slash for numeric values
                if !value.starts_with('/') {
                    format!("/{}", value)
                } else {
                    value.to_string()
                }
            }
        }
    }

    fn has_slash_prefix(pattern: &str) -> bool {
        !matches!(Self::get_slash_type(pattern), SlashPrefixType::None)
    }

    fn get_slash_aware_regex(pattern: &str, base_regex: &str) -> String {
        let slash_type = Self::get_slash_type(pattern);

        match slash_type {
            SlashPrefixType::None => base_regex.to_string(),

            SlashPrefixType::Optional => {
                // For optional slash, the whole thing is optional including the slash
                format!("(?:/{})?", base_regex)
            }

            SlashPrefixType::Required => {
                // Required slash followed by the content
                format!("/({})", base_regex)
            }

            SlashPrefixType::Double => {
                // Double slash followed by content
                format!("//({})", base_regex)
            }

            SlashPrefixType::Wrapper => {
                // Wrapped in slashes
                format!("/({})/", base_regex)
            }

            SlashPrefixType::OptionalNumeric(_) => {
                // Optional slash with numeric content
                format!("(?:/({})?)", base_regex)
            }
        }
    }
}

/// Extract numeric length from a pattern like "5n" or "34x"
fn extract_numeric_length(pattern: &str) -> Option<usize> {
    // Look for patterns like "5n", "2n", etc.
    if pattern.len() >= 2 && pattern.ends_with('n')
        && let Ok(n) = pattern[..pattern.len()-1].parse::<usize>() {
            return Some(n);
        }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slash_type_detection() {
        // Test various patterns
        assert_eq!(SwiftSlashHandler::get_slash_type("[/34x]"), SlashPrefixType::Optional);
        assert_eq!(SwiftSlashHandler::get_slash_type("/34x"), SlashPrefixType::Required);
        assert_eq!(SwiftSlashHandler::get_slash_type("[//16x]"), SlashPrefixType::Double);
        assert_eq!(SwiftSlashHandler::get_slash_type("/8c/"), SlashPrefixType::Wrapper);
        assert_eq!(SwiftSlashHandler::get_slash_type("[/5n]"), SlashPrefixType::OptionalNumeric(5));
        assert_eq!(SwiftSlashHandler::get_slash_type("34x"), SlashPrefixType::None);
    }

    #[test]
    fn test_parse_with_slash() {
        // Test optional slash
        assert_eq!(
            SwiftSlashHandler::parse_with_slash("/ACC123", SlashPrefixType::Optional),
            Ok("ACC123".to_string())
        );
        assert_eq!(
            SwiftSlashHandler::parse_with_slash("ACC123", SlashPrefixType::Optional),
            Ok("ACC123".to_string())
        );

        // Test required slash
        assert_eq!(
            SwiftSlashHandler::parse_with_slash("/ACC456", SlashPrefixType::Required),
            Ok("ACC456".to_string())
        );

        // Test double slash
        assert_eq!(
            SwiftSlashHandler::parse_with_slash("//TRN789", SlashPrefixType::Double),
            Ok("TRN789".to_string())
        );

        // Test wrapper
        assert_eq!(
            SwiftSlashHandler::parse_with_slash("/30E/", SlashPrefixType::Wrapper),
            Ok("30E".to_string())
        );

        // Test optional numeric
        assert_eq!(
            SwiftSlashHandler::parse_with_slash("/12345", SlashPrefixType::OptionalNumeric(5)),
            Ok("12345".to_string())
        );
    }

    #[test]
    fn test_serialize_with_slash() {
        // Test optional slash
        assert_eq!(
            SwiftSlashHandler::serialize_with_slash("ACC123", SlashPrefixType::Optional),
            "/ACC123"
        );
        assert_eq!(
            SwiftSlashHandler::serialize_with_slash("", SlashPrefixType::Optional),
            ""
        );

        // Test required slash
        assert_eq!(
            SwiftSlashHandler::serialize_with_slash("ACC456", SlashPrefixType::Required),
            "/ACC456"
        );

        // Test double slash
        assert_eq!(
            SwiftSlashHandler::serialize_with_slash("TRN789", SlashPrefixType::Double),
            "//TRN789"
        );

        // Test wrapper
        assert_eq!(
            SwiftSlashHandler::serialize_with_slash("30E", SlashPrefixType::Wrapper),
            "/30E/"
        );

        // Test optional numeric
        assert_eq!(
            SwiftSlashHandler::serialize_with_slash("12345", SlashPrefixType::OptionalNumeric(5)),
            "/12345"
        );
    }

    #[test]
    fn test_idempotent_serialization() {
        // Test that serializing already-formatted values doesn't duplicate slashes
        assert_eq!(
            SwiftSlashHandler::serialize_with_slash("/ACC123", SlashPrefixType::Optional),
            "/ACC123"
        );
        assert_eq!(
            SwiftSlashHandler::serialize_with_slash("//TRN789", SlashPrefixType::Double),
            "//TRN789"
        );
        assert_eq!(
            SwiftSlashHandler::serialize_with_slash("/30E/", SlashPrefixType::Wrapper),
            "/30E/"
        );
    }
}