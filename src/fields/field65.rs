use super::swift_utils::{parse_amount, parse_currency, parse_date_yymmdd, parse_exact_length};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// **Field 65: Forward Available Balance**
///
/// Funds that will be available on a future value date.
///
/// **Format:** `1!a6!n3!a15d` (D/C mark + YYMMDD + currency + amount)
///
/// **Example:**
/// ```text
/// :65:C240115USD2500,75
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field65 {
    /// Debit/Credit mark (D or C)
    pub debit_credit_mark: String,

    /// Future value date (YYMMDD)
    #[cfg_attr(feature = "jsonschema", schemars(with = "String"))]
    pub value_date: NaiveDate,

    /// ISO 4217 currency code
    pub currency: String,

    /// Forward available balance amount
    pub amount: f64,
}

impl SwiftField for Field65 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Format: 1!a6!n3!a15d - DebitCredit + Date + Currency + Amount
        if input.len() < 10 {
            return Err(ParseError::InvalidFormat {
                message: "Field 65 must be at least 10 characters long".to_string(),
            });
        }

        // Parse debit/credit mark (1 character)
        let debit_credit_mark = parse_exact_length(&input[0..1], 1, "Field 65 debit/credit mark")?;
        if debit_credit_mark != "D" && debit_credit_mark != "C" {
            return Err(ParseError::InvalidFormat {
                message: "Field 65 debit/credit mark must be 'D' or 'C'".to_string(),
            });
        }

        // Parse value date (6 digits)
        let date_str = parse_exact_length(&input[1..7], 6, "Field 65 value date")?;
        let value_date = parse_date_yymmdd(&date_str)?;

        // Parse currency (3 characters)
        let currency = parse_exact_length(&input[7..10], 3, "Field 65 currency")?;
        let currency = parse_currency(&currency)?;

        // Parse amount (remaining characters)
        let amount_str = &input[10..];
        let amount = parse_amount(amount_str)?;

        Ok(Field65 {
            debit_credit_mark,
            value_date,
            currency,
            amount,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":65:{}{}{}{}",
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
    fn test_field65_parse_valid() {
        let field = Field65::parse("C231225USD1234,56").unwrap();
        assert_eq!(field.debit_credit_mark, "C");
        assert_eq!(
            field.value_date,
            NaiveDate::from_ymd_opt(2023, 12, 25).unwrap()
        );
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 1234.56);
    }

    #[test]
    fn test_field65_parse_debit() {
        let field = Field65::parse("D240101EUR750,00").unwrap();
        assert_eq!(field.debit_credit_mark, "D");
        assert_eq!(
            field.value_date,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
        );
        assert_eq!(field.currency, "EUR");
        assert_eq!(field.amount, 750.00);
    }

    #[test]
    fn test_field65_invalid_debit_credit_mark() {
        assert!(Field65::parse("X231225USD1234,56").is_err());
    }

    #[test]
    fn test_field65_too_short() {
        assert!(Field65::parse("C2312").is_err());
    }

    #[test]
    fn test_field65_to_swift_string() {
        let field = Field65 {
            debit_credit_mark: "C".to_string(),
            value_date: NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(),
            currency: "USD".to_string(),
            amount: 1234.56,
        };
        assert_eq!(field.to_swift_string(), ":65:C231225USD1234,56");
    }

    #[test]
    fn test_field65_future_date_to_swift_string() {
        let field = Field65 {
            debit_credit_mark: "D".to_string(),
            value_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency: "GBP".to_string(),
            amount: 2500.75,
        };
        assert_eq!(field.to_swift_string(), ":65:D240115GBP2500,75");
    }
}
