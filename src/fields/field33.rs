use super::swift_utils::{
    format_swift_amount_for_currency, parse_amount_with_currency, parse_currency_non_commodity,
};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 33B: Currency / Instructed Amount**
///
/// ## Purpose
/// Specifies the currency code and original instructed amount when the settlement amount
/// differs from the instructed amount due to currency conversion, exchange rate application,
/// or charge deductions. This field preserves the original instruction details for audit,
/// reconciliation, and regulatory reporting purposes.
///
/// ## Format
/// - **Swift Format**: `3!a15d`
/// - **Currency Component**: `3!a` - ISO 4217 currency code
/// - **Amount Component**: `15d` - Decimal amount with comma separator
/// - **Precision**: Follows currency-specific decimal place rules
///
/// ## Presence
/// - **Status**: Conditional - Required when currency conversion performed
/// - **Swift Error Codes**: T52 (invalid currency), T40/T43 (invalid amount), T51 (format error)
/// - **Usage Context**: Cross-currency transactions and charge applications
///
/// ## Usage Rules
/// - **Currency Conversion**: Mandatory when different from settlement currency
/// - **Amount Preservation**: Original instructed amount must be maintained
/// - **Forwarding**: Must be forwarded unchanged through transaction chain
/// - **Reconciliation**: Enables matching with original customer instructions
///
/// ## Network Validation Rules
/// - **Currency Validation**: Must be valid ISO 4217 currency code
/// - **Amount Format**: Decimal comma mandatory, proper precision required
/// - **Positive Amount**: Amount must be greater than zero
/// - **Precision Rules**: Must follow currency-specific decimal places
/// - **Format Compliance**: Exact adherence to Swift format specifications
///
/// ## Business Context
/// - **Cross-Currency Payments**: Preserves original currency and amount
/// - **Exchange Rate Application**: Shows amount before rate conversion
/// - **Charge Processing**: Amount before charge deductions
/// - **Multi-Currency Processing**: Original instruction currency preservation
///
/// ## Currency Conversion Logic
/// ```logic
/// Field 33B (Instructed Amount) × Field 36 (Exchange Rate) = Converted Amount
/// Converted Amount ± Charges = Field 32A (Settlement Amount)
/// ```
///
/// ## Examples
/// ```logic
/// :33B:USD1250,00     // Original instruction: USD 1,250.00
/// :33B:EUR950,50      // Original instruction: EUR 950.50
/// :33B:GBP750,25      // Original instruction: GBP 750.25
/// :33B:JPY125000      // Original instruction: JPY 125,000 (no decimals)
/// ```
///
/// ## Currency Precision by Type
/// - **Most Currencies**: 2 decimal places (USD, EUR, GBP, CHF, etc.)
/// - **Japanese Yen**: 0 decimal places (JPY)
/// - **Bahraini Dinar**: 3 decimal places (BHD)
/// - **Special Cases**: Currency-specific precision requirements
///
/// ## Transaction Flow Integration
/// - **Customer Instruction**: Original amount specified by customer
/// - **Bank Processing**: Conversion and charge application
/// - **Settlement**: Final amount after all adjustments
/// - **Reporting**: Original vs. settled amount reconciliation
///
/// ## Regional Considerations
/// - **European Payments**: EUR conversion requirements for SEPA
/// - **US Payments**: USD conversion for domestic processing
/// - **Asian Markets**: Local currency conversion needs
/// - **Cross-Border**: Multiple currency conversion scenarios
///
/// ## Error Prevention
/// - **Currency Validation**: Verify currency code is valid and supported
/// - **Amount Verification**: Confirm amount format and precision
/// - **Conversion Logic**: Ensure proper relationship with exchange rate
/// - **Forwarding Rules**: Maintain amount integrity through transaction chain
///
/// ## Related Fields
/// - **Field 32A**: Value Date, Currency, Settlement Amount (final amount)
/// - **Field 36**: Exchange Rate (conversion factor)
/// - **Field 71F**: Sender's Charges (deducted amounts)
/// - **Field 71G**: Receiver's Charges (additional charges)
///
/// ## Reconciliation Support
/// - **Amount Matching**: Links original instruction to settlement
/// - **Audit Trail**: Maintains complete transaction history
/// - **Variance Analysis**: Explains differences between instructed and settled
/// - **Compliance Reporting**: Supports regulatory reporting requirements
///
/// ## STP Processing
/// - **Format Standardization**: Consistent currency and amount formatting
/// - **Automated Conversion**: System-driven currency conversion processing
/// - **Validation Enhancement**: Real-time format and precision validation
/// - **Exception Handling**: Automated detection of conversion discrepancies
///
/// ## Compliance Framework
/// - **Regulatory Reporting**: Original instruction amount for compliance
/// - **AML Monitoring**: Enhanced monitoring of currency conversion patterns
/// - **Audit Documentation**: Complete record of instructed vs. settled amounts
/// - **Investigation Support**: Original instruction details for compliance reviews
///
/// ## Multi-Currency Scenarios
/// - **Trade Finance**: Original contract currency preservation
/// - **Treasury Operations**: Multi-currency deal processing
/// - **Corporate Payments**: Group company cross-currency transfers
/// - **Investment Services**: Portfolio currency conversion tracking
///
/// ## See Also
/// - Swift FIN User Handbook: Currency and Amount Specifications
/// - ISO 4217: Currency Code Standards
/// - Exchange Rate Guidelines: Conversion Calculation Rules
/// - Reconciliation Standards: Original vs. Settlement Amount Matching
///
///   **Field 33B: Currency/Instructed Amount Structure**
///
/// Contains the original instructed currency and amount before conversion
/// and charge applications.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field33B {
    /// Currency code of original instruction
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must be valid and supported currency for cross-border transactions
    pub currency: String,

    /// Original instructed amount
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Precision must match currency requirements (JPY=0, BHD=3, most=2)
    pub amount: f64,
}

