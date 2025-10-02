use super::swift_utils::{parse_amount, parse_currency, parse_date_yymmdd, parse_exact_length};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

///   **Field 64: Closing Available Balance**
///
/// ## Purpose
/// Specifies the closing available balance of an account, representing the funds that are
/// immediately available for use by the account holder. This field distinguishes between
/// the book balance (Field 62) and the available balance, accounting for holds, pending
/// transactions, and other restrictions that may limit fund availability.
///
/// ## Business Context Applications
/// - **Cash Management**: Available funds for immediate use
/// - **Liquidity Assessment**: Actual usable funds for operations
/// - **Credit Decisions**: Available balance for credit facilities
/// - **Risk Management**: Available funds for risk assessment
///
/// ## Network Validation Requirements
/// - **Date Validation**: Value date must be valid calendar date
/// - **Currency Validation**: Must be valid ISO 4217 currency code
/// - **Amount Format**: Decimal amount with proper precision
/// - **Mark Validation**: Debit/Credit mark must be D (Debit) or C (Credit)
/// - **Balance Logic**: Available balance should not exceed book balance for most scenarios
///
/// ## Available Balance Calculation
/// ### Balance Components
/// ```logic
/// Available Balance = Book Balance - Holds - Pending Debits + Pending Credits - Reserves
/// ```
///
/// ### Availability Factors
/// - **Holds**: Funds held for pending transactions or legal requirements
/// - **Float**: Funds not yet cleared or collected
/// - **Reserves**: Required reserves or minimum balance requirements
/// - **Credit Facilities**: Available credit lines that increase available balance
///
/// ## Regional Considerations
/// - **European Banking**: European fund availability regulations
/// - **US Banking**: Regulation CC and fund availability requirements
/// - **Asian Markets**: Local fund availability and hold practices
/// - **Cross-Border**: International fund availability considerations
///
/// ## Error Prevention Guidelines
/// - **Balance Logic**: Verify available balance is consistent with book balance
/// - **Date Alignment**: Ensure value date aligns with statement period
/// - **Currency Consistency**: Verify currency matches account currency
/// - **Amount Validation**: Confirm precision meets currency standards
///
/// ## Related Fields Integration
/// - **Field 62**: Closing Balance (book balance comparison)
/// - **Field 65**: Forward Available Balance (future availability)
/// - **Field 60**: Opening Balance (period context)
/// - **Field 61**: Statement Line (transactions affecting availability)
///
/// ## Compliance Framework
/// - **Regulatory Requirements**: Fund availability disclosure requirements
/// - **Consumer Protection**: Clear communication of available funds
/// - **Risk Management**: Available balance for credit and operational risk
/// - **Audit Documentation**: Proper available balance calculation documentation
///
/// ## See Also
/// - Swift FIN User Handbook: Available Balance Specifications
/// - Banking Regulations: Fund Availability Requirements
/// - Cash Management Standards: Available Balance Calculation
/// - Risk Management: Available Fund Assessment
///
///   **Field 64: Closing Available Balance Structure**
///
/// Contains the closing available balance with debit/credit indication, value date,
/// currency, and amount representing immediately usable funds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field64 {
    /// Debit or Credit mark
    ///
    /// Format: 1!a - 'D' (Debit) or 'C' (Credit)
    /// Indicates whether the available balance is a debit or credit position
    pub debit_credit_mark: String,

    /// Value date of the available balance
    ///
    /// Format: 6!n (YYMMDD) - Date when balance is effective
    /// Typically aligns with statement period end date
    pub value_date: NaiveDate,

    /// Currency of the available balance
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must match account currency for consistency
    pub currency: String,

    /// Available balance amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Represents funds immediately available for use
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
