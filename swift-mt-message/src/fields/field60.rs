use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

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
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field60F {
    /// Debit or Credit mark
    ///
    /// Format: 1!a - 'D' (Debit) or 'C' (Credit)
    /// Indicates whether the opening balance is a debit or credit position
    #[component("1!a")]
    pub debit_credit_mark: String,

    /// Value date of the opening balance
    ///
    /// Format: 6!n (YYMMDD) - Date when balance is effective
    /// Must be valid calendar date within acceptable range
    #[component("6!n")]
    pub value_date: NaiveDate,

    /// Currency of the balance
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must match account currency for consistency
    #[component("3!a")]
    pub currency: String,

    /// Opening balance amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Precision must match currency requirements
    #[component("15d")]
    pub amount: f64,
}

///   **Field 60M: Intermediate Opening Balance**
///
/// Opening balance after a sequence break within a statement period.
/// Used to maintain balance continuity across statement sequences.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field60M {
    /// Debit or Credit mark
    ///
    /// Format: 1!a - 'D' (Debit) or 'C' (Credit)
    /// Indicates whether the intermediate opening balance is a debit or credit position
    #[component("1!a")]
    pub debit_credit_mark: String,

    /// Value date of the intermediate opening balance
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

    /// Intermediate opening balance amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Represents balance carried forward from previous sequence
    #[component("15d")]
    pub amount: f64,
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field60 {
    F(Field60F),
    M(Field60M),
}
