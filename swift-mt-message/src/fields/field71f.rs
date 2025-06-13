use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Field 71F: Sender's Charges
///
/// Format: 3!a15d (3 alphabetic characters for currency + amount with up to 15 digits)
///
/// This field specifies the charges borne by the sender.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field71F {
    /// Currency code (3 letters, ISO 4217)
    pub currency: String,
    /// Charge amount
    pub amount: f64,
    /// Raw amount string as received (preserves original formatting)
    pub raw_amount: String,
}

impl Field71F {
    /// Create a new Field71F with validation
    pub fn new(currency: impl Into<String>, amount: f64) -> Result<Self, crate::ParseError> {
        let currency = currency.into().to_uppercase();

        // Validate currency code
        if currency.len() != 3 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "71F".to_string(),
                message: "Currency code must be exactly 3 characters".to_string(),
            });
        }

        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "71F".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount
        if amount < 0.0 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "71F".to_string(),
                message: "Charge amount cannot be negative".to_string(),
            });
        }

        let raw_amount = Self::format_amount(amount);

        Ok(Field71F {
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

        Ok(Field71F {
            currency,
            amount,
            raw_amount: raw_amount.to_string(),
        })
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
                field_tag: "71F".to_string(),
                message: "Invalid charge amount format".to_string(),
            })
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!("Sender's Charges: {} {}", self.currency, self.raw_amount)
    }
}

impl SwiftField for Field71F {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":71F:") {
            stripped // Remove ":71F:" prefix
        } else if let Some(stripped) = value.strip_prefix("71F:") {
            stripped // Remove "71F:" prefix
        } else {
            value
        };

        let content = content.trim();

        if content.len() < 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "71F".to_string(),
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
                field_tag: "71F".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        let amount = Self::parse_amount(amount_str)?;

        if amount < 0.0 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "71F".to_string(),
                message: "Charge amount cannot be negative".to_string(),
            });
        }

        Ok(Field71F {
            currency,
            amount,
            raw_amount: amount_str.to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":71F:{}{}", self.currency, self.raw_amount)
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate currency code
        if self.currency.len() != 3 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "71F".to_string(),
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
                field_tag: "71F".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount
        if self.amount < 0.0 {
            errors.push(ValidationError::ValueValidation {
                field_tag: "71F".to_string(),
                message: "Charge amount cannot be negative".to_string(),
            });
        }

        // Validate raw amount format
        if self.raw_amount.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "71F".to_string(),
                message: "Charge amount cannot be empty".to_string(),
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

impl std::fmt::Display for Field71F {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.currency, self.raw_amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field71f_creation() {
        let field = Field71F::new("USD", 10.50).unwrap();
        assert_eq!(field.currency(), "USD");
        assert_eq!(field.amount(), 10.50);
        assert_eq!(field.raw_amount(), "10,50");
    }

    #[test]
    fn test_field71f_from_raw() {
        let field = Field71F::from_raw("EUR", "25,75").unwrap();
        assert_eq!(field.currency(), "EUR");
        assert_eq!(field.amount(), 25.75);
        assert_eq!(field.raw_amount(), "25,75");
    }

    #[test]
    fn test_field71f_parse() {
        let field = Field71F::parse("USD15,00").unwrap();
        assert_eq!(field.currency(), "USD");
        assert_eq!(field.amount(), 15.0);
        assert_eq!(field.raw_amount(), "15,00");
    }

    #[test]
    fn test_field71f_parse_with_prefix() {
        let field = Field71F::parse(":71F:GBP5,25").unwrap();
        assert_eq!(field.currency(), "GBP");
        assert_eq!(field.amount(), 5.25);
        assert_eq!(field.raw_amount(), "5,25");
    }

    #[test]
    fn test_field71f_to_swift_string() {
        let field = Field71F::new("CHF", 100.0).unwrap();
        assert_eq!(field.to_swift_string(), ":71F:CHF100,00");
    }

    #[test]
    fn test_field71f_invalid_currency() {
        let result = Field71F::new("US", 10.0);
        assert!(result.is_err());

        let result = Field71F::new("123", 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_field71f_negative_amount() {
        let result = Field71F::new("USD", -10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_field71f_validation() {
        let field = Field71F::new("USD", 50.0).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field71f_display() {
        let field = Field71F::new("EUR", 75.50).unwrap();
        assert_eq!(format!("{}", field), "EUR 75,50");
    }

    #[test]
    fn test_field71f_description() {
        let field = Field71F::new("USD", 20.0).unwrap();
        assert_eq!(field.description(), "Sender's Charges: USD 20,00");
    }
}
