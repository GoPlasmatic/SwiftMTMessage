use chrono::NaiveDate;
use swift_mt_message_macros::serde_swift_fields;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

///   **Field 61: Statement Line**
///
/// ## Purpose
/// Represents individual transaction entries in customer statement messages (MT 940),
/// providing detailed information about each debit or credit transaction affecting
/// the account balance. This field is fundamental to statement processing, enabling
/// detailed transaction tracking, reconciliation, and audit trail maintenance.
///
/// ## Format Specification
/// - **Swift Format**: `6!n[4!n]2a[1!a]15d1!a3!c[16x][//16x][34x]`
/// - **Complex Structure**: Multiple components with optional elements
/// - **Variable Length**: Components can be present or absent based on transaction type
/// - **Structured Data**: Each component serves specific business purpose
///
/// ## Business Context Applications
/// - **Customer Statements**: Core component of MT 940 Customer Statement Message
/// - **Transaction Detail**: Complete transaction information for account holders
/// - **Reconciliation**: Detailed transaction data for account reconciliation
/// - **Audit Trail**: Complete transaction history for compliance and audit
///
/// ## Component Structure
/// ### Mandatory Components
/// - **Value Date**: Date when transaction affects account balance
/// - **Debit/Credit Mark**: Transaction direction (debit/credit)
/// - **Amount**: Transaction amount in account currency
/// - **Transaction Type**: Classification of transaction type
///
/// ### Optional Components
/// - **Entry Date**: Date transaction was posted (if different from value date)
/// - **Funds Code**: Availability of funds (immediate/float)
/// - **Customer Reference**: Transaction reference for customer
/// - **Bank Reference**: Bank's internal transaction reference
/// - **Supplementary Details**: Additional transaction information
///
/// ## Network Validation Requirements
/// - **Date Validation**: Value date must be valid calendar date
/// - **Amount Format**: Decimal amount with proper precision
/// - **Reference Format**: References must follow specified format rules
/// - **Transaction Code**: Must be valid transaction type code
/// - **Character Set**: All components must use valid character sets
///
/// ## Transaction Processing
/// ### Balance Impact
/// - **Debit Transactions**: Reduce account balance (payments, charges, withdrawals)
/// - **Credit Transactions**: Increase account balance (deposits, transfers, interest)
/// - **Reversal Transactions**: Correct previous transaction errors
/// - **Adjustment Transactions**: Administrative balance adjustments
///
/// ### Transaction Types
/// - **Customer Transfers**: Payments and receipts
/// - **Bank Services**: Fees, charges, and service transactions
/// - **Interest**: Interest credits and debits
/// - **Foreign Exchange**: Currency conversion transactions
///
/// ## Regional Considerations
/// - **European Banking**: SEPA transaction processing and reporting
/// - **US Banking**: ACH, wire transfer, and check processing
/// - **Asian Markets**: Local payment system integration
/// - **Cross-Border**: International transaction processing
///
/// ## Error Prevention Guidelines
/// - **Date Consistency**: Verify dates are logical and within statement period
/// - **Amount Verification**: Confirm amount format and precision
/// - **Reference Validation**: Ensure references follow format requirements
/// - **Balance Verification**: Confirm transactions sum to balance changes
///
/// ## Related Fields Integration
/// - **Field 60**: Opening Balance (starting position)
/// - **Field 62**: Closing Balance (ending position after transactions)
/// - **Field 64**: Closing Available Balance (availability impact)
/// - **Field 86**: Information to Account Owner (additional details)
///
/// ## Compliance Framework
/// - **Banking Regulations**: Transaction reporting requirements
/// - **Audit Standards**: Complete transaction documentation
/// - **Customer Rights**: Detailed transaction information provision
/// - **Data Retention**: Transaction history retention requirements
///
/// ## See Also
/// - Swift FIN User Handbook: Statement Line Specifications
/// - MT 940 Message Standards: Customer Statement Processing
/// - Transaction Processing: Banking Transaction Standards
/// - Account Statement Requirements: Regional Banking Regulations
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field61 {
    /// Value date (6!n format, YYMMDD)
    #[component("6!n")]
    pub value_date: NaiveDate,

    /// Optional entry date (4!n format, MMDD)
    #[component("[4!n]")]
    pub entry_date: Option<String>,

    /// Debit/Credit mark (2a format: D, C, RD, RC)
    #[component("2a")]
    pub debit_credit_mark: String,

    /// Optional funds code (1!a format)
    #[component("[1!a]")]
    pub funds_code: Option<char>,

    /// Amount (15d format)
    #[component("15d")]
    pub amount: f64,

    /// Transaction type identification code (4!a format)
    #[component("1!a3!c")]
    pub transaction_type: String,

    /// Customer reference (16x format - up to 16 characters)
    #[component("16x")]
    pub customer_reference: String,

    /// Bank reference (16x format, preceded by //)
    #[component("[//16x]")]
    pub bank_reference: Option<String>,

    /// Optional supplementary details (34x format)
    #[component("[34x]")]
    pub supplementary_details: Option<String>,
}
