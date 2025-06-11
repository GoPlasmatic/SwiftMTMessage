//! Shared utilities for SWIFT MT field validation and parsing
//!
//! This module contains common validation logic, BIC validation, account parsing,
//! and other utilities shared across multiple field implementations.

use crate::config::{ConfigLoader, FieldValidationRule, ValidationPattern};
use crate::errors::{FieldParseError, Result};
use std::sync::OnceLock;

/// BIC (Bank Identifier Code) validation utility
pub mod bic {
    use super::*;

    /// Validate a BIC code according to SWIFT standards
    pub fn validate_bic(bic: &str) -> Result<()> {
        if bic.is_empty() {
            return Err(FieldParseError::invalid_format("BIC", "BIC cannot be empty").into());
        }

        // BIC must be 8 or 11 characters
        if bic.len() != 8 && bic.len() != 11 {
            return Err(FieldParseError::invalid_format(
                "BIC",
                "BIC must be 8 or 11 characters long",
            )
            .into());
        }

        // First 4 characters: Institution Code (letters only)
        let institution_code = &bic[0..4];
        if !institution_code
            .chars()
            .all(|c| c.is_alphabetic() && c.is_ascii())
        {
            return Err(FieldParseError::invalid_format(
                "BIC",
                "Institution code (first 4 characters) must be alphabetic",
            )
            .into());
        }

        // Next 2 characters: Country Code (letters only)
        let country_code = &bic[4..6];
        if !country_code
            .chars()
            .all(|c| c.is_alphabetic() && c.is_ascii())
        {
            return Err(FieldParseError::invalid_format(
                "BIC",
                "Country code (characters 5-6) must be alphabetic",
            )
            .into());
        }

        // Validate country code exists
        if !is_valid_country_code(country_code) {
            return Err(FieldParseError::invalid_format(
                "BIC",
                &format!("Invalid country code: {}", country_code),
            )
            .into());
        }

        // Next 2 characters: Location Code (alphanumeric)
        let location_code = &bic[6..8];
        if !location_code
            .chars()
            .all(|c| c.is_alphanumeric() && c.is_ascii())
        {
            return Err(FieldParseError::invalid_format(
                "BIC",
                "Location code (characters 7-8) must be alphanumeric",
            )
            .into());
        }

        // If 11 characters, last 3 are branch code (alphanumeric)
        if bic.len() == 11 {
            let branch_code = &bic[8..11];
            if !branch_code
                .chars()
                .all(|c| c.is_alphanumeric() && c.is_ascii())
            {
                return Err(FieldParseError::invalid_format(
                    "BIC",
                    "Branch code (characters 9-11) must be alphanumeric",
                )
                .into());
            }
        }

        Ok(())
    }

