use crate::{SwiftField, ValidationError, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Field 34F - Floor Limit
///
/// ## Overview
/// Field 34F represents floor limit amounts for transaction reporting thresholds.
/// It specifies the minimum amount above which transactions must be reported.
/// The field includes currency, optional debit/credit indicator, and amount.
///
/// ## Format Specification
/// **Format**: `3!a[1!a]15d`
/// - **3!a**: Currency code (3 alphabetic characters, ISO 4217)
/// - **[1!a]**: Optional sign indicator (D=Debit, C=Credit)
/// - **15d**: Amount with up to 15 digits including decimal places
///
/// ## Usage Context
/// Used in MT920 (Request Message) and MT942 (Interim Transaction Report) for:
/// - **Debit Floor Limit**: Minimum amount for debit transaction reporting
/// - **Credit Floor Limit**: Minimum amount for credit transaction reporting
/// - **Combined Limit**: When both debit and credit use same threshold
///
/// ## Usage Examples
/// ```text
/// USDD1000000,00
/// └─── USD 1,000,000.00 debit floor limit
///
/// EURC500000,00
/// └─── EUR 500,000.00 credit floor limit
///
/// GBP250000,00
/// └─── GBP 250,000.00 combined floor limit
/// ```
///
/// ## Validation Rules
/// 1. **Currency format**: Must be exactly 3 alphabetic characters
/// 2. **Currency validity**: Should be valid ISO 4217 code
/// 3. **Sign indicator**: If present, must be 'D' or 'C'
/// 4. **Amount format**: Must follow SWIFT decimal format (comma separator)
/// 5. **Amount value**: Must be positive (> 0)
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Currency code must be exactly 3 characters (Error: T52)
/// - Currency must be valid ISO 4217 code (Error: T52)
/// - Amount must be properly formatted (Error: T40)
/// - Amount must be positive (Error: T13)
/// - Decimal separator must be comma (Error: T41)
/// - Sign indicator must be D or C if present (Error: T18)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field34F {
    /// Currency code (3 letters, ISO 4217)
    pub currency: String,
    /// Optional sign indicator (D=Debit, C=Credit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_indicator: Option<char>,
    /// Amount value as floating point
    pub amount: f64,
    /// Raw amount string as received (preserves original formatting)
    pub raw_amount: String,
}

impl Field34F {
    /// Create a new Field34F with validation
    ///
    /// # Arguments
    /// * `currency` - ISO 4217 currency code (will be converted to uppercase)
    /// * `sign_indicator` - Optional sign indicator ('D' or 'C')
    /// * `amount` - Amount value (must be positive)
    ///
    /// # Returns
    /// Result containing the Field34F instance or validation error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field34F;
    /// let field = Field34F::new("USD", Some('D'), 1000000.00).unwrap();
    /// assert_eq!(field.currency(), "USD");
    /// assert_eq!(field.sign_indicator(), Some('D'));
    /// assert_eq!(field.amount(), 1000000.00);
    /// ```
    pub fn new(
        currency: impl Into<String>,
        sign_indicator: Option<char>,
        amount: f64,
    ) -> Result<Self, ParseError> {
        let currency = currency.into().to_uppercase();

        // Validate currency code
        if currency.len() != 3 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "34F".to_string(),
                message: "Currency code must be exactly 3 characters".to_string(),
            });
        }

        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "34F".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate sign indicator if present
        if let Some(sign) = sign_indicator {
            if sign != 'D' && sign != 'C' {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "34F".to_string(),
                    message: "Sign indicator must be 'D' (Debit) or 'C' (Credit)".to_string(),
                });
            }
        }

        // Validate amount
        if amount <= 0.0 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "34F".to_string(),
                message: "Amount must be positive".to_string(),
            });
        }

        let raw_amount = Self::format_amount(amount);

        Ok(Field34F {
            currency,
            sign_indicator,
            amount,
            raw_amount,
        })
    }

    /// Create from raw amount string
    ///
    /// # Arguments
    /// * `currency` - ISO 4217 currency code
    /// * `sign_indicator` - Optional sign indicator ('D' or 'C')
    /// * `raw_amount` - Raw amount string (preserves original formatting)
    ///
    /// # Returns
    /// Result containing the Field34F instance or validation error
    pub fn from_raw(
        currency: impl Into<String>,
        sign_indicator: Option<char>,
        raw_amount: impl Into<String>,
    ) -> Result<Self, ParseError> {
        let currency = currency.into().to_uppercase();
        let raw_amount = raw_amount.into();

        // Validate currency
        if currency.len() != 3 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "34F".to_string(),
                message: "Currency code must be exactly 3 characters".to_string(),
            });
        }

        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "34F".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate sign indicator if present
        if let Some(sign) = sign_indicator {
            if sign != 'D' && sign != 'C' {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "34F".to_string(),
                    message: "Sign indicator must be 'D' (Debit) or 'C' (Credit)".to_string(),
                });
            }
        }

        let amount = Self::parse_amount(&raw_amount)?;

        if amount <= 0.0 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "34F".to_string(),
                message: "Amount must be positive".to_string(),
            });
        }

        Ok(Field34F {
            currency,
            sign_indicator,
            amount,
            raw_amount: raw_amount.to_string(),
        })
    }

    /// Get the currency code
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Get the sign indicator
    pub fn sign_indicator(&self) -> Option<char> {
        self.sign_indicator
    }

    /// Get the amount value
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Get the raw amount string
    pub fn raw_amount(&self) -> &str {
        &self.raw_amount
    }

    /// Check if this is a debit floor limit
    pub fn is_debit_limit(&self) -> bool {
        self.sign_indicator == Some('D')
    }

    /// Check if this is a credit floor limit
    pub fn is_credit_limit(&self) -> bool {
        self.sign_indicator == Some('C')
    }

    /// Check if this is a combined (no sign) floor limit
    pub fn is_combined_limit(&self) -> bool {
        self.sign_indicator.is_none()
    }

    /// Get the limit type description
    pub fn limit_type(&self) -> &'static str {
        match self.sign_indicator {
            Some('D') => "Debit Floor Limit",
            Some('C') => "Credit Floor Limit",
            _ => "Combined Floor Limit",
        }
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
                field_tag: "34F".to_string(),
                message: "Invalid amount format".to_string(),
            })
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!(
            "{}: {} {}",
            self.limit_type(),
            self.currency,
            self.raw_amount
        )
    }

    /// Check if amount exceeds this floor limit
    pub fn exceeds_limit(&self, amount: f64) -> bool {
        amount > self.amount
    }

    /// Check if this floor limit is for high-value transactions
    pub fn is_high_value_limit(&self) -> bool {
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
}

