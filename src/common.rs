//! Common structures and field definitions for SWIFT MT messages

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{MTError, Result};

/// Represents a SWIFT message tag
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tag(pub String);

impl Tag {
    pub fn new(tag: impl Into<String>) -> Self {
        Self(tag.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Tag {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Tag {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Represents a field in a SWIFT message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub tag: Tag,
    pub value: String,
    pub raw_value: String, // Original value before any processing
}

impl Field {
    pub fn new(tag: impl Into<Tag>, value: impl Into<String>) -> Self {
        let value = value.into();
        Self {
            tag: tag.into(),
            raw_value: value.clone(),
            value,
        }
    }

    /// Get the field value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the raw field value (before processing)
    pub fn raw_value(&self) -> &str {
        &self.raw_value
    }

    /// Get the field tag
    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    /// Check if field is empty
    pub fn is_empty(&self) -> bool {
        self.value.trim().is_empty()
    }
}

/// SWIFT message blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageBlock {
    /// Block 1: Basic Header Block
    BasicHeader {
        application_id: String,
        service_id: String,
        logical_terminal: String,
        session_number: String,
        sequence_number: String,
    },
    /// Block 2: Application Header Block
    ApplicationHeader {
        input_output_identifier: String,
        message_type: String,
        destination_address: String,
        priority: String,
        delivery_monitoring: Option<String>,
        obsolescence_period: Option<String>,
    },
    /// Block 3: User Header Block (optional)
    UserHeader { fields: HashMap<String, String> },
    /// Block 4: Text Block
    TextBlock { fields: Vec<Field> },
    /// Block 5: Trailer Block (optional)
    TrailerBlock { fields: HashMap<String, String> },
}

/// Common field types used across MT messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub value: f64,
    pub currency: String,
    pub raw: String,
}

impl Amount {
    pub fn parse(input: &str) -> Result<Self> {
        // Parse amount in format like "EUR1234567,89" or "USD1000.00"
        if input.len() < 4 {
            return Err(MTError::AmountParseError {
                message: "Amount string too short".to_string(),
            });
        }

        let currency = &input[0..3];
        let amount_str = &input[3..];

        // Handle both comma and dot as decimal separators
        let normalized_amount = amount_str.replace(',', ".");

        let value = normalized_amount
            .parse::<f64>()
            .map_err(|_| MTError::AmountParseError {
                message: format!("Invalid amount format: {}", amount_str),
            })?;

        Ok(Amount {
            value,
            currency: currency.to_string(),
            raw: input.to_string(),
        })
    }
}

/// Date field representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwiftDate {
    pub date: NaiveDate,
    pub raw: String,
}

impl SwiftDate {
    /// Parse SWIFT date format (YYMMDD)
    pub fn parse_yymmdd(input: &str) -> Result<Self> {
        if input.len() != 6 {
            return Err(MTError::DateParseError {
                message: format!("Invalid date format, expected YYMMDD, got: {}", input),
            });
        }

        let year: i32 = input[0..2].parse().map_err(|_| MTError::DateParseError {
            message: format!("Invalid year in date: {}", input),
        })?;

        let month: u32 = input[2..4].parse().map_err(|_| MTError::DateParseError {
            message: format!("Invalid month in date: {}", input),
        })?;

        let day: u32 = input[4..6].parse().map_err(|_| MTError::DateParseError {
            message: format!("Invalid day in date: {}", input),
        })?;

        // Handle Y2K: assume 00-49 is 20xx, 50-99 is 19xx
        let full_year = if year <= 49 { 2000 + year } else { 1900 + year };

        let date = NaiveDate::from_ymd_opt(full_year, month, day).ok_or_else(|| {
            MTError::DateParseError {
                message: format!("Invalid date: {}-{:02}-{:02}", full_year, month, day),
            }
        })?;

        Ok(SwiftDate {
            date,
            raw: input.to_string(),
        })
    }

