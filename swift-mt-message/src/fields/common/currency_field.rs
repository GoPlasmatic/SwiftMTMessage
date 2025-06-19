use crate::{SwiftField, ValidationError, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Generic Currency Amount Field
///
/// ## Overview
/// A generic field structure for SWIFT currency and amount fields that follow the
/// `3!a15d` pattern (3-character currency code + decimal amount). This structure
/// consolidates the common functionality used by Field33B, Field71F, and Field71G.
///
/// ## Format Specification
/// **Format**: `3!a15d`
/// - **3!a**: Currency code (3 alphabetic characters, ISO 4217)
/// - **15d**: Amount with up to 15 digits including decimal places
///
/// ### Component Details
/// 1. **Currency Code (3!a)**:
///    - ISO 4217 standard currency codes
///    - Exactly 3 alphabetic characters
///    - Case-insensitive input, stored as uppercase
///    - Must be valid and active currency code
///    - Examples: USD, EUR, GBP, JPY, CHF
///
/// 2. **Amount (15d)**:
///    - Up to 15 digits including decimal places
///    - Decimal separator: comma (,) in SWIFT format
///    - No thousands separators allowed
///    - Must be non-negative (≥ 0)
///    - Precision varies by currency (typically 2 decimal places)
///
/// ## Usage Context
/// Used in various SWIFT MT message types for monetary amounts:
/// - **Field 32A**: Value Date, Currency Code, Amount (settlement amount)
/// - **Field 33B**: Currency/Instructed Amount (original amount)
/// - **Field 71F**: Sender's Charges (charge amounts)
/// - **Field 71G**: Receiver's Charges (charge amounts)
///
/// ## Usage Examples
/// ```text
/// USD1234567,89
/// └─── USD 1,234,567.89
///
/// EUR500000,00
/// └─── EUR 500,000.00
///
/// JPY1000000
/// └─── JPY 1,000,000 (no decimal places)
///
/// GBP75000,50
/// └─── GBP 75,000.50
/// ```
///
/// ## Validation Rules
/// 1. **Currency format**: Must be exactly 3 alphabetic characters
/// 2. **Currency validity**: Should be valid ISO 4217 code
/// 3. **Amount format**: Must follow SWIFT decimal format (comma separator)
/// 4. **Amount value**: Must be non-negative (zero allowed for certain scenarios)
/// 5. **Precision**: Should match currency-specific decimal place rules
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Currency code must be exactly 3 characters (Error: T52)
/// - Currency must be valid ISO 4217 code (Error: T52)
/// - Amount must be properly formatted (Error: T40)
/// - Amount cannot be negative (Error: T13)
/// - Decimal separator must be comma (Error: T41)
/// - Maximum 15 digits in amount (Error: T50)
/// - Currency must be alphabetic only (Error: T15)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenericCurrencyAmountField {
    /// Currency code (3 letters, ISO 4217)
    pub currency: String,
    /// Amount value as floating point
    pub amount: f64,
    /// Raw amount string as received (preserves original formatting)
    pub raw_amount: String,
}

