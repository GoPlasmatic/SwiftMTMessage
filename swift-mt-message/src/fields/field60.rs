use super::swift_utils::{parse_amount, parse_currency, parse_date_yymmdd, parse_exact_length};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

///   **Field 60: Opening Balance**
///
/// ## Purpose
/// Specifies the opening balance of an account in customer statement messages (MT 940)
/// and other cash management contexts. This field establishes the starting position for
/// account balance calculations and provides the foundation for statement processing
/// and account reconciliation. Essential for cash management and account monitoring.
///
/// ## Format Options Overview
/// - **Option F**: First opening balance - initial balance at statement start
/// - **Option M**: Intermediate opening balance - balance after sequence breaks
///
/// ## Business Context Applications
/// - **Customer Statements**: Opening balance for MT 940 Customer Statement Message
/// - **Cash Management**: Starting position for balance calculations
/// - **Account Reconciliation**: Foundation for account balance verification
/// - **Sequence Processing**: Balance continuation across statement sequences
///
/// ## Network Validation Requirements
/// - **Date Validation**: Value date must be valid calendar date
/// - **Currency Validation**: Must be valid ISO 4217 currency code
/// - **Amount Format**: Decimal amount with proper precision
/// - **Mark Validation**: Debit/Credit mark must be D (Debit) or C (Credit)
///
/// ## Balance Calculation Context
/// ### Opening Balance Logic
/// - **First Balance (F)**: Initial balance at beginning of statement period
/// - **Intermediate Balance (M)**: Balance at sequence break within statement
/// - **Continuity**: Ensures balance continuity across statement processing
/// - **Verification**: Enables balance verification and reconciliation
///
/// ### Statement Processing
/// - **MT 940 Integration**: Core component of customer statement messages
/// - **Sequence Management**: Handles statement sequence breaks
/// - **Balance Chain**: Links to statement lines (Field 61) and closing balance (Field 62)
/// - **Period Definition**: Establishes statement period starting point
///
/// ## Regional Considerations
/// - **European Banking**: SEPA statement requirements and Euro processing
/// - **US Banking**: Federal Reserve and commercial bank statement standards
/// - **Asian Markets**: Local banking statement requirements
/// - **Cross-Border**: Multi-currency account statement processing
///
/// ## Error Prevention Guidelines
/// - **Date Verification**: Confirm value date is within acceptable range
/// - **Currency Consistency**: Ensure currency matches account currency
/// - **Amount Precision**: Verify amount precision matches currency requirements
/// - **Mark Validation**: Confirm debit/credit mark is appropriate
///
/// ## Related Fields Integration
/// - **Field 61**: Statement Line (transaction details)
/// - **Field 62**: Closing Balance (ending balance)
/// - **Field 64**: Closing Available Balance (available funds)
/// - **Field 65**: Forward Available Balance (future availability)
///
/// ## Compliance Framework
/// - **Banking Regulations**: Compliance with local banking statement requirements
/// - **Audit Documentation**: Proper balance documentation for audit trails
/// - **Customer Communication**: Clear balance communication to account holders
/// - **Reconciliation Support**: Foundation for account reconciliation processes
///
/// ## See Also
/// - Swift FIN User Handbook: Opening Balance Specifications
/// - MT 940 Message Standards: Customer Statement Message
/// - Cash Management Guidelines: Balance Processing Standards
/// - Account Statement Requirements: Regional Banking Standards
///
///   **Field 60F: First Opening Balance**
///
/// Initial opening balance at the beginning of a statement period.
/// Used when starting a new statement or account balance sequence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field60F {
    /// Debit or Credit mark
    ///
    /// Format: 1!a - 'D' (Debit) or 'C' (Credit)
    /// Indicates whether the opening balance is a debit or credit position
    pub debit_credit_mark: String,

    /// Value date of the opening balance
    ///
    /// Format: 6!n (YYMMDD) - Date when balance is effective
    /// Must be valid calendar date within acceptable range
    pub value_date: NaiveDate,

    /// Currency of the balance
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must match account currency for consistency
    pub currency: String,

    /// Opening balance amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Precision must match currency requirements
    pub amount: f64,
}

///   **Field 60M: Intermediate Opening Balance**
///
/// Opening balance after a sequence break within a statement period.
/// Used to maintain balance continuity across statement sequences.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field60M {
    /// Debit or Credit mark
    ///
    /// Format: 1!a - 'D' (Debit) or 'C' (Credit)
    /// Indicates whether the intermediate opening balance is a debit or credit position
    pub debit_credit_mark: String,

    /// Value date of the intermediate opening balance
    ///
    /// Format: 6!n (YYMMDD) - Date when balance is effective
    /// Represents balance at sequence break point
    pub value_date: NaiveDate,

    /// Currency of the balance
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must match account currency for consistency
    pub currency: String,

    /// Intermediate opening balance amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Represents balance carried forward from previous sequence
    pub amount: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
