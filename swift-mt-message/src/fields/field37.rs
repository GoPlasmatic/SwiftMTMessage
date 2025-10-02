use super::swift_utils::parse_amount;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field37H {
    /// Rate type indicator
    ///
    /// Format: 1!a - 'C' (Credit rate) or 'D' (Debit rate)
    /// Specifies whether rate applies to credit or debit transactions
    pub rate_indicator: char,

    /// Negative rate indicator
    ///
    /// Format: [1!a] - Optional indicator for negative interest rates
    /// True when rate is negative (below zero), None for positive rates
    pub is_negative: Option<bool>,

    /// Interest rate value
    ///
    /// Format: 12d - Decimal rate with comma separator (typically percentage)
    /// Example: 2,5000 represents 2.5% interest rate
    pub rate: f64,
}

impl SwiftField for Field37H {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut remaining = input;

        // Parse rate indicator (1!a)
        if remaining.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field37H requires rate indicator".to_string(),
            });
        }

        let rate_indicator = remaining.chars().next().unwrap();
        if rate_indicator != 'C' && rate_indicator != 'D' {
            return Err(ParseError::InvalidFormat {
                message: "Field37H rate indicator must be 'C' or 'D'".to_string(),
            });
        }
        remaining = &remaining[1..];

        // Parse optional negative indicator ([1!a])
        let is_negative = if remaining.starts_with('N') {
            remaining = &remaining[1..];
            Some(true)
        } else {
            None
        };

        // Parse rate value (12d)
        if remaining.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field37H requires rate value".to_string(),
            });
        }

        let rate = if is_negative.is_some() {
            -parse_amount(remaining)?
        } else {
            parse_amount(remaining)?
        };

        Ok(Field37H {
            rate_indicator,
            is_negative,
            rate,
        })
    }

    fn to_swift_string(&self) -> String {
        let negative_indicator = if self.is_negative.is_some() { "N" } else { "" };
        let rate_str = format!("{:.4}", self.rate.abs()).replace('.', ",");
        format!(
            ":37H:{}{}{}",
            self.rate_indicator, negative_indicator, rate_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field37h_parse() {
        // Test positive credit rate
        let field = Field37H::parse("C2,5000").unwrap();
        assert_eq!(field.rate_indicator, 'C');
        assert_eq!(field.is_negative, None);
        assert_eq!(field.rate, 2.5);

        // Test positive debit rate
        let field = Field37H::parse("D3,7500").unwrap();
        assert_eq!(field.rate_indicator, 'D');
        assert_eq!(field.is_negative, None);
        assert_eq!(field.rate, 3.75);

        // Test negative credit rate
        let field = Field37H::parse("CN0,2500").unwrap();
        assert_eq!(field.rate_indicator, 'C');
        assert_eq!(field.is_negative, Some(true));
        assert_eq!(field.rate, -0.25);
    }

    #[test]
    fn test_field37h_to_swift_string() {
        let field = Field37H {
            rate_indicator: 'C',
            is_negative: None,
            rate: 2.5,
        };
        assert_eq!(field.to_swift_string(), ":37H:C2,5000");

        let field = Field37H {
            rate_indicator: 'D',
            is_negative: None,
            rate: 3.75,
        };
        assert_eq!(field.to_swift_string(), ":37H:D3,7500");

        let field = Field37H {
            rate_indicator: 'C',
            is_negative: Some(true),
            rate: -0.25,
        };
        assert_eq!(field.to_swift_string(), ":37H:CN0,2500");
    }

    #[test]
    fn test_field37h_parse_invalid() {
        // Invalid rate indicator
        assert!(Field37H::parse("X2,5000").is_err());

        // Missing rate
        assert!(Field37H::parse("C").is_err());

        // Invalid rate format
        assert!(Field37H::parse("Cabc").is_err());

        // Empty input
        assert!(Field37H::parse("").is_err());
    }
}
