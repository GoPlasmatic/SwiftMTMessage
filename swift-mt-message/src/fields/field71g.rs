use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 71G: Receiver's Charges
///
/// ## Overview
/// Field 71G specifies the charges borne by the receiver in SWIFT payment messages. This field
/// contains the currency and amount of charges that the beneficiary or receiving institution
/// pays for processing the payment transaction. These charges are deducted from the payment
/// amount or billed separately, providing transparency in fee allocation and supporting
/// accurate payment reconciliation and regulatory compliance requirements.
///
/// ## Format Specification
/// **Format**: `3!a15d`
/// - **3!a**: Currency code (3 alphabetic characters, ISO 4217)
/// - **15d**: Amount with up to 15 digits (including decimal places)
/// - **Decimal separator**: Comma (,) as per SWIFT standards
/// - **Amount format**: No thousands separators, up to 2 decimal places
///
/// ## Structure
/// ```text
/// EUR12,75
/// │││└──┘
/// │││  └─ Amount (12.75)
/// └┴┴─── Currency (EUR)
/// ```
///
/// ## Field Components
/// - **Currency Code**: ISO 4217 three-letter currency code
///   - Must be valid and recognized currency
///   - Alphabetic characters only
///   - Case-insensitive but normalized to uppercase
/// - **Charge Amount**: Monetary amount of receiver's charges
///   - Maximum 15 digits including decimal places
///   - Comma as decimal separator
///   - Non-negative values only
///
/// ## Usage Context
/// Field 71G is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Charge transparency**: Detailed fee disclosure for receivers
/// - **Payment reconciliation**: Accurate net amount calculation
/// - **Correspondent banking**: Fee settlement with receiving banks
/// - **Regulatory compliance**: Charge reporting and disclosure
/// - **Customer communication**: Clear fee breakdown for beneficiaries
/// - **Audit trails**: Complete transaction cost documentation
///
/// ## Examples
/// ```text
/// :71G:EUR12,75
/// └─── EUR 12.75 in receiver's charges
///
/// :71G:USD20,00
/// └─── USD 20.00 in beneficiary bank fees
///
/// :71G:GBP8,50
/// └─── GBP 8.50 in receiving charges
///
/// :71G:CHF15,25
/// └─── CHF 15.25 in processing fees
///
/// :71G:JPY1500,00
/// └─── JPY 1,500.00 in local charges
/// ```
///
/// ## Charge Types
/// - **Receiving fees**: Basic charges for incoming payments
/// - **Processing charges**: Fees for payment processing and crediting
/// - **Correspondent fees**: Charges from correspondent banking arrangements
/// - **Regulatory fees**: Compliance and reporting related charges
/// - **Investigation fees**: Charges for payment inquiries or research
/// - **Account maintenance**: Fees related to account services
///
/// ## Currency Guidelines
/// - **ISO 4217 compliance**: Must use standard currency codes
/// - **Local currency**: Often in receiving country's currency
/// - **Payment currency**: May match main payment currency
/// - **Active currencies**: Should use currently active currency codes
/// - **Consistency**: Should align with local banking practices
///
/// ## Amount Calculation
/// - **Deduction method**: Typically deducted from payment amount
/// - **Separate billing**: May be billed separately to beneficiary
/// - **Net amount**: Payment amount minus receiver's charges
/// - **Currency conversion**: May involve currency conversion costs
/// - **Rate application**: Applied at current exchange rates
///
/// ## Validation Rules
/// 1. **Currency format**: Must be exactly 3 alphabetic characters
/// 2. **Currency validity**: Must be valid ISO 4217 currency code
/// 3. **Amount format**: Must follow SWIFT decimal format with comma
/// 4. **Amount range**: Must be non-negative
/// 5. **Length limits**: Total field length within SWIFT limits
/// 6. **Character validation**: Only allowed characters in amount
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Currency must be valid ISO 4217 code (Error: T52)
/// - Amount must be properly formatted (Error: T40)
/// - Amount cannot be negative (Error: T13)
/// - Decimal separator must be comma (Error: T41)
/// - Maximum 15 digits in amount (Error: T50)
/// - Currency must be alphabetic only (Error: T15)
/// - Field format must comply with specification (Error: T26)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field71G {
    /// Currency code (3 letters, ISO 4217)
    pub currency: String,
    /// Charge amount
    pub amount: f64,
    /// Raw amount string as received (preserves original formatting)
    pub raw_amount: String,
}

