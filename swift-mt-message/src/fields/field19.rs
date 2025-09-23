//! # Field 19: Sum of Amounts
//!
//! ## Purpose
//! Specifies the sum of all individual transaction amounts appearing in sequence transactions.
//! This field is essential for reconciliation and validation when the total transaction amount
//! differs from the settlement amount due to charging arrangements or fee allocations.
//!
//! ## Format Specification
//! - **Swift Format**: `17d`
//! - **Description**: Up to 17 digits including decimal places
//! - **Decimal Separator**: Comma as decimal separator, included in maximum length
//! - **Precision**: Follows currency-specific decimal precision rules
//!
//! ## Presence and Usage
//! - **Status**: Optional in MT102 Settlement Details sequence
//! - **Swift Error Codes**: C03, T40, T43 (amount validation), T51 (invalid amount format)
//! - **Referenced in Rule**: C1 (MT102 validation logic)
//! - **Context**: Multiple customer credit transfers with charge handling
//!
//! ## Business Applications
//! ### Batch Payment Processing
//! - **Multiple Payments**: Used in MT102 for multiple customer credit transfers
//! - **Charge Handling**: Accommodates scenarios where charges affect individual vs. total amounts
//! - **Settlement Logic**: Enables different settlement and transaction amounts
//! - **Batch Processing**: Supports batch payment scenarios with varying charge allocations
//!
//! ### Reconciliation and Validation
//! - **Settlement Variance**: Used when sum of amounts differs from settlement amount in field 32A
//! - **Charge Allocation**: Applied when transactions contain charging option "OUR" in field 71A
//! - **Transaction Summation**: Must equal sum of all field 32B amounts in each sequence
//! - **Audit Trail**: Enables proper reconciliation between individual and total amounts
//!
//! ## Calculation Logic
//! Field 19 must equal the sum of all Field 32B amounts in the sequence:
//! - **Validation Rule**: Field 19 = Σ(Field 32B amounts)
//! - **Settlement Difference**: If Field 19 ≠ Field 32A, difference typically represents charges or fees
//! - **Charge Impact**: Accounts for "OUR" charge deductions from individual transactions
//!
//! ## Network Validation Rules
//! - **Positive Amount**: Amount must be greater than zero
//! - **Integer Validation**: Integer part must contain at least one digit
//! - **Decimal Precision**: Number of digits after decimal comma must not exceed currency maximum
//! - **Format Compliance**: Must follow decimal amount formatting standards
//! - **Currency Alignment**: Precision must match currency specified in field 32A
//!
//! ## Amount Precision by Currency
//! - **Most Currencies**: 2 decimal places (USD, EUR, GBP, etc.)
//! - **Japanese Yen**: 0 decimal places (JPY)
//! - **Bahraini Dinar**: 3 decimal places (BHD)
//! - **Special Cases**: Some currencies have specific precision requirements
//!
//! ## Usage Scenarios
//! ### Charge Management
//! - **Charge Deduction**: When "OUR" charges are deducted from individual transactions
//! - **Fee Allocation**: When fees are distributed across multiple transactions
//! - **Settlement Coordination**: When settlement amount differs from transaction total
//! - **Cost Distribution**: Managing how processing costs affect final amounts
//!
//! ### Batch Reconciliation
//! - **Validation**: Ensuring sum of individual transactions matches expected total
//! - **Error Detection**: Identifying discrepancies in batch payment processing
//! - **Audit Support**: Providing clear reconciliation trail for compliance
//! - **Quality Control**: Automated validation of batch payment integrity
//!
//! ## Regional Considerations
//! - **European Payments**: EUR precision and formatting rules
//! - **US Payments**: USD decimal handling and validation
//! - **Asian Markets**: Local currency precision requirements
//! - **Multi-Currency**: Handling different precision rules in same batch
//!
//! ## STP Compliance
//! - **Automated Validation**: STP systems automatically validate sum calculations
//! - **Precision Requirements**: Enhanced precision validation for automated processing
//! - **Format Standardization**: Strict adherence to decimal formatting rules
//! - **Error Handling**: Automated rejection for calculation mismatches
//!
//! ## Error Prevention Guidelines
//! - **Precision Validation**: Ensure decimal places match currency requirements
//! - **Sum Verification**: Verify sum equals individual transaction amounts
//! - **Format Checking**: Confirm proper decimal formatting with comma separator
//! - **Range Validation**: Ensure amount is within reasonable business limits
//!
//! ## Related Fields Integration
//! - **Field 32A**: Value Date, Currency, Settlement Amount (may differ from Field 19)
//! - **Field 32B**: Transaction Amount (individual amounts that sum to Field 19)
//! - **Field 71A**: Details of Charges (affects relationship between 19 and 32A)
//! - **Field 33B**: Instructed Amount (in multi-currency scenarios)
//!
//! ## Compliance and Audit
//! - **Reconciliation Records**: Maintains audit trail for amount differences
//! - **Regulatory Reporting**: Supports accurate reporting of transaction vs. settlement amounts
//! - **Internal Controls**: Enables proper validation of batch payment processing
//! - **Exception Handling**: Facilitates investigation of amount discrepancies
//!
//! ## See Also
//! - Swift FIN User Handbook: Amount Field Specifications
//! - MT102 Usage Rules: Settlement Details Sequence
//! - Currency Code Standards: Decimal Precision Requirements
//! - Batch Payment Guidelines: Amount Reconciliation Procedures

use swift_mt_message_macros::serde_swift_fields;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// **Field 19: Sum of Amounts**
///
/// Transaction sum variant of [Field 19 module](index.html). Specifies the sum of all individual
/// transaction amounts in sequence transactions for reconciliation and validation.
///
/// **Components:**
/// - Amount (17d, up to 17 digits with decimal comma)
///
/// For complete documentation, see the [Field 19 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field19 {
    /// Sum of all transaction amounts in the sequence
    ///
    /// Format: 17d - Up to 17 digits with decimal comma
    /// Must equal sum of all Field 32B amounts in sequence
    /// Precision must match currency in Field 32A
    #[component("17d")]
    pub amount: f64,
}