impl SwiftField for Field34F {
    fn parse(content: &str) -> Result<Self, ParseError> {
        let content = content.trim();
        if content.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "34F".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Remove field tag prefix if present
        let content = if let Some(stripped) = content.strip_prefix(":34F:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("34F:") {
            stripped
        } else {
            content
        };

        if content.len() < 4 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "34F".to_string(),
                message: "Field content too short (minimum 4 characters)".to_string(),
            });
        }

        // Parse components: first 3 characters are currency
        let currency_str = &content[0..3];
        let remaining = &content[3..];

        let currency = currency_str.to_uppercase();

        // Validate currency
        if !currency.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "34F".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Check for optional sign indicator
        let (sign_indicator, amount_str) = if !remaining.is_empty() {
            let first_char = remaining.chars().next().unwrap();
            if first_char == 'D' || first_char == 'C' {
                (Some(first_char), &remaining[1..])
            } else {
                (None, remaining)
            }
        } else {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "34F".to_string(),
                message: "Amount is required".to_string(),
            });
        };

        if amount_str.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "34F".to_string(),
                message: "Amount cannot be empty".to_string(),
            });
        }

        Self::from_raw(currency, sign_indicator, amount_str)
    }

    fn to_swift_string(&self) -> String {
        let sign_part = match self.sign_indicator {
            Some(sign) => sign.to_string(),
            None => String::new(),
        };
        format!(":34F:{}{}{}", self.currency, sign_part, self.raw_amount)
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate currency code
        if self.currency.len() != 3 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "34F".to_string(),
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
                field_tag: "34F".to_string(),
                message: "Currency code must contain only alphabetic characters".to_string(),
            });
        }

        // Validate sign indicator
        if let Some(sign) = self.sign_indicator {
            if sign != 'D' && sign != 'C' {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "34F".to_string(),
                    message: "Sign indicator must be 'D' or 'C'".to_string(),
                });
            }
        }

        // Validate amount
        if self.amount <= 0.0 {
            errors.push(ValidationError::ValueValidation {
                field_tag: "34F".to_string(),
                message: "Amount must be positive".to_string(),
            });
        }

        // Validate raw amount format
        if self.raw_amount.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "34F".to_string(),
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
        "3!a[1!a]15d"
    }
}

