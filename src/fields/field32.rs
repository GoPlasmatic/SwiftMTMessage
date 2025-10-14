//! # Field 32: Value Date, Currency, Amount
//!
//! Settlement amount and value date for payment instructions.
//!
//! **Variants:**
//! - **32A:** Date + Currency + Amount (YYMMDD + 3!a + 15d)
//! - **32B:** Currency + Amount (3!a + 15d)
//! - **32C:** Date + Currency + Credit Amount (MT n90 messages)
//! - **32D:** Date + Currency + Debit Amount (MT n90 messages)
//!
//! **Example:**
//! ```text
//! :32A:240719USD1000,50
//! :32B:EUR500,00
//! ```

use super::swift_utils::{
    format_swift_amount_for_currency, parse_amount_with_currency, parse_currency_non_commodity,
    parse_date_yymmdd,
};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// **Field 32A: Value Date, Currency, Amount**
///
/// Settlement information with value date.
/// Format: `6!n3!a15d` (YYMMDD + currency + amount)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field32A {
    /// Value date (YYMMDD)
    #[serde(with = "date_string")]
    pub value_date: NaiveDate,
    /// ISO 4217 currency code
    pub currency: String,
    /// Settlement amount
    pub amount: f64,
}

impl SwiftField for Field32A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Field32A format: 6!n3!a15d (date + currency + amount)
        if input.len() < 10 {
            // Minimum: 6 digits date + 3 chars currency + 1 digit amount
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 32A must be at least 10 characters, found {}",
                    input.len()
                ),
            });
        }

        // Parse value date (first 6 characters) - T50 validation
        let value_date = parse_date_yymmdd(&input[0..6])?;

        // Parse currency code (next 3 characters) - T52 + C08 validation
        let currency = parse_currency_non_commodity(&input[6..9])?;

        // Parse amount (remaining characters) - T40/T43 + C03 validation
        let amount_str = &input[9..];
        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 32A amount cannot be empty".to_string(),
            });
        }

        let amount = parse_amount_with_currency(amount_str, &currency)?;

        // Amount must be positive
        if amount <= 0.0 {
            return Err(ParseError::InvalidFormat {
                message: "Field 32A amount must be greater than zero".to_string(),
            });
        }

        Ok(Field32A {
            value_date,
            currency,
            amount,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":32A:{}{}{}",
            self.value_date.format("%y%m%d"),
            self.currency,
            format_swift_amount_for_currency(self.amount, &self.currency)
        )
    }
}

/// **Field 32B: Currency, Amount**
///
/// Currency and amount without value date.
/// Format: `3!a15d` (currency + amount)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field32B {
    /// ISO 4217 currency code
    pub currency: String,
    /// Amount
    pub amount: f64,
}

impl SwiftField for Field32B {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Field32B format: 3!a15d (currency + amount)
        if input.len() < 4 {
            // Minimum: 3 chars currency + 1 digit amount
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 32B must be at least 4 characters, found {}",
                    input.len()
                ),
            });
        }

        // Parse currency code (first 3 characters) - T52 + C08 validation
        let currency = parse_currency_non_commodity(&input[0..3])?;

        // Parse amount (remaining characters) - T40/T43 + C03 validation
        let amount_str = &input[3..];
        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 32B amount cannot be empty".to_string(),
            });
        }

        let amount = parse_amount_with_currency(amount_str, &currency)?;

        // Amount must be positive
        if amount <= 0.0 {
            return Err(ParseError::InvalidFormat {
                message: "Field 32B amount must be greater than zero".to_string(),
            });
        }

        Ok(Field32B { currency, amount })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":32B:{}{}",
            self.currency,
            format_swift_amount_for_currency(self.amount, &self.currency)
        )
    }
}

/// **Field 32C: Value Date, Currency, Credit Amount**
///
/// Credit amount with value date (MT n90 messages).
/// Format: `6!n3!a15d`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field32C {
    /// Value date (YYMMDD)
    #[serde(with = "date_string")]
    pub value_date: NaiveDate,
    /// ISO 4217 currency code
    pub currency: String,
    /// Credit amount
    pub amount: f64,
}

// Custom serialization for dates as strings
mod date_string {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.format("%Y-%m-%d").to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
    }
}

impl SwiftField for Field32C {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Same format as Field32A
        if input.len() < 10 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 32C must be at least 10 characters, found {}",
                    input.len()
                ),
            });
        }

        let value_date = parse_date_yymmdd(&input[0..6])?;
        let currency = parse_currency_non_commodity(&input[6..9])?;
        let amount_str = &input[9..];

        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 32C amount cannot be empty".to_string(),
            });
        }

        let amount = parse_amount_with_currency(amount_str, &currency)?;

        if amount <= 0.0 {
            return Err(ParseError::InvalidFormat {
                message: "Field 32C amount must be greater than zero".to_string(),
            });
        }

        Ok(Field32C {
            value_date,
            currency,
            amount,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":32C:{}{}{}",
            self.value_date.format("%y%m%d"),
            self.currency,
            format_swift_amount_for_currency(self.amount, &self.currency)
        )
    }
}

