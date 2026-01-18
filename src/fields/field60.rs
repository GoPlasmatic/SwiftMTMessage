use super::swift_utils::{parse_amount, parse_currency, parse_date_yymmdd, parse_exact_length};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// **Field 60: Opening Balance**
///
/// Opening balance for account statements (MT 940).
///
/// **Format:** `1!a6!n3!a15d` (D/C mark + YYMMDD + currency + amount)
/// **Variants:** F (first opening balance), M (intermediate opening balance)
///
/// **Example:**
/// ```text
/// :60F:C231225USD1234,56
/// ```
///
/// **Field 60F: First Opening Balance**
///
/// Initial balance at statement start.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field60F {
    /// Debit/Credit mark (D or C)
    pub debit_credit_mark: String,

    /// Value date (YYMMDD)
    #[cfg_attr(feature = "jsonschema", schemars(with = "String"))]
    pub value_date: NaiveDate,

    /// ISO 4217 currency code
    pub currency: String,

    /// Opening balance amount
    pub amount: f64,
}

/// **Field 60M: Intermediate Opening Balance**
///
/// Balance at sequence break within statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field60M {
    /// Debit/Credit mark (D or C)
    pub debit_credit_mark: String,

    /// Value date (YYMMDD)
    #[cfg_attr(feature = "jsonschema", schemars(with = "String"))]
    pub value_date: NaiveDate,

    /// ISO 4217 currency code
    pub currency: String,

    /// Intermediate opening balance amount
    pub amount: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field60 {
    F(Field60F),
    M(Field60M),
}

impl SwiftField for Field60F {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Format: 1!a6!n3!a15d - DebitCredit + Date + Currency + Amount
        if input.len() < 10 {
            return Err(ParseError::InvalidFormat {
                message: "Field 60F must be at least 10 characters long".to_string(),
            });
        }

        // Parse debit/credit mark (1 character)
        let debit_credit_mark = parse_exact_length(&input[0..1], 1, "Field 60F debit/credit mark")?;
        if debit_credit_mark != "D" && debit_credit_mark != "C" {
            return Err(ParseError::InvalidFormat {
                message: "Field 60F debit/credit mark must be 'D' or 'C'".to_string(),
            });
        }

        // Parse value date (6 digits)
        let date_str = parse_exact_length(&input[1..7], 6, "Field 60F value date")?;
        let value_date = parse_date_yymmdd(&date_str)?;

        // Parse currency (3 characters)
        let currency = parse_exact_length(&input[7..10], 3, "Field 60F currency")?;
        let currency = parse_currency(&currency)?;

        // Parse amount (remaining characters)
        let amount_str = &input[10..];
        let amount = parse_amount(amount_str)?;

        Ok(Field60F {
            debit_credit_mark,
            value_date,
            currency,
            amount,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":60F:{}{}{}{}",
            self.debit_credit_mark,
            self.value_date.format("%y%m%d"),
            self.currency,
            format!("{:.2}", self.amount).replace('.', ",")
        )
    }
}

impl SwiftField for Field60M {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Format: 1!a6!n3!a15d - DebitCredit + Date + Currency + Amount
        if input.len() < 10 {
            return Err(ParseError::InvalidFormat {
                message: "Field 60M must be at least 10 characters long".to_string(),
            });
        }

        // Parse debit/credit mark (1 character)
        let debit_credit_mark = parse_exact_length(&input[0..1], 1, "Field 60M debit/credit mark")?;
        if debit_credit_mark != "D" && debit_credit_mark != "C" {
            return Err(ParseError::InvalidFormat {
                message: "Field 60M debit/credit mark must be 'D' or 'C'".to_string(),
            });
        }

        // Parse value date (6 digits)
        let date_str = parse_exact_length(&input[1..7], 6, "Field 60M value date")?;
        let value_date = parse_date_yymmdd(&date_str)?;

        // Parse currency (3 characters)
        let currency = parse_exact_length(&input[7..10], 3, "Field 60M currency")?;
        let currency = parse_currency(&currency)?;

        // Parse amount (remaining characters)
        let amount_str = &input[10..];
        let amount = parse_amount(amount_str)?;

        Ok(Field60M {
            debit_credit_mark,
            value_date,
            currency,
            amount,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":60M:{}{}{}{}",
            self.debit_credit_mark,
            self.value_date.format("%y%m%d"),
            self.currency,
            format!("{:.2}", self.amount).replace('.', ",")
        )
    }
}

impl SwiftField for Field60 {
    fn parse(_input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // This should not be called directly - parsing is handled by the message parser
        // which determines the variant (F or M) from the field tag
        Err(ParseError::InvalidFormat {
            message: "Field60 enum should not be parsed directly".to_string(),
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
        match variant {
            Some("F") => {
                let field = Field60F::parse(value)?;
                Ok(Field60::F(field))
            }
            Some("M") => {
                let field = Field60M::parse(value)?;
                Ok(Field60::M(field))
            }
            _ => {
                // No variant specified or unknown variant
                Err(ParseError::InvalidFormat {
                    message: "Field60 requires variant F or M".to_string(),
                })
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field60::F(field) => field.to_swift_string(),
            Field60::M(field) => field.to_swift_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_field60f_parse_valid() {
        let field = Field60F::parse("C231225USD1234,56").unwrap();
        assert_eq!(field.debit_credit_mark, "C");
        assert_eq!(
            field.value_date,
            NaiveDate::from_ymd_opt(2023, 12, 25).unwrap()
        );
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 1234.56);
    }

    #[test]
    fn test_field60m_parse_valid() {
        let field = Field60M::parse("D991231EUR500,00").unwrap();
        assert_eq!(field.debit_credit_mark, "D");
        assert_eq!(
            field.value_date,
            NaiveDate::from_ymd_opt(1999, 12, 31).unwrap()
        );
        assert_eq!(field.currency, "EUR");
        assert_eq!(field.amount, 500.00);
    }

    #[test]
    fn test_field60f_invalid_debit_credit_mark() {
        assert!(Field60F::parse("X231225USD1234,56").is_err());
    }

    #[test]
    fn test_field60f_to_swift_string() {
        let field = Field60F {
            debit_credit_mark: "C".to_string(),
            value_date: NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(),
            currency: "USD".to_string(),
            amount: 1234.56,
        };
        assert_eq!(field.to_swift_string(), ":60F:C231225USD1234,56");
    }

    #[test]
    fn test_field60_enum_to_swift_string() {
        let field_f = Field60::F(Field60F {
            debit_credit_mark: "C".to_string(),
            value_date: NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(),
            currency: "USD".to_string(),
            amount: 1234.56,
        });
        assert_eq!(field_f.to_swift_string(), ":60F:C231225USD1234,56");

        let field_m = Field60::M(Field60M {
            debit_credit_mark: "D".to_string(),
            value_date: NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(),
            currency: "EUR".to_string(),
            amount: 500.00,
        });
        assert_eq!(field_m.to_swift_string(), ":60M:D231225EUR500,00");
    }
}
