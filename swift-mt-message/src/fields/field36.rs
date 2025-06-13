use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Field 36: Exchange Rate
///
/// Format: 12d (up to 12 digits including decimal)
///
/// Rate of exchange between the currency codes in Field 33B and Field 32A.
/// Only present when Field 33B currency differs from Field 32A currency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field36 {
    /// Exchange rate value
    pub rate: f64,
    /// Raw rate string as received (preserves original formatting)
    pub raw_rate: String,
}

impl Field36 {
    /// Create a new Field36 with validation
    pub fn new(rate: f64) -> Result<Self, crate::ParseError> {
        // Validate rate
        if rate <= 0.0 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "36".to_string(),
                message: "Exchange rate must be positive".to_string(),
            });
        }

        let raw_rate = Self::format_rate(rate);

        // Check if rate string is reasonable length (up to 12 digits)
        let normalized_rate = raw_rate.replace(',', ".");
        if normalized_rate.len() > 12 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "36".to_string(),
                message: "Exchange rate too long (max 12 digits)".to_string(),
            });
        }

        Ok(Field36 { rate, raw_rate })
    }

    /// Create from raw rate string
    pub fn from_raw(raw_rate: impl Into<String>) -> Result<Self, crate::ParseError> {
        let raw_rate = raw_rate.into();
        let rate = Self::parse_rate(&raw_rate)?;

        Ok(Field36 {
            rate,
            raw_rate: raw_rate.to_string(),
        })
    }

    /// Get the exchange rate value
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Get the raw rate string
    pub fn raw_rate(&self) -> &str {
        &self.raw_rate
    }

    /// Format rate for SWIFT output (preserving decimal places, using comma)
    pub fn format_rate(rate: f64) -> String {
        // Format with up to 6 decimal places, remove trailing zeros
        let formatted = format!("{:.6}", rate);
        let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
        trimmed.replace('.', ",")
    }

    /// Parse rate from string (handles both comma and dot as decimal separator)
    fn parse_rate(rate_str: &str) -> Result<f64, crate::ParseError> {
        let normalized_rate = rate_str.replace(',', ".");

        let rate =
            normalized_rate
                .parse::<f64>()
                .map_err(|_| crate::ParseError::InvalidFieldFormat {
                    field_tag: "36".to_string(),
                    message: "Invalid exchange rate format".to_string(),
                })?;

        if rate <= 0.0 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "36".to_string(),
                message: "Exchange rate must be positive".to_string(),
            });
        }

        Ok(rate)
    }

    /// Check if this is a reasonable exchange rate (between 0.0001 and 10000)
    pub fn is_reasonable_rate(&self) -> bool {
        self.rate >= 0.0001 && self.rate <= 10000.0
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!("Exchange Rate: {}", self.raw_rate)
    }
}

impl SwiftField for Field36 {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":36:") {
            stripped // Remove ":36:" prefix
        } else if let Some(stripped) = value.strip_prefix("36:") {
            stripped // Remove "36:" prefix
        } else {
            value
        };

        let content = content.trim();

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "36".to_string(),
                message: "Exchange rate cannot be empty".to_string(),
            });
        }

        // Check length constraint
        if content.len() > 12 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "36".to_string(),
                message: "Exchange rate too long (max 12 characters)".to_string(),
            });
        }

        // Validate characters (digits, comma, dot only)
        if !content
            .chars()
            .all(|c| c.is_ascii_digit() || c == ',' || c == '.')
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "36".to_string(),
                message: "Exchange rate must contain only digits and decimal separator".to_string(),
            });
        }

        let rate = Self::parse_rate(content)?;

        Ok(Field36 {
            rate,
            raw_rate: content.to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":36:{}", self.raw_rate)
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Validate rate value
        if self.rate <= 0.0 {
            errors.push(ValidationError::ValueValidation {
                field_tag: "36".to_string(),
                message: "Exchange rate must be positive".to_string(),
            });
        }

        // Validate raw rate format
        if self.raw_rate.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "36".to_string(),
                message: "Exchange rate cannot be empty".to_string(),
            });
        }

        // Check length constraint
        if self.raw_rate.len() > 12 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "36".to_string(),
                expected: "max 12 characters".to_string(),
                actual: self.raw_rate.len(),
            });
        }

        // Warning for unreasonable rates
        if !self.is_reasonable_rate() {
            warnings.push(format!(
                "Exchange rate {} may be unreasonable (typical range: 0.0001 to 10000)",
                self.rate
            ));
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    fn format_spec() -> &'static str {
        "12d"
    }
}

