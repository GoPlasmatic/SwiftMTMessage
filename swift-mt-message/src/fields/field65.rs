use chrono::NaiveDate;
use swift_mt_message_macros::serde_swift_fields;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

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
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field65 {
    /// Debit or Credit mark
    ///
    /// Format: 1!a - 'D' (Debit) or 'C' (Credit)
    /// Indicates whether the forward available balance will be a debit or credit position
    #[component("1!a")]
    pub debit_credit_mark: String,

    /// Future value date of the available balance
    ///
    /// Format: 6!n (YYMMDD) - Future date when balance will be effective
    /// Should be later than current statement date
    #[component("6!n")]
    pub value_date: NaiveDate,

    /// Currency of the forward available balance
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must match account currency for consistency
    #[component("3!a")]
    pub currency: String,

    /// Forward available balance amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Projected funds that will be available on the future value date
    #[component("15d")]
    pub amount: f64,
}
