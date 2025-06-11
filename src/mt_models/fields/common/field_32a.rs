//! Field 32A: Value Date, Currency Code, Amount
//!
//! Contains the value date, currency code, and amount of the transaction.
//! Format: 6!n3!a15d (YYMMDD + CCC + amount with up to 15 digits including 2 decimal places)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// Field 32A: Value Date, Currency Code, Amount
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field32A {
    /// Value date (YYMMDD format)
    pub value_date: NaiveDate,
    /// Currency code (3 characters, ISO 4217)
    pub currency: String,
    /// Amount (decimal value)
    pub amount: f64,
    /// Raw amount string (preserves original formatting)
    pub raw_amount: String,
}

impl Field32A {
    /// Create a new Field32A with validation
    pub fn new(
        value_date: NaiveDate,
        currency: impl Into<String>,
        amount: f64,
        raw_amount: Option<String>,
    ) -> Result<Self> {
        let currency = currency.into().trim().to_uppercase();

        // Validate currency code
        if currency.len() != 3 {
            return Err(FieldParseError::invalid_format(
                "32A",
                "Currency code must be exactly 3 characters",
            )
            .into());
        }

        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(FieldParseError::invalid_format(
                "32A",
                "Currency code must contain only alphabetic characters",
            )
            .into());
        }

        // Validate amount
        if amount < 0.0 {
            return Err(FieldParseError::invalid_format("32A", "Amount cannot be negative").into());
        }

        // Generate raw amount if not provided
        let raw_amount = raw_amount.unwrap_or_else(|| {
            // Format with 2 decimal places and comma as decimal separator (SWIFT standard)
            format!("{:.2}", amount).replace('.', ",")
        });

        Ok(Field32A {
            value_date,
            currency,
            amount,
            raw_amount,
        })
    }

    /// Get the value date
    pub fn date(&self) -> NaiveDate {
        self.value_date
    }

    /// Get the currency code
    pub fn currency_code(&self) -> &str {
        &self.currency
    }

    /// Get the amount as a float
    pub fn amount_value(&self) -> f64 {
        self.amount
    }

    /// Get the raw amount string (as it appears in the SWIFT message)
    pub fn amount_raw(&self) -> &str {
        &self.raw_amount
    }

    /// Format value date as YYMMDD
    pub fn format_date(&self) -> String {
        // Handle Y2K convention: convert 4-digit year to 2-digit year
        let year = self.value_date.year();
        let yy = if year >= 2000 {
            year - 2000 // 2000-2099 becomes 00-99
        } else {
            year - 1900 // 1900-1999 becomes 00-99
        };

        format!(
            "{:02}{:02}{:02}",
            yy,
            self.value_date.month(),
            self.value_date.day()
        )
    }

    /// Parse date from YYMMDD format
    fn parse_date(date_str: &str) -> Result<NaiveDate> {
        if date_str.len() != 6 {
            return Err(
                FieldParseError::invalid_format("32A", "Date must be in YYMMDD format").into(),
            );
        }

        let year: i32 = date_str[0..2]
            .parse()
            .map_err(|_| FieldParseError::invalid_format("32A", "Invalid year in date"))?;
        let month: u32 = date_str[2..4]
            .parse()
            .map_err(|_| FieldParseError::invalid_format("32A", "Invalid month in date"))?;
        let day: u32 = date_str[4..6]
            .parse()
            .map_err(|_| FieldParseError::invalid_format("32A", "Invalid day in date"))?;

        // Handle Y2K: assume 00-49 is 20xx, 50-99 is 19xx
        let full_year = if year <= 49 { 2000 + year } else { 1900 + year };

        NaiveDate::from_ymd_opt(full_year, month, day)
            .ok_or_else(|| FieldParseError::invalid_format("32A", "Invalid date").into())
    }

    /// Parse amount from SWIFT format (supports both comma and dot as decimal separator)
    fn parse_amount(amount_str: &str) -> Result<(f64, String)> {
        let raw_amount = amount_str.to_string();

        // Handle both comma and dot as decimal separators
        let normalized_amount = amount_str.replace(',', ".");

        let amount = normalized_amount
            .parse::<f64>()
            .map_err(|_| FieldParseError::invalid_format("32A", "Invalid amount format"))?;

        if amount < 0.0 {
            return Err(FieldParseError::invalid_format("32A", "Amount cannot be negative").into());
        }

        Ok((amount, raw_amount))
    }
}

impl SwiftField for Field32A {
    const TAG: &'static str = "32A";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();

        if content.len() < 9 {
            return Err(FieldParseError::invalid_format(
                "32A",
                "Field content too short (minimum 9 characters: YYMMDDCCCAMOUNT)",
            )
            .into());
        }