impl std::fmt::Display for Field36 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw_rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field36_creation() {
        let field = Field36::new(1.2345).unwrap();
        assert_eq!(field.rate(), 1.2345);
        assert_eq!(field.raw_rate(), "1,2345");
    }

    #[test]
    fn test_field36_from_raw() {
        let field = Field36::from_raw("0,8567").unwrap();
        assert_eq!(field.rate(), 0.8567);
        assert_eq!(field.raw_rate(), "0,8567");
    }

    #[test]
    fn test_field36_parse() {
        let field = Field36::parse("1,5678").unwrap();
        assert_eq!(field.rate(), 1.5678);
        assert_eq!(field.raw_rate(), "1,5678");
    }

    #[test]
    fn test_field36_parse_with_prefix() {
        let field = Field36::parse(":36:2,3456").unwrap();
        assert_eq!(field.rate(), 2.3456);
        assert_eq!(field.raw_rate(), "2,3456");
    }

    #[test]
    fn test_field36_parse_dot_decimal() {
        let field = Field36::parse("1.2345").unwrap();
        assert_eq!(field.rate(), 1.2345);
        assert_eq!(field.raw_rate(), "1.2345");
    }

    #[test]
    fn test_field36_to_swift_string() {
        let field = Field36::new(0.9876).unwrap();
        assert_eq!(field.to_swift_string(), ":36:0,9876");
    }

    #[test]
    fn test_field36_zero_rate() {
        let result = Field36::new(0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_field36_negative_rate() {
        let result = Field36::new(-1.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_field36_too_long() {
        let result = Field36::parse("123456789012345"); // 15 characters
        assert!(result.is_err());
    }

    #[test]
    fn test_field36_invalid_characters() {
        let result = Field36::parse("1.23a45");
        assert!(result.is_err());

        let result = Field36::parse("1,23-45");
        assert!(result.is_err());
    }

    #[test]
    fn test_field36_empty() {
        let result = Field36::parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_field36_validation() {
        let field = Field36::new(1.5).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field36_unreasonable_rate_warning() {
        let field = Field36::new(50000.0).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid); // Still valid, just warning
        assert!(!validation.warnings.is_empty());
    }

    #[test]
    fn test_field36_is_reasonable_rate() {
        let field1 = Field36::new(1.5).unwrap();
        assert!(field1.is_reasonable_rate());

        let field2 = Field36::new(0.00001).unwrap();
        assert!(!field2.is_reasonable_rate());

        let field3 = Field36::new(50000.0).unwrap();
        assert!(!field3.is_reasonable_rate());
    }

    #[test]
    fn test_field36_display() {
        let field = Field36::new(1.2345).unwrap();
        assert_eq!(format!("{}", field), "1,2345");
    }

    #[test]
    fn test_field36_description() {
        let field = Field36::new(0.8765).unwrap();
        assert_eq!(field.description(), "Exchange Rate: 0,8765");
    }

    #[test]
    fn test_field36_format_rate() {
        assert_eq!(Field36::format_rate(1.0), "1");
        assert_eq!(Field36::format_rate(1.5), "1,5");
        assert_eq!(Field36::format_rate(1.123456), "1,123456");
        assert_eq!(Field36::format_rate(1.100000), "1,1");
    }
}