/// **Field 32D: Value Date, Currency, Debit Amount**
///
/// Debit amount with value date (MT n90 messages).
/// Format: `6!n3!a15d`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field32D {
    /// Value date (YYMMDD)
    #[serde(with = "date_string")]
    pub value_date: NaiveDate,
    /// ISO 4217 currency code
    pub currency: String,
    /// Debit amount
    pub amount: f64,
}

impl SwiftField for Field32D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Same format as Field32A
        if input.len() < 10 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 32D must be at least 10 characters, found {}",
                    input.len()
                ),
            });
        }

        let value_date = parse_date_yymmdd(&input[0..6])?;
        let currency = parse_currency_non_commodity(&input[6..9])?;
        let amount_str = &input[9..];

        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 32D amount cannot be empty".to_string(),
            });
        }

        let amount = parse_amount_with_currency(amount_str, &currency)?;

        if amount <= 0.0 {
            return Err(ParseError::InvalidFormat {
                message: "Field 32D amount must be greater than zero".to_string(),
            });
        }

        Ok(Field32D {
            value_date,
            currency,
            amount,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":32D:{}{}{}",
            self.value_date.format("%y%m%d"),
            self.currency,
            format_swift_amount_for_currency(self.amount, &self.currency)
        )
    }
}

/// **Field 32: Settlement Amount Variants**
///
/// Enum wrapper for Field 32 variants (A/B/C/D).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field32 {
    #[serde(rename = "32A")]
    A(Field32A),
    #[serde(rename = "32B")]
    B(Field32B),
    #[serde(rename = "32C")]
    C(Field32C),
    #[serde(rename = "32D")]
    D(Field32D),
}

impl SwiftField for Field32 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try to determine variant based on content
        // If it starts with 6 digits (date), it's A, C, or D
        // Otherwise it's B (currency + amount only)
        if input.len() >= 6 {
            // Check if first 6 chars are digits (date)
            if input[0..6].chars().all(|c| c.is_ascii_digit()) {
                // Default to A for date variants
                Ok(Field32::A(Field32A::parse(input)?))
            } else if input.len() >= 3 && input[0..3].chars().all(|c| c.is_ascii_alphabetic()) {
                // Starts with currency, must be B
                Ok(Field32::B(Field32B::parse(input)?))
            } else {
                Err(ParseError::InvalidFormat {
                    message:
                        "Field 32 must start with either date (6 digits) or currency (3 letters)"
                            .to_string(),
                })
            }
        } else {
            Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 32 must be at least 6 characters, found {}",
                    input.len()
                ),
            })
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field32::A(field) => field.to_swift_string(),
            Field32::B(field) => field.to_swift_string(),
            Field32::C(field) => field.to_swift_string(),
            Field32::D(field) => field.to_swift_string(),
        }
    }
}

/// **Field32AB: Options A or B only**
///
/// Used in MT110, MT111, MT112 (cheque messages).
/// Supports only 32A (with date) or 32B (without date).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field32AB {
    #[serde(rename = "32A")]
    A(Field32A),
    #[serde(rename = "32B")]
    B(Field32B),
}

impl SwiftField for Field32AB {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try parsing as Field32A first (has value date)
        if let Ok(field) = Field32A::parse(input) {
            return Ok(Field32AB::A(field));
        }

        // If that fails, try as Field32B (no value date)
        if let Ok(field) = Field32B::parse(input) {
            return Ok(Field32AB::B(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 32 must be either format 32A (YYMMDD + Currency + Amount) or 32B (Currency + Amount)".to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field32AB::A(field) => field.to_swift_string(),
            Field32AB::B(field) => field.to_swift_string(),
        }
    }
}

/// **Field32AmountCD: Credit or Debit**
///
/// Used in MT190 and similar messages.
/// Supports 32C (credit) or 32D (debit).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field32AmountCD {
    #[serde(rename = "32C")]
    C(Field32C),
    #[serde(rename = "32D")]
    D(Field32D),
}

impl SwiftField for Field32AmountCD {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Both C and D variants have the same format (date + currency + amount)
        // Try to parse as Field32C first (credit)
        if let Ok(field) = Field32C::parse(input) {
            return Ok(Field32AmountCD::C(field));
        }

