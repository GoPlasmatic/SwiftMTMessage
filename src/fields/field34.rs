use super::swift_utils::{parse_amount, parse_currency};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 34F: Floor Limit**
///
/// Specifies floor limit amount and currency for automatic processing thresholds.
///
/// **Format:** `3!a[1!a]15d` (currency + optional D/C indicator + amount)
/// **Constraints:** Valid ISO 4217 currency, positive amount, indicator must be D (Debit) or C (Credit)
///
/// **Example:**
/// ```text
/// :34F:USD5000,00
/// :34F:USDD2500,00
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field34F {
    /// ISO 4217 currency code (e.g., USD, EUR, GBP)
    pub currency: String,

    /// Optional indicator: 'D' (Debit) or 'C' (Credit)
    pub indicator: Option<char>,

    /// Floor limit amount (positive)
    pub amount: f64,
}

impl SwiftField for Field34F {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Field34F format: 3!a[1!a]15d (currency + optional indicator + amount)
        if input.len() < 4 {
            // Minimum: 3 chars currency + 1 digit amount
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 34F must be at least 4 characters, found {}",
                    input.len()
                ),
            });
        }

        // Parse currency code (first 3 characters)
        let currency = parse_currency(&input[0..3])?;

        // Check for optional indicator (4th character might be D or C)
        let (indicator, amount_start) = if input.len() > 3 {
            let fourth_char = input.chars().nth(3).unwrap();
            if fourth_char == 'D' || fourth_char == 'C' {
                (Some(fourth_char), 4)
            } else {
                (None, 3)
            }
        } else {
            (None, 3)
        };

        // Parse amount (remaining characters)
        let amount_str = &input[amount_start..];
        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 34F amount cannot be empty".to_string(),
            });
        }

        let amount = parse_amount(amount_str)?;

        // Amount must be positive
        if amount <= 0.0 {
            return Err(ParseError::InvalidFormat {
                message: "Field 34F amount must be greater than zero".to_string(),
            });
        }

        Ok(Field34F {
            currency,
            indicator,
            amount,
        })
    }

    fn to_swift_string(&self) -> String {
        let indicator_str = self.indicator.map_or(String::new(), |c| c.to_string());
        format!(
            ":34F:{}{}{}",
            self.currency,
            indicator_str,
            super::swift_utils::format_swift_amount(self.amount, 2)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field34f_valid() {
        // Without indicator
        let field = Field34F::parse("USD5000,00").unwrap();
        assert_eq!(field.currency, "USD");
        assert_eq!(field.indicator, None);
        assert_eq!(field.amount, 5000.00);
        assert_eq!(field.to_swift_string(), ":34F:USD5000,00");

        // With D indicator
        let field = Field34F::parse("USDD2500,00").unwrap();
        assert_eq!(field.currency, "USD");
        assert_eq!(field.indicator, Some('D'));
        assert_eq!(field.amount, 2500.00);

        // With C indicator
        let field = Field34F::parse("EURC1000,00").unwrap();
        assert_eq!(field.currency, "EUR");
        assert_eq!(field.indicator, Some('C'));
        assert_eq!(field.amount, 1000.00);

        // Large amount without indicator
        let field = Field34F::parse("GBP10000,00").unwrap();
        assert_eq!(field.currency, "GBP");
        assert_eq!(field.indicator, None);
        assert_eq!(field.amount, 10000.00);
    }

    #[test]
    fn test_field34f_invalid() {
        // Invalid currency
        assert!(Field34F::parse("12A100").is_err());
        assert!(Field34F::parse("US100").is_err());

        // Invalid indicator
        assert!(Field34F::parse("USDX100").is_err());

        // Zero amount
        assert!(Field34F::parse("USD0").is_err());
        assert!(Field34F::parse("USDD0").is_err());

        // Negative amount
        assert!(Field34F::parse("USD-100").is_err());

        // Missing amount
        assert!(Field34F::parse("USD").is_err());
        assert!(Field34F::parse("USDD").is_err());
    }
}
