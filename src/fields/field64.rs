use super::swift_utils::{parse_amount, parse_currency, parse_date_yymmdd, parse_exact_length};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// **Field 64: Closing Available Balance**
///
/// Funds immediately available for use, accounting for holds and pending transactions.
///
/// **Format:** `1!a6!n3!a15d` (D/C mark + YYMMDD + currency + amount)
///
/// **Example:**
/// ```text
/// :64:C231225USD1234,56
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field64 {
    /// Debit/Credit mark (D or C)
    pub debit_credit_mark: String,

    /// Value date (YYMMDD)
    pub value_date: NaiveDate,

    /// ISO 4217 currency code
    pub currency: String,

    /// Available balance amount
    pub amount: f64,
}

impl SwiftField for Field64 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Format: 1!a6!n3!a15d - DebitCredit + Date + Currency + Amount
        if input.len() < 10 {
            return Err(ParseError::InvalidFormat {
                message: "Field 64 must be at least 10 characters long".to_string(),
            });
        }

        // Parse debit/credit mark (1 character)
        let debit_credit_mark = parse_exact_length(&input[0..1], 1, "Field 64 debit/credit mark")?;
        if debit_credit_mark != "D" && debit_credit_mark != "C" {
            return Err(ParseError::InvalidFormat {
                message: "Field 64 debit/credit mark must be 'D' or 'C'".to_string(),
            });
        }

        // Parse value date (6 digits)
        let date_str = parse_exact_length(&input[1..7], 6, "Field 64 value date")?;
        let value_date = parse_date_yymmdd(&date_str)?;

        // Parse currency (3 characters)
        let currency = parse_exact_length(&input[7..10], 3, "Field 64 currency")?;
        let currency = parse_currency(&currency)?;

        // Parse amount (remaining characters)
        let amount_str = &input[10..];
        let amount = parse_amount(amount_str)?;

        Ok(Field64 {
            debit_credit_mark,
            value_date,
            currency,
            amount,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":64:{}{}{}{}",
            self.debit_credit_mark,
            self.value_date.format("%y%m%d"),
            self.currency,
            format!("{:.2}", self.amount).replace('.', ",")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_field64_parse_valid() {
        let field = Field64::parse("C231225USD1234,56").unwrap();
        assert_eq!(field.debit_credit_mark, "C");
        assert_eq!(
            field.value_date,
            NaiveDate::from_ymd_opt(2023, 12, 25).unwrap()
        );
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 1234.56);
    }

    #[test]
    fn test_field64_parse_debit() {
        let field = Field64::parse("D991231EUR500,00").unwrap();
        assert_eq!(field.debit_credit_mark, "D");
        assert_eq!(
            field.value_date,
            NaiveDate::from_ymd_opt(1999, 12, 31).unwrap()
        );
        assert_eq!(field.currency, "EUR");
        assert_eq!(field.amount, 500.00);
    }

    #[test]
    fn test_field64_invalid_debit_credit_mark() {
        assert!(Field64::parse("X231225USD1234,56").is_err());
    }

    #[test]
    fn test_field64_too_short() {
        assert!(Field64::parse("C2312").is_err());
    }

    #[test]
    fn test_field64_to_swift_string() {
        let field = Field64 {
            debit_credit_mark: "C".to_string(),
            value_date: NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(),
            currency: "USD".to_string(),
            amount: 1234.56,
        };
        assert_eq!(field.to_swift_string(), ":64:C231225USD1234,56");
    }

    #[test]
    fn test_field64_debit_to_swift_string() {
        let field = Field64 {
            debit_credit_mark: "D".to_string(),
            value_date: NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(),
            currency: "EUR".to_string(),
            amount: 500.00,
        };
        assert_eq!(field.to_swift_string(), ":64:D231225EUR500,00");
    }
}
