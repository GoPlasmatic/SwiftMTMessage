use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 33B: Currency/Instructed Amount
///
/// ## Overview
/// Field 33B specifies the original ordered amount in currency conversions and
/// multi-currency transactions. This field is used when the instructed amount differs
/// from the settlement amount, typically in foreign exchange transactions, currency
/// conversions, or when fees are deducted from the principal amount. It provides
/// transparency about the original instruction versus the actual settlement.
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
/// Field 33B appears in various SWIFT MT message types where currency conversion
/// or amount differentiation is required:
///
/// ### Primary Usage
/// - **MT103**: Single Customer Credit Transfer - when original amount differs from settlement
/// - **MT202**: General Financial Institution Transfer - for currency conversion scenarios
/// - **MT202COV**: Cover for customer credit transfer - original instructed amount
/// - **MT205**: Financial Institution Transfer - when amounts differ due to charges
///
/// ### Secondary Usage
/// - **MT400**: Advice of Payment - original payment instruction amount
/// - **MT410**: Acknowledgement - acknowledged original amount
/// - **MT420**: Tracer - original traced amount
/// - **MT900/910**: Confirmation messages - original instruction amount
///
/// ## Business Applications
/// - **Currency conversion**: Original amount before FX conversion
/// - **Charge deduction**: Principal amount before fee deduction
/// - **Multi-currency processing**: Cross-currency transaction handling
/// - **Reconciliation**: Matching original instructions with settlements
/// - **Audit trails**: Maintaining complete transaction history
/// - **Compliance reporting**: Regulatory reporting of original amounts
/// - **Customer transparency**: Showing original vs. settled amounts
/// - **FX risk management**: Tracking exposure in original currency
///
/// ## Related Fields
/// Field 33B works in conjunction with other amount fields:
///
/// ### Field 32A (Value Date/Currency/Amount)
/// - **32A**: Settlement amount and currency
/// - **33B**: Original instructed amount and currency
/// - **Relationship**: 33B shows original, 32A shows final settlement
///
/// ### Field 71A (Details of Charges)
/// - **71A**: Charge allocation (OUR/BEN/SHA)
/// - **33B**: Amount before charge deduction
/// - **Usage**: When charges affect the settlement amount
///
/// ### Field 36 (Exchange Rate)
/// - **36**: Exchange rate applied
/// - **33B**: Amount in original currency
/// - **Usage**: FX transactions showing rate and original amount
///
/// ## Currency Conversion Scenarios
/// 1. **Customer instructs**: EUR 100,000
/// 2. **Bank converts to**: USD 108,500 (at rate 1.085)
/// 3. **Field 33B**: EUR100000,00 (original instruction)
/// 4. **Field 32A**: USD108500,00 (settlement amount)
///
/// ## Charge Deduction Scenarios
/// 1. **Customer instructs**: USD 50,000
/// 2. **Bank deducts charges**: USD 25 (wire fee)
/// 3. **Field 33B**: USD50000,00 (original amount)
/// 4. **Field 32A**: USD49975,00 (net settlement)
///
/// ## Validation Rules
/// 1. **Currency format**: Must be exactly 3 alphabetic characters
/// 2. **Currency validity**: Should be valid ISO 4217 code
/// 3. **Amount format**: Must follow SWIFT decimal format (comma separator)
/// 4. **Amount value**: Must be non-negative (zero allowed for certain scenarios)
/// 5. **Precision**: Should match currency-specific decimal place rules
/// 6. **Consistency**: Should be logically consistent with Field 32A
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Currency code must be exactly 3 characters (Error: T52)
/// - Currency must be valid ISO 4217 code (Error: T52)
/// - Amount must be properly formatted (Error: T40)
/// - Amount cannot be negative (Error: T13)
/// - Field should be present when amounts differ (Warning: recommended)
/// - Currency should be actively traded (Warning: recommended)
///
///
/// ## Examples
/// ```text
/// :33B:EUR100000,00
/// └─── Original instruction: EUR 100,000.00
///
/// :33B:USD50000,00
/// └─── Before charges: USD 50,000.00
///
/// :33B:GBP25000,50
/// └─── Original amount: GBP 25,000.50
/// ```
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field33B {
    /// ISO 4217 currency code (3 alphabetic characters)
    ///
    /// Specifies the currency of the original instructed amount using the
    /// international standard ISO 4217 currency codes. This represents the
    /// currency in which the customer originally instructed the transaction.
    ///
    /// **Format**: Exactly 3 uppercase alphabetic characters
    /// **Standard**: ISO 4217 (International Organization for Standardization)
    /// **Case handling**: Automatically converted to uppercase
    /// **Validation**: Must be valid and preferably active currency code
    ///
    /// # Common Scenarios
    /// - **FX conversion**: Original currency before conversion to settlement currency
    /// - **Multi-currency**: Different from settlement currency in Field 32A
    /// - **Charge scenarios**: Same as settlement currency but different amount
    ///
    /// # Examples
    /// - `"EUR"` - Euro (original instruction currency)
    /// - `"USD"` - US Dollar (before conversion to EUR settlement)
    /// - `"GBP"` - British Pound (customer's account currency)
    pub currency: String,

    /// Original instructed amount as decimal value
    ///
    /// The monetary amount as originally instructed by the customer or
    /// ordering party, before any currency conversion, charge deduction,
    /// or other modifications that result in a different settlement amount.
    ///
    /// **Range**: Must be non-negative (≥ 0.0)
    /// **Precision**: Should follow currency-specific decimal place rules
    /// **Usage**: Represents the "gross" or "original" amount
    ///
    /// # Business Context
    /// - **Before FX**: Amount before currency conversion
    /// - **Before charges**: Amount before fee/charge deduction
    /// - **Customer view**: Amount as seen by the ordering customer
    /// - **Audit trail**: Original instruction for compliance purposes
    ///
    /// # Examples
    /// - `100000.00` - EUR 100,000 before conversion to USD
    /// - `50000.00` - USD 50,000 before $25 wire fee deduction
    /// - `25000.50` - GBP 25,000.50 original instruction amount
    pub amount: f64,

    /// Raw amount string as received (preserves original formatting)
    ///
    /// Maintains the exact string representation of the amount as received
    /// in the SWIFT message, preserving the original formatting including
    /// decimal separator, precision, and any leading/trailing characters.
    ///
    /// **Format**: SWIFT standard with comma as decimal separator
    /// **Preservation**: Exact reproduction of original message format
    /// **Usage**: For message reconstruction and audit purposes
    ///
    /// # Format Examples
    /// - `"100000,00"` - SWIFT format with comma separator
    /// - `"50000,00"` - Two decimal places preserved
    /// - `"25000,50"` - Original precision maintained
    /// - `"0,01"` - Minimum amount with leading zero
    pub raw_amount: String,
}

