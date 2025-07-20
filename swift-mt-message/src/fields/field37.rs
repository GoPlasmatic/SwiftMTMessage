use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

///   **Field 37H: Interest Rate**
///
/// ## Purpose
/// Specifies interest rates for financial instruments, derivatives, and investment products.
/// This field family provides precise rate specifications required for interest calculations,
/// derivative pricing, option valuations, and various financial product definitions.
/// Interest rates are fundamental to financial market operations and risk management.
///
/// ## Format
/// - **Swift Format**: `1!a[N]12d` (Field 37H), `[N]12d` (Field 37R), `12d` (Field 37L)
/// - **Rate Indicator**: `1!a` - C (Credit) or D (Debit) for directional rates
/// - **Negative Sign**: `[N]` - Optional 'N' for negative rates
/// - **Rate Value**: `12d` - Decimal rate with comma separator
///
/// ## Presence
/// - **Status**: Conditional/Mandatory depending on instrument type and market requirements
/// - **Swift Error Codes**: T40 (invalid rate), T51 (format violation), T50 (invalid indicator)
/// - **Usage Context**: Interest rate specifications and derivative pricing
///
/// ## Usage Rules
/// - **Rate Expression**: Typically expressed as percentage (e.g., 2.5000 = 2.5%)
/// - **Sign Logic**: Negative rates supported in low interest rate environments
/// - **Precision**: Adequate precision for accurate interest calculations
/// - **Market Standards**: Compliance with market rate quotation conventions
///
/// ## Network Validation Rules
/// - **Format Validation**: Must follow exact rate format specifications
/// - **Rate Range**: Must be within reasonable market rate ranges
/// - **Precision Rules**: Integer part must contain at least one digit
/// - **Decimal Requirement**: Decimal comma mandatory for proper formatting
/// - **Negative Validation**: If rate is zero, negative sign must not be present
///
/// ## Field Variants and Applications
///
/// ### Field 37H - Interest Rate with Indicator
/// - **Format**: `1!a[N]12d`
/// - **Usage**: Directional interest rates with credit/debit specification
/// - **Indicator Logic**: C (Credit rate), D (Debit rate)
/// - **Applications**: Account interest, loan rates, deposit rates
///
/// ### Field 37R - Settlement Rate
/// - **Format**: `[N]12d`
/// - **Usage**: Settlement rates for derivatives and financial instruments
/// - **Applications**: Forward rate agreements, interest rate swaps
/// - **Context**: Final settlement rate determination
///
/// ### Field 37L - Lower Barrier Level
/// - **Format**: `12d`
/// - **Usage**: Barrier levels for structured products and options
/// - **Applications**: Knock-in/knock-out options, barrier derivatives
/// - **Context**: Risk management and option pricing
///
/// ## Business Context
/// - **Interest Calculations**: Precise interest accrual and payment calculations
/// - **Derivative Pricing**: Essential component of derivative valuation models
/// - **Risk Management**: Interest rate risk assessment and hedging
/// - **Market Operations**: Standard market rate communication and processing
///
/// ## Examples
/// ```logic
/// :37H:C2,5000        // 2.5% credit interest rate
/// :37H:D3,7500        // 3.75% debit interest rate
/// :37H:CN0,2500       // -0.25% negative credit rate
/// :37R:N0,1000        // -0.1% negative settlement rate
/// :37L:1,2500         // 1.25% lower barrier level
/// ```
///
/// ## Interest Rate Types
/// - **Fixed Rates**: Predetermined rates for entire term
/// - **Floating Rates**: Variable rates linked to reference rates
/// - **Negative Rates**: Below-zero rates in low interest environments
/// - **Barrier Levels**: Trigger levels for structured products
///
/// ## Rate Calculation Applications
/// - **Simple Interest**: Principal × Rate × Time
/// - **Compound Interest**: Principal × (1 + Rate)^Time
/// - **Day Count Conventions**: ACT/360, ACT/365, 30/360
/// - **Accrual Periods**: Daily, monthly, quarterly, annual
///
/// ## Regional Considerations
/// - **European Markets**: ECB rates, EURIBOR, negative rate environments
/// - **US Markets**: Federal funds rate, LIBOR transition, SOFR adoption
/// - **Asian Markets**: Local reference rates and central bank policies
/// - **Emerging Markets**: High volatility and inflation considerations
///
/// ## Rate Precision Standards
/// - **Standard Rates**: Typically 4 decimal places (basis points)
/// - **High Precision**: 6+ decimal places for complex calculations
/// - **Market Convention**: Alignment with market quoting standards
/// - **Regulatory Compliance**: Meeting precision requirements for reporting
///
/// ## Error Prevention
/// - **Rate Validation**: Verify rate is within reasonable market ranges
/// - **Sign Consistency**: Ensure negative sign usage is appropriate
/// - **Precision Check**: Confirm adequate precision for calculations
/// - **Market Alignment**: Validate rate against current market conditions
///
/// ## Related Fields
/// - **Field 30**: Date specifications (rate effective dates)
/// - **Field 32A**: Value Date, Currency, Amount (principal amounts)
/// - **Field 36**: Exchange Rate (currency conversion rates)
/// - **Derivative Terms**: Rate application and calculation periods
///
/// ## Interest Rate Environment
/// - **Normal Rates**: Positive interest rate environments
/// - **Zero Rates**: Zero interest rate policy (ZIRP) periods
/// - **Negative Rates**: Negative interest rate policy (NIRP) environments
/// - **Volatile Rates**: High volatility and uncertainty periods
///
/// ## STP Processing
/// - **Rate Standardization**: Consistent rate format for automation
/// - **Automated Calculations**: System-driven interest calculations
/// - **Validation Enhancement**: Real-time rate validation and verification
/// - **Exception Handling**: Automated detection of rate anomalies
///
/// ## Compliance Framework
/// - **Regulatory Rates**: Central bank and regulatory rate requirements
/// - **Market Conduct**: Fair and transparent rate setting and application
/// - **Documentation**: Comprehensive rate documentation and audit trails
/// - **Risk Management**: Rate risk assessment and control frameworks
///
/// ## Risk Management Applications
/// - **Interest Rate Risk**: Duration and convexity analysis
/// - **Credit Risk**: Credit spread and default rate considerations
/// - **Market Risk**: Rate volatility and scenario analysis
/// - **Operational Risk**: Rate accuracy and calculation precision
///
/// ## See Also
/// - Swift FIN User Handbook: Interest Rate Field Specifications
/// - Market Rate Standards: Rate Quotation and Calculation Conventions
/// - Central Bank Guidelines: Reference Rate Standards and Policies
/// - Risk Management: Interest Rate Risk Measurement and Control
///   **Field 37H: Interest Rate Structure**
///
/// Contains interest rate with directional indicator and negative rate support.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field37H {
    /// Rate type indicator
    ///
    /// Format: 1!a - 'C' (Credit rate) or 'D' (Debit rate)
    /// Specifies whether rate applies to credit or debit transactions
    #[component("1!a")]
    pub rate_indicator: char,

    /// Negative rate indicator
    ///
    /// Format: \[1!a\] - Optional indicator for negative interest rates
    /// True when rate is negative (below zero), None for positive rates
    #[component("[1!a]")]
    pub is_negative: Option<bool>,

    /// Interest rate value
    ///
    /// Format: 12d - Decimal rate with comma separator (typically percentage)
    /// Example: 2,5000 represents 2.5% interest rate
    #[component("12d")]
    pub rate: f64,
}
