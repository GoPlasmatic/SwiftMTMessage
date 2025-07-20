use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field64 {
    /// Debit or Credit mark
    ///
    /// Format: 1!a - 'D' (Debit) or 'C' (Credit)
    /// Indicates whether the available balance is a debit or credit position
    #[component("1!a")]
    pub debit_credit_mark: String,

    /// Value date of the available balance
    ///
    /// Format: 6!n (YYMMDD) - Date when balance is effective
    /// Typically aligns with statement period end date
    #[component("6!n")]
    pub value_date: NaiveDate,

    /// Currency of the available balance
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must match account currency for consistency
    #[component("3!a")]
    pub currency: String,

    /// Available balance amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Represents funds immediately available for use
    #[component("15d")]
    pub amount: f64,
}