impl Field33B {
    /// Create a new Field33B with validation
    ///
    /// Creates a new Field33B instance with comprehensive validation of both
    /// currency code and amount. The currency is normalized to uppercase and
    /// the amount is formatted according to SWIFT standards.
    ///
    /// # Arguments
    /// * `currency` - ISO 4217 currency code (will be converted to uppercase)
    /// * `amount` - Original instructed amount (must be non-negative)
    ///
    /// # Returns
    /// Result containing the Field33B instance or validation error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let field = Field33B::new("EUR", 100000.00).unwrap();
    /// assert_eq!(field.currency(), "EUR");
    /// assert_eq!(field.amount(), 100000.00);
    /// ```
    pub fn new(currency: impl Into<String>, amount: f64) -> Result<Self, crate::ParseError> {
        let currency = currency.into().to_uppercase();

        // Validate currency code
        if currency.len() != 3 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "33B".to_string(),
                message: "Currency code must be exactly 3 characters".to_string(),
            });
        }

        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "33B".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount
        if amount < 0.0 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "33B".to_string(),
                message: "Amount cannot be negative".to_string(),
            });
        }

        let raw_amount = Self::format_amount(amount);

        Ok(Field33B {
            currency,
            amount,
            raw_amount,
        })
    }

    /// Create from raw amount string
    ///
    /// Creates a Field33B instance from a raw amount string, preserving
    /// the original formatting while parsing the numeric value.
    ///
    /// # Arguments
    /// * `currency` - ISO 4217 currency code
    /// * `raw_amount` - Amount string in SWIFT format
    ///
    /// # Returns
    /// Result containing the Field33B instance or parse error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let field = Field33B::from_raw("USD", "50000,00").unwrap();
    /// assert_eq!(field.amount(), 50000.00);
    /// assert_eq!(field.raw_amount(), "50000,00");
    /// ```
    pub fn from_raw(
        currency: impl Into<String>,
        raw_amount: impl Into<String>,
    ) -> Result<Self, crate::ParseError> {
        let currency = currency.into().to_uppercase();
        let raw_amount = raw_amount.into();

        let amount = Self::parse_amount(&raw_amount)?;

        Ok(Field33B {
            currency,
            amount,
            raw_amount: raw_amount.to_string(),
        })
    }

    /// Get the currency code
    ///
    /// Returns the ISO 4217 currency code for the original instructed amount.
    ///
    /// # Returns
    /// Currency code as string slice
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let field = Field33B::new("GBP", 25000.50).unwrap();
    /// assert_eq!(field.currency(), "GBP");
    /// ```
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Get the amount value
    ///
    /// Returns the original instructed amount as a floating-point number.
    ///
    /// # Returns
    /// Amount as f64
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let field = Field33B::new("USD", 75000.25).unwrap();
    /// assert_eq!(field.amount(), 75000.25);
    /// ```
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Get the raw amount string
    ///
    /// Returns the original amount string as received, preserving
    /// the exact formatting from the SWIFT message.
    ///
    /// # Returns
    /// Raw amount string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let field = Field33B::from_raw("EUR", "1000,50").unwrap();
    /// assert_eq!(field.raw_amount(), "1000,50");
    /// ```
    pub fn raw_amount(&self) -> &str {
        &self.raw_amount
    }

    /// Format amount for SWIFT output (with comma as decimal separator)
    ///
    /// Formats a decimal amount according to SWIFT standards using
    /// comma as the decimal separator.
    ///
    /// # Arguments
    /// * `amount` - Amount to format
    ///
    /// # Returns
    /// Formatted amount string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let formatted = Field33B::format_amount(1234.56);
    /// assert_eq!(formatted, "1234,56");
    /// ```
    pub fn format_amount(amount: f64) -> String {
        format!("{:.2}", amount).replace('.', ",")
    }

    /// Parse amount from string (handles both comma and dot as decimal separator)
    fn parse_amount(amount_str: &str) -> Result<f64, crate::ParseError> {
        let normalized_amount = amount_str.replace(',', ".");

        normalized_amount
            .parse::<f64>()
            .map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "33B".to_string(),
                message: "Invalid amount format".to_string(),
            })
    }

    /// Check if this is a valid ISO 4217 currency code (basic validation)
    ///
    /// Performs basic format validation to check if the currency code
    /// follows ISO 4217 standards (3 alphabetic characters).
    ///
    /// # Returns
    /// `true` if the currency code format is valid
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let field = Field33B::new("USD", 1000.00).unwrap();
    /// assert!(field.is_valid_currency());
    /// ```
    pub fn is_valid_currency(&self) -> bool {
        self.currency.len() == 3 && self.currency.chars().all(|c| c.is_alphabetic())
    }

    /// Check if the currency is a major currency
    ///
    /// Determines if the currency is one of the major internationally
    /// traded currencies with high liquidity and frequent usage.
    ///
    /// # Returns
    /// `true` if the currency is a major currency
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let usd_field = Field33B::new("USD", 1000.00).unwrap();
    /// assert!(usd_field.is_major_currency());
    ///
    /// let exotic_field = Field33B::new("XYZ", 1000.00).unwrap();
    /// assert!(!exotic_field.is_major_currency());
    /// ```
    pub fn is_major_currency(&self) -> bool {
        matches!(
            self.currency.as_str(),
            "USD" | "EUR" | "GBP" | "JPY" | "CHF" | "CAD" | "AUD" | "NZD" | "SEK" | "NOK" | "DKK"
        )
    }

    /// Check if the currency typically has decimal places
    ///
    /// Determines if the currency typically uses decimal places in
    /// amount representation. Some currencies like JPY typically
    /// don't use decimal places.
    ///
    /// # Returns
    /// `true` if the currency typically uses decimal places
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let usd_field = Field33B::new("USD", 1000.00).unwrap();
    /// assert!(usd_field.has_decimal_places());
    ///
    /// let jpy_field = Field33B::new("JPY", 1000.00).unwrap();
    /// assert!(!jpy_field.has_decimal_places());
    /// ```
    pub fn has_decimal_places(&self) -> bool {
        !matches!(
            self.currency.as_str(),
            "JPY" | "KRW" | "VND" | "IDR" | "CLP" | "PYG" | "UGX" | "RWF" | "GNF" | "MGA"
        )
    }

    /// Get the typical decimal places for this currency
    ///
    /// Returns the number of decimal places typically used for
    /// this currency in financial transactions.
    ///
    /// # Returns
    /// Number of decimal places (0, 2, or 3)
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let usd_field = Field33B::new("USD", 1000.00).unwrap();
    /// assert_eq!(usd_field.decimal_places(), 2);
    ///
    /// let jpy_field = Field33B::new("JPY", 1000.00).unwrap();
    /// assert_eq!(jpy_field.decimal_places(), 0);
    /// ```
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
    ///
    /// Determines if the original instructed amount exceeds typical
    /// high-value thresholds that may require special handling or reporting.
    ///
    /// # Returns
    /// `true` if this is considered a high-value transaction
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let high_value = Field33B::new("USD", 1500000.00).unwrap();
    /// assert!(high_value.is_high_value_transaction());
    ///
    /// let normal_value = Field33B::new("USD", 50000.00).unwrap();
    /// assert!(!normal_value.is_high_value_transaction());
    /// ```
    pub fn is_high_value_transaction(&self) -> bool {
        // High-value thresholds vary by currency
        let threshold = match self.currency.as_str() {
            "USD" | "EUR" | "GBP" | "CHF" | "CAD" | "AUD" => 1_000_000.0,
            "JPY" => 100_000_000.0,
            "SEK" | "NOK" | "DKK" => 10_000_000.0,
            _ => 1_000_000.0, // Default threshold
        };

        self.amount >= threshold
    }

    /// Check if this represents a currency conversion scenario
    ///
    /// This method would typically be used in conjunction with Field 32A
    /// to determine if the transaction involves currency conversion.
    ///
    /// # Arguments
    /// * `settlement_currency` - Currency from Field 32A for comparison
    ///
    /// # Returns
    /// `true` if currencies differ, indicating conversion
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let field = Field33B::new("EUR", 100000.00).unwrap();
    /// assert!(field.is_currency_conversion("USD"));
    /// assert!(!field.is_currency_conversion("EUR"));
    /// ```
    pub fn is_currency_conversion(&self, settlement_currency: &str) -> bool {
        self.currency != settlement_currency.to_uppercase()
    }

    /// Calculate potential FX exposure
    ///
    /// Estimates the foreign exchange exposure based on the original
    /// amount and currency. This is useful for risk management purposes.
    ///
    /// # Returns
    /// Exposure category as string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let field = Field33B::new("USD", 2000000.00).unwrap();
    /// assert_eq!(field.fx_exposure_category(), "High");
    /// ```
    pub fn fx_exposure_category(&self) -> &'static str {
        if self.is_high_value_transaction() {
            if self.is_major_currency() {
                "High"
            } else {
                "Very High" // High value in exotic currency
            }
        } else if self.is_major_currency() {
            "Low"
        } else {
            "Medium" // Exotic currency but lower amount
        }
    }

    /// Format amount with proper currency precision
    ///
    /// Formats the amount according to the typical precision
    /// rules for the currency.
    ///
    /// # Returns
    /// Formatted amount string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let usd_field = Field33B::new("USD", 1234.56).unwrap();
    /// assert_eq!(usd_field.formatted_amount(), "1234.56");
    ///
    /// let jpy_field = Field33B::new("JPY", 1234.00).unwrap();
    /// assert_eq!(jpy_field.formatted_amount(), "1234");
    /// ```
    pub fn formatted_amount(&self) -> String {
        let decimal_places = self.decimal_places();
        match decimal_places {
            0 => format!("{:.0}", self.amount),
            2 => format!("{:.2}", self.amount),
            3 => format!("{:.3}", self.amount),
            _ => format!("{:.2}", self.amount), // Default to 2
        }
    }

    /// Get transaction purpose classification
    ///
    /// Provides a classification of the likely transaction purpose
    /// based on amount and currency characteristics.
    ///
    /// # Returns
    /// Transaction purpose category
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let field = Field33B::new("USD", 50000.00).unwrap();
    /// let purpose = field.transaction_purpose();
    /// assert!(!purpose.is_empty());
    /// ```
    pub fn transaction_purpose(&self) -> &'static str {
        if self.is_high_value_transaction() {
            if self.is_major_currency() {
                "Corporate/Institutional Transfer"
            } else {
                "High-Value Cross-Border Transfer"
            }
        } else if self.amount < 10000.0 {
            "Personal/Retail Transfer"
        } else if self.is_major_currency() {
            "Commercial Transfer"
        } else {
            "Cross-Border Commercial Transfer"
        }
    }

    /// Get human-readable description
    ///
    /// Returns a comprehensive description of the original instructed
    /// amount including currency, amount, and transaction characteristics.
    ///
    /// # Returns
    /// Formatted description string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let field = Field33B::new("EUR", 100000.00).unwrap();
    /// let desc = field.description();
    /// assert!(desc.contains("EUR"));
    /// assert!(desc.contains("100000.00"));
    /// ```
    pub fn description(&self) -> String {
        format!(
            "Currency/Instructed Amount: {} {:.2}",
            self.currency, self.amount
        )
    }

    /// Get comprehensive transaction analysis
    ///
    /// Returns a detailed analysis of the transaction including currency
    /// characteristics, amount classification, and risk assessment.
    ///
    /// # Returns
    /// Formatted analysis string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field33B;
    /// let field = Field33B::new("USD", 1500000.00).unwrap();
    /// let analysis = field.comprehensive_analysis();
    /// assert!(analysis.contains("USD"));
    /// assert!(analysis.contains("High-value"));
    /// assert!(analysis.contains("Major currency"));
    /// ```
    pub fn comprehensive_analysis(&self) -> String {
        let currency_type = if self.is_major_currency() {
            "Major currency"
        } else {
            "Other currency"
        };

        let amount_category = if self.is_high_value_transaction() {
            "High-value"
        } else {
            "Standard"
        };

        let fx_exposure = self.fx_exposure_category();
        let purpose = self.transaction_purpose();

        format!(
            "Original Amount: {} {} ({}) | Category: {} | FX Exposure: {} | Purpose: {}",
            self.formatted_amount(),
            self.currency,
            currency_type,
            amount_category,
            fx_exposure,
            purpose
        )
    }
}