    /// Check if a country code is valid (simplified list for common codes)
    fn is_valid_country_code(code: &str) -> bool {
        // This is a simplified list. In production, you'd want a comprehensive list
        // of ISO 3166-1 alpha-2 country codes
        const VALID_CODES: &[&str] = &[
            "AD", "AE", "AF", "AG", "AI", "AL", "AM", "AO", "AQ", "AR", "AS", "AT", "AU", "AW",
            "AX", "AZ", "BA", "BB", "BD", "BE", "BF", "BG", "BH", "BI", "BJ", "BL", "BM", "BN",
            "BO", "BQ", "BR", "BS", "BT", "BV", "BW", "BY", "BZ", "CA", "CC", "CD", "CF", "CG",
            "CH", "CI", "CK", "CL", "CM", "CN", "CO", "CR", "CU", "CV", "CW", "CX", "CY", "CZ",
            "DE", "DJ", "DK", "DM", "DO", "DZ", "EC", "EE", "EG", "EH", "ER", "ES", "ET", "FI",
            "FJ", "FK", "FM", "FO", "FR", "GA", "GB", "GD", "GE", "GF", "GG", "GH", "GI", "GL",
            "GM", "GN", "GP", "GQ", "GR", "GS", "GT", "GU", "GW", "GY", "HK", "HM", "HN", "HR",
            "HT", "HU", "ID", "IE", "IL", "IM", "IN", "IO", "IQ", "IR", "IS", "IT", "JE", "JM",
            "JO", "JP", "KE", "KG", "KH", "KI", "KM", "KN", "KP", "KR", "KW", "KY", "KZ", "LA",
            "LB", "LC", "LI", "LK", "LR", "LS", "LT", "LU", "LV", "LY", "MA", "MC", "MD", "ME",
            "MF", "MG", "MH", "MK", "ML", "MM", "MN", "MO", "MP", "MQ", "MR", "MS", "MT", "MU",
            "MV", "MW", "MX", "MY", "MZ", "NA", "NC", "NE", "NF", "NG", "NI", "NL", "NO", "NP",
            "NR", "NU", "NZ", "OM", "PA", "PE", "PF", "PG", "PH", "PK", "PL", "PM", "PN", "PR",
            "PS", "PT", "PW", "PY", "QA", "RE", "RO", "RS", "RU", "RW", "SA", "SB", "SC", "SD",
            "SE", "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SR", "SS", "ST", "SV",
            "SX", "SY", "SZ", "TC", "TD", "TF", "TG", "TH", "TJ", "TK", "TL", "TM", "TN", "TO",
            "TR", "TT", "TV", "TW", "TZ", "UA", "UG", "UM", "US", "UY", "UZ", "VA", "VC", "VE",
            "VG", "VI", "VN", "VU", "WF", "WS", "YE", "YT", "ZA", "ZM", "ZW",
        ];
        VALID_CODES.contains(&code)
    }
}

/// Account validation utilities
pub mod account {
    use super::*;

    /// Validate account line indicator (single character)
    pub fn validate_account_line_indicator(indicator: &str) -> Result<()> {
        if indicator.len() != 1 {
            return Err(FieldParseError::invalid_format(
                "Account Line Indicator",
                "Must be exactly 1 character",
            )
            .into());
        }

        let ch = indicator.chars().next().unwrap();
        if !ch.is_alphanumeric() || !ch.is_ascii() {
            return Err(FieldParseError::invalid_format(
                "Account Line Indicator",
                "Must be alphanumeric ASCII character",
            )
            .into());
        }

        Ok(())
    }

    /// Validate account number (up to 34 characters, ASCII printable)
    pub fn validate_account_number(account: &str) -> Result<()> {
        if account.is_empty() {
            return Err(
                FieldParseError::missing_data("Account", "Account number cannot be empty").into(),
            );
        }

        if account.len() > 34 {
            return Err(FieldParseError::invalid_format(
                "Account",
                "Account number cannot exceed 34 characters",
            )
            .into());
        }

        if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(FieldParseError::invalid_format(
                "Account",
                "Account number contains invalid characters",
            )
            .into());
        }

        Ok(())
    }

    /// Parse account line format: [/indicator][/account]
    /// Returns (account_line_indicator, account_number, remaining_content)
    pub fn parse_account_line_and_content(
        content: &str,
    ) -> Result<(Option<String>, Option<String>, String)> {
        if content.is_empty() {
            return Ok((None, None, content.to_string()));
        }

        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Ok((None, None, content.to_string()));
        }

        let first_line = lines[0];
        let remaining_content = if lines.len() > 1 {
            lines[1..].join("\n")
        } else {
            String::new()
        };

        // Check if first line starts with account information
        if !first_line.starts_with('/') {
            return Ok((None, None, content.to_string()));
        }

        // Parse account information
        let account_part = &first_line[1..]; // Remove leading '/'
        let parts: Vec<&str> = account_part.split('/').collect();

        match parts.len() {
            1 => {
                // Just account indicator or account number
                if parts[0].len() == 1 {
                    // Single character = account line indicator
                    Ok((Some(parts[0].to_string()), None, remaining_content))
                } else {
                    // Multiple characters = account number
                    Ok((None, Some(parts[0].to_string()), remaining_content))
                }
            }
            2 => {
                // Both account line indicator and account number
                Ok((
                    Some(parts[0].to_string()),
                    Some(parts[1].to_string()),
                    remaining_content,
                ))
            }
            _ => Err(FieldParseError::invalid_format(
                "Account Line",
                "Invalid account line format",
            )
            .into()),
        }
    }
}

