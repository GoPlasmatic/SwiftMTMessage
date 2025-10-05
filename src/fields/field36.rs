use super::swift_utils::parse_amount;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 36: Exchange Rate**
///
/// Specifies the exchange rate used to convert instructed currency to settlement currency.
/// Used when Field 33B currency differs from Field 32A currency.
///
/// **Format:** `12d` (decimal rate with comma separator)
/// **Constraints:** Positive rate, within reasonable market range (0.0001 to 100000)
///
/// **Example:**
/// ```text
/// :36:1,2500
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field36 {
    /// Exchange rate (from Field 33B currency to Field 32A currency)
    pub rate: f64,
}

impl SwiftField for Field36 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if input.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 36 exchange rate cannot be empty".to_string(),
            });
        }

        // Parse rate (up to 12 digits including decimal)
        if input.len() > 12 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 36 must not exceed 12 characters, found {}",
                    input.len()
                ),
            });
        }

        let rate = parse_amount(input)?;

        // Rate must be positive
        if rate <= 0.0 {
            return Err(ParseError::InvalidFormat {
                message: "Field 36 exchange rate must be greater than zero".to_string(),
            });
        }

        // Basic sanity check - exchange rate shouldn't be absurdly high or low
        // Most real-world exchange rates are between 0.0001 and 100000
        if !(0.0001..=100000.0).contains(&rate) {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 36 exchange rate {} appears to be outside reasonable range",
                    rate
                ),
            });
        }

        Ok(Field36 { rate })
    }

    fn to_swift_string(&self) -> String {
        // Format with comma as decimal separator
        format!(":36:{}", self.rate.to_string().replace('.', ","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field36_valid() {
        let field = Field36::parse("1,2500").unwrap();
        assert_eq!(field.rate, 1.25);
        assert_eq!(field.to_swift_string(), ":36:1,25");

        let field = Field36::parse("0,8500").unwrap();
        assert_eq!(field.rate, 0.85);

        let field = Field36::parse("110,2500").unwrap();
        assert_eq!(field.rate, 110.25);

        let field = Field36::parse("1").unwrap();
        assert_eq!(field.rate, 1.0);

        // Edge cases within reasonable range
        let field = Field36::parse("0,0001").unwrap();
        assert_eq!(field.rate, 0.0001);

        let field = Field36::parse("99999").unwrap();
        assert_eq!(field.rate, 99999.0);
    }

    #[test]
    fn test_field36_invalid() {
        // Empty
        assert!(Field36::parse("").is_err());

        // Too long
        assert!(Field36::parse("1234567890123").is_err());

        // Zero rate
        assert!(Field36::parse("0").is_err());
        assert!(Field36::parse("0,00").is_err());

        // Negative rate
        assert!(Field36::parse("-1,25").is_err());

        // Unreasonably small rate
        assert!(Field36::parse("0,00001").is_err());

        // Unreasonably large rate
        assert!(Field36::parse("999999").is_err());
    }
}
