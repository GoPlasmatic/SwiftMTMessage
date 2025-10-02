//! # Field 32: Value Date, Currency Code, Amount
//!
//! ## Purpose
//! Specifies the value date, currency, and settlement amount for payment instructions.
//! This is the core monetary field that defines when and how much will be transferred,
//! serving as the foundation for all payment processing and settlement calculations.
//!
//! ## Options Overview
//! - **Option A**: Value Date + Currency + Amount (complete settlement information)
//! - **Option B**: Currency + Amount (amount without specific value date)
//!
//! ## Format Specifications
//! ### Option A Format
//! - **Swift Format**: `6!n3!a15d`
//! - **Components**:
//!   - `6!n`: Value date (YYMMDD format)
//!   - `3!a`: Currency code (ISO 4217, 3 alphabetic characters)
//!   - `15d`: Amount (up to 15 digits including decimal, comma as decimal separator)
//!
//! ### Option B Format
//! - **Swift Format**: `3!a15d`
//! - **Components**:
//!   - `3!a`: Currency code (ISO 4217, 3 alphabetic characters)
//!   - `15d`: Amount (up to 15 digits including decimal, comma as decimal separator)
//!
//! ## Presence and Usage
//! - **Status**: Mandatory in all payment messages (MT103, MT202, etc.)
//! - **Swift Error Codes**: T40 (invalid date), T52 (invalid currency), T51 (invalid amount)
//! - **Referenced in Rules**: C1, C7, C8, C9 (MT103), currency validation across message types
//!
//! ## Value Date Rules (Option A)
//! - **Format**: YYMMDD (2-digit year, month, day)
//! - **Validation**: Must be a valid calendar date
//! - **Business Rules**: Cannot be more than 1 year in the past or future (typical limit)
//! - **Weekends/Holidays**: System may adjust for banking days depending on currency
//!
//! ## Currency Code Rules
//! - **Standard**: ISO 4217 three-letter currency codes
//! - **Validation**: Must be an active, tradeable currency
//! - **Examples**: USD, EUR, GBP, JPY, CHF, CAD, AUD
//! - **Restrictions**: Some currencies may be restricted for certain corridors
//!
//! ## Amount Rules
//! - **Format**: Up to 15 digits with decimal precision
//! - **Decimal Separator**: Comma (,) for decimal values in Swift format
//! - **Precision**: Typically 2 decimal places, varies by currency (JPY has 0, BHD has 3)
//! - **Range**: Must be positive (> 0), maximum depends on currency and institution limits
//!
//! ## Network Validation Rules
//! - **C1 (MT103)**: If field 33B differs from 32A currency, field 36 (Exchange Rate) required
//! - **C7**: Amount must be positive and properly formatted for currency
//! - **C8**: If charges apply (71F/71G), 33B becomes mandatory for charge calculations
//! - **C9**: Currency in 71G must match 32A currency for charge consistency
//!
//! ## Usage Guidelines
//! - **Settlement**: This amount determines the final settlement obligation
//! - **Exchange Rates**: When currency differs from instructed amount (33B), exchange rate (36) needed
//! - **Charges**: Original instructed amount before any fee deductions
//! - **Precision**: Must respect currency-specific decimal precision rules
//!
//! ## STP Compliance
//! - **Amount Format**: Must comply with STP formatting standards (no trailing zeros)
//! - **Currency Support**: STP corridors may support limited currency pairs
//! - **Validation**: Enhanced validation for STP messages to prevent manual intervention
//!
//! ## Regional Considerations
//! - **SEPA**: EUR payments within SEPA zone have specific amount and date rules
//! - **US Domestic**: USD payments may require different value date handling
//! - **Emerging Markets**: Some currencies have additional restrictions or validations
//!
//! ## Examples
//! ```text
//! :32A:240719EUR1250,50     // July 19, 2024, EUR 1,250.50
//! :32A:240720USD10000,00    // July 20, 2024, USD 10,000.00
//! :32A:240721JPY1500000     // July 21, 2024, JPY 1,500,000 (no decimal)
//! :32B:EUR5000,00          // EUR 5,000.00 (no value date)
//! ```
//!
//! ## Related Fields Integration
//! - **Field 33B**: Instructed Amount (if different from settlement amount)
//! - **Field 36**: Exchange Rate (when 33B currency differs from 32A)
//! - **Field 71F/71G**: Sender's/Receiver's Charges (affect final settlement)
//! - **Field 30**: Execution Date (in some message types)
//!
//! ## Error Prevention
//! - **Invalid Date**: T40 error if date is malformed or unrealistic
//! - **Invalid Currency**: T52 error if currency code not recognized
//! - **Invalid Amount**: T51 error if amount format incorrect or negative
//! - **Business Rule**: C-rule violations if currency/amount conflicts with other fields
//!
//! ## Amount Precision by Currency
//! - **Most Currencies**: 2 decimal places (USD, EUR, GBP, etc.)
//! - **Japanese Yen**: 0 decimal places (JPY)
//! - **Bahraini Dinar**: 3 decimal places (BHD)
//! - **Cryptocurrency**: Variable precision (check current standards)
//!
//! ## See Also
//! - Swift FIN User Handbook: Currency and Amount Specifications
//! - ISO 4217: Currency Code Standard
//! - MT103 Usage Rules: Value Date and Settlement Guidelines
//! - STP Implementation Guide: Amount Format Requirements

