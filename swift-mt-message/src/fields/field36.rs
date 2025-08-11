use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

///   **Field 36: Exchange Rate**
///
/// ## Purpose
/// Specifies the exchange rate used to convert the instructed currency amount to the
/// settlement currency amount in cross-currency transactions. This field is critical
/// for currency conversion calculations, enabling precise conversion between different
/// currencies while maintaining audit trails and enabling proper reconciliation.
///
/// ## Format
/// - **Swift Format**: `12d`
/// - **Description**: Decimal rate with comma as decimal separator
/// - **Precision**: Up to 12 digits including decimal places
/// - **Rate Expression**: Direct rate from instructed currency to settlement currency
///
/// ## Presence
/// - **Status**: Mandatory when currency conversion is performed on sender's side
/// - **Swift Error Codes**: T40 (invalid rate), T51 (format violation), T43 (rate validation)
/// - **Usage Context**: Cross-currency payment processing and currency conversion
///
/// ## Usage Rules
/// - **Conversion Requirement**: Required when Field 33B currency differs from Field 32A currency
/// - **Rate Direction**: Rate from instructed currency (33B) to settlement currency (32A)
/// - **Calculation Logic**: Instructed Amount × Exchange Rate = Settlement Amount (before charges)
/// - **Precision**: Must provide sufficient precision for accurate conversion
///
/// ## Network Validation Rules
/// - **Format Validation**: Must follow 12d format with decimal comma
/// - **Positive Rate**: Exchange rate must be greater than zero
/// - **Reasonable Range**: Rate must be within acceptable market ranges
/// - **Precision Rules**: Integer part must contain at least one digit
/// - **Decimal Requirement**: Decimal comma is mandatory even for whole numbers
///
/// ## Exchange Rate Calculation
/// ```logic
/// Basic Formula:
/// Field 33B Amount × Field 36 Rate = Converted Amount
/// Converted Amount ± Charges = Field 32A Amount
///
/// Example:
/// EUR 1,000.00 × 1,2500 = USD 1,250.00
/// USD 1,250.00 - USD 25.00 (charges) = USD 1,225.00 (settlement)
/// ```
///
/// ## Business Context
/// - **Currency Conversion**: Essential for multi-currency transaction processing
/// - **Market Rates**: Reflects prevailing market exchange rates at execution time
/// - **Risk Management**: Enables proper currency risk assessment and hedging
/// - **Reconciliation**: Provides audit trail for currency conversion calculations
///
/// ## Examples
/// ```logic
/// :36:1,2500          // EUR to USD rate: 1 EUR = 1.2500 USD
/// :36:0,8500          // USD to EUR rate: 1 USD = 0.8500 EUR
/// :36:110,2500        // USD to JPY rate: 1 USD = 110.2500 JPY
/// :36:1,3250          // GBP to USD rate: 1 GBP = 1.3250 USD
/// ```
///
/// ## Rate Types and Sources
/// - **Market Rates**: Current interbank market rates
/// - **Customer Rates**: Institution-specific customer rates
/// - **Fixed Rates**: Predetermined contractual rates
/// - **Spot Rates**: Real-time market rates for immediate settlement
///
/// ## Rate Precision Considerations
/// - **Major Currencies**: Typically 4-6 decimal places (EUR/USD, GBP/USD)
/// - **Emerging Markets**: May require higher precision for accuracy
/// - **Cross Rates**: Calculated rates may need additional precision
/// - **Rounding Rules**: Institutional rounding policies for rate application
///
/// ## Regional Considerations
/// - **European Markets**: EUR cross-rates and ECB reference rates
/// - **US Markets**: USD-based rates and Federal Reserve considerations
/// - **Asian Markets**: Local currency rates and central bank policies
/// - **Emerging Markets**: Volatility considerations and rate validation
///
/// ## Error Prevention
/// - **Rate Validation**: Verify rate is within reasonable market ranges
/// - **Currency Pair Check**: Ensure rate applies to correct currency pair
/// - **Precision Verification**: Confirm adequate precision for accurate conversion
/// - **Market Validation**: Check rate against current market conditions
///
/// ## Related Fields
/// - **Field 33B**: Currency/Instructed Amount (source currency and amount)
/// - **Field 32A**: Value Date, Currency, Amount (target currency and amount)
/// - **Field 71F**: Sender's Charges (deductions from converted amount)
/// - **Field 71G**: Receiver's Charges (additions to final amount)
///
/// ## Conversion Flow
/// 1. **Source**: Field 33B provides original currency and amount
/// 2. **Conversion**: Field 36 rate applied to convert currency
/// 3. **Charges**: Fields 71F/71G adjust for transaction charges
/// 4. **Settlement**: Field 32A shows final currency and amount
///
/// ## Market Rate Management
/// - **Rate Sources**: Reuters, Bloomberg, central bank rates
/// - **Rate Timing**: Execution time, value date, or agreed timing
/// - **Rate Updates**: Real-time or periodic rate refreshes
/// - **Rate Validation**: Market reasonableness checks
///
/// ## STP Processing
/// - **Automated Conversion**: System-driven rate application and calculation
/// - **Rate Validation**: Real-time market rate validation
/// - **Exception Handling**: Automated detection of unreasonable rates
/// - **Quality Control**: Continuous monitoring of conversion accuracy
///
/// ## Compliance Framework
/// - **Regulatory Rates**: Central bank or regulatory mandated rates
/// - **Audit Requirements**: Complete rate documentation and justification
/// - **Market Conduct**: Fair and reasonable rate application
/// - **Documentation**: Proper rate source and timing documentation
///
/// ## Risk Management
/// - **Currency Risk**: Exposure assessment and hedging implications
/// - **Market Risk**: Rate volatility and timing considerations
/// - **Operational Risk**: Rate accuracy and conversion precision
/// - **Compliance Risk**: Regulatory rate requirements and documentation
///
/// ## See Also
/// - Swift FIN User Handbook: Exchange Rate Specifications
/// - Currency Conversion Guidelines: Rate Application Standards
/// - Market Rate Sources: Authorized Rate Providers
/// - Risk Management: Currency Conversion Risk Controls
///   **Field 36: Exchange Rate Structure**
///
/// Contains the exchange rate for currency conversion calculations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field36 {
    /// Exchange rate for currency conversion
    ///
    /// Format: 12d - Decimal rate with comma separator (up to 12 digits)
    /// Rate from instructed currency (Field 33B) to settlement currency (Field 32A)
    /// Must be positive and within reasonable market ranges
    #[component("12d")]
    pub rate: f64,
}
