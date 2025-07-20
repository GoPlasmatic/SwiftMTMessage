use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// **Field 62: Closing Balance**
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

/// **Field 62F: Final Closing Balance**
///
/// Final closing balance at the end of a complete statement period.
/// Represents the definitive account position after all transactions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field62F {
    /// Debit or Credit mark
    ///
    /// Format: 1!a - 'D' (Debit) or 'C' (Credit)
    /// Indicates whether the closing balance is a debit or credit position
    #[component("1!a")]
    pub debit_credit_mark: String,

    /// Value date of the closing balance
    ///
    /// Format: 6!n (YYMMDD) - Date when balance is effective
    /// Typically the last business day of the statement period
    #[component("6!n")]
    pub value_date: NaiveDate,

    /// Currency of the balance
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must match account currency and opening balance currency
    #[component("3!a")]
    pub currency: String,

    /// Final closing balance amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Result of opening balance plus all statement line transactions
    #[component("15d")]
    pub amount: f64,
}

/// **Field 62M: Intermediate Closing Balance**
///
/// Closing balance at a sequence break within a statement period.
/// Used to maintain balance continuity across statement sequences.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field62M {
    /// Debit or Credit mark
    ///
    /// Format: 1!a - 'D' (Debit) or 'C' (Credit)
    /// Indicates whether the intermediate closing balance is a debit or credit position
    #[component("1!a")]
    pub debit_credit_mark: String,

    /// Value date of the intermediate closing balance
    ///
    /// Format: 6!n (YYMMDD) - Date when balance is effective
    /// Represents balance at sequence break point
    #[component("6!n")]
    pub value_date: NaiveDate,

    /// Currency of the balance
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must match account currency for consistency
    #[component("3!a")]
    pub currency: String,

    /// Intermediate closing balance amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Becomes opening balance for next sequence
    #[component("15d")]
    pub amount: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field62 {
    F(Field62F),
    M(Field62M),
}