/// Multi-line field validation utilities
pub mod multiline {
    use super::*;

    /// Validate multi-line field content
    pub fn validate_multiline_field(
        field_tag: &str,
        lines: &[String],
        max_lines: usize,
        max_chars_per_line: usize,
    ) -> Result<()> {
        if lines.is_empty() {
            return Err(FieldParseError::missing_data(field_tag, "Content cannot be empty").into());
        }

        if lines.len() > max_lines {
            return Err(FieldParseError::invalid_format(
                field_tag,
                &format!("Too many lines (max {})", max_lines),
            )
            .into());
        }

        for (i, line) in lines.iter().enumerate() {
            if line.len() > max_chars_per_line {
                return Err(FieldParseError::invalid_format(
                    field_tag,
                    &format!("Line {} exceeds {} characters", i + 1, max_chars_per_line),
                )
                .into());
            }

            if line.trim().is_empty() {
                return Err(FieldParseError::invalid_format(
                    field_tag,
                    &format!("Line {} cannot be empty", i + 1),
                )
                .into());
            }

            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(FieldParseError::invalid_format(
                    field_tag,
                    &format!("Line {} contains invalid characters", i + 1),
                )
                .into());
            }
        }

        Ok(())
    }

    /// Parse content into lines, filtering empty lines
    pub fn parse_lines(content: &str) -> Vec<String> {
        content
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect()
    }
}

/// Character validation utilities
pub mod character {
    use super::*;

    /// Validate that content contains only ASCII printable characters
    pub fn validate_ascii_printable(
        field_tag: &str,
        content: &str,
        description: &str,
    ) -> Result<()> {
        if !content.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(FieldParseError::invalid_format(
                field_tag,
                &format!("{} contains invalid characters", description),
            )
            .into());
        }
        Ok(())
    }

    /// Validate that content contains only alphanumeric characters
    pub fn validate_alphanumeric(field_tag: &str, content: &str, description: &str) -> Result<()> {
        if !content.chars().all(|c| c.is_alphanumeric() && c.is_ascii()) {
            return Err(FieldParseError::invalid_format(
                field_tag,
                &format!("{} must contain only alphanumeric characters", description),
            )
            .into());
        }
        Ok(())
    }

    /// Validate that content contains only alphabetic characters
    pub fn validate_alphabetic(field_tag: &str, content: &str, description: &str) -> Result<()> {
        if !content.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(FieldParseError::invalid_format(
                field_tag,
                &format!("{} must contain only alphabetic characters", description),
            )
            .into());
        }
        Ok(())
    }

    /// Validate exact length
    pub fn validate_exact_length(
        field_tag: &str,
        content: &str,
        expected_length: usize,
        _description: &str,
    ) -> Result<()> {
        if content.len() != expected_length {
            return Err(FieldParseError::InvalidLength {
                field: field_tag.to_string(),
                max_length: expected_length,
                actual_length: content.len(),
            }
            .into());
        }
        Ok(())
    }

    /// Validate maximum length
    pub fn validate_max_length(
        field_tag: &str,
        content: &str,
        max_length: usize,
        description: &str,
    ) -> Result<()> {
        if content.len() > max_length {
            return Err(FieldParseError::invalid_format(
                field_tag,
                &format!("{} exceeds {} characters", description, max_length),
            )
            .into());
        }
        Ok(())
    }
}

/// Global configuration instance
static CONFIG_LOADER: OnceLock<ConfigLoader> = OnceLock::new();

/// Get or initialize the global configuration
pub fn get_config() -> &'static ConfigLoader {
    CONFIG_LOADER.get_or_init(|| {
        // Try to load from config directory first, fall back to defaults
        match ConfigLoader::load_from_directory("config") {
            Ok(loader) => loader,
            Err(_) => {
                // If config directory doesn't exist or fails to load, use defaults
                ConfigLoader::load_defaults()
            }
        }
    })
}

