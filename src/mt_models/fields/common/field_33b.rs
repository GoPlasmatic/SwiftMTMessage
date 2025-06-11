//! Field 33B: Currency/Instructed Amount
//!
//! The original ordered amount in currency conversions.
//! Format: 3!a15d (currency code + amount with up to 15 digits including 2 decimal places)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 33B: Currency/Instructed Amount
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field33B {
    /// Currency code (3 letters)
    pub currency: String,
    /// Amount value
    pub amount: f64,
    /// Raw amount string as received (preserves original formatting)
    pub raw_amount: String,
}

impl Field33B {
    /// Create a new Field33B with validation
    pub fn new(
        currency: impl Into<String>,
        amount: f64,
        raw_amount: Option<String>,
    ) -> Result<Self> {
        let currency = currency.into().to_uppercase();
        let raw_amount = raw_amount.unwrap_or_else(|| Self::format_amount(amount));

        // Validate currency code
        if currency.len() != 3 {
            return Err(FieldParseError::invalid_format(
                "33B",
                "Currency code must be exactly 3 characters",
            )
            .into());
        }

        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(FieldParseError::invalid_format(
                "33B",
                "Currency code must contain only alphabetic characters",
            )
            .into());
        }

        // Validate amount
        if amount < 0.0 {
            return Err(FieldParseError::invalid_format("33B", "Amount cannot be negative").into());
        }

        Ok(Field33B {
            currency,
            amount,
            raw_amount,
        })
    }

    /// Format amount for SWIFT output
    pub fn format_amount(amount: f64) -> String {
        // Format with 2 decimal places, replace . with ,
        format!("{:.2}", amount).replace('.', ",")
    }

    /// Parse amount from string (handles both comma and dot as decimal separator)
    fn parse_amount(amount_str: &str) -> Result<(f64, String)> {
        let raw_amount = amount_str.to_string();

        // Handle both comma and dot as decimal separators
        let normalized_amount = amount_str.replace(',', ".");

        let amount = normalized_amount
            .parse::<f64>()
            .map_err(|_| FieldParseError::invalid_format("33B", "Invalid amount format"))?;

        if amount < 0.0 {
            return Err(FieldParseError::invalid_format("33B", "Amount cannot be negative").into());
        }

        Ok((amount, raw_amount))
    }
}

impl SwiftField for Field33B {
    const TAG: &'static str = "33B";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();

        if content.len() < 4 {
            return Err(FieldParseError::invalid_format(
                "33B",
                "Field content too short (minimum 4 characters: CCCAMOUNT)",
            )
            .into());
        }

        // Parse components
        let currency_str = &content[0..3];
        let amount_str = &content[3..];

        let currency = currency_str.to_uppercase();
        let (amount, raw_amount) = Self::parse_amount(amount_str)?;

        Self::new(currency, amount, Some(raw_amount))
    }

    fn to_swift_string(&self) -> String {
        format!(":33B:{}{}", self.currency, self.raw_amount)
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        let content = format!("{}{}", self.currency, self.raw_amount);
        rules.validate_field("33B", &content)
    }

    fn description() -> &'static str {
        "Currency/Instructed Amount - Original ordered amount in currency conversions"
    }
}

impl std::fmt::Display for Field33B {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:.2}", self.currency, self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_parser::FormatRules;
    use std::collections::HashMap;

    #[test]
    fn test_field33b_creation() {
        let field = Field33B::new("USD", 1000.50, None).unwrap();
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 1000.50);
        assert_eq!(field.raw_amount, "1000,50");
    }

    #[test]
    fn test_field33b_parse() {
        let field = Field33B::parse("EUR12345,67").unwrap();
        assert_eq!(field.currency, "EUR");
        assert_eq!(field.amount, 12345.67);
        assert_eq!(field.raw_amount, "12345,67");
    }

    #[test]
    fn test_field33b_parse_with_dot() {
        let field = Field33B::parse("USD1000.50").unwrap();
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 1000.50);
        assert_eq!(field.raw_amount, "1000.50");
    }

    #[test]
    fn test_field33b_invalid_currency() {
        let result = Field33B::parse("AB1000,50"); // Currency too short
        assert!(result.is_err());

        let result = Field33B::parse("1234567,89"); // Currency not alphabetic
        assert!(result.is_err());
    }

    #[test]
    fn test_field33b_invalid_amount() {
        let result = Field33B::parse("EURXYZ"); // Invalid amount
        assert!(result.is_err());
    }

    #[test]
    fn test_field33b_too_short() {
        let result = Field33B::parse("EUR"); // Too short
        assert!(result.is_err());
    }

    #[test]
    fn test_field33b_to_swift_string() {
        let field = Field33B::new("USD", 1234.56, None).unwrap();
        assert_eq!(field.to_swift_string(), ":33B:USD1234,56");
    }

    #[test]
    fn test_field33b_validation() {
        let field = Field33B::new("EUR", 1000.00, None).unwrap();
        let rules = FormatRules {
            fields: HashMap::new(),
        };
        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field33b_display() {
        let field = Field33B::new("GBP", 999.99, None).unwrap();
        assert_eq!(format!("{}", field), "GBP 999.99");
    }
}
