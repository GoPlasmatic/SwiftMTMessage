//! # Field 90: Number & Sum
//!
//! ## Purpose
//! Specifies the number of transactions and total sum for summary and control purposes
//! in financial messages. This field provides aggregated transaction information that
//! enables verification, reconciliation, and control totals for transaction batches
//! and statement processing.
//!
//! ## Format Options Overview
//! - **Option C**: Credit entries - number and sum of credit transactions
//! - **Option D**: Debit entries - number and sum of debit transactions
//!
//! ## Business Context Applications
//! - **Statement Control**: Summary totals for customer statements
//! - **Batch Processing**: Control totals for transaction batches
//! - **Reconciliation**: Verification totals for account reconciliation
//! - **Audit Control**: Control information for audit purposes
//!
//! ## Network Validation Requirements
//! - **Number Validation**: Transaction count must be valid positive integer
//! - **Currency Validation**: Must be valid ISO 4217 currency code
//! - **Amount Format**: Decimal amount with proper precision
//! - **Logical Consistency**: Numbers and amounts must be logically consistent
//!
//! ## Control Total Applications
//! ### Transaction Counting
//! - **Credit Count**: Total number of credit transactions
//! - **Debit Count**: Total number of debit transactions
//! - **Verification**: Cross-verification with individual transaction entries
//! - **Completeness**: Ensuring all transactions are included
//!
//! ### Amount Summation
//! - **Credit Sum**: Total amount of all credit transactions
//! - **Debit Sum**: Total amount of all debit transactions
//! - **Balance Verification**: Verification against balance changes
//! - **Precision**: Maintaining precision in summary calculations
//!
//! ## Regional Considerations
//! - **European Banking**: SEPA statement control requirements
//! - **US Banking**: Federal and commercial bank control standards
//! - **Asian Markets**: Local banking control and summary requirements
//! - **Cross-Border**: International summary and control standards
//!
//! ## Error Prevention Guidelines
//! - **Count Verification**: Verify transaction counts match actual entries
//! - **Amount Verification**: Confirm summary amounts equal individual totals
//! - **Currency Consistency**: Ensure currency matches transaction currency
//! - **Precision Checking**: Verify amount precision meets standards
//!
//! ## Related Fields Integration
//! - **Field 61**: Statement Line (individual transactions being summarized)
//! - **Field 60/62**: Opening/Closing Balance (balance change verification)
//! - **Field 28C**: Statement Number/Sequence Number (statement context)
//! - **Field 25**: Account Identification (account context)
//!
//! ## Compliance Framework
//! - **Control Standards**: Meeting banking control and summary standards
//! - **Audit Requirements**: Providing adequate control information for audits
//! - **Reconciliation Support**: Supporting account reconciliation processes
//! - **Quality Control**: Ensuring transaction processing quality
//!
//! ## See Also
//! - Swift FIN User Handbook: Number & Sum Field Specifications
//! - Banking Control Standards: Transaction Summary Requirements
//! - Account Statement Standards: Control Total Requirements
//! - Audit Guidelines: Financial Transaction Control

use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

/// **Field 90D: Number & Sum of Debit Entries**
///
/// Debit variant of [Field 90 module](index.html). Specifies the number and total sum of debit transactions.
///
/// **Components:**
/// - Number of debit transactions (5n)
/// - Currency code (3!a)
/// - Total sum of debit amounts (15d)
///
/// For complete documentation, see the [Field 90 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field90D {
    /// Number of debit transactions
    ///
    /// Format: 5n - Up to 5 digit number
    /// Count of all debit transactions in the summary
    #[component("5n")]
    pub number: u32,

    /// Currency of debit amounts
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must match currency of summarized transactions
    #[component("3!a")]
    pub currency: String,

    /// Total sum of debit amounts
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Sum of all debit transaction amounts
    #[component("15d")]
    pub amount: f64,
}

/// **Field 90C: Number & Sum of Credit Entries**
///
/// Credit variant of [Field 90 module](index.html). Specifies the number and total sum of credit transactions.
///
/// **Components:**
/// - Number of credit transactions (5n)
/// - Currency code (3!a)
/// - Total sum of credit amounts (15d)
///
/// For complete documentation, see the [Field 90 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field90C {
    /// Number of credit transactions
    ///
    /// Format: 5n - Up to 5 digit number
    /// Count of all credit transactions in the summary
    #[component("5n")]
    pub number: u32,

    /// Currency of credit amounts
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must match currency of summarized transactions
    #[component("3!a")]
    pub currency: String,

    /// Total sum of credit amounts
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Sum of all credit transaction amounts
    #[component("15d")]
    pub amount: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftField;

    #[test]
    fn test_field90d_parsing_basic() {
        let value = "2GBP250050";
        match Field90D::parse(value) {
            Ok(field) => {
                assert_eq!(field.number, 2);
                assert_eq!(field.currency, "GBP");
                assert_eq!(field.amount, 250050.0);
            }
            Err(e) => {
                panic!("Failed to parse Field90D '{}': {:?}", value, e);
            }
        }
    }
}