impl SwiftField for Field33B {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Field33B format: 3!a15d (currency + amount)
        if input.len() < 4 {
            // Minimum: 3 chars currency + 1 digit amount
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 33B must be at least 4 characters, found {}",
                    input.len()
                ),
            });
        }

        // Parse currency code (first 3 characters) - T52 + C08 validation
        let currency = parse_currency_non_commodity(&input[0..3])?;

        // Parse amount (remaining characters) - T40/T43 + C03 validation
        let amount_str = &input[3..];
        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 33B amount cannot be empty".to_string(),
            });
        }

        let amount = parse_amount_with_currency(amount_str, &currency)?;

        // Amount must be positive
        if amount <= 0.0 {
            return Err(ParseError::InvalidFormat {
                message: "Field 33B amount must be greater than zero".to_string(),
            });
        }

        Ok(Field33B { currency, amount })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":33B:{}{}",
            self.currency,
            format_swift_amount_for_currency(self.amount, &self.currency)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field33b_valid() {
        let field = Field33B::parse("USD1250,00").unwrap();
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 1250.00);
        assert_eq!(field.to_swift_string(), ":33B:USD1250");

        let field = Field33B::parse("EUR950,50").unwrap();
        assert_eq!(field.currency, "EUR");
        assert_eq!(field.amount, 950.50);

        let field = Field33B::parse("JPY125000").unwrap();
        assert_eq!(field.currency, "JPY");
        assert_eq!(field.amount, 125000.0);
    }

    #[test]
    fn test_field33b_invalid() {
        // Invalid currency
        assert!(Field33B::parse("12A100").is_err());
        assert!(Field33B::parse("US100").is_err());

        // Zero amount
        assert!(Field33B::parse("USD0").is_err());

        // Negative amount
        assert!(Field33B::parse("USD-100").is_err());

        // Missing amount
        assert!(Field33B::parse("USD").is_err());
    }
}
