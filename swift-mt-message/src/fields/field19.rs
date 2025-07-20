use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// **Field 19: Sum of Amounts**
///
/// ## Purpose
/// Specifies the sum of all individual transaction amounts appearing in sequence transactions.
/// This field is essential for reconciliation and validation when the total transaction amount
/// differs from the settlement amount due to charging arrangements or fee allocations.
///
/// ## Format
/// - **Swift Format**: `17d`
/// - **Description**: Up to 17 digits including decimal places
/// - **Decimal**: Comma as decimal separator, included in maximum length
/// - **Precision**: Follows currency-specific decimal precision rules
///
/// ## Presence
/// - **Status**: Optional in MT102 Settlement Details sequence
/// - **Swift Error Codes**: C03, T40, T43 (amount validation), T51 (invalid amount format)
/// - **Referenced in Rule**: C1 (MT102 validation logic)
///
/// ## Usage Rules
/// - **Settlement Variance**: Used when sum of amounts differs from settlement amount in field 32A
/// - **Charge Allocation**: Applied when one or more transactions contain charging option "OUR" in field 71A
/// - **Transaction Summation**: Must equal sum of all field 32B amounts in each sequence B occurrence
/// - **Reconciliation**: Enables proper reconciliation between individual and total amounts
///
/// ## Network Validation Rules
/// - **Positive Amount**: Amount must be greater than zero
/// - **Integer Validation**: Integer part must contain at least one digit
/// - **Decimal Precision**: Number of digits after decimal comma must not exceed currency maximum
/// - **Format Compliance**: Must follow decimal amount formatting standards
/// - **Currency Alignment**: Precision must match currency specified in field 32A
///
/// ## Business Context
/// - **Multiple Payments**: Used in MT102 for multiple customer credit transfers
/// - **Charge Handling**: Accommodates scenarios where charges affect individual vs. total amounts
/// - **Settlement Logic**: Enables different settlement and transaction amounts
/// - **Batch Processing**: Supports batch payment scenarios with varying charge allocations
///
/// ## Amount Precision by Currency
/// - **Most Currencies**: 2 decimal places (USD, EUR, GBP, etc.)
/// - **Japanese Yen**: 0 decimal places (JPY)
/// - **Bahraini Dinar**: 3 decimal places (BHD)
/// - **Special Cases**: Some currencies have specific precision requirements
///
/// ## Examples
/// ```logic
/// :19:125000,50          // Sum of €1,250.50 in transactions
/// :19:1500000            // Sum of ¥1,500,000 (no decimals for JPY)
/// :19:75000,000          // Sum of BHD 75,000.000 (3 decimals)
/// :19:999999999999999,99 // Maximum precision example
/// ```
///
/// ## Usage Scenarios
/// - **Charge Deduction**: When "OUR" charges are deducted from individual transactions
/// - **Fee Allocation**: When fees are distributed across multiple transactions
/// - **Settlement Coordination**: When settlement amount differs from transaction total
/// - **Batch Reconciliation**: For validating batch payment totals
///
/// ## Calculation Logic
/// ```logic
/// Field 19 = Sum of all Field 32B amounts in sequence
///
/// If Field 19 ≠ Field 32A:
///   Difference typically represents charges or fees
///   
/// Validation:
///   Field 19 must equal Σ(Field 32B amounts)
/// ```
///
/// ## Regional Considerations
/// - **European Payments**: EUR precision and formatting rules
/// - **US Payments**: USD decimal handling and validation
/// - **Asian Markets**: Local currency precision requirements
/// - **Multi-Currency**: Handling different precision rules in same batch
///
/// ## Error Prevention
/// - **Precision Validation**: Ensure decimal places match currency requirements
/// - **Sum Verification**: Verify sum equals individual transaction amounts
/// - **Format Checking**: Confirm proper decimal formatting with comma separator
/// - **Range Validation**: Ensure amount is within reasonable business limits
///
/// ## Related Fields
/// - **Field 32A**: Value Date, Currency, Settlement Amount (may differ from Field 19)
/// - **Field 32B**: Transaction Amount (individual amounts that sum to Field 19)
/// - **Field 71A**: Details of Charges (affects relationship between 19 and 32A)
/// - **Field 33B**: Instructed Amount (in multi-currency scenarios)
///
/// ## STP Compliance
/// - **Automated Validation**: STP systems automatically validate sum calculations
/// - **Precision Requirements**: Enhanced precision validation for automated processing
/// - **Format Standardization**: Strict adherence to decimal formatting rules
/// - **Error Handling**: Automated rejection for calculation mismatches
///
/// ## Compliance and Audit
/// - **Reconciliation Records**: Maintains audit trail for amount differences
/// - **Regulatory Reporting**: Supports accurate reporting of transaction vs. settlement amounts
/// - **Internal Controls**: Enables proper validation of batch payment processing
/// - **Exception Handling**: Facilitates investigation of amount discrepancies
///
/// ## See Also
/// - Swift FIN User Handbook: Amount Field Specifications
/// - MT102 Usage Rules: Settlement Details Sequence
/// - Currency Code Standards: Decimal Precision Requirements
/// - Batch Payment Guidelines: Amount Reconciliation Procedures
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field19 {
    /// Sum of all transaction amounts in the sequence
    ///
    /// Format: 17d - Up to 17 digits with decimal comma
    /// Must equal sum of all Field 32B amounts in sequence
    /// Precision must match currency in Field 32A
    #[component("17d")]
    pub amount: f64,
}
