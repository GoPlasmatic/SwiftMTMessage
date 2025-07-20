use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// **Field 34F: Floor Limit**
///
/// ## Purpose
/// Specifies the floor limit amount and currency for automatic processing thresholds
/// in financial transactions. This field defines minimum amounts for automated handling,
/// exception processing triggers, and regulatory compliance thresholds. Floor limits
/// enable institutions to manage risk and processing efficiency through amount-based routing.
///
/// ## Format
/// - **Swift Format**: `3!a[1!a]15d`
/// - **Currency Component**: `3!a` - ISO 4217 currency code
/// - **Indicator Component**: `[1!a]` - Optional floor limit indicator (D or C)
/// - **Amount Component**: `15d` - Decimal amount with comma separator
///
/// ## Presence
/// - **Status**: Optional in most contexts, mandatory for specific processing requirements
/// - **Swift Error Codes**: T52 (invalid currency), T40/T43 (invalid amount), T50 (invalid indicator)
/// - **Usage Context**: Automated processing thresholds and risk management
///
/// ## Usage Rules
/// - **Threshold Definition**: Defines minimum amount for specific processing treatment
/// - **Indicator Logic**: D (Debit) or C (Credit) specifies threshold direction
/// - **Currency Alignment**: Must align with transaction currency context
/// - **Processing Logic**: Determines automated vs. manual processing paths
///
/// ## Network Validation Rules
/// - **Currency Validation**: Must be valid ISO 4217 currency code
/// - **Amount Format**: Decimal comma mandatory, proper precision required
/// - **Positive Amount**: Amount must be greater than zero
/// - **Indicator Validation**: If present, must be 'D' (Debit) or 'C' (Credit)
/// - **Logic Validation**: Must be consistent with business processing rules
///
/// ## Floor Limit Indicators
///
/// ### D (Debit) Indicator
/// - **Purpose**: Threshold for debit transactions
/// - **Processing**: Amounts below threshold may receive automated processing
/// - **Risk Management**: Higher amounts trigger enhanced controls
/// - **Usage Context**: Payment instructions, fund transfers
///
/// ### C (Credit) Indicator
/// - **Purpose**: Threshold for credit transactions
/// - **Processing**: Credits below threshold may bypass certain checks
/// - **Compliance**: Regulatory thresholds for credit processing
/// - **Usage Context**: Incoming payments, deposit processing
///
/// ### No Indicator
/// - **Purpose**: General threshold regardless of direction
/// - **Processing**: Applies to both debit and credit transactions
/// - **Usage Context**: Universal processing limits
///
/// ## Business Context
/// - **Risk Management**: Automated risk assessment based on transaction amounts
/// - **Processing Efficiency**: Streamlined handling for routine transactions
/// - **Regulatory Compliance**: Meeting regulatory threshold requirements
/// - **Cost Management**: Efficient processing of high-volume, low-value transactions
///
/// ## Examples
/// ```logic
/// :34F:USD5000,00         // USD 5,000 general floor limit
/// :34F:USDD2500,00        // USD 2,500 debit threshold
/// :34F:EURC1000,00        // EUR 1,000 credit threshold
/// :34F:GBP10000,00        // GBP 10,000 general limit
/// ```
///
/// ## Processing Applications
/// - **Automated Processing**: Transactions below threshold receive automated handling
/// - **Manual Review**: Transactions above threshold require manual intervention
/// - **STP Qualification**: Floor limits determine STP eligibility
/// - **Exception Processing**: Threshold-based exception routing
///
/// ## Risk Management Integration
/// - **Amount-Based Controls**: Different control levels based on transaction size
/// - **Automated Monitoring**: System-driven monitoring for threshold breaches
/// - **Escalation Procedures**: Automatic escalation for amounts exceeding limits
/// - **Audit Trail**: Comprehensive logging of threshold applications
///
/// ## Regional Considerations
/// - **European Markets**: SEPA processing thresholds and regulations
/// - **US Markets**: Federal Reserve and ACH processing limits
/// - **Asian Markets**: Local regulatory threshold requirements
/// - **Cross-Border**: International processing limit coordination
///
/// ## Regulatory Framework
/// - **AML Thresholds**: Anti-money laundering reporting thresholds
/// - **KYC Requirements**: Know-your-customer enhanced due diligence limits
/// - **Sanctions Screening**: Enhanced screening for high-value transactions
/// - **Reporting Obligations**: Regulatory reporting threshold compliance
///
/// ## Error Prevention
/// - **Threshold Validation**: Verify floor limit is appropriate for context
/// - **Currency Consistency**: Ensure currency aligns with transaction context
/// - **Amount Verification**: Confirm amount format and precision
/// - **Business Logic**: Validate threshold makes business sense
///
/// ## Related Fields
/// - **Field 32A**: Value Date, Currency, Amount (transaction amount comparison)
/// - **Field 33B**: Currency/Instructed Amount (original amount context)
/// - **Field 71A**: Details of Charges (charge-related thresholds)
/// - **Processing Rules**: System configuration for threshold handling
///
/// ## Threshold Management
/// - **Dynamic Limits**: Ability to adjust thresholds based on risk profiles
/// - **Customer-Specific**: Different limits for different customer categories
/// - **Product-Specific**: Varying thresholds for different transaction types
/// - **Time-Based**: Different limits for different processing windows
///
/// ## STP Compliance
/// - **Threshold Standardization**: Consistent floor limit application
/// - **Automated Processing**: System-driven threshold evaluation
/// - **Exception Handling**: Automated routing based on threshold breaches
/// - **Quality Control**: Real-time threshold validation and application
///
/// ## Compliance and Audit
/// - **Regulatory Alignment**: Floor limits aligned with regulatory requirements
/// - **Audit Documentation**: Complete record of threshold applications
/// - **Investigation Support**: Threshold-based transaction analysis
/// - **Risk Assessment**: Regular review and adjustment of floor limits
///
/// ## See Also
/// - Swift FIN User Handbook: Floor Limit Specifications
/// - Risk Management Guidelines: Amount-Based Processing Controls
/// - Regulatory Standards: Transaction Threshold Requirements
/// - Processing Manuals: Automated vs. Manual Transaction Handling
/// **Field 34F: Floor Limit Structure**
///
/// Contains currency, optional indicator, and amount for processing threshold definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field34F {
    /// Currency code for floor limit
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must be valid currency for threshold processing context
    #[component("3!a")]
    pub currency: String,

    /// Floor limit indicator
    ///
    /// Format: \[1!a\] - Optional indicator: 'D' (Debit) or 'C' (Credit)
    /// Specifies whether threshold applies to debits, credits, or both (if omitted)
    #[component("[1!a]")]
    pub indicator: Option<String>,

    /// Floor limit amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Defines the threshold amount for automated processing decisions
    #[component("15d")]
    pub amount: f64,
}
