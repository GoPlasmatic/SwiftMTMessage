use crate::{SwiftField, ValidationResult};
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// Field 32A: Value Date, Currency Code, Amount
///
/// Format: 6!n3!a15d (YYMMDD + 3-letter currency + amount with decimal)
///
/// This field contains the value date, currency, and amount of the transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field32A {
    /// Value date
    pub value_date: NaiveDate,

    /// Currency code (3 letters, ISO 4217)
    pub currency: String,

    /// Transaction amount
    pub amount: f64,

    /// Raw amount string as received (preserves original formatting)
    pub raw_amount: String,
}

impl SwiftField for Field32A {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":32A:") {
            stripped // Remove ":32A:" prefix
        } else if let Some(stripped) = value.strip_prefix("32A:") {
            stripped // Remove "32A:" prefix
        } else {
            value
        };

        let content = content.trim();

        if content.len() < 9 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "32A".to_string(),
                message: "Field too short (minimum 9 characters: YYMMDDCCCAMOUNT)".to_string(),
            });
        }

        // Parse date (YYMMDD format)
        let date_str = &content[0..6];
        let year = format!("20{}", &date_str[0..2]);
        let month = &date_str[2..4];
        let day = &date_str[4..6];
        let full_date_str = format!("{}-{}-{}", year, month, day);

        let value_date = NaiveDate::parse_from_str(&full_date_str, "%Y-%m-%d").map_err(|_| {
            crate::ParseError::InvalidFieldFormat {
                field_tag: "32A".to_string(),
                message: "Invalid date format".to_string(),
            }
        })?;

        // Parse currency (3 characters)
        let currency = content[6..9].to_string().to_uppercase();

        // Parse amount (remaining characters)
        let raw_amount = content[9..].to_string();
        let amount = raw_amount.replace(',', ".").parse::<f64>().map_err(|_| {
            crate::ParseError::InvalidFieldFormat {
                field_tag: "32A".to_string(),
                message: "Invalid amount format".to_string(),
            }
        })?;

        Ok(Self {
            value_date,
            currency,
            amount,
            raw_amount,
        })
    }

    fn to_swift_string(&self) -> String {
        // Format date as YYMMDD
        let date_str = format!(
            "{:02}{:02}{:02}",
            self.value_date.year() % 100,
            self.value_date.month(),
            self.value_date.day()
        );

        format!(":32A:{}{}{}", date_str, self.currency, self.raw_amount)
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate currency length
        if self.currency.len() != 3 {
            errors.push(crate::ValidationError::LengthValidation {
                field_tag: "32A".to_string(),
                expected: "3 characters".to_string(),
                actual: self.currency.len(),
            });
        }

        // Validate currency contains only letters
        if !self.currency.chars().all(|c| c.is_alphabetic()) {
            errors.push(crate::ValidationError::ValueValidation {
                field_tag: "32A".to_string(),
                message: "Currency must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount is positive
        if self.amount <= 0.0 {
            errors.push(crate::ValidationError::ValueValidation {
                field_tag: "32A".to_string(),
                message: "Amount must be positive".to_string(),
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "6!n3!a15d"
    }
}

impl Field32A {
    /// Create a new Field32A
    pub fn new(value_date: NaiveDate, currency: String, amount: f64) -> Self {
        // Format amount with comma as decimal separator (SWIFT standard)
        let raw_amount = format!("{:.2}", amount).replace('.', ",");
        Self {
            value_date,
            currency: currency.to_uppercase(),
            amount,
            raw_amount,
        }
    }

    /// Create from raw values
    pub fn from_raw(
        value_date: NaiveDate,
        currency: String,
        raw_amount: String,
    ) -> Result<Self, std::num::ParseFloatError> {
        let amount = raw_amount.replace(',', ".").parse::<f64>()?;
        Ok(Self {
            value_date,
            currency: currency.to_uppercase(),
            amount,
            raw_amount,
        })
    }

    /// Get the currency code
    pub fn currency_code(&self) -> &str {
        &self.currency
    }

    /// Get the amount as decimal
    pub fn amount_decimal(&self) -> f64 {
        self.amount
    }

    /// Format date as YYMMDD string
    pub fn date_string(&self) -> String {
        format!(
            "{:02}{:02}{:02}",
            self.value_date.year() % 100,
            self.value_date.month(),
            self.value_date.day()
        )
    }
}

impl std::fmt::Display for Field32A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.date_string(),
            self.currency,
            self.raw_amount
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_field32a_creation() {
        let date = NaiveDate::from_ymd_opt(2021, 3, 15).unwrap();
        let field = Field32A::new(date, "EUR".to_string(), 1234567.89);

        assert_eq!(field.value_date.year(), 2021);
        assert_eq!(field.value_date.month(), 3);
        assert_eq!(field.value_date.day(), 15);
        assert_eq!(field.currency_code(), "EUR");
        assert_eq!(field.amount, 1234567.89);
    }

    #[test]
    fn test_field32a_parse() {
        let field = Field32A::parse("210315EUR1234567,89").unwrap();
        assert_eq!(field.value_date.year(), 2021);
        assert_eq!(field.value_date.month(), 3);
        assert_eq!(field.value_date.day(), 15);
        assert_eq!(field.currency_code(), "EUR");
        assert_eq!(field.amount, 1234567.89);
    }

    #[test]
    fn test_field32a_date_string() {
        let date = NaiveDate::from_ymd_opt(2021, 3, 15).unwrap();
        let field = Field32A::new(date, "EUR".to_string(), 1234567.89);

        assert_eq!(field.date_string(), "210315");
    }

    #[test]
    fn test_field32a_to_swift_string() {
        let date = NaiveDate::from_ymd_opt(2021, 3, 15).unwrap();
        let field = Field32A::new(date, "EUR".to_string(), 1234567.89);

        assert_eq!(field.to_swift_string(), ":32A:210315EUR1234567,89");
    }
}
