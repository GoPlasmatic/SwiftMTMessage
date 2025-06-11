//! Field 36: Exchange Rate
//!
//! Rate of exchange between the currency codes in Field 33B and Field 32A.
//! Format: 12d (up to 12 digits including decimal)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 36: Exchange Rate
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field36 {
    /// Exchange rate value
    pub rate: f64,
    /// Raw rate string as received (preserves original formatting)
    pub raw_rate: String,
}

impl Field36 {
    /// Create a new Field36 with validation
    pub fn new(rate: f64, raw_rate: Option<String>) -> Result<Self> {
        let raw_rate = raw_rate.unwrap_or_else(|| Self::format_rate(rate));

        // Validate rate
        if rate <= 0.0 {
            return Err(
                FieldParseError::invalid_format("36", "Exchange rate must be positive").into(),
            );
        }

        // Check if rate string is reasonable length (up to 12 digits)
        let normalized_rate = raw_rate.replace(',', ".");
        if normalized_rate.len() > 12 {
            return Err(FieldParseError::invalid_format(
                "36",
                "Exchange rate too long (max 12 digits)",
            )
            .into());
        }

        Ok(Field36 { rate, raw_rate })
    }

    /// Format rate for SWIFT output (preserving decimal places)
    pub fn format_rate(rate: f64) -> String {
        // Format with up to 6 decimal places, remove trailing zeros
        let formatted = format!("{:.6}", rate);
        let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
        trimmed.replace('.', ",")
    }

    /// Parse rate from string (handles both comma and dot as decimal separator)
    fn parse_rate(rate_str: &str) -> Result<(f64, String)> {
        let raw_rate = rate_str.to_string();

        // Handle both comma and dot as decimal separators
        let normalized_rate = rate_str.replace(',', ".");

        let rate = normalized_rate
            .parse::<f64>()
            .map_err(|_| FieldParseError::invalid_format("36", "Invalid exchange rate format"))?;

        if rate <= 0.0 {
            return Err(
                FieldParseError::invalid_format("36", "Exchange rate must be positive").into(),
            );
        }

        Ok((rate, raw_rate))
    }

    /// Get the exchange rate value
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Get the raw rate string as received
    pub fn raw_rate(&self) -> &str {
        &self.raw_rate
    }
}

impl SwiftField for Field36 {
    const TAG: &'static str = "36";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();

        if content.is_empty() {
            return Err(
                FieldParseError::missing_data("36", "Exchange rate cannot be empty").into(),
            );
        }

        let (rate, raw_rate) = Self::parse_rate(content)?;
        Self::new(rate, Some(raw_rate))
    }

    fn to_swift_string(&self) -> String {
        format!(":36:{}", self.raw_rate)
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        rules.validate_field("36", &self.raw_rate)
    }

    fn description() -> &'static str {
        "Exchange Rate - Rate between currencies in Field 33B and Field 32A"
    }
}

impl std::fmt::Display for Field36 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.6}", self.rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_parser::FormatRules;
    use std::collections::HashMap;

    #[test]
    fn test_field36_creation() {
        let field = Field36::new(1.2345, None).unwrap();
        assert_eq!(field.rate, 1.2345);
        assert_eq!(field.raw_rate, "1,2345");
    }

    #[test]
    fn test_field36_parse() {
        let field = Field36::parse("1,234567").unwrap();
        assert_eq!(field.rate, 1.234567);
        assert_eq!(field.raw_rate, "1,234567");
    }

    #[test]
    fn test_field36_parse_with_dot() {
        let field = Field36::parse("0.98765").unwrap();
        assert_eq!(field.rate, 0.98765);
        assert_eq!(field.raw_rate, "0.98765");
    }

    #[test]
    fn test_field36_format_rate() {
        assert_eq!(Field36::format_rate(1.23), "1,23");
        assert_eq!(Field36::format_rate(1.234560), "1,23456");
        assert_eq!(Field36::format_rate(2.0), "2");
    }

    #[test]
    fn test_field36_invalid_rate() {
        let result = Field36::new(0.0, None); // Zero rate
        assert!(result.is_err());

        let result = Field36::new(-1.5, None); // Negative rate
        assert!(result.is_err());

        let result = Field36::parse("0"); // Zero
        assert!(result.is_err());

        let result = Field36::parse("-1.5"); // Negative
        assert!(result.is_err());
    }

    #[test]
    fn test_field36_invalid_format() {
        let result = Field36::parse("ABC"); // Non-numeric
        assert!(result.is_err());

        let result = Field36::parse(""); // Empty
        assert!(result.is_err());
    }

    #[test]
    fn test_field36_too_long() {
        let result = Field36::parse("1234567890123"); // Too many digits
        assert!(result.is_err());
    }

    #[test]
    fn test_field36_to_swift_string() {
        let field = Field36::new(1.5678, None).unwrap();
        assert_eq!(field.to_swift_string(), ":36:1,5678");
    }

    #[test]
    fn test_field36_validation() {
        let field = Field36::new(1.25, None).unwrap();
        let rules = FormatRules {
            fields: HashMap::new(),
        };
        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field36_display() {
        let field = Field36::new(1.23456, None).unwrap();
        assert_eq!(format!("{}", field), "1.234560");
    }

    #[test]
    fn test_field36_accessors() {
        let field = Field36::new(2.5, Some("2,5".to_string())).unwrap();
        assert_eq!(field.rate(), 2.5);
        assert_eq!(field.raw_rate(), "2,5");
    }
}