/// Configuration-based validation utilities
pub mod validation {
    use super::*;

    /// Validate field using configuration rules
    pub fn validate_field_with_config(field_tag: &str, content: &str) -> Result<()> {
        let config = get_config();

        // Try to get base field tag for field options (e.g., "50A" -> "50")
        let base_tag = if field_tag.len() > 2 {
            let chars: Vec<char> = field_tag.chars().collect();
            if chars.len() == 3
                && chars[0].is_ascii_digit()
                && chars[1].is_ascii_digit()
                && chars[2].is_alphabetic()
            {
                &field_tag[..2]
            } else {
                field_tag
            }
        } else {
            field_tag
        };

        // First try exact field tag, then base tag
        let rule = config.get_field_validation(field_tag).or_else(|| {
            if base_tag != field_tag {
                config.get_field_validation(base_tag)
            } else {
                None
            }
        });

        if let Some(rule) = rule {
            validate_with_rule(field_tag, content, rule, config)?;
        }

        Ok(())
    }

    /// Validate content against a specific validation rule
    pub fn validate_with_rule(
        field_tag: &str,
        content: &str,
        rule: &FieldValidationRule,
        config: &ConfigLoader,
    ) -> Result<()> {
        // Check empty content
        if content.is_empty() && rule.allow_empty == Some(false) {
            return Err(FieldParseError::missing_data(field_tag, "Field cannot be empty").into());
        }

        // Length validations
        if let Some(max_length) = rule.max_length {
            if content.len() > max_length {
                return Err(
                    FieldParseError::invalid_length(field_tag, max_length, content.len()).into(),
                );
            }
        }

        if let Some(exact_length) = rule.exact_length {
            if content.len() != exact_length {
                return Err(FieldParseError::invalid_format(
                    field_tag,
                    &format!("Must be exactly {} characters", exact_length),
                )
                .into());
            }
        }

        if let Some(min_length) = rule.min_length {
            if content.len() < min_length {
                return Err(FieldParseError::invalid_format(
                    field_tag,
                    &format!("Must be at least {} characters", min_length),
                )
                .into());
            }
        }

        // Pattern-based validation
        if let Some(pattern_ref) = &rule.pattern_ref {
            if let Some(pattern) = config.get_validation_pattern(pattern_ref) {
                validate_with_pattern(field_tag, content, pattern)?;
            }
        }

        // Valid values check
        if let Some(valid_values) = &rule.valid_values {
            let normalized_content = match rule.case_normalization.as_deref() {
                Some("upper") => content.to_uppercase(),
                Some("lower") => content.to_lowercase(),
                _ => content.to_string(),
            };

            if !valid_values.contains(&normalized_content) {
                return Err(FieldParseError::invalid_format(
                    field_tag,
                    &format!("Must be one of: {:?}", valid_values),
                )
                .into());
            }
        }

        // Multi-line validation
        if rule.max_lines.is_some() || rule.max_chars_per_line.is_some() {
            let lines: Vec<&str> = content.lines().collect();

            if let Some(max_lines) = rule.max_lines {
                if lines.len() > max_lines {
                    return Err(FieldParseError::invalid_format(
                        field_tag,
                        &format!("Too many lines: {} (max {})", lines.len(), max_lines),
                    )
                    .into());
                }
            }

            if let Some(max_chars) = rule.max_chars_per_line {
                for (i, line) in lines.iter().enumerate() {
                    if line.len() > max_chars {
                        return Err(FieldParseError::invalid_format(
                            field_tag,
                            &format!(
                                "Line {} too long: {} chars (max {})",
                                i + 1,
                                line.len(),
                                max_chars
                            ),
                        )
                        .into());
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate content against a validation pattern
    pub fn validate_with_pattern(
        field_tag: &str,
        content: &str,
        pattern: &ValidationPattern,
    ) -> Result<()> {
        // Regex validation
        if let Some(regex_str) = &pattern.regex {
            match regex::Regex::new(regex_str) {
                Ok(regex) => {
                    if !regex.is_match(content) {
                        return Err(FieldParseError::invalid_format(
                            field_tag,
                            &format!("Does not match pattern: {}", pattern.description),
                        )
                        .into());
                    }
                }
                Err(e) => {
                    return Err(FieldParseError::invalid_format(
                        field_tag,
                        &format!("Invalid regex pattern: {}", e),
                    )
                    .into());
                }
            }
        }

        // Character set validation
        if let Some(charset) = &pattern.charset {
            if charset.ascii_printable == Some(true) {
                super::character::validate_ascii_printable(
                    field_tag,
                    content,
                    &pattern.description,
                )?;
            }
            if charset.alphanumeric == Some(true) {
                super::character::validate_alphanumeric(field_tag, content, &pattern.description)?;
            }
            if charset.alphabetic == Some(true) {
                super::character::validate_alphabetic(field_tag, content, &pattern.description)?;
            }
            if charset.numeric == Some(true) && !content.chars().all(|c| c.is_ascii_digit()) {
                return Err(FieldParseError::invalid_format(
                    field_tag,
                    "Must contain only numeric characters",
                )
                .into());
            }
        }

        Ok(())
    }
}

/// Check if a field is mandatory for a specific message type using configuration
pub fn is_field_mandatory(field_tag: &str, message_type: &str) -> bool {
    get_config().is_field_mandatory(field_tag, message_type)
}

/// Get all mandatory fields for a message type using configuration
pub fn get_mandatory_fields(message_type: &str) -> Vec<String> {
    get_config().get_mandatory_fields(message_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bic_tests {
        use super::*;

        #[test]
        fn test_valid_bic_8_chars() {
            assert!(bic::validate_bic("BANKDEFF").is_ok());
        }

        #[test]
        fn test_valid_bic_11_chars() {
            assert!(bic::validate_bic("BANKDEFFXXX").is_ok());
        }

        #[test]
        fn test_invalid_bic_length() {
            assert!(bic::validate_bic("BANK").is_err());
            assert!(bic::validate_bic("BANKDEFFTOOLONG").is_err());
        }

        #[test]
        fn test_invalid_bic_characters() {
            assert!(bic::validate_bic("BAN1DEFF").is_err()); // Number in institution code
            assert!(bic::validate_bic("BANKD3FF").is_err()); // Number in country code
        }
    }

    mod account_tests {
        use super::*;

        #[test]
        fn test_valid_account_line_indicator() {
            assert!(account::validate_account_line_indicator("A").is_ok());
            assert!(account::validate_account_line_indicator("1").is_ok());
        }

        #[test]
        fn test_invalid_account_line_indicator() {
            assert!(account::validate_account_line_indicator("").is_err());
            assert!(account::validate_account_line_indicator("AB").is_err());
            assert!(account::validate_account_line_indicator("@").is_err());
        }

        #[test]
        fn test_parse_account_line() {
            let (indicator, account, content) =
                account::parse_account_line_and_content("/A/12345\nBANKDEFF").unwrap();
            assert_eq!(indicator, Some("A".to_string()));
            assert_eq!(account, Some("12345".to_string()));
            assert_eq!(content, "BANKDEFF");
        }
    }

    mod config_tests {
        use super::*;

        #[test]
        fn test_config_based_mandatory_fields() {
            assert!(is_field_mandatory("20", "103"));
            assert!(is_field_mandatory("50A", "103")); // Option handling
            assert!(is_field_mandatory("50K", "103")); // Option handling
            assert!(!is_field_mandatory("72", "103")); // Optional field
            assert!(!is_field_mandatory("20", "999")); // Unknown message type
        }

        #[test]
        fn test_config_based_validation() {
            // Test valid field validation
            assert!(validation::validate_field_with_config("20", "TESTREF123").is_ok());

            // Test field too long
            assert!(
                validation::validate_field_with_config("20", "TESTREF123456789012345").is_err()
            );

            // Test field with exact length requirement
            assert!(validation::validate_field_with_config("23B", "CRED").is_ok());
            assert!(validation::validate_field_with_config("23B", "CRE").is_err()); // Too short
        }
    }
}
