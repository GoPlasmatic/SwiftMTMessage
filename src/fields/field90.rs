//! **Field 90: Number & Sum**
//!
//! Number of transactions and total sum for control and reconciliation in statements.
//!
//! **Variants:**
//! - **Field 90C**: Credit entries (number and sum)
//! - **Field 90D**: Debit entries (number and sum)
//!
//! **Format:** `5n3!a15d` (number, currency, amount)
//! **Used in:** MT 940, MT 942 (statement messages)

use super::swift_utils::{parse_amount, parse_currency, parse_swift_digits};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 90D: Number & Sum of Debit Entries**
///
/// Number and total sum of debit transactions for control purposes.
///
/// **Format:** `5n3!a15d` (number, currency, amount)
///
/// **Example:**
/// ```text
/// :90D:2GBP250050,00
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field90D {
    /// Number of debit transactions (max 5 digits)
    pub number: u32,

    /// Currency code (ISO 4217)
    pub currency: String,

    /// Total sum of debit amounts
    pub amount: f64,
}

impl SwiftField for Field90D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut remaining = input;

        // Parse number of transactions (5n)
        if remaining.len() < 8 {
            return Err(ParseError::InvalidFormat {
                message: "Field90D requires at least 8 characters (5n + 3!a)".to_string(),
            });
        }

        // Find where number ends by looking for non-digit
        let mut number_end = 0;
        for (i, c) in remaining.char_indices() {
            if !c.is_ascii_digit() {
                number_end = i;
                break;
            }
            if i >= 4 {
                // Max 5 digits
                number_end = i + 1;
                break;
            }
        }

        if number_end == 0 {
            return Err(ParseError::InvalidFormat {
                message: "Field90D number part not found".to_string(),
            });
        }

        let number_str = &remaining[..number_end];
        if number_str.len() > 5 {
            return Err(ParseError::InvalidFormat {
                message: "Field90D number cannot exceed 5 digits".to_string(),
            });
        }

        parse_swift_digits(number_str, "Field90D number")?;
        let number: u32 = number_str.parse().map_err(|_| ParseError::InvalidFormat {
            message: "Invalid number in Field90D".to_string(),
        })?;

        remaining = &remaining[number_end..];

        // Parse currency (3!a)
        if remaining.len() < 3 {
            return Err(ParseError::InvalidFormat {
                message: "Field90D requires currency code".to_string(),
            });
        }

        let currency = parse_currency(&remaining[..3])?;
        remaining = &remaining[3..];

        // Parse amount (15d)
        if remaining.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field90D requires amount".to_string(),
            });
        }

        let amount = parse_amount(remaining)?;

        Ok(Field90D {
            number,
            currency,
            amount,
        })
    }

    fn to_swift_string(&self) -> String {
        let amount_str = format!("{:.2}", self.amount).replace('.', ",");
        format!(":90D:{}{}{}", self.number, self.currency, amount_str)
    }
}

/// **Field 90C: Number & Sum of Credit Entries**
///
/// Number and total sum of credit transactions for control purposes.
///
/// **Format:** `5n3!a15d` (number, currency, amount)
///
/// **Example:**
/// ```text
/// :90C:5USD12500,50
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field90C {
    /// Number of credit transactions (max 5 digits)
    pub number: u32,

    /// Currency code (ISO 4217)
    pub currency: String,

    /// Total sum of credit amounts
    pub amount: f64,
}

impl SwiftField for Field90C {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut remaining = input;

        // Parse number of transactions (5n)
        if remaining.len() < 8 {
            return Err(ParseError::InvalidFormat {
                message: "Field90C requires at least 8 characters (5n + 3!a)".to_string(),
            });
        }

        // Find where number ends by looking for non-digit
        let mut number_end = 0;
        for (i, c) in remaining.char_indices() {
            if !c.is_ascii_digit() {
                number_end = i;
                break;
            }
            if i >= 4 {
                // Max 5 digits
                number_end = i + 1;
                break;
            }
        }

        if number_end == 0 {
            return Err(ParseError::InvalidFormat {
                message: "Field90C number part not found".to_string(),
            });
        }

        let number_str = &remaining[..number_end];
        if number_str.len() > 5 {
            return Err(ParseError::InvalidFormat {
                message: "Field90C number cannot exceed 5 digits".to_string(),
            });
        }

        parse_swift_digits(number_str, "Field90C number")?;
        let number: u32 = number_str.parse().map_err(|_| ParseError::InvalidFormat {
            message: "Invalid number in Field90C".to_string(),
        })?;

        remaining = &remaining[number_end..];

        // Parse currency (3!a)
        if remaining.len() < 3 {
            return Err(ParseError::InvalidFormat {
                message: "Field90C requires currency code".to_string(),
            });
        }

        let currency = parse_currency(&remaining[..3])?;
        remaining = &remaining[3..];

        // Parse amount (15d)
        if remaining.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field90C requires amount".to_string(),
            });
        }

        let amount = parse_amount(remaining)?;

        Ok(Field90C {
            number,
            currency,
            amount,
        })
    }

    fn to_swift_string(&self) -> String {
        let amount_str = format!("{:.2}", self.amount).replace('.', ",");
        format!(":90C:{}{}{}", self.number, self.currency, amount_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field90d_parsing_basic() {
        let value = "2GBP250050";
        match Field90D::parse(value) {
            Ok(field) => {
                assert_eq!(field.number, 2);
                assert_eq!(field.currency, "GBP");
                assert_eq!(field.amount, 250050.0);
            }
            Err(e) => {
                panic!("Failed to parse Field90D '{}': {:?}", value, e);
            }
        }
    }
}