impl SwiftField for Field33B {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":33B:") {
            stripped // Remove ":33B:" prefix
        } else if let Some(stripped) = value.strip_prefix("33B:") {
            stripped // Remove "33B:" prefix
        } else {
            value
        };

        let content = content.trim();

        if content.len() < 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "33B".to_string(),
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
                field_tag: "33B".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        let amount = Self::parse_amount(amount_str)?;

        if amount < 0.0 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "33B".to_string(),
                message: "Amount cannot be negative".to_string(),
            });
        }

        Ok(Field33B {
            currency,
            amount,
            raw_amount: amount_str.to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":33B:{}{}", self.currency, self.raw_amount)
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate currency code
        if self.currency.len() != 3 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "33B".to_string(),
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
                field_tag: "33B".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount
        if self.amount < 0.0 {
            errors.push(ValidationError::ValueValidation {
                field_tag: "33B".to_string(),
                message: "Amount cannot be negative".to_string(),
            });
        }

        // Validate raw amount format
        if self.raw_amount.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "33B".to_string(),
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

impl std::fmt::Display for Field33B {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.currency, self.raw_amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field33b_creation() {
        let field = Field33B::new("USD", 1234.56).unwrap();
        assert_eq!(field.currency(), "USD");
        assert_eq!(field.amount(), 1234.56);
        assert_eq!(field.raw_amount(), "1234,56");
    }

    #[test]
    fn test_field33b_from_raw() {
        let field = Field33B::from_raw("EUR", "999,99").unwrap();
        assert_eq!(field.currency(), "EUR");
        assert_eq!(field.amount(), 999.99);
        assert_eq!(field.raw_amount(), "999,99");
    }

    #[test]
    fn test_field33b_parse() {
        let field = Field33B::parse("USD1234,56").unwrap();
        assert_eq!(field.currency(), "USD");
        assert_eq!(field.amount(), 1234.56);
        assert_eq!(field.raw_amount(), "1234,56");
    }

    #[test]
    fn test_field33b_parse_with_prefix() {
        let field = Field33B::parse(":33B:EUR500,00").unwrap();
        assert_eq!(field.currency(), "EUR");
        assert_eq!(field.amount(), 500.0);
        assert_eq!(field.raw_amount(), "500,00");
    }

    #[test]
    fn test_field33b_to_swift_string() {
        let field = Field33B::new("GBP", 750.25).unwrap();
        assert_eq!(field.to_swift_string(), ":33B:GBP750,25");
    }

    #[test]
    fn test_field33b_invalid_currency_length() {
        let result = Field33B::new("US", 100.0);
        assert!(result.is_err());

        let result = Field33B::new("USDD", 100.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_field33b_invalid_currency_characters() {
        let result = Field33B::new("U$D", 100.0);
        assert!(result.is_err());

        let result = Field33B::new("123", 100.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_field33b_negative_amount() {
        let result = Field33B::new("USD", -100.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_field33b_parse_invalid_format() {
        let result = Field33B::parse("USD");
        assert!(result.is_err());

        let result = Field33B::parse("US1234,56");
        assert!(result.is_err());
    }

    #[test]
    fn test_field33b_validation() {
        let field = Field33B::new("USD", 1000.0).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field33b_display() {
        let field = Field33B::new("CHF", 2500.75).unwrap();
        assert_eq!(format!("{}", field), "CHF 2500,75");
    }

    #[test]
    fn test_field33b_is_valid_currency() {
        let field = Field33B::new("USD", 100.0).unwrap();
        assert!(field.is_valid_currency());
    }

    #[test]
    fn test_field33b_description() {
        let field = Field33B::new("EUR", 1500.0).unwrap();
        assert_eq!(
            field.description(),
            "Currency/Instructed Amount: EUR 1500.00"
        );
    }

    #[test]
    fn test_field33b_parse_dot_decimal() {
        let field = Field33B::parse("USD1234.56").unwrap();
        assert_eq!(field.amount(), 1234.56);
        assert_eq!(field.raw_amount(), "1234.56");
    }

    #[test]
    fn test_field33b_major_currencies() {
        let major_currencies = [
            "USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "NZD", "SEK", "NOK", "DKK",
        ];

        for currency in major_currencies {
            let field = Field33B::new(currency, 1000.00).unwrap();
            assert!(
                field.is_major_currency(),
                "Currency {} should be major currency",
                currency
            );
        }

        let exotic_currencies = ["XYZ", "ABC", "THB", "MXN"];
        for currency in exotic_currencies {
            let field = Field33B::new(currency, 1000.00).unwrap();
            assert!(
                !field.is_major_currency(),
                "Currency {} should not be major currency",
                currency
            );
        }
    }

    #[test]
    fn test_field33b_decimal_places() {
        // Currencies with no decimal places
        let no_decimal_currencies = ["JPY", "KRW", "VND"];
        for currency in no_decimal_currencies {
            let field = Field33B::new(currency, 1000.00).unwrap();
            assert_eq!(
                field.decimal_places(),
                0,
                "Currency {} should have 0 decimal places",
                currency
            );
            assert!(!field.has_decimal_places());
        }

        // Currencies with 3 decimal places
        let three_decimal_currencies = ["BHD", "JOD", "KWD"];
        for currency in three_decimal_currencies {
            let field = Field33B::new(currency, 1000.00).unwrap();
            assert_eq!(
                field.decimal_places(),
                3,
                "Currency {} should have 3 decimal places",
                currency
            );
            assert!(field.has_decimal_places());
        }

        // Currencies with 2 decimal places (most common)
        let two_decimal_currencies = ["USD", "EUR", "GBP"];
        for currency in two_decimal_currencies {
            let field = Field33B::new(currency, 1000.00).unwrap();
            assert_eq!(
                field.decimal_places(),
                2,
                "Currency {} should have 2 decimal places",
                currency
            );
            assert!(field.has_decimal_places());
        }
    }

    #[test]
    fn test_field33b_high_value_transactions() {
        // High-value USD transaction
        let high_usd = Field33B::new("USD", 1_500_000.00).unwrap();
        assert!(high_usd.is_high_value_transaction());

        let normal_usd = Field33B::new("USD", 500_000.00).unwrap();
        assert!(!normal_usd.is_high_value_transaction());

        // High-value JPY transaction (different threshold)
        let high_jpy = Field33B::new("JPY", 150_000_000.00).unwrap();
        assert!(high_jpy.is_high_value_transaction());

        let normal_jpy = Field33B::new("JPY", 50_000_000.00).unwrap();
        assert!(!normal_jpy.is_high_value_transaction());
    }

    #[test]
    fn test_field33b_currency_conversion() {
        let field = Field33B::new("EUR", 100000.00).unwrap();

        // Different currencies indicate conversion
        assert!(field.is_currency_conversion("USD"));
        assert!(field.is_currency_conversion("GBP"));

        // Same currency indicates no conversion
        assert!(!field.is_currency_conversion("EUR"));
        assert!(!field.is_currency_conversion("eur")); // Case insensitive
    }

    #[test]
    fn test_field33b_fx_exposure_category() {
        // High value, major currency
        let high_major = Field33B::new("USD", 2_000_000.00).unwrap();
        assert_eq!(high_major.fx_exposure_category(), "High");

        // High value, exotic currency
        let high_exotic = Field33B::new("THB", 2_000_000.00).unwrap();
        assert_eq!(high_exotic.fx_exposure_category(), "Very High");

        // Low value, major currency
        let low_major = Field33B::new("EUR", 50_000.00).unwrap();
        assert_eq!(low_major.fx_exposure_category(), "Low");

        // Low value, exotic currency
        let low_exotic = Field33B::new("MXN", 50_000.00).unwrap();
        assert_eq!(low_exotic.fx_exposure_category(), "Medium");
    }

    #[test]
    fn test_field33b_formatted_amount() {
        // USD with 2 decimal places
        let usd_field = Field33B::new("USD", 1234.56).unwrap();
        assert_eq!(usd_field.formatted_amount(), "1234.56");

        // JPY with 0 decimal places
        let jpy_field = Field33B::new("JPY", 1234.00).unwrap();
        assert_eq!(jpy_field.formatted_amount(), "1234");

        // KWD with 3 decimal places
        let kwd_field = Field33B::new("KWD", 1234.567).unwrap();
        assert_eq!(kwd_field.formatted_amount(), "1234.567");
    }

    #[test]
    fn test_field33b_transaction_purpose() {
        // High-value major currency
        let corporate = Field33B::new("USD", 2_000_000.00).unwrap();
        assert_eq!(
            corporate.transaction_purpose(),
            "Corporate/Institutional Transfer"
        );

        // High-value exotic currency
        let cross_border_high = Field33B::new("THB", 2_000_000.00).unwrap();
        assert_eq!(
            cross_border_high.transaction_purpose(),
            "High-Value Cross-Border Transfer"
        );

        // Small amount
        let personal = Field33B::new("USD", 5_000.00).unwrap();
        assert_eq!(personal.transaction_purpose(), "Personal/Retail Transfer");

        // Medium amount, major currency
        let commercial = Field33B::new("EUR", 50_000.00).unwrap();
        assert_eq!(commercial.transaction_purpose(), "Commercial Transfer");

        // Medium amount, exotic currency
        let cross_border_commercial = Field33B::new("MXN", 50_000.00).unwrap();
        assert_eq!(
            cross_border_commercial.transaction_purpose(),
            "Cross-Border Commercial Transfer"
        );
    }

    #[test]
    fn test_field33b_comprehensive_analysis() {
        let field = Field33B::new("USD", 1_500_000.00).unwrap();
        let analysis = field.comprehensive_analysis();

        assert!(analysis.contains("USD"));
        assert!(analysis.contains("1500000.00"));
        assert!(analysis.contains("Major currency"));
        assert!(analysis.contains("High-value"));
        assert!(analysis.contains("High")); // FX exposure
        assert!(analysis.contains("Corporate/Institutional Transfer"));
    }

    #[test]
    fn test_field33b_format_amount_static() {
        assert_eq!(Field33B::format_amount(1234.56), "1234,56");
        assert_eq!(Field33B::format_amount(100.00), "100,00");
        assert_eq!(Field33B::format_amount(0.01), "0,01");
    }

    #[test]
    fn test_field33b_serialization() {
        let field = Field33B::new("EUR", 1234.56).unwrap();
        let serialized = serde_json::to_string(&field).unwrap();
        let deserialized: Field33B = serde_json::from_str(&serialized).unwrap();

        assert_eq!(field.currency(), deserialized.currency());
        assert_eq!(field.amount(), deserialized.amount());
        assert_eq!(field.raw_amount(), deserialized.raw_amount());
        assert_eq!(field.is_major_currency(), deserialized.is_major_currency());
    }

    #[test]
    fn test_field33b_business_logic_combinations() {
        // Major currency, high-value transaction
        let field = Field33B::new("USD", 2_000_000.00).unwrap();
        assert!(field.is_major_currency());
        assert!(field.is_high_value_transaction());
        assert_eq!(field.fx_exposure_category(), "High");
        assert_eq!(
            field.transaction_purpose(),
            "Corporate/Institutional Transfer"
        );
        assert_eq!(field.decimal_places(), 2);

        // Exotic currency, normal value transaction
        let field = Field33B::new("THB", 100_000.00).unwrap();
        assert!(!field.is_major_currency());
        assert!(!field.is_high_value_transaction());
        assert_eq!(field.fx_exposure_category(), "Medium");
        assert_eq!(
            field.transaction_purpose(),
            "Cross-Border Commercial Transfer"
        );
        assert_eq!(field.decimal_places(), 2);
    }

    #[test]
    fn test_field33b_edge_cases() {
        // Zero amount (allowed)
        let zero_field = Field33B::new("USD", 0.0).unwrap();
        assert_eq!(zero_field.amount(), 0.0);
        assert!(!zero_field.is_high_value_transaction());

        // Very small amount
        let small_field = Field33B::new("USD", 0.01).unwrap();
        assert_eq!(small_field.amount(), 0.01);
        assert_eq!(
            small_field.transaction_purpose(),
            "Personal/Retail Transfer"
        );

        // Very large amount
        let large_field = Field33B::new("USD", 999_999_999.99).unwrap();
        assert!(large_field.is_high_value_transaction());
        assert_eq!(
            large_field.transaction_purpose(),
            "Corporate/Institutional Transfer"
        );
    }

    #[test]
    fn test_field33b_real_world_scenarios() {
        // Scenario 1: FX conversion (EUR to USD)
        let fx_conversion = Field33B::new("EUR", 100_000.00).unwrap();
        assert!(fx_conversion.is_currency_conversion("USD"));
        assert!(fx_conversion.is_major_currency());
        assert!(!fx_conversion.is_high_value_transaction());
        assert_eq!(fx_conversion.transaction_purpose(), "Commercial Transfer");

        // Scenario 2: High-value corporate transfer
        let corporate_transfer = Field33B::new("USD", 5_000_000.00).unwrap();
        assert!(corporate_transfer.is_high_value_transaction());
        assert_eq!(corporate_transfer.fx_exposure_category(), "High");
        assert_eq!(
            corporate_transfer.transaction_purpose(),
            "Corporate/Institutional Transfer"
        );

        // Scenario 3: Personal remittance
        let remittance = Field33B::new("USD", 2_500.00).unwrap();
        assert_eq!(remittance.transaction_purpose(), "Personal/Retail Transfer");
        assert_eq!(remittance.fx_exposure_category(), "Low");

        // Scenario 4: Exotic currency transaction
        let exotic_transfer = Field33B::new("THB", 3_000_000.00).unwrap();
        assert!(!exotic_transfer.is_major_currency());
        assert!(exotic_transfer.is_high_value_transaction());
        assert_eq!(exotic_transfer.fx_exposure_category(), "Very High");
        assert_eq!(
            exotic_transfer.transaction_purpose(),
            "High-Value Cross-Border Transfer"
        );
    }

    #[test]
    fn test_field33b_currency_specific_behavior() {
        let test_cases = [
            ("USD", true, 2, 1_000_000.0),
            ("EUR", true, 2, 1_000_000.0),
            ("JPY", true, 0, 100_000_000.0),
            ("KWD", false, 3, 1_000_000.0),
            ("THB", false, 2, 1_000_000.0),
        ];

        for (currency, is_major, decimal_places, high_value_threshold) in test_cases {
            let field = Field33B::new(currency, 1000.00).unwrap();

            assert_eq!(
                field.is_major_currency(),
                is_major,
                "Major currency check failed for {}",
                currency
            );
            assert_eq!(
                field.decimal_places(),
                decimal_places,
                "Decimal places check failed for {}",
                currency
            );

            // Test high-value threshold
            let high_value_field = Field33B::new(currency, high_value_threshold).unwrap();
            assert!(
                high_value_field.is_high_value_transaction(),
                "High value threshold check failed for {}",
                currency
            );
        }
    }

    #[test]
    fn test_field33b_cross_field_integration() {
        // Test scenarios that would typically involve Field 32A integration
        let field33b = Field33B::new("EUR", 100_000.00).unwrap();

        // Currency conversion scenario
        assert!(field33b.is_currency_conversion("USD"));
        assert!(!field33b.is_currency_conversion("EUR"));

        // Charge deduction scenario (same currency, different amount)
        assert!(!field33b.is_currency_conversion("EUR"));
        // In real usage, amount comparison with Field 32A would show charge deduction

        // Multi-currency analysis
        let analysis = field33b.comprehensive_analysis();
        assert!(analysis.contains("EUR"));
        assert!(analysis.contains("Major currency"));
        assert!(analysis.contains("Standard")); // Not high-value
        assert!(analysis.contains("Low")); // FX exposure for major currency, standard amount
    }
}