        // If that fails, try as Field32D (debit)
        if let Ok(field) = Field32D::parse(input) {
            return Ok(Field32AmountCD::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 32 must be in format: YYMMDD + Currency + Amount".to_string(),
        })
    }

    fn parse_with_variant(
        value: &str,
        variant: Option<&str>,
        _field_tag: Option<&str>,
    ) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Use the variant letter to determine which type to parse
        match variant {
            Some("C") => {
                let field = Field32C::parse(value)?;
                Ok(Field32AmountCD::C(field))
            }
            Some("D") => {
                let field = Field32D::parse(value)?;
                Ok(Field32AmountCD::D(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field32AmountCD::C(field) => field.to_swift_string(),
            Field32AmountCD::D(field) => field.to_swift_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_field32a_valid() {
        let field = Field32A::parse("240719EUR1250,50").unwrap();
        assert_eq!(
            field.value_date,
            NaiveDate::from_ymd_opt(2024, 7, 19).unwrap()
        );
        assert_eq!(field.currency, "EUR");
        assert_eq!(field.amount, 1250.50);
        assert_eq!(
            field.to_swift_string(),
            ":32A:240719EUR1250,50"
        );

        let field = Field32A::parse("240720USD10000,00").unwrap();
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 10000.0);

        let field = Field32A::parse("240721JPY1500000").unwrap();
        assert_eq!(field.currency, "JPY");
        assert_eq!(field.amount, 1500000.0);
    }

    #[test]
    fn test_field32a_invalid() {
        // Invalid date
        assert!(Field32A::parse("991332EUR100").is_err());

        // Invalid currency
        assert!(Field32A::parse("240719EU1100").is_err());
        assert!(Field32A::parse("2407191UR100").is_err());

        // Zero amount
        assert!(Field32A::parse("240719EUR0").is_err());

        // Negative amount
        assert!(Field32A::parse("240719EUR-100").is_err());

        // Too short
        assert!(Field32A::parse("240719EUR").is_err());
    }

    #[test]
    fn test_field32b_valid() {
        let field = Field32B::parse("EUR5000,00").unwrap();
        assert_eq!(field.currency, "EUR");
        assert_eq!(field.amount, 5000.0);
        assert_eq!(field.to_swift_string(), ":32B:EUR5000,00");

        let field = Field32B::parse("USD100").unwrap();
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 100.0);
    }

    #[test]
    fn test_field32b_invalid() {
        // Invalid currency
        assert!(Field32B::parse("12A100").is_err());

        // Zero amount
        assert!(Field32B::parse("EUR0").is_err());

        // Missing amount
        assert!(Field32B::parse("EUR").is_err());
    }

    #[test]
    fn test_field32c_valid() {
        let field = Field32C::parse("240719EUR500,25").unwrap();
        assert_eq!(
            field.value_date,
            NaiveDate::from_ymd_opt(2024, 7, 19).unwrap()
        );
        assert_eq!(field.currency, "EUR");
        assert_eq!(field.amount, 500.25);
    }

    #[test]
    fn test_field32d_valid() {
        let field = Field32D::parse("240719USD750,50").unwrap();
        assert_eq!(
            field.value_date,
            NaiveDate::from_ymd_opt(2024, 7, 19).unwrap()
        );
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 750.50);
    }

    #[test]
    fn test_field32_enum() {
        // Should parse as Field32A (has date)
        let field = Field32::parse("240719EUR1000").unwrap();
        match field {
            Field32::A(f) => {
                assert_eq!(f.currency, "EUR");
                assert_eq!(f.amount, 1000.0);
            }
            _ => panic!("Expected Field32::A"),
        }

        // Should parse as Field32B (no date)
        let field = Field32::parse("EUR2000").unwrap();
        match field {
            Field32::B(f) => {
                assert_eq!(f.currency, "EUR");
                assert_eq!(f.amount, 2000.0);
            }
            _ => panic!("Expected Field32::B"),
        }
    }

    #[test]
    fn test_field32_ab() {
        // Test parsing as Field32A (with value date)
        let field = Field32AB::parse("240719EUR500,25").unwrap();
        match field {
            Field32AB::A(f) => {
                assert_eq!(f.value_date, NaiveDate::from_ymd_opt(2024, 7, 19).unwrap());
                assert_eq!(f.currency, "EUR");
                assert_eq!(f.amount, 500.25);
            }
            _ => panic!("Expected Field32AB::A"),
        }

        // Test parsing as Field32B (no value date)
        let field = Field32AB::parse("USD1000,00").unwrap();
        match field {
            Field32AB::B(f) => {
                assert_eq!(f.currency, "USD");
                assert_eq!(f.amount, 1000.00);
            }
            _ => panic!("Expected Field32AB::B"),
        }

        // Test to_swift_string for A (Field32A includes field tag)
        let field_a = Field32AB::A(Field32A {
            value_date: NaiveDate::from_ymd_opt(2024, 7, 19).unwrap(),
            currency: "EUR".to_string(),
            amount: 500.25,
        });
        assert_eq!(field_a.to_swift_string(), ":32A:240719EUR500,25");

        // Test to_swift_string for B (Field32B includes field tag)
        let field_b = Field32AB::B(Field32B {
            currency: "USD".to_string(),
            amount: 1000.00,
        });
        assert_eq!(field_b.to_swift_string(), ":32B:USD1000,00");
    }

    #[test]
    fn test_field32_amount_cd() {
        // Test parsing as credit (32C)
        let field = Field32AmountCD::parse("240719EUR500,25").unwrap();
        match field {
            Field32AmountCD::C(f) => {
                assert_eq!(f.value_date, NaiveDate::from_ymd_opt(2024, 7, 19).unwrap());
                assert_eq!(f.currency, "EUR");
                assert_eq!(f.amount, 500.25);
            }
            _ => panic!("Expected Field32AmountCD::C"),
        }

        // Test parsing as debit (32D) - same format
        let field = Field32AmountCD::parse("240720USD750,50").unwrap();
        match field {
            Field32AmountCD::C(f) => {
                // Since both C and D have same format, it will parse as C first
                assert_eq!(f.value_date, NaiveDate::from_ymd_opt(2024, 7, 20).unwrap());
                assert_eq!(f.currency, "USD");
                assert_eq!(f.amount, 750.50);
            }
            _ => panic!("Expected Field32AmountCD::C"),
        }

        // Test to_swift_string
        let credit_field = Field32AmountCD::C(Field32C {
            value_date: NaiveDate::from_ymd_opt(2024, 7, 19).unwrap(),
            currency: "EUR".to_string(),
            amount: 500.25,
        });
        assert_eq!(credit_field.to_swift_string(), ":32C:240719EUR500,25");

        let debit_field = Field32AmountCD::D(Field32D {
            value_date: NaiveDate::from_ymd_opt(2024, 7, 20).unwrap(),
            currency: "USD".to_string(),
            amount: 750.50,
        });
        assert_eq!(debit_field.to_swift_string(), ":32D:240720USD750,50");
    }

    #[test]
    fn test_field32a_c08_commodity_currency_rejection() {
        // Test that commodity currencies are rejected (C08 validation)
        assert!(Field32A::parse("240719XAU1000").is_err()); // Gold
        assert!(Field32A::parse("240719XAG500").is_err()); // Silver
        assert!(Field32A::parse("240719XPT250").is_err()); // Platinum
        assert!(Field32A::parse("240719XPD100").is_err()); // Palladium

        // Verify error message contains C08
        let err = Field32A::parse("240719XAU1000").unwrap_err();
        let err_msg = format!("{}", err);
        assert!(err_msg.contains("C08"));
    }

    #[test]
    fn test_field32a_c03_decimal_precision_validation() {
        // USD allows 2 decimals
        assert!(Field32A::parse("240719USD100.50").is_ok());
        assert!(Field32A::parse("240719USD100,50").is_ok());
        assert!(Field32A::parse("240719USD100.505").is_err()); // 3 decimals - should fail

        // JPY allows 0 decimals
        assert!(Field32A::parse("240719JPY1500000").is_ok());
        assert!(Field32A::parse("240719JPY1500000.5").is_err()); // Has decimals - should fail

        // BHD allows 3 decimals
        assert!(Field32A::parse("240719BHD100.505").is_ok());
        assert!(Field32A::parse("240719BHD100,505").is_ok());
        assert!(Field32A::parse("240719BHD100.5055").is_err()); // 4 decimals - should fail

        // Verify error message contains C03
        let err = Field32A::parse("240719USD100.505").unwrap_err();
        let err_msg = format!("{}", err);
        assert!(err_msg.contains("C03"));
    }

    #[test]
    fn test_field32a_currency_specific_formatting() {
        // Test that to_swift_string uses currency-specific decimal places
        let field_usd = Field32A {
            value_date: NaiveDate::from_ymd_opt(2024, 7, 19).unwrap(),
            currency: "USD".to_string(),
            amount: 1000.50,
        };
        assert_eq!(field_usd.to_swift_string(), ":32A:240719USD1000,50");

        let field_jpy = Field32A {
            value_date: NaiveDate::from_ymd_opt(2024, 7, 19).unwrap(),
            currency: "JPY".to_string(),
            amount: 1500000.0,
        };
        assert_eq!(field_jpy.to_swift_string(), ":32A:240719JPY1500000");

        let field_bhd = Field32A {
            value_date: NaiveDate::from_ymd_opt(2024, 7, 19).unwrap(),
            currency: "BHD".to_string(),
            amount: 123.456,
        };
        assert_eq!(field_bhd.to_swift_string(), ":32A:240719BHD123,456");
    }
}