impl std::fmt::Display for Field34F {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign_part = match self.sign_indicator {
            Some(sign) => format!(" ({})", if sign == 'D' { "Debit" } else { "Credit" }),
            None => String::new(),
        };
        write!(f, "{} {}{}", self.currency, self.raw_amount, sign_part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field34f_creation_with_sign() {
        let field = Field34F::new("USD", Some('D'), 1000000.00).unwrap();
        assert_eq!(field.currency(), "USD");
        assert_eq!(field.sign_indicator(), Some('D'));
        assert_eq!(field.amount(), 1000000.00);
        assert!(field.is_debit_limit());
        assert!(!field.is_credit_limit());
        assert!(!field.is_combined_limit());
    }

    #[test]
    fn test_field34f_creation_without_sign() {
        let field = Field34F::new("EUR", None, 500000.00).unwrap();
        assert_eq!(field.currency(), "EUR");
        assert!(field.sign_indicator().is_none());
        assert_eq!(field.amount(), 500000.00);
        assert!(!field.is_debit_limit());
        assert!(!field.is_credit_limit());
        assert!(field.is_combined_limit());
    }

    #[test]
    fn test_field34f_from_raw() {
        let field = Field34F::from_raw("GBP", Some('C'), "250000,50").unwrap();
        assert_eq!(field.currency(), "GBP");
        assert_eq!(field.sign_indicator(), Some('C'));
        assert_eq!(field.amount(), 250000.50);
        assert_eq!(field.raw_amount(), "250000,50");
        assert!(field.is_credit_limit());
    }

    #[test]
    fn test_field34f_parse_with_sign() {
        let field = Field34F::parse("USDD1000000,00").unwrap();
        assert_eq!(field.currency(), "USD");
        assert_eq!(field.sign_indicator(), Some('D'));
        assert_eq!(field.amount(), 1000000.00);
    }

    #[test]
    fn test_field34f_parse_without_sign() {
        let field = Field34F::parse("EUR500000,00").unwrap();
        assert_eq!(field.currency(), "EUR");
        assert!(field.sign_indicator().is_none());
        assert_eq!(field.amount(), 500000.00);
    }

    #[test]
    fn test_field34f_parse_with_field_tag() {
        let field = Field34F::parse(":34F:GBPC250000,50").unwrap();
        assert_eq!(field.currency(), "GBP");
        assert_eq!(field.sign_indicator(), Some('C'));
        assert_eq!(field.amount(), 250000.50);
    }

    #[test]
    fn test_field34f_to_swift_string() {
        let field1 = Field34F::new("USD", Some('D'), 1000000.00).unwrap();
        assert_eq!(field1.to_swift_string(), ":34F:USDD1000000,00");

        let field2 = Field34F::new("EUR", None, 500000.00).unwrap();
        assert_eq!(field2.to_swift_string(), ":34F:EUR500000,00");
    }

    #[test]
    fn test_field34f_display() {
        let field1 = Field34F::new("USD", Some('D'), 1000000.00).unwrap();
        assert_eq!(format!("{}", field1), "USD 1000000,00 (Debit)");

        let field2 = Field34F::new("EUR", None, 500000.00).unwrap();
        assert_eq!(format!("{}", field2), "EUR 500000,00");
    }

    #[test]
    fn test_field34f_validation_errors() {
        // Invalid currency length
        let result = Field34F::new("US", Some('D'), 1000.0);
        assert!(result.is_err());

        // Invalid currency characters
        let result = Field34F::new("US1", Some('D'), 1000.0);
        assert!(result.is_err());

        // Invalid sign indicator
        let result = Field34F::new("USD", Some('X'), 1000.0);
        assert!(result.is_err());

        // Zero amount
        let result = Field34F::new("USD", Some('D'), 0.0);
        assert!(result.is_err());

        // Negative amount
        let result = Field34F::new("USD", Some('D'), -100.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_field34f_limit_type() {
        let debit_field = Field34F::new("USD", Some('D'), 1000.0).unwrap();
        assert_eq!(debit_field.limit_type(), "Debit Floor Limit");

        let credit_field = Field34F::new("USD", Some('C'), 1000.0).unwrap();
        assert_eq!(credit_field.limit_type(), "Credit Floor Limit");

        let combined_field = Field34F::new("USD", None, 1000.0).unwrap();
        assert_eq!(combined_field.limit_type(), "Combined Floor Limit");
    }

    #[test]
    fn test_field34f_exceeds_limit() {
        let field = Field34F::new("USD", Some('D'), 1000.0).unwrap();
        assert!(field.exceeds_limit(1500.0));
        assert!(!field.exceeds_limit(500.0));
        assert!(!field.exceeds_limit(1000.0));
    }

    #[test]
    fn test_field34f_is_high_value_limit() {
        let high_field = Field34F::new("USD", Some('D'), 1500000.0).unwrap();
        assert!(high_field.is_high_value_limit());

        let normal_field = Field34F::new("USD", Some('D'), 50000.0).unwrap();
        assert!(!normal_field.is_high_value_limit());
    }

    #[test]
    fn test_field34f_description() {
        let field = Field34F::new("EUR", Some('C'), 750000.0).unwrap();
        assert_eq!(field.description(), "Credit Floor Limit: EUR 750000,00");
    }

    #[test]
    fn test_field34f_parse_dot_decimal() {
        let field = Field34F::parse("USD1234.56").unwrap();
        assert_eq!(field.amount(), 1234.56);
        assert_eq!(field.raw_amount(), "1234.56");
    }

    #[test]
    fn test_field34f_format_amount() {
        let formatted = Field34F::format_amount(1234.56);
        assert_eq!(formatted, "1234,56");
    }

    #[test]
    fn test_field34f_validation() {
        let field = Field34F::new("USD", Some('D'), 1000.0).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field34f_parse_errors() {
        // Empty content
        let result = Field34F::parse("");
        assert!(result.is_err());

        // Too short
        let result = Field34F::parse("USD");
        assert!(result.is_err());

        // Invalid currency
        let result = Field34F::parse("12D1000,00");
        assert!(result.is_err());

        // Missing amount
        let result = Field34F::parse("USDD");
        assert!(result.is_err());
    }
}
