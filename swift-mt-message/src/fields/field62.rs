use super::swift_utils::{parse_amount, parse_currency, parse_date_yymmdd, parse_exact_length};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

///   **Field 62: Closing Balance**
///
/// ## Purpose
/// Specifies the closing balance of an account in customer statement messages (MT 940)
/// and other cash management contexts. This field represents the final balance position
/// after processing all transactions within the statement period. Essential for account
/// balance verification, reconciliation, and cash management reporting.
///
/// ## Format Options Overview
/// - **Option F**: Final closing balance - balance at statement end
/// - **Option M**: Intermediate closing balance - balance at sequence break
///
/// ## Business Context Applications
/// - **Customer Statements**: Closing balance for MT 940 Customer Statement Message
/// - **Cash Management**: Final balance position for period
/// - **Account Reconciliation**: End position for balance verification
/// - **Sequence Processing**: Balance handoff between statement sequences
///
/// ## Network Validation Requirements
/// - **Date Validation**: Value date must be valid calendar date
/// - **Currency Validation**: Must be valid ISO 4217 currency code
/// - **Amount Format**: Decimal amount with proper precision
/// - **Mark Validation**: Debit/Credit mark must be D (Debit) or C (Credit)
/// - **Balance Continuity**: Must align with opening balance plus transactions
///
/// ## Balance Calculation Logic
/// ### Closing Balance Formula
/// ```logic
/// Closing Balance = Opening Balance (Field 60) + Sum of Statement Lines (Field 61)
/// ```
///
/// ### Balance Types
/// - **Final Balance (F)**: Balance at end of complete statement period
/// - **Intermediate Balance (M)**: Balance at sequence break within statement
/// - **Verification**: Mathematical verification against transaction totals
/// - **Continuity**: Becomes opening balance for next period
///
/// ## Statement Processing Integration
/// - **MT 940 Component**: Essential element of customer statement messages
/// - **Transaction Summary**: Reflects cumulative effect of all statement transactions
/// - **Period Closure**: Defines end of statement period
/// - **Reconciliation**: Enables customer balance reconciliation
///
/// ## Regional Considerations
/// - **European Banking**: SEPA statement requirements and Euro processing
/// - **US Banking**: Federal Reserve and commercial bank statement standards
/// - **Asian Markets**: Local banking statement requirements
/// - **Cross-Border**: Multi-currency account statement processing
///
/// ## Error Prevention Guidelines
/// - **Balance Verification**: Confirm closing balance equals opening plus transactions
/// - **Date Consistency**: Ensure value date aligns with statement period
/// - **Currency Matching**: Verify currency matches account and transaction currency
/// - **Precision Validation**: Confirm amount precision meets currency standards
///
/// ## Related Fields Integration
/// - **Field 60**: Opening Balance (period starting point)
/// - **Field 61**: Statement Line (individual transactions)
/// - **Field 64**: Closing Available Balance (available funds)
/// - **Field 65**: Forward Available Balance (future availability)
///
/// ## Compliance Framework
/// - **Banking Regulations**: Compliance with local banking statement requirements
/// - **Audit Documentation**: Proper closing balance documentation
/// - **Customer Communication**: Clear final balance communication
/// - **Reconciliation Standards**: Foundation for account reconciliation
///
/// ## Cash Management Applications
/// - **Liquidity Management**: Final position for liquidity planning
/// - **Cash Forecasting**: Input for cash flow forecasting
/// - **Risk Management**: Position assessment for risk management
/// - **Performance Reporting**: Balance reporting for performance analysis
///
/// ## See Also
/// - Swift FIN User Handbook: Closing Balance Specifications
/// - MT 940 Message Standards: Customer Statement Message
/// - Cash Management Guidelines: Balance Processing Standards
/// - Account Statement Requirements: Regional Banking Standards
///
///   **Field 62F: Final Closing Balance**
///
/// Final closing balance at the end of a complete statement period.
/// Represents the definitive account position after all transactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field62F {
    /// Debit or Credit mark
    ///
    /// Format: 1!a - 'D' (Debit) or 'C' (Credit)
    /// Indicates whether the closing balance is a debit or credit position
    pub debit_credit_mark: String,

    /// Value date of the closing balance
    ///
    /// Format: 6!n (YYMMDD) - Date when balance is effective
    /// Typically the last business day of the statement period
    pub value_date: NaiveDate,

    /// Currency of the balance
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must match account currency and opening balance currency
    pub currency: String,

    /// Final closing balance amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Result of opening balance plus all statement line transactions
    pub amount: f64,
}

///   **Field 62M: Intermediate Closing Balance**
///
/// Closing balance at a sequence break within a statement period.
/// Used to maintain balance continuity across statement sequences.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field62M {
    /// Debit or Credit mark
    ///
    /// Format: 1!a - 'D' (Debit) or 'C' (Credit)
    /// Indicates whether the intermediate closing balance is a debit or credit position
    pub debit_credit_mark: String,

    /// Value date of the intermediate closing balance
    ///
    /// Format: 6!n (YYMMDD) - Date when balance is effective
    /// Represents balance at sequence break point
    pub value_date: NaiveDate,

    /// Currency of the balance
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must match account currency for consistency
    pub currency: String,

    /// Intermediate closing balance amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Becomes opening balance for next sequence
    pub amount: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field62 {
    F(Field62F),
    M(Field62M),
}