use super::swift_utils::{parse_amount, parse_currency, parse_date_yymmdd};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// **Field 32A: Value Date, Currency Code, Amount**
///
/// Complete settlement information variant of [Field 32 module](index.html). Specifies the value date,
/// currency, and settlement amount for payment instructions.
///
/// **Components:**
/// - Value date (6!n, YYMMDD format)
/// - Currency code (3!a, ISO 4217)
/// - Amount (15d, decimal with comma separator)
///
/// For complete documentation, see the [Field 32 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field32A {
    /// Value date when the payment becomes effective
    ///
    /// Format: 6!n (YYMMDD) - Must be valid calendar date
    /// Business rule: Typically within 1 year of today
    #[serde(with = "date_string")]
    pub value_date: NaiveDate,

    /// ISO 4217 three-letter currency code
    ///
    /// Format: 3!a - Must be valid, active currency
    /// Examples: USD, EUR, GBP, JPY, CHF
    pub currency: String,

    /// Settlement amount in the specified currency
    ///
    /// Format: 15d - Up to 15 digits, comma decimal separator
    /// Must be positive, respect currency precision rules
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

        // Parse value date (first 6 characters)
        let value_date = parse_date_yymmdd(&input[0..6])?;

        // Parse currency code (next 3 characters)
        let currency = parse_currency(&input[6..9])?;

        // Parse amount (remaining characters)
        let amount_str = &input[9..];
        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 32A amount cannot be empty".to_string(),
            });
        }

        let amount = parse_amount(amount_str)?;

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
            "{}{}{}",
            self.value_date.format("%y%m%d"),
            self.currency,
            self.amount.to_string().replace('.', ",")
        )
    }
}

/// **Field 32B: Currency Code, Amount**
///
/// Currency and amount variant of [Field 32 module](index.html). Specifies currency and amount
/// without a specific value date.
///
/// **Components:**
/// - Currency code (3!a, ISO 4217)
/// - Amount (15d, decimal with comma separator)
///
/// For complete documentation, see the [Field 32 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field32B {
    /// Currency code (ISO 4217)
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

        // Parse currency code (first 3 characters)
        let currency = parse_currency(&input[0..3])?;

        // Parse amount (remaining characters)
        let amount_str = &input[3..];
        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 32B amount cannot be empty".to_string(),
            });
        }

        let amount = parse_amount(amount_str)?;

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
            self.amount.to_string().replace('.', ",")
        )
    }
}

