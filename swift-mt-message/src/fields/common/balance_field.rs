use crate::{SwiftField, ValidationError, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Generic Balance Field
///
/// ## Overview
/// A generic field structure for SWIFT balance fields that follow the
/// `1!a6!n3!a15d` pattern (debit/credit mark + date + currency + amount).
/// This structure consolidates the common functionality used by various
/// balance reporting fields in MT9xx message types.
///
/// ## Format Specification
/// **Format**: `1!a6!n3!a15d`
/// - **1!a**: Debit/Credit mark (D=Debit, C=Credit)
/// - **6!n**: Date in YYMMDD format
/// - **3!a**: Currency code (ISO 4217)
/// - **15d**: Amount with up to 15 digits including decimal places
///
/// ## Usage Context
/// Used in various SWIFT MT message types for balance reporting:
/// - **Field 60F/60a**: Opening Balance
/// - **Field 62F/62a**: Closing Balance (Booked Funds)
/// - **Field 64**: Closing Available Balance
/// - **Field 65**: Forward Available Balance
///
/// ## Usage Examples
/// ```text
/// D240315USD1234567,89
/// └─── Debit balance of USD 1,234,567.89 on March 15, 2024
///
/// C240315EUR500000,00
/// └─── Credit balance of EUR 500,000.00 on March 15, 2024
/// ```
///
/// ## Validation Rules
/// 1. **Debit/Credit mark**: Must be 'D' or 'C'
/// 2. **Date format**: Must be valid YYMMDD format
/// 3. **Currency format**: Must be exactly 3 alphabetic characters
/// 4. **Amount format**: Must follow SWIFT decimal format (comma separator)
/// 5. **Amount value**: Must be non-negative (≥ 0)
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Debit/Credit mark must be D or C (Error: T18)
/// - Date must be valid YYMMDD format (Error: T50)
/// - Currency code must be exactly 3 characters (Error: T52)
/// - Currency must be valid ISO 4217 code (Error: T52)
/// - Amount must be properly formatted (Error: T40)
/// - Decimal separator must be comma (Error: T41)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenericBalanceField {
    /// Debit/Credit mark (D=Debit, C=Credit)
    pub debit_credit_mark: char,
    /// Date in YYMMDD format
    pub date: String,
    /// Currency code (3 letters, ISO 4217)
    pub currency: String,
    /// Amount value as floating point
    pub amount: f64,
    /// Raw amount string as received (preserves original formatting)
    pub raw_amount: String,
}