        // Parse components
        let date_str = &content[0..6];
        let currency_str = &content[6..9];
        let amount_str = &content[9..];

        let value_date = Self::parse_date(date_str)?;
        let currency = currency_str.to_uppercase();
        let (amount, raw_amount) = Self::parse_amount(amount_str)?;

        Self::new(value_date, currency, amount, Some(raw_amount))
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":32A:{}{}{}",
            self.format_date(),
            self.currency,
            self.raw_amount
        )
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        let content = format!("{}{}{}", self.format_date(), self.currency, self.raw_amount);
        rules.validate_field("32A", &content)
    }

    fn description() -> &'static str {
        "Value Date, Currency Code, Amount - Specifies the value date, currency, and amount of the transaction"
    }
}

impl std::fmt::Display for Field32A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.value_date.format("%Y-%m-%d"),
            self.currency,
            self.raw_amount
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_parser::FormatRules;
    use chrono::NaiveDate;
    use std::collections::HashMap;

    #[test]
    fn test_field32a_creation() {
        let date = NaiveDate::from_ymd_opt(2021, 3, 15).unwrap();
        let field = Field32A::new(date, "EUR", 1234567.89, None).unwrap();

        assert_eq!(field.date(), date);
        assert_eq!(field.currency_code(), "EUR");
        assert_eq!(field.amount_value(), 1234567.89);
        assert_eq!(field.to_swift_string(), ":32A:210315EUR1234567,89");
    }

    #[test]
    fn test_field32a_parse() {
        let field = Field32A::parse("210315EUR1234567,89").unwrap();

        assert_eq!(field.date(), NaiveDate::from_ymd_opt(2021, 3, 15).unwrap());
        assert_eq!(field.currency_code(), "EUR");
        assert_eq!(field.amount_value(), 1234567.89);
        assert_eq!(field.amount_raw(), "1234567,89");
    }

    #[test]
    fn test_field32a_parse_with_dot_separator() {
        let field = Field32A::parse("210315USD1000.50").unwrap();

        assert_eq!(field.currency_code(), "USD");
        assert_eq!(field.amount_value(), 1000.50);
        assert_eq!(field.amount_raw(), "1000.50");
    }

    #[test]
    fn test_field32a_y2k_date_handling() {
        // Test 21st century dates (00-49)
        let field = Field32A::parse("210315EUR1000,00").unwrap();
        assert_eq!(field.date().year(), 2021);

        // Test 20th century dates (50-99)
        let field = Field32A::parse("990315EUR1000,00").unwrap();
        assert_eq!(field.date().year(), 1999);
    }

    #[test]
    fn test_field32a_invalid_date() {
        let result = Field32A::parse("213215EUR1000,00"); // Invalid month 32
        assert!(result.is_err());
    }

    #[test]
    fn test_field32a_invalid_currency() {
        let result = Field32A::parse("210315E1R1000,00"); // Invalid currency with number
        assert!(result.is_err());

        let result = Field32A::parse("210315EURO1000,00"); // Currency too long
        assert!(result.is_err());
    }

    #[test]
    fn test_field32a_invalid_amount() {
        let result = Field32A::parse("210315EUR-1000,00"); // Negative amount
        assert!(result.is_err());

        let result = Field32A::parse("210315EURABC"); // Non-numeric amount
        assert!(result.is_err());
    }

    #[test]
    fn test_field32a_too_short() {
        let result = Field32A::parse("210315EU"); // Too short
        assert!(result.is_err());
    }

    #[test]
    fn test_field32a_format_date() {
        let date = NaiveDate::from_ymd_opt(2021, 3, 5).unwrap();
        let field = Field32A::new(date, "EUR", 1000.0, None).unwrap();
        assert_eq!(field.format_date(), "210305");

        // Test Y2K handling for 20th century
        let date = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
        let field = Field32A::new(date, "EUR", 1000.0, None).unwrap();
        assert_eq!(field.format_date(), "991231");
    }

    #[test]
    fn test_field32a_validation() {
        let date = NaiveDate::from_ymd_opt(2021, 3, 15).unwrap();
        let field = Field32A::new(date, "EUR", 1234567.89, None).unwrap();
        let rules = FormatRules {
            fields: HashMap::new(),
        };

        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field32a_display() {
        let date = NaiveDate::from_ymd_opt(2021, 3, 15).unwrap();
        let field = Field32A::new(date, "EUR", 1234567.89, None).unwrap();
        assert_eq!(format!("{}", field), "2021-03-15 EUR 1234567,89");
    }
}