    /// Parse SWIFT date format (YYYYMMDD)
    pub fn parse_yyyymmdd(input: &str) -> Result<Self> {
        if input.len() != 8 {
            return Err(MTError::DateParseError {
                message: format!("Invalid date format, expected YYYYMMDD, got: {}", input),
            });
        }

        let year: i32 = input[0..4].parse().map_err(|_| MTError::DateParseError {
            message: format!("Invalid year in date: {}", input),
        })?;

        let month: u32 = input[4..6].parse().map_err(|_| MTError::DateParseError {
            message: format!("Invalid month in date: {}", input),
        })?;

        let day: u32 = input[6..8].parse().map_err(|_| MTError::DateParseError {
            message: format!("Invalid day in date: {}", input),
        })?;

        let date =
            NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| MTError::DateParseError {
                message: format!("Invalid date: {}-{:02}-{:02}", year, month, day),
            })?;

        Ok(SwiftDate {
            date,
            raw: input.to_string(),
        })
    }
}

/// Time field representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwiftTime {
    pub time: NaiveDateTime,
    pub raw: String,
}

/// Currency code validation
pub fn validate_currency_code(code: &str) -> Result<()> {
    if code.len() != 3 {
        return Err(MTError::CurrencyError {
            message: format!("Currency code must be 3 characters, got: {}", code),
        });
    }

    if !code.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(MTError::CurrencyError {
            message: format!("Currency code must be uppercase letters, got: {}", code),
        });
    }

    Ok(())
}

/// Common SWIFT field tags
pub mod tags {
    pub const SENDER_REFERENCE: &str = "20";
    pub const BANK_OPERATION_CODE: &str = "23B";
    pub const VALUE_DATE_CURRENCY_AMOUNT: &str = "32A";
    pub const ORDERING_CUSTOMER: &str = "50K";
    pub const ORDERING_INSTITUTION: &str = "52A";
    pub const SENDERS_CORRESPONDENT: &str = "53A";
    pub const RECEIVERS_CORRESPONDENT: &str = "54A";
    pub const THIRD_REIMBURSEMENT_INSTITUTION: &str = "55A";
    pub const INTERMEDIARY_INSTITUTION: &str = "56A";
    pub const ACCOUNT_WITH_INSTITUTION: &str = "57A";
    pub const BENEFICIARY_CUSTOMER: &str = "59";
    pub const REMITTANCE_INFORMATION: &str = "70";
    pub const DETAILS_OF_CHARGES: &str = "71A";
    pub const SENDERS_CHARGES: &str = "71F";
    pub const RECEIVERS_CHARGES: &str = "71G";

    // MT940 specific tags
    pub const TRANSACTION_REFERENCE: &str = "20";
    pub const ACCOUNT_IDENTIFICATION: &str = "25";
    pub const STATEMENT_NUMBER: &str = "28C";
    pub const OPENING_BALANCE: &str = "60F";
    pub const STATEMENT_LINE: &str = "61";
    pub const INFORMATION_TO_ACCOUNT_OWNER: &str = "86";
    pub const CLOSING_BALANCE: &str = "62F";
    pub const CLOSING_AVAILABLE_BALANCE: &str = "64";
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_amount_parsing() {
        let amount = Amount::parse("EUR1234567,89").unwrap();
        assert_eq!(amount.currency, "EUR");
        assert_eq!(amount.value, 1234567.89);

        let amount = Amount::parse("USD1000.50").unwrap();
        assert_eq!(amount.currency, "USD");
        assert_eq!(amount.value, 1000.50);
    }

    #[test]
    fn test_date_parsing() {
        let date = SwiftDate::parse_yymmdd("210315").unwrap();
        assert_eq!(date.date.year(), 2021);
        assert_eq!(date.date.month(), 3);
        assert_eq!(date.date.day(), 15);

        let date = SwiftDate::parse_yymmdd("991231").unwrap();
        assert_eq!(date.date.year(), 1999);
        assert_eq!(date.date.month(), 12);
        assert_eq!(date.date.day(), 31);
    }

    #[test]
    fn test_currency_validation() {
        assert!(validate_currency_code("EUR").is_ok());
        assert!(validate_currency_code("USD").is_ok());
        assert!(validate_currency_code("eur").is_err());
        assert!(validate_currency_code("EURO").is_err());
        assert!(validate_currency_code("EU").is_err());
    }
}