impl GenericBalanceField {
    /// Create a new GenericBalanceField with validation
    ///
    /// # Arguments
    /// * `debit_credit_mark` - Debit/Credit mark ('D' or 'C')
    /// * `date` - Date in YYMMDD format
    /// * `currency` - ISO 4217 currency code (will be converted to uppercase)
    /// * `amount` - Amount value (must be non-negative)
    ///
    /// # Returns
    /// Result containing the GenericBalanceField instance or validation error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::common::GenericBalanceField;
    /// let field = GenericBalanceField::new('C', "240315", "USD", 1234567.89).unwrap();
    /// assert_eq!(field.debit_credit_mark(), 'C');
    /// assert_eq!(field.date(), "240315");
    /// assert_eq!(field.currency(), "USD");
    /// assert_eq!(field.amount(), 1234567.89);
    /// ```
    pub fn new(
        debit_credit_mark: char,
        date: impl Into<String>,
        currency: impl Into<String>,
        amount: f64,
    ) -> Result<Self, ParseError> {
        let date = date.into();
        let currency = currency.into().to_uppercase();

        // Validate debit/credit mark
        if debit_credit_mark != 'D' && debit_credit_mark != 'C' {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Debit/Credit mark must be 'D' (Debit) or 'C' (Credit)".to_string(),
            });
        }

        // Validate date format
        if date.len() != 6 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Date must be exactly 6 digits (YYMMDD)".to_string(),
            });
        }

        if !date.chars().all(|c| c.is_ascii_digit()) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Date must contain only digits".to_string(),
            });
        }

        // Basic date validation
        Self::validate_date(&date)?;

        // Validate currency code
        if currency.len() != 3 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Currency code must be exactly 3 characters".to_string(),
            });
        }

        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount
        if amount < 0.0 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Amount cannot be negative".to_string(),
            });
        }

        let raw_amount = Self::format_amount(amount);

        Ok(GenericBalanceField {
            debit_credit_mark,
            date: date.to_string(),
            currency,
            amount,
            raw_amount,
        })
    }

    /// Create from raw amount string
    ///
    /// # Arguments
    /// * `debit_credit_mark` - Debit/Credit mark ('D' or 'C')
    /// * `date` - Date in YYMMDD format
    /// * `currency` - ISO 4217 currency code
    /// * `raw_amount` - Raw amount string (preserves original formatting)
    ///
    /// # Returns
    /// Result containing the GenericBalanceField instance or validation error
    pub fn from_raw(
        debit_credit_mark: char,
        date: impl Into<String>,
        currency: impl Into<String>,
        raw_amount: impl Into<String>,
    ) -> Result<Self, ParseError> {
        let date = date.into();
        let currency = currency.into().to_uppercase();
        let raw_amount = raw_amount.into();

        // Validate debit/credit mark
        if debit_credit_mark != 'D' && debit_credit_mark != 'C' {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Debit/Credit mark must be 'D' (Debit) or 'C' (Credit)".to_string(),
            });
        }

        // Validate date format
        if date.len() != 6 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Date must be exactly 6 digits (YYMMDD)".to_string(),
            });
        }

        if !date.chars().all(|c| c.is_ascii_digit()) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Date must contain only digits".to_string(),
            });
        }

        Self::validate_date(&date)?;

        // Validate currency
        if currency.len() != 3 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Currency code must be exactly 3 characters".to_string(),
            });
        }

        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        let amount = Self::parse_amount(&raw_amount)?;

        if amount < 0.0 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Amount cannot be negative".to_string(),
            });
        }

        Ok(GenericBalanceField {
            debit_credit_mark,
            date: date.to_string(),
            currency,
            amount,
            raw_amount: raw_amount.to_string(),
        })
    }

    /// Get the debit/credit mark
    pub fn debit_credit_mark(&self) -> char {
        self.debit_credit_mark
    }

    /// Get the date
    pub fn date(&self) -> &str {
        &self.date
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

    /// Check if this is a debit balance
    pub fn is_debit(&self) -> bool {
        self.debit_credit_mark == 'D'
    }

    /// Check if this is a credit balance
    pub fn is_credit(&self) -> bool {
        self.debit_credit_mark == 'C'
    }

    /// Get the balance type description
    pub fn balance_type(&self) -> &'static str {
        if self.is_debit() { "Debit" } else { "Credit" }
    }

    /// Get the year from the date (20YY format)
    pub fn year(&self) -> u16 {
        let yy: u16 = self.date[0..2].parse().unwrap_or(0);
        if yy <= 50 { 2000 + yy } else { 1900 + yy }
    }

    /// Get the month from the date
    pub fn month(&self) -> u8 {
        self.date[2..4].parse().unwrap_or(0)
    }

    /// Get the day from the date
    pub fn day(&self) -> u8 {
        self.date[4..6].parse().unwrap_or(0)
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
                field_tag: "GenericBalanceField".to_string(),
                message: "Invalid amount format".to_string(),
            })
    }

    /// Validate date format and basic date logic
    fn validate_date(date: &str) -> Result<(), ParseError> {
        if date.len() != 6 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Date must be exactly 6 digits (YYMMDD)".to_string(),
            });
        }

        let month: u8 = date[2..4]
            .parse()
            .map_err(|_| ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Invalid month in date".to_string(),
            })?;

        let day: u8 = date[4..6]
            .parse()
            .map_err(|_| ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Invalid day in date".to_string(),
            })?;

        if !(1..=12).contains(&month) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Month must be between 01 and 12".to_string(),
            });
        }

        if !(1..=31).contains(&day) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericBalanceField".to_string(),
                message: "Day must be between 01 and 31".to_string(),
            });
        }

        Ok(())
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

        if content.len() < 11 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: field_tag.to_string(),
                message: "Field content too short (minimum 11 characters: DYYMMDDCCCAMOUNT)"
                    .to_string(),
            });
        }

        // Parse components
        let debit_credit_mark = content.chars().next().unwrap();
        let date_str = &content[1..7];
        let currency_str = &content[7..10];
        let amount_str = &content[10..];

        Self::from_raw(debit_credit_mark, date_str, currency_str, amount_str).map_err(|e| {
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
        })
    }

    /// Convert to SWIFT string format with custom field tag
    pub fn to_swift_string_with_tag(&self, field_tag: &str) -> String {
        format!(
            ":{}:{}{}{}{}",
            field_tag, self.debit_credit_mark, self.date, self.currency, self.raw_amount
        )
    }

    /// Get human-readable description with custom context
    pub fn description(&self, context: &str) -> String {
        format!(
            "{}: {} {} {} on {}/{}/{}",
            context,
            self.balance_type(),
            self.currency,
            self.raw_amount,
            self.day(),
            self.month(),
            self.year()
        )
    }

    /// Get formatted date string (DD/MM/YYYY)
    pub fn formatted_date(&self) -> String {
        format!("{:02}/{:02}/{}", self.day(), self.month(), self.year())
    }

    /// Check if this is a high-value balance
    pub fn is_high_value_balance(&self) -> bool {
        // Convert to USD equivalent for comparison (simplified)
        let usd_equivalent = match self.currency.as_str() {
            "EUR" => self.amount * 1.1,   // Approximate EUR to USD
            "GBP" => self.amount * 1.25,  // Approximate GBP to USD
            "JPY" => self.amount * 0.007, // Approximate JPY to USD
            "CHF" => self.amount * 1.08,  // Approximate CHF to USD
            _ => self.amount,             // Assume USD or treat as USD equivalent
        };

        usd_equivalent >= 10_000_000.0 // $10M threshold for balance
    }
}

