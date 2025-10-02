use super::swift_utils::{parse_amount, parse_currency, parse_date_yymmdd, parse_exact_length};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

///   **Field 65: Forward Available Balance**
///
/// ## Purpose
/// Specifies the forward available balance of an account, representing the funds that will
/// be available on a future value date. This field provides forward-looking liquidity
/// information, accounting for future-dated transactions, maturity dates, and scheduled
/// fund movements that will affect account availability.
///
/// ## Business Context Applications
/// - **Cash Forecasting**: Future fund availability for planning
/// - **Liquidity Management**: Forward liquidity position assessment
/// - **Treasury Operations**: Future cash position planning
/// - **Credit Facilities**: Future available credit assessments
///
/// ## Network Validation Requirements
/// - **Date Validation**: Value date must be valid future calendar date
/// - **Currency Validation**: Must be valid ISO 4217 currency code
/// - **Amount Format**: Decimal amount with proper precision
/// - **Mark Validation**: Debit/Credit mark must be D (Debit) or C (Credit)
/// - **Future Date Logic**: Value date should be in the future relative to statement date
///
/// ## Forward Balance Calculation
/// ### Calculation Components
/// ```logic
/// Forward Available = Current Available + Scheduled Credits - Scheduled Debits - Future Holds
/// ```
///
/// ### Forward Factors
/// - **Scheduled Transactions**: Future-dated transactions affecting availability
/// - **Maturity Events**: Investment maturities and loan repayments
/// - **Standing Orders**: Recurring payment obligations
/// - **Credit Facilities**: Available credit that may be utilized
/// - **Float Projections**: Expected clearing and settlement timing
///
/// ## Time Horizon Considerations
/// - **Short-term Forward**: 1-7 days forward availability
/// - **Medium-term Forward**: 1-4 weeks forward availability
/// - **Long-term Forward**: Monthly or quarterly forward projections
/// - **Scenario Analysis**: Multiple forward balance scenarios
///
/// ## Regional Considerations
/// - **European Banking**: Euro area liquidity forecasting requirements
/// - **US Banking**: Federal Reserve and commercial bank forward planning
/// - **Asian Markets**: Local market forward liquidity requirements
/// - **Cross-Border**: Multi-currency forward balance coordination
///
/// ## Error Prevention Guidelines
/// - **Date Logic**: Verify forward date is logical and within reasonable range
/// - **Calculation Verification**: Confirm forward balance calculation methodology
/// - **Currency Consistency**: Ensure currency matches account and related balances
/// - **Scenario Validation**: Verify forward projections are realistic
///
/// ## Related Fields Integration
/// - **Field 64**: Closing Available Balance (current availability baseline)
/// - **Field 62**: Closing Balance (book balance context)
/// - **Field 61**: Statement Line (transactions affecting forward balance)
/// - **Field 60**: Opening Balance (period context)
///
/// ## Compliance Framework
/// - **Regulatory Reporting**: Forward liquidity reporting requirements
/// - **Risk Management**: Forward liquidity risk assessment
/// - **Basel Requirements**: Liquidity coverage ratio and forward planning
/// - **Audit Documentation**: Forward balance calculation methodology
///
/// ## Treasury Management Applications
/// - **Cash Flow Forecasting**: Input for cash flow projections
/// - **Investment Planning**: Available funds for future investments
/// - **Debt Management**: Future capacity for debt service
/// - **Working Capital**: Forward working capital availability
///
/// ## See Also
/// - Swift FIN User Handbook: Forward Available Balance Specifications
/// - Treasury Management: Forward Liquidity Planning
/// - Cash Flow Forecasting: Forward Balance Projections
/// - Basel Liquidity Standards: Forward Liquidity Requirements
///
///   **Field 65: Forward Available Balance Structure**
///
/// Contains the forward available balance with debit/credit indication, future value date,
/// currency, and amount representing funds that will be available.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field65 {
    /// Debit or Credit mark
    ///
    /// Format: 1!a - 'D' (Debit) or 'C' (Credit)
    /// Indicates whether the forward available balance will be a debit or credit position
    pub debit_credit_mark: String,

    /// Future value date of the available balance
    ///
    /// Format: 6!n (YYMMDD) - Future date when balance will be effective
    /// Should be later than current statement date
    pub value_date: NaiveDate,

    /// Currency of the forward available balance
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must match account currency for consistency
    pub currency: String,

    /// Forward available balance amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Projected funds that will be available on the future value date
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