impl SwiftField for Field62F {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Format: 1!a6!n3!a15d - DebitCredit + Date + Currency + Amount
        if input.len() < 10 {
            return Err(ParseError::InvalidFormat {
                message: "Field 62F must be at least 10 characters long".to_string(),
            });
        }

        // Parse debit/credit mark (1 character)
        let debit_credit_mark = parse_exact_length(&input[0..1], 1, "Field 62F debit/credit mark")?;
        if debit_credit_mark != "D" && debit_credit_mark != "C" {
            return Err(ParseError::InvalidFormat {
                message: "Field 62F debit/credit mark must be 'D' or 'C'".to_string(),
            });
        }

        // Parse value date (6 digits)
        let date_str = parse_exact_length(&input[1..7], 6, "Field 62F value date")?;
        let value_date = parse_date_yymmdd(&date_str)?;

        // Parse currency (3 characters)
        let currency = parse_exact_length(&input[7..10], 3, "Field 62F currency")?;
        let currency = parse_currency(&currency)?;

        // Parse amount (remaining characters)
        let amount_str = &input[10..];
        let amount = parse_amount(amount_str)?;

        Ok(Field62F {
            debit_credit_mark,
            value_date,
            currency,
            amount,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":62F:{}{}{}{}",
            self.debit_credit_mark,
            self.value_date.format("%y%m%d"),
            self.currency,
            format!("{:.2}", self.amount).replace('.', ",")
        )
    }
}

impl SwiftField for Field62M {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Format: 1!a6!n3!a15d - DebitCredit + Date + Currency + Amount
        if input.len() < 10 {
            return Err(ParseError::InvalidFormat {
                message: "Field 62M must be at least 10 characters long".to_string(),
            });
        }

        // Parse debit/credit mark (1 character)
        let debit_credit_mark = parse_exact_length(&input[0..1], 1, "Field 62M debit/credit mark")?;
        if debit_credit_mark != "D" && debit_credit_mark != "C" {
            return Err(ParseError::InvalidFormat {
                message: "Field 62M debit/credit mark must be 'D' or 'C'".to_string(),
            });
        }

        // Parse value date (6 digits)
        let date_str = parse_exact_length(&input[1..7], 6, "Field 62M value date")?;
        let value_date = parse_date_yymmdd(&date_str)?;

        // Parse currency (3 characters)
        let currency = parse_exact_length(&input[7..10], 3, "Field 62M currency")?;
        let currency = parse_currency(&currency)?;

        // Parse amount (remaining characters)
        let amount_str = &input[10..];
        let amount = parse_amount(amount_str)?;

        Ok(Field62M {
            debit_credit_mark,
            value_date,
            currency,
            amount,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":62M:{}{}{}{}",
            self.debit_credit_mark,
            self.value_date.format("%y%m%d"),
            self.currency,
            format!("{:.2}", self.amount).replace('.', ",")
        )
    }
}

impl SwiftField for Field62 {
    fn parse(_input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // This should not be called directly - parsing is handled by the message parser
        // which determines the variant (F or M) from the field tag
        Err(ParseError::InvalidFormat {
            message: "Field62 enum should not be parsed directly".to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field62::F(field) => field.to_swift_string(),
            Field62::M(field) => field.to_swift_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_field62f_parse_valid() {
        let field = Field62F::parse("C231225USD1234,56").unwrap();
        assert_eq!(field.debit_credit_mark, "C");
        assert_eq!(
            field.value_date,
            NaiveDate::from_ymd_opt(2023, 12, 25).unwrap()
        );
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 1234.56);
    }

    #[test]
    fn test_field62m_parse_valid() {
        let field = Field62M::parse("D991231EUR500,00").unwrap();
        assert_eq!(field.debit_credit_mark, "D");
        assert_eq!(
            field.value_date,
            NaiveDate::from_ymd_opt(1999, 12, 31).unwrap()
        );
        assert_eq!(field.currency, "EUR");
        assert_eq!(field.amount, 500.00);
    }

    #[test]
    fn test_field62f_invalid_debit_credit_mark() {
        assert!(Field62F::parse("X231225USD1234,56").is_err());
    }

    #[test]
    fn test_field62f_to_swift_string() {
        let field = Field62F {
            debit_credit_mark: "C".to_string(),
            value_date: NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(),
            currency: "USD".to_string(),
            amount: 1234.56,
        };
        assert_eq!(field.to_swift_string(), ":62F:C231225USD1234,56");
    }

    #[test]
    fn test_field62_enum_to_swift_string() {
        let field_f = Field62::F(Field62F {
            debit_credit_mark: "C".to_string(),
            value_date: NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(),
            currency: "USD".to_string(),
            amount: 1234.56,
        });
        assert_eq!(field_f.to_swift_string(), ":62F:C231225USD1234,56");

        let field_m = Field62::M(Field62M {
            debit_credit_mark: "D".to_string(),
            value_date: NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(),
            currency: "EUR".to_string(),
            amount: 500.00,
        });
        assert_eq!(field_m.to_swift_string(), ":62M:D231225EUR500,00");
    }
}