impl SwiftField for GenericBalanceField {
    fn parse(content: &str) -> Result<Self, ParseError> {
        Self::parse_with_tag(content, "GenericBalanceField")
    }

    fn to_swift_string(&self) -> String {
        self.to_swift_string_with_tag("GenericBalanceField")
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate debit/credit mark
        if self.debit_credit_mark != 'D' && self.debit_credit_mark != 'C' {
            errors.push(ValidationError::FormatValidation {
                field_tag: "GenericBalanceField".to_string(),
                message: "Debit/Credit mark must be 'D' or 'C'".to_string(),
            });
        }

        // Validate date
        if self.date.len() != 6 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "GenericBalanceField".to_string(),
                expected: "6 digits".to_string(),
                actual: self.date.len(),
            });
        }

        if Self::validate_date(&self.date).is_err() {
            errors.push(ValidationError::FormatValidation {
                field_tag: "GenericBalanceField".to_string(),
                message: "Invalid date format or values".to_string(),
            });
        }

        // Validate currency code
        if self.currency.len() != 3 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "GenericBalanceField".to_string(),
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
                field_tag: "GenericBalanceField".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount
        if self.amount < 0.0 {
            errors.push(ValidationError::ValueValidation {
                field_tag: "GenericBalanceField".to_string(),
                message: "Amount cannot be negative".to_string(),
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "1!a6!n3!a15d"
    }
}

impl std::fmt::Display for GenericBalanceField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} ({})",
            self.balance_type(),
            self.currency,
            self.raw_amount,
            self.formatted_date()
        )
    }
}

// Remove the macro-based balance field implementations
// We'll use GenericBalanceField directly in message structs with field tag attributes
// This follows the same pattern as field_33b using GenericCurrencyAmountField directly

// The balance field types are now type aliases for cleaner code organization
// but the actual message structs will use GenericBalanceField directly with #[field("60A")] attributes

/// Type alias for Field 60A - Opening Balance (Intermediate)
pub type Field60A = GenericBalanceField;

/// Type alias for Field 60F - Opening Balance (Final/Booked)  
pub type Field60F = GenericBalanceField;

/// Type alias for Field 62A - Closing Balance (Intermediate)
pub type Field62A = GenericBalanceField;

/// Type alias for Field 62F - Closing Balance (Final/Booked)
pub type Field62F = GenericBalanceField;

/// Type alias for Field 64 - Closing Available Balance
pub type Field64 = GenericBalanceField;

/// Type alias for Field 65 - Forward Available Balance
pub type Field65 = GenericBalanceField;

#[cfg(test)]
mod balance_field_tests {
    use super::*;

