use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Field 33B: Currency/Instructed Amount
///
/// Format: 3!a15d (3 alphabetic characters for currency + amount with up to 15 digits)
///
/// This field specifies the original ordered amount in currency conversions.
/// Used when the instructed amount differs from the settlement amount.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field33B {
    /// Currency code (3 letters, ISO 4217)
    pub currency: String,
    /// Amount value
    pub amount: f64,
    /// Raw amount string as received (preserves original formatting)
    pub raw_amount: String,
}

impl Field33B {
    /// Create a new Field33B with validation
    pub fn new(currency: impl Into<String>, amount: f64) -> Result<Self, crate::ParseError> {
        let currency = currency.into().to_uppercase();

        // Validate currency code
        if currency.len() != 3 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "33B".to_string(),
                message: "Currency code must be exactly 3 characters".to_string(),
            });
        }

        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "33B".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount
        if amount < 0.0 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "33B".to_string(),
                message: "Amount cannot be negative".to_string(),
            });
        }

        let raw_amount = Self::format_amount(amount);

        Ok(Field33B {
            currency,
            amount,
            raw_amount,
        })
    }

    /// Create from raw amount string
    pub fn from_raw(
        currency: impl Into<String>,
        raw_amount: impl Into<String>,
    ) -> Result<Self, crate::ParseError> {
        let currency = currency.into().to_uppercase();
        let raw_amount = raw_amount.into();

        let amount = Self::parse_amount(&raw_amount)?;

        Ok(Field33B {
            currency,
            amount,
            raw_amount: raw_amount.to_string(),
        })
    }

    /// Get the currency code
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Get the amount value
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Get the raw amount string
    pub fn raw_amount(&self) -> &str {
        &self.raw_amount
    }

    /// Format amount for SWIFT output (with comma as decimal separator)
    pub fn format_amount(amount: f64) -> String {
        format!("{:.2}", amount).replace('.', ",")
    }

    /// Parse amount from string (handles both comma and dot as decimal separator)
    fn parse_amount(amount_str: &str) -> Result<f64, crate::ParseError> {
        let normalized_amount = amount_str.replace(',', ".");

        normalized_amount
            .parse::<f64>()
            .map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "33B".to_string(),
                message: "Invalid amount format".to_string(),
            })
    }

    /// Check if this is a valid ISO 4217 currency code (basic validation)
    pub fn is_valid_currency(&self) -> bool {
        self.currency.len() == 3 && self.currency.chars().all(|c| c.is_alphabetic())
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!(
            "Currency/Instructed Amount: {} {}",
            self.currency, self.raw_amount
        )
    }
}

impl SwiftField for Field33B {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        // Handle input that includes field tag prefix (e.g., ":33B:USD1234,56")
        let content = if value.starts_with(":33B:") {
            &value[5..] // Remove ":33B:" prefix
        } else if value.starts_with("33B:") {
            &value[4..] // Remove "33B:" prefix
        } else {
            value // Use as-is if no prefix
        };

        let content = content.trim();

        if content.len() < 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "33B".to_string(),
                message: "Field content too short (minimum 4 characters: CCCAMOUNT)".to_string(),
            });
        }

        // Parse components: first 3 characters are currency, rest is amount
        let currency_str = &content[0..3];
        let amount_str = &content[3..];

        let currency = currency_str.to_uppercase();

        // Validate currency
        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "33B".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        let amount = Self::parse_amount(amount_str)?;

        if amount < 0.0 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "33B".to_string(),
                message: "Amount cannot be negative".to_string(),
            });
        }

        Ok(Field33B {
            currency,
            amount,
            raw_amount: amount_str.to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":33B:{}{}", self.currency, self.raw_amount)
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate currency code
        if self.currency.len() != 3 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "33B".to_string(),
                expected: "3 characters".to_string(),
                actual: self.currency.len(),
            });
        }

        if !self
            .currency
            .chars()
            .all(|c| c.is_alphabetic() && c.is_ascii())
        {
            errors.push(ValidationError::FormatValidation {
                field_tag: "33B".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount
        if self.amount < 0.0 {
            errors.push(ValidationError::ValueValidation {
                field_tag: "33B".to_string(),
                message: "Amount cannot be negative".to_string(),
            });
        }

        // Validate raw amount format
        if self.raw_amount.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "33B".to_string(),
                message: "Amount cannot be empty".to_string(),
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "3!a15d"
    }
}

impl std::fmt::Display for Field33B {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.currency, self.raw_amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field33b_creation() {
        let field = Field33B::new("USD", 1234.56).unwrap();
        assert_eq!(field.currency(), "USD");
        assert_eq!(field.amount(), 1234.56);
        assert_eq!(field.raw_amount(), "1234,56");
    }

    #[test]
    fn test_field33b_from_raw() {
        let field = Field33B::from_raw("EUR", "999,99").unwrap();
        assert_eq!(field.currency(), "EUR");
        assert_eq!(field.amount(), 999.99);
        assert_eq!(field.raw_amount(), "999,99");
    }

    #[test]
    fn test_field33b_parse() {
        let field = Field33B::parse("USD1234,56").unwrap();
        assert_eq!(field.currency(), "USD");
        assert_eq!(field.amount(), 1234.56);
        assert_eq!(field.raw_amount(), "1234,56");
    }

    #[test]
    fn test_field33b_parse_with_prefix() {
        let field = Field33B::parse(":33B:EUR500,00").unwrap();
        assert_eq!(field.currency(), "EUR");
        assert_eq!(field.amount(), 500.0);
        assert_eq!(field.raw_amount(), "500,00");
    }

    #[test]
    fn test_field33b_to_swift_string() {
        let field = Field33B::new("GBP", 750.25).unwrap();
        assert_eq!(field.to_swift_string(), ":33B:GBP750,25");
    }

    #[test]
    fn test_field33b_invalid_currency_length() {
        let result = Field33B::new("US", 100.0);
        assert!(result.is_err());

        let result = Field33B::new("USDD", 100.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_field33b_invalid_currency_characters() {
        let result = Field33B::new("U$D", 100.0);
        assert!(result.is_err());

        let result = Field33B::new("123", 100.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_field33b_negative_amount() {
        let result = Field33B::new("USD", -100.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_field33b_parse_invalid_format() {
        let result = Field33B::parse("USD");
        assert!(result.is_err());

        let result = Field33B::parse("US1234,56");
        assert!(result.is_err());
    }

    #[test]
    fn test_field33b_validation() {
        let field = Field33B::new("USD", 1000.0).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field33b_display() {
        let field = Field33B::new("CHF", 2500.75).unwrap();
        assert_eq!(format!("{}", field), "CHF 2500,75");
    }

    #[test]
    fn test_field33b_is_valid_currency() {
        let field = Field33B::new("USD", 100.0).unwrap();
        assert!(field.is_valid_currency());
    }

    #[test]
    fn test_field33b_description() {
        let field = Field33B::new("EUR", 1500.0).unwrap();
        assert_eq!(
            field.description(),
            "Currency/Instructed Amount: EUR 1500,00"
        );
    }

    #[test]
    fn test_field33b_parse_dot_decimal() {
        let field = Field33B::parse("USD1234.56").unwrap();
        assert_eq!(field.amount(), 1234.56);
        assert_eq!(field.raw_amount(), "1234.56");
    }
}