impl Field71G {
    /// Create a new Field71G with validation
    pub fn new(currency: impl Into<String>, amount: f64) -> Result<Self, crate::ParseError> {
        let currency = currency.into().to_uppercase();

        // Validate currency code
        if currency.len() != 3 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "71G".to_string(),
                message: "Currency code must be exactly 3 characters".to_string(),
            });
        }

        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "71G".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount
        if amount < 0.0 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "71G".to_string(),
                message: "Charge amount cannot be negative".to_string(),
            });
        }

        let raw_amount = Self::format_amount(amount);

        Ok(Field71G {
            currency,
            amount,
            raw_amount,
        })
    }

    /// Create from raw amount string
    pub fn from_raw(
        currency: impl Into<String>,
        raw_amount: impl Into<String>,
    ) -> Result<Self, crate::ParseError> {
        let currency = currency.into().to_uppercase();
        let raw_amount = raw_amount.into();

        let amount = Self::parse_amount(&raw_amount)?;

        Ok(Field71G {
            currency,
            amount,
            raw_amount: raw_amount.to_string(),
        })
    }

    /// Get the currency code
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Get the charge amount
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Get the raw amount string
    pub fn raw_amount(&self) -> &str {
        &self.raw_amount
    }

    /// Format amount for SWIFT output (with comma as decimal separator)
    pub fn format_amount(amount: f64) -> String {
        format!("{:.2}", amount).replace('.', ",")
    }

    /// Parse amount from string (handles both comma and dot as decimal separator)
    fn parse_amount(amount_str: &str) -> Result<f64, crate::ParseError> {
        let normalized_amount = amount_str.replace(',', ".");

        normalized_amount
            .parse::<f64>()
            .map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "71G".to_string(),
                message: "Invalid charge amount format".to_string(),
            })
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!("Receiver's Charges: {} {}", self.currency, self.raw_amount)
    }
}

impl SwiftField for Field71G {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":71G:") {
            stripped // Remove ":71G:" prefix
        } else if let Some(stripped) = value.strip_prefix("71G:") {
            stripped // Remove "71G:" prefix
        } else {
            value
        };

        let content = content.trim();

        if content.len() < 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "71G".to_string(),
                message: "Field content too short (minimum 4 characters: CCCAMOUNT)".to_string(),
            });
        }

        // Parse components: first 3 characters are currency, rest is amount
        let currency_str = &content[0..3];
        let amount_str = &content[3..];

        let currency = currency_str.to_uppercase();

        // Validate currency
        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "71G".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        let amount = Self::parse_amount(amount_str)?;

        if amount < 0.0 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "71G".to_string(),
                message: "Charge amount cannot be negative".to_string(),
            });
        }

        Ok(Field71G {
            currency,
            amount,
            raw_amount: amount_str.to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":71G:{}{}", self.currency, self.raw_amount)
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate currency code
        if self.currency.len() != 3 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "71G".to_string(),
                expected: "3 characters".to_string(),
                actual: self.currency.len(),
            });
        }

        if !self
            .currency
            .chars()
            .all(|c| c.is_alphabetic() && c.is_ascii())
        {
            errors.push(ValidationError::FormatValidation {
                field_tag: "71G".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount
        if self.amount < 0.0 {
            errors.push(ValidationError::ValueValidation {
                field_tag: "71G".to_string(),
                message: "Charge amount cannot be negative".to_string(),
            });
        }

        // Validate raw amount format
        if self.raw_amount.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "71G".to_string(),
                message: "Charge amount cannot be empty".to_string(),
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "3!a15d"
    }
}

impl std::fmt::Display for Field71G {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.currency, self.raw_amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field71g_creation() {
        let field = Field71G::new("USD", 10.50).unwrap();
        assert_eq!(field.currency(), "USD");
        assert_eq!(field.amount(), 10.50);
        assert_eq!(field.raw_amount(), "10,50");
    }

    #[test]
    fn test_field71g_from_raw() {
        let field = Field71G::from_raw("EUR", "25,75").unwrap();
        assert_eq!(field.currency(), "EUR");
        assert_eq!(field.amount(), 25.75);
        assert_eq!(field.raw_amount(), "25,75");
    }

    #[test]
    fn test_field71g_parse() {
        let field = Field71G::parse("USD15,00").unwrap();
        assert_eq!(field.currency(), "USD");
        assert_eq!(field.amount(), 15.0);
        assert_eq!(field.raw_amount(), "15,00");
    }

    #[test]
    fn test_field71g_parse_with_prefix() {
        let field = Field71G::parse(":71G:GBP5,25").unwrap();
        assert_eq!(field.currency(), "GBP");
        assert_eq!(field.amount(), 5.25);
        assert_eq!(field.raw_amount(), "5,25");
    }

    #[test]
    fn test_field71g_to_swift_string() {
        let field = Field71G::new("CHF", 100.0).unwrap();
        assert_eq!(field.to_swift_string(), ":71G:CHF100,00");
    }

    #[test]
    fn test_field71g_invalid_currency() {
        let result = Field71G::new("US", 10.0);
        assert!(result.is_err());

        let result = Field71G::new("123", 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_field71g_negative_amount() {
        let result = Field71G::new("USD", -10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_field71g_validation() {
        let field = Field71G::new("USD", 50.0).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field71g_display() {
        let field = Field71G::new("EUR", 75.50).unwrap();
        assert_eq!(format!("{}", field), "EUR 75,50");
    }

    #[test]
    fn test_field71g_description() {
        let field = Field71G::new("USD", 20.0).unwrap();
        assert_eq!(field.description(), "Receiver's Charges: USD 20,00");
    }
}
