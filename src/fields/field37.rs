use super::swift_utils::parse_amount;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 37H: Interest Rate**
///
/// Specifies interest rates for financial instruments, typically expressed as percentage.
/// Supports negative rates for low interest rate environments.
///
/// **Format:** `1!a[N]12d` (indicator + optional N for negative + rate)
/// **Constraints:** C (Credit) or D (Debit) indicator, rate with comma separator
///
/// **Example:**
/// ```text
/// :37H:C2,5000
/// :37H:CN0,2500
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field37H {
    /// Rate type indicator: 'C' (Credit) or 'D' (Debit)
    pub rate_indicator: char,

    /// Negative rate indicator (Some(true) if rate is negative)
    pub is_negative: Option<bool>,

    /// Interest rate value (e.g., 2.5000 = 2.5%)
    pub rate: f64,
}

impl SwiftField for Field37H {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut remaining = input;

        // Parse rate indicator (1!a)
        if remaining.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field37H requires rate indicator".to_string(),
            });
        }

        let rate_indicator = remaining.chars().next().unwrap();
        if rate_indicator != 'C' && rate_indicator != 'D' {
            return Err(ParseError::InvalidFormat {
                message: "Field37H rate indicator must be 'C' or 'D'".to_string(),
            });
        }
        remaining = &remaining[1..];

        // Parse optional negative indicator ([1!a])
        let is_negative = if remaining.starts_with('N') {
            remaining = &remaining[1..];
            Some(true)
        } else {
            None
        };

        // Parse rate value (12d)
        if remaining.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field37H requires rate value".to_string(),
            });
        }

        let rate = if is_negative.is_some() {
            -parse_amount(remaining)?
        } else {
            parse_amount(remaining)?
        };

        Ok(Field37H {
            rate_indicator,
            is_negative,
            rate,
        })
    }

    fn to_swift_string(&self) -> String {
        let negative_indicator = if self.is_negative.is_some() { "N" } else { "" };
        let rate_str = format!("{:.4}", self.rate.abs()).replace('.', ",");
        format!(
            ":37H:{}{}{}",
            self.rate_indicator, negative_indicator, rate_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field37h_parse() {
        // Test positive credit rate
        let field = Field37H::parse("C2,5000").unwrap();
        assert_eq!(field.rate_indicator, 'C');
        assert_eq!(field.is_negative, None);
        assert_eq!(field.rate, 2.5);

        // Test positive debit rate
        let field = Field37H::parse("D3,7500").unwrap();
        assert_eq!(field.rate_indicator, 'D');
        assert_eq!(field.is_negative, None);
        assert_eq!(field.rate, 3.75);

        // Test negative credit rate
        let field = Field37H::parse("CN0,2500").unwrap();
        assert_eq!(field.rate_indicator, 'C');
        assert_eq!(field.is_negative, Some(true));
        assert_eq!(field.rate, -0.25);
    }

    #[test]
    fn test_field37h_to_swift_string() {
        let field = Field37H {
            rate_indicator: 'C',
            is_negative: None,
            rate: 2.5,
        };
        assert_eq!(field.to_swift_string(), ":37H:C2,5000");

        let field = Field37H {
            rate_indicator: 'D',
            is_negative: None,
            rate: 3.75,
        };
        assert_eq!(field.to_swift_string(), ":37H:D3,7500");

        let field = Field37H {
            rate_indicator: 'C',
            is_negative: Some(true),
            rate: -0.25,
        };
        assert_eq!(field.to_swift_string(), ":37H:CN0,2500");
    }

    #[test]
    fn test_field37h_parse_invalid() {
        // Invalid rate indicator
        assert!(Field37H::parse("X2,5000").is_err());

        // Missing rate
        assert!(Field37H::parse("C").is_err());

        // Invalid rate format
        assert!(Field37H::parse("Cabc").is_err());

        // Empty input
        assert!(Field37H::parse("").is_err());
    }
}
