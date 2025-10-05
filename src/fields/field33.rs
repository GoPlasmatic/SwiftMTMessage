use super::swift_utils::{
    format_swift_amount_for_currency, parse_amount_with_currency, parse_currency_non_commodity,
};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 33B: Currency/Instructed Amount**
///
/// Original instructed currency and amount before conversion or charge deductions.
/// Used when settlement amount differs from originally instructed amount.
///
/// **Format:** `3!a15d` (currency + amount, e.g., `USD1250,00`)
/// **Constraints:** Valid ISO 4217 currency, positive amount, currency-specific precision
///
/// **Example:**
/// ```text
/// :33B:USD1250,00
/// :33B:JPY125000
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field33B {
    /// ISO 4217 currency code (e.g., USD, EUR, GBP)
    pub currency: String,
    /// Original instructed amount (precision follows currency rules)
    pub amount: f64,
}

impl SwiftField for Field33B {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Field33B format: 3!a15d (currency + amount)
        if input.len() < 4 {
            // Minimum: 3 chars currency + 1 digit amount
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 33B must be at least 4 characters, found {}",
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
                message: "Field 33B amount cannot be empty".to_string(),
            });
        }

        let amount = parse_amount_with_currency(amount_str, &currency)?;

        // Amount must be positive
        if amount <= 0.0 {
            return Err(ParseError::InvalidFormat {
                message: "Field 33B amount must be greater than zero".to_string(),
            });
        }

        Ok(Field33B { currency, amount })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":33B:{}{}",
            self.currency,
            format_swift_amount_for_currency(self.amount, &self.currency)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field33b_valid() {
        let field = Field33B::parse("USD1250,00").unwrap();
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 1250.00);
        assert_eq!(field.to_swift_string(), ":33B:USD1250");

        let field = Field33B::parse("EUR950,50").unwrap();
        assert_eq!(field.currency, "EUR");
        assert_eq!(field.amount, 950.50);

        let field = Field33B::parse("JPY125000").unwrap();
        assert_eq!(field.currency, "JPY");
        assert_eq!(field.amount, 125000.0);
    }

    #[test]
    fn test_field33b_invalid() {
        // Invalid currency
        assert!(Field33B::parse("12A100").is_err());
        assert!(Field33B::parse("US100").is_err());

        // Zero amount
        assert!(Field33B::parse("USD0").is_err());

        // Negative amount
        assert!(Field33B::parse("USD-100").is_err());

        // Missing amount
        assert!(Field33B::parse("USD").is_err());
    }
}