impl GenericCurrencyAmountField {
    /// Create a new GenericCurrencyAmountField with validation
    ///
    /// # Arguments
    /// * `currency` - ISO 4217 currency code (will be converted to uppercase)
    /// * `amount` - Amount value (must be non-negative)
    ///
    /// # Returns
    /// Result containing the GenericCurrencyAmountField instance or validation error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::GenericCurrencyAmountField;
    /// let field = GenericCurrencyAmountField::new("USD", 1234.56).unwrap();
    /// assert_eq!(field.currency(), "USD");
    /// assert_eq!(field.amount(), 1234.56);
    /// ```
    pub fn new(currency: impl Into<String>, amount: f64) -> Result<Self, ParseError> {
        let currency = currency.into().to_uppercase();

        // Validate currency code
        if currency.len() != 3 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericCurrencyAmountField".to_string(),
                message: "Currency code must be exactly 3 characters".to_string(),
            });
        }

        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericCurrencyAmountField".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount
        if amount < 0.0 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericCurrencyAmountField".to_string(),
                message: "Amount cannot be negative".to_string(),
            });
        }

        let raw_amount = Self::format_amount(amount);

        Ok(GenericCurrencyAmountField {
            currency,
            amount,
            raw_amount,
        })
    }

    /// Create from raw amount string
    ///
    /// # Arguments
    /// * `currency` - ISO 4217 currency code
    /// * `raw_amount` - Raw amount string (preserves original formatting)
    ///
    /// # Returns
    /// Result containing the GenericCurrencyAmountField instance or validation error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::GenericCurrencyAmountField;
    /// let field = GenericCurrencyAmountField::from_raw("EUR", "1000,50").unwrap();
    /// assert_eq!(field.raw_amount(), "1000,50");
    /// ```
    pub fn from_raw(
        currency: impl Into<String>,
        raw_amount: impl Into<String>,
    ) -> Result<Self, ParseError> {
        let currency = currency.into().to_uppercase();
        let raw_amount = raw_amount.into();

        // Validate currency
        if currency.len() != 3 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericCurrencyAmountField".to_string(),
                message: "Currency code must be exactly 3 characters".to_string(),
            });
        }

        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericCurrencyAmountField".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        let amount = Self::parse_amount(&raw_amount)?;

        Ok(GenericCurrencyAmountField {
            currency,
            amount,
            raw_amount: raw_amount.to_string(),
        })
    }

    /// Get the currency code
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Get the amount value
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
    fn parse_amount(amount_str: &str) -> Result<f64, ParseError> {
        let normalized_amount = amount_str.replace(',', ".");

        normalized_amount
            .parse::<f64>()
            .map_err(|_| ParseError::InvalidFieldFormat {
                field_tag: "GenericCurrencyAmountField".to_string(),
                message: "Invalid amount format".to_string(),
            })
    }

    /// Parse content with custom field tag for error messages
    pub fn parse_with_tag(content: &str, field_tag: &str) -> Result<Self, ParseError> {
        let content = content.trim();
        if content.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: field_tag.to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Remove field tag prefix if present
        let content = if let Some(stripped) = content.strip_prefix(&format!(":{}:", field_tag)) {
            stripped
        } else if let Some(stripped) = content.strip_prefix(&format!("{}:", field_tag)) {
            stripped
        } else {
            content
        };

        if content.len() < 4 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: field_tag.to_string(),
                message: "Field content too short (minimum 4 characters: CCCAMOUNT)".to_string(),
            });
        }

        // Parse components: first 3 characters are currency, rest is amount
        let currency_str = &content[0..3];
        let amount_str = &content[3..];

        let currency = currency_str.to_uppercase();

        // Validate currency
        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: field_tag.to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        let amount = Self::parse_amount(amount_str).map_err(|e| {
            if let ParseError::InvalidFieldFormat {
                field_tag: _,
                message,
            } = e
            {
                ParseError::InvalidFieldFormat {
                    field_tag: field_tag.to_string(),
                    message,
                }
            } else {
                e
            }
        })?;

        if amount < 0.0 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: field_tag.to_string(),
                message: "Amount cannot be negative".to_string(),
            });
        }

        Ok(GenericCurrencyAmountField {
            currency,
            amount,
            raw_amount: amount_str.to_string(),
        })
    }

    /// Convert to SWIFT string format with custom field tag
    pub fn to_swift_string_with_tag(&self, field_tag: &str) -> String {
        format!(":{}:{}{}", field_tag, self.currency, self.raw_amount)
    }

    /// Check if this is a valid ISO 4217 currency code (basic validation)
    pub fn is_valid_currency(&self) -> bool {
        self.currency.len() == 3 && self.currency.chars().all(|c| c.is_alphabetic())
    }

    /// Check if the currency is a major currency
    pub fn is_major_currency(&self) -> bool {
        matches!(
            self.currency.as_str(),
            "USD" | "EUR" | "GBP" | "JPY" | "CHF" | "CAD" | "AUD" | "NZD" | "SEK" | "NOK" | "DKK"
        )
    }

    /// Check if the currency typically has decimal places
    pub fn has_decimal_places(&self) -> bool {
        !matches!(
            self.currency.as_str(),
            "JPY" | "KRW" | "VND" | "IDR" | "CLP" | "PYG" | "UGX" | "RWF" | "GNF" | "MGA"
        )
    }

    /// Get the typical decimal places for this currency
    pub fn decimal_places(&self) -> u8 {
        match self.currency.as_str() {
            // Currencies with no decimal places
            "JPY" | "KRW" | "VND" | "IDR" | "CLP" | "PYG" | "UGX" | "RWF" | "GNF" | "MGA" => 0,
            // Currencies with 3 decimal places
            "BHD" | "IQD" | "JOD" | "KWD" | "LYD" | "OMR" | "TND" => 3,
            // Most currencies use 2 decimal places
            _ => 2,
        }
    }

    /// Check if the amount is a high-value transaction
    pub fn is_high_value_transaction(&self) -> bool {
        // Convert to USD equivalent for comparison (simplified)
        let usd_equivalent = match self.currency.as_str() {
            "EUR" => self.amount * 1.1,   // Approximate EUR to USD
            "GBP" => self.amount * 1.25,  // Approximate GBP to USD
            "JPY" => self.amount * 0.007, // Approximate JPY to USD
            "CHF" => self.amount * 1.08,  // Approximate CHF to USD
            _ => self.amount,             // Assume USD or treat as USD equivalent
        };

        usd_equivalent >= 1_000_000.0 // $1M threshold
    }

    /// Get human-readable description with custom context
    pub fn description(&self, context: &str) -> String {
        format!("{}: {} {}", context, self.currency, self.raw_amount)
    }
}