    #[test]
    fn test_balance_field_60a() {
        let field = Field60A::new('C', "240315", "USD", 1000.0).unwrap();
        assert_eq!(field.currency(), "USD");
        assert_eq!(field.amount(), 1000.0);
        assert!(field.is_credit());
        assert_eq!(
            field.to_swift_string_with_tag("60A"),
            ":60A:C240315USD1000,00"
        );
        assert!(
            field
                .description("Opening Balance")
                .contains("Opening Balance")
        );
    }

    #[test]
    fn test_balance_field_64() {
        let field = Field64::new('D', "240315", "EUR", 500.0).unwrap();
        assert_eq!(field.currency(), "EUR");
        assert_eq!(field.amount(), 500.0);
        assert!(field.is_debit());
        assert_eq!(field.to_swift_string_with_tag("64"), ":64:D240315EUR500,00");
        assert!(
            field
                .description("Closing Available Balance")
                .contains("Closing Available Balance")
        );
    }

    #[test]
    fn test_field_specific_validation() {
        // Test opening balance (60A)
        let field = Field60A::new('D', "240315", "USD", 1000.0).unwrap();
        let result = field.validate();
        assert!(result.is_valid);

        // Test available balance (64)
        let field = Field64::new('C', "240315", "USD", 0.0).unwrap();
        let result = field.validate();
        assert!(result.is_valid);

        // Test forward balance (65)
        let field = Field65::new('D', "240315", "USD", 15000.0).unwrap();
        let result = field.validate();
        assert!(result.is_valid);
    }

    #[test]
    fn test_validation_with_field_tag() {
        let field = Field62F::new('C', "240315", "USD", 1000.0).unwrap();
        let result = field.validate();
        assert!(result.is_valid);

        // Test that error field tags are correctly set
        let invalid_field = Field62F::from_raw('C', "24031", "USD", "1000,00"); // Invalid date
        assert!(invalid_field.is_err());
    }

    #[test]
    fn test_display_formatting() {
        let field = Field60F::new('C', "240315", "USD", 1000.0).unwrap();
        let display = format!("{}", field);
        assert!(display.contains("Credit USD 1000,00"));

        let field = Field65::new('D', "240320", "EUR", 500.0).unwrap();
        let display = format!("{}", field);
        assert!(display.contains("Debit EUR 500,00"));
    }

    #[test]
    fn test_field_specific_methods() {
        let field = Field64::new('C', "240315", "USD", 150000.0).unwrap();
        assert!(field.is_credit());
        assert!(field.amount() > 0.0);
        assert!(!field.is_debit());

        let zero_field = Field60A::new('C', "240315", "USD", 0.0).unwrap();
        assert_eq!(zero_field.amount(), 0.0);
    }

    #[test]
    fn test_cross_field_type_usage() {
        // Test that all balance field types work the same way
        let field_60a = Field60A::new('C', "240315", "USD", 1000.0).unwrap();
        let field_60f = Field60F::new('C', "240315", "USD", 1000.0).unwrap();
        let field_62a = Field62A::new('C', "240315", "USD", 1000.0).unwrap();
        let field_62f = Field62F::new('C', "240315", "USD", 1000.0).unwrap();
        let field_64 = Field64::new('C', "240315", "USD", 1000.0).unwrap();
        let field_65 = Field65::new('C', "240315", "USD", 1000.0).unwrap();

        // All should have the same basic properties since they're the same type
        assert_eq!(field_60a.currency(), "USD");
        assert_eq!(field_60f.currency(), "USD");
        assert_eq!(field_62a.currency(), "USD");
        assert_eq!(field_62f.currency(), "USD");
        assert_eq!(field_64.currency(), "USD");
        assert_eq!(field_65.currency(), "USD");

        // All should format differently with their respective field tags
        assert_eq!(
            field_60a.to_swift_string_with_tag("60A"),
            ":60A:C240315USD1000,00"
        );
        assert_eq!(
            field_60f.to_swift_string_with_tag("60F"),
            ":60F:C240315USD1000,00"
        );
        assert_eq!(
            field_62a.to_swift_string_with_tag("62A"),
            ":62A:C240315USD1000,00"
        );
        assert_eq!(
            field_62f.to_swift_string_with_tag("62F"),
            ":62F:C240315USD1000,00"
        );
        assert_eq!(
            field_64.to_swift_string_with_tag("64"),
            ":64:C240315USD1000,00"
        );
        assert_eq!(
            field_65.to_swift_string_with_tag("65"),
            ":65:C240315USD1000,00"
        );
    }
}
