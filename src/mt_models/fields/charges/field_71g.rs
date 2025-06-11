//! Field 71G: Receiver's Charges
//!
//! Charges borne by the receiver.
//! Format: 3!a15d (currency code + amount)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 71G: Receiver's Charges
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field71G {
    /// Currency code (3 letters)
    pub currency: String,
    /// Charge amount
    pub amount: f64,
    /// Raw amount string as received (preserves original formatting)
    pub raw_amount: String,
}

impl Field71G {
    /// Create a new Field71G with validation
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
                "71G",
                "Currency code must be exactly 3 characters",
            )
            .into());
        }

        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(FieldParseError::invalid_format(
                "71G",
                "Currency code must contain only alphabetic characters",
            )
            .into());
        }

        // Validate amount
        if amount < 0.0 {
            return Err(FieldParseError::invalid_format("71G", "Amount cannot be negative").into());
        }

        Ok(Field71G {
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
            .map_err(|_| FieldParseError::invalid_format("71G", "Invalid amount format"))?;

        if amount < 0.0 {
            return Err(FieldParseError::invalid_format("71G", "Amount cannot be negative").into());
        }

        Ok((amount, raw_amount))
    }

    /// Get the currency code
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Get the charge amount
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Get the raw amount string
    pub fn raw_amount(&self) -> &str {
        &self.raw_amount
    }
}

impl SwiftField for Field71G {
    const TAG: &'static str = "71G";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();

        if content.len() < 4 {
            return Err(FieldParseError::invalid_format(
                "71G",
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
        format!(":71G:{}{}", self.currency, self.raw_amount)
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        let content = format!("{}{}", self.currency, self.raw_amount);
        rules.validate_field("71G", &content)
    }

    fn description() -> &'static str {
        "Receiver's Charges - Charges borne by the receiver"
    }
}

impl std::fmt::Display for Field71G {
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
    fn test_field71g_creation() {
        let field = Field71G::new("USD", 25.00, None).unwrap();
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 25.00);
        assert_eq!(field.raw_amount, "25,00");
    }

    #[test]
    fn test_field71g_parse() {
        let field = Field71G::parse("EUR15,50").unwrap();
        assert_eq!(field.currency, "EUR");
        assert_eq!(field.amount, 15.50);
        assert_eq!(field.raw_amount, "15,50");
    }

    #[test]
    fn test_field71g_parse_with_dot() {
        let field = Field71G::parse("USD10.25").unwrap();
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 10.25);
        assert_eq!(field.raw_amount, "10.25");
    }

    #[test]
    fn test_field71g_invalid_currency() {
        let result = Field71G::parse("AB10,00"); // Currency too short
        assert!(result.is_err());

        let result = Field71G::parse("12310,00"); // Currency not alphabetic
        assert!(result.is_err());
    }

    #[test]
    fn test_field71g_invalid_amount() {
        let result = Field71G::parse("EURXYZ"); // Invalid amount
        assert!(result.is_err());
    }

    #[test]
    fn test_field71g_to_swift_string() {
        let field = Field71G::new("USD", 12.34, None).unwrap();
        assert_eq!(field.to_swift_string(), ":71G:USD12,34");
    }

    #[test]
    fn test_field71g_validation() {
        let field = Field71G::new("EUR", 50.00, None).unwrap();
        let rules = FormatRules {
            fields: HashMap::new(),
        };
        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field71g_display() {
        let field = Field71G::new("GBP", 99.99, None).unwrap();
        assert_eq!(format!("{}", field), "GBP 99.99");
    }

    #[test]
    fn test_field71g_accessors() {
        let field = Field71G::new("CHF", 123.45, Some("123,45".to_string())).unwrap();
        assert_eq!(field.currency(), "CHF");
        assert_eq!(field.amount(), 123.45);
        assert_eq!(field.raw_amount(), "123,45");
    }
}