/// **Field 32C: Value Date, Currency Code, Amount (Credit)**
///
/// Credit variant of [Field 32 module](index.html). Specifies the value date,
/// currency, and amount credited. Used in MT n90 messages (MT190, MT290, etc.)
/// to indicate credit adjustments.
///
/// **Components:**
/// - Value date (6!n, YYMMDD format)
/// - Currency code (3!a, ISO 4217)
/// - Amount (15d, decimal with comma separator)
///
/// For complete documentation, see the [Field 32 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field32C {
    /// Value date when the credit becomes effective
    ///
    /// Format: 6!n (YYMMDD) - Must be valid calendar date
    #[serde(with = "date_string")]
    pub value_date: NaiveDate,

    /// ISO 4217 three-letter currency code
    ///
    /// Format: 3!a - Must be valid, active currency
    pub currency: String,

    /// Credit amount in the specified currency
    ///
    /// Format: 15d - Up to 15 digits, comma decimal separator
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
        NaiveDate::parse_from_str(&s, "%Y-%m-%d")
            .map_err(serde::de::Error::custom)
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
        let currency = parse_currency(&input[6..9])?;
        let amount_str = &input[9..];

        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 32C amount cannot be empty".to_string(),
            });
        }

        let amount = parse_amount(amount_str)?;

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
            "{}{}{}",
            self.value_date.format("%y%m%d"),
            self.currency,
            self.amount.to_string().replace('.', ",")
        )
    }
}

/// **Field 32D: Value Date, Currency Code, Amount (Debit)**
///
/// Debit variant of [Field 32 module](index.html). Specifies the value date,
/// currency, and amount debited. Used in MT n90 messages (MT190, MT290, etc.)
/// to indicate debit adjustments.
///
/// **Components:**
/// - Value date (6!n, YYMMDD format)
/// - Currency code (3!a, ISO 4217)
/// - Amount (15d, decimal with comma separator)
///
/// For complete documentation, see the [Field 32 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field32D {
    /// Value date when the debit becomes effective
    ///
    /// Format: 6!n (YYMMDD) - Must be valid calendar date
    #[serde(with = "date_string")]
    pub value_date: NaiveDate,

    /// ISO 4217 three-letter currency code
    ///
    /// Format: 3!a - Must be valid, active currency
    pub currency: String,

    /// Debit amount in the specified currency
    ///
    /// Format: 15d - Up to 15 digits, comma decimal separator
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
        let currency = parse_currency(&input[6..9])?;
        let amount_str = &input[9..];

        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 32D amount cannot be empty".to_string(),
            });
        }

        let amount = parse_amount(amount_str)?;

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
            "{}{}{}",
            self.value_date.format("%y%m%d"),
            self.currency,
            self.amount.to_string().replace('.', ",")
        )
    }
}

/// **Field 32 Enum: Value Date, Currency, Amount Variants**
///
/// Enum wrapper for [Field 32 module](index.html) variants providing different
/// levels of settlement information detail.
///
/// For complete documentation, see the [Field 32 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field32 {
    A(Field32A),
    B(Field32B),
    C(Field32C),
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

/// **Field32AmountCD: Credit or Debit Amount**
///
/// Used in MT290 and similar messages to specify either a credit (32C) or debit (32D) amount.
/// This enum provides JSON flattening to directly serialize as "32C" or "32D" fields.
///
/// **Variants:**
/// - C: Credit amount with value date (Field32C)
/// - D: Debit amount with value date (Field32D)
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

    fn to_swift_string(&self) -> String {
        match self {
            Field32AmountCD::C(field) => format!(":32C:{}", field.to_swift_string()),
            Field32AmountCD::D(field) => format!(":32D:{}", field.to_swift_string()),
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
        assert_eq!(field.to_swift_string(), "240719EUR1250.5".replace('.', ","));

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
        assert_eq!(field.to_swift_string(), ":32B:EUR5000");

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
    fn test_field32_amount_cd() {
        // Test parsing as credit (32C)
        let field = Field32AmountCD::parse("240719EUR500,25").unwrap();
        match field {
            Field32AmountCD::C(f) => {
                assert_eq!(
                    f.value_date,
                    NaiveDate::from_ymd_opt(2024, 7, 19).unwrap()
                );
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
                assert_eq!(
                    f.value_date,
                    NaiveDate::from_ymd_opt(2024, 7, 20).unwrap()
                );
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
        assert_eq!(debit_field.to_swift_string(), ":32D:240720USD750,5");
    }
}