impl SwiftField for GenericCurrencyAmountField {
    fn parse(content: &str) -> Result<Self, ParseError> {
        // Extract field tag from content if present
        let field_tag = if content.starts_with(":33B:") {
            "33B"
        } else if content.starts_with(":71F:") {
            "71F"
        } else if content.starts_with(":71G:") {
            "71G"
        } else {
            "GenericCurrencyAmountField"
        };

        Self::parse_with_tag(content, field_tag)
    }

    fn to_swift_string(&self) -> String {
        self.to_swift_string_with_tag("GenericCurrencyAmountField")
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate currency code
        if self.currency.len() != 3 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "GenericCurrencyAmountField".to_string(),
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
                field_tag: "GenericCurrencyAmountField".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount
        if self.amount < 0.0 {
            errors.push(ValidationError::ValueValidation {
                field_tag: "GenericCurrencyAmountField".to_string(),
                message: "Amount cannot be negative".to_string(),
            });
        }

        // Validate raw amount format
        if self.raw_amount.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "GenericCurrencyAmountField".to_string(),
                message: "Amount cannot be empty".to_string(),
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

impl std::fmt::Display for GenericCurrencyAmountField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.currency, self.raw_amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_currency_amount_field_creation() {
        let field = GenericCurrencyAmountField::new("USD", 1234.56).unwrap();
        assert_eq!(field.currency(), "USD");
        assert_eq!(field.amount(), 1234.56);
        assert_eq!(field.raw_amount(), "1234,56");
    }

    #[test]
    fn test_generic_currency_amount_field_from_raw() {
        let field = GenericCurrencyAmountField::from_raw("EUR", "1000,50").unwrap();
        assert_eq!(field.currency(), "EUR");
        assert_eq!(field.amount(), 1000.50);
        assert_eq!(field.raw_amount(), "1000,50");
    }

    #[test]
    fn test_generic_currency_amount_field_parse_with_tag() {
        let field = GenericCurrencyAmountField::parse_with_tag("USD1234,56", "33B").unwrap();
        assert_eq!(field.currency(), "USD");
        assert_eq!(field.amount(), 1234.56);
    }

    #[test]
    fn test_generic_currency_amount_field_to_swift_string_with_tag() {
        let field = GenericCurrencyAmountField::new("EUR", 1000.00).unwrap();
        assert_eq!(field.to_swift_string_with_tag("33B"), ":33B:EUR1000,00");
    }

    #[test]
    fn test_generic_currency_amount_field_validation_errors() {
        // Invalid currency length
        let result = GenericCurrencyAmountField::new("US", 1000.0);
        assert!(result.is_err());

        // Invalid currency characters
        let result = GenericCurrencyAmountField::new("US1", 1000.0);
        assert!(result.is_err());

        // Negative amount
        let result = GenericCurrencyAmountField::new("USD", -100.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_generic_currency_amount_field_currency_properties() {
        let usd_field = GenericCurrencyAmountField::new("USD", 1000.0).unwrap();
        assert!(usd_field.is_valid_currency());
        assert!(usd_field.is_major_currency());
        assert!(usd_field.has_decimal_places());
        assert_eq!(usd_field.decimal_places(), 2);

        let jpy_field = GenericCurrencyAmountField::new("JPY", 1000.0).unwrap();
        assert!(!jpy_field.has_decimal_places());
        assert_eq!(jpy_field.decimal_places(), 0);

        let bhd_field = GenericCurrencyAmountField::new("BHD", 1000.0).unwrap();
        assert_eq!(bhd_field.decimal_places(), 3);
    }

    #[test]
    fn test_generic_currency_amount_field_high_value_transaction() {
        let high_value = GenericCurrencyAmountField::new("USD", 1500000.0).unwrap();
        assert!(high_value.is_high_value_transaction());

        let normal_value = GenericCurrencyAmountField::new("USD", 50000.0).unwrap();
        assert!(!normal_value.is_high_value_transaction());
    }

    #[test]
    fn test_generic_currency_amount_field_display() {
        let field = GenericCurrencyAmountField::new("CHF", 2500.75).unwrap();
        assert_eq!(format!("{}", field), "CHF 2500,75");
    }

    #[test]
    fn test_generic_currency_amount_field_description() {
        let field = GenericCurrencyAmountField::new("EUR", 1500.0).unwrap();
        assert_eq!(field.description("Test Amount"), "Test Amount: EUR 1500,00");
    }

    #[test]
    fn test_generic_currency_amount_field_parse_dot_decimal() {
        let field = GenericCurrencyAmountField::parse_with_tag("USD1234.56", "test").unwrap();
        assert_eq!(field.amount(), 1234.56);
        assert_eq!(field.raw_amount(), "1234.56");
    }

    #[test]
    fn test_generic_currency_amount_field_format_amount() {
        let formatted = GenericCurrencyAmountField::format_amount(1234.56);
        assert_eq!(formatted, "1234,56");
    }

    #[test]
    fn test_generic_currency_amount_field_validation() {
        let field = GenericCurrencyAmountField::new("USD", 1000.0).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }
}
