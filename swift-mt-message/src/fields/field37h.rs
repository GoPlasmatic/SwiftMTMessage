use crate::{SwiftField, ValidationError, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Field 37H - Interest Rate
///
/// ## Overview
/// Field 37H represents interest rates in SWIFT MT messages. It includes
/// an indicator for the type of rate and the actual rate value. The field
/// supports both positive and negative rates with optional 'N' indicator
/// for negative values.
///
/// ## Format Specification
/// **Format**: `1!a[N]12d`
/// - **1!a**: Rate type indicator (various codes)
/// - **[N]**: Optional 'N' for negative rates
/// - **12d**: Rate value with up to 12 digits including decimal places
///
/// ## Usage Context
/// Used in MT940 (Customer Statement Message) and MT950 (Statement Message) for:
/// - **Interest rates**: Applied to account balances
/// - **Penalty rates**: For overdraft situations
/// - **Credit rates**: For positive balances
/// - **Debit rates**: For negative balances
///
/// ## Usage Examples
/// ```text
/// D3,250
/// └─── Debit rate of 3.25%
///
/// CN2,500
/// └─── Credit rate of -2.50% (negative rate)
///
/// P5,000
/// └─── Penalty rate of 5.00%
/// ```
///
/// ## Validation Rules
/// 1. **Rate indicator**: Must be single alphabetic character
/// 2. **Negative indicator**: If present, must be 'N'
/// 3. **Rate format**: Must follow SWIFT decimal format (comma separator)
/// 4. **Rate range**: Typically between -100.00% and +100.00%
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Rate indicator must be single character (Error: T18)
/// - Rate must be properly formatted (Error: T40)
/// - Decimal separator must be comma (Error: T41)
/// - Rate value must be reasonable (Warning: business validation)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field37H {
    /// Rate type indicator (single character)
    pub rate_indicator: char,
    /// Whether this is a negative rate (N indicator present)
    pub is_negative: bool,
    /// Rate value as floating point (percentage)
    pub rate: f64,
    /// Raw rate string as received (preserves original formatting)
    pub raw_rate: String,
}

impl Field37H {
    /// Create a new Field37H with validation
    ///
    /// # Arguments
    /// * `rate_indicator` - Rate type indicator (single alphabetic character)
    /// * `is_negative` - Whether this is a negative rate
    /// * `rate` - Rate value as percentage (e.g., 3.25 for 3.25%)
    ///
    /// # Returns
    /// Result containing the Field37H instance or validation error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field37H;
    /// let field = Field37H::new('D', false, 3.25).unwrap();
    /// assert_eq!(field.rate_indicator(), 'D');
    /// assert!(!field.is_negative());
    /// assert_eq!(field.rate(), 3.25);
    /// ```
    pub fn new(rate_indicator: char, is_negative: bool, rate: f64) -> Result<Self, ParseError> {
        // Validate rate indicator
        if !rate_indicator.is_alphabetic() || !rate_indicator.is_ascii() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "37H".to_string(),
                message: "Rate indicator must be a single alphabetic character".to_string(),
            });
        }

        // Validate rate range (reasonable business limits)
        if !(-100.0..=100.0).contains(&rate) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "37H".to_string(),
                message: "Rate must be between -100.00% and +100.00%".to_string(),
            });
        }

        let raw_rate = Self::format_rate(rate);

        Ok(Field37H {
            rate_indicator: rate_indicator.to_ascii_uppercase(),
            is_negative,
            rate,
            raw_rate,
        })
    }

    /// Create from raw rate string
    ///
    /// # Arguments
    /// * `rate_indicator` - Rate type indicator
    /// * `is_negative` - Whether this is a negative rate
    /// * `raw_rate` - Raw rate string (preserves original formatting)
    ///
    /// # Returns
    /// Result containing the Field37H instance or validation error
    pub fn from_raw(
        rate_indicator: char,
        is_negative: bool,
        raw_rate: impl Into<String>,
    ) -> Result<Self, ParseError> {
        let raw_rate = raw_rate.into();

        // Validate rate indicator
        if !rate_indicator.is_alphabetic() || !rate_indicator.is_ascii() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "37H".to_string(),
                message: "Rate indicator must be a single alphabetic character".to_string(),
            });
        }

        let rate = Self::parse_rate(&raw_rate)?;

        // Validate rate range
        if !(-100.0..=100.0).contains(&rate) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "37H".to_string(),
                message: "Rate must be between -100.00% and +100.00%".to_string(),
            });
        }

        Ok(Field37H {
            rate_indicator: rate_indicator.to_ascii_uppercase(),
            is_negative,
            rate,
            raw_rate: raw_rate.to_string(),
        })
    }

    /// Get the rate indicator
    pub fn rate_indicator(&self) -> char {
        self.rate_indicator
    }

    /// Check if this is a negative rate
    pub fn is_negative(&self) -> bool {
        self.is_negative
    }

    /// Get the rate value
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Get the raw rate string
    pub fn raw_rate(&self) -> &str {
        &self.raw_rate
    }

    /// Get the effective rate (considering negative indicator)
    pub fn effective_rate(&self) -> f64 {
        if self.is_negative {
            -self.rate.abs()
        } else {
            self.rate
        }
    }

    /// Get the rate type description
    pub fn rate_type(&self) -> &'static str {
        match self.rate_indicator {
            'D' => "Debit Rate",
            'C' => "Credit Rate",
            'P' => "Penalty Rate",
            'B' => "Base Rate",
            'O' => "Overdraft Rate",
            'S' => "Savings Rate",
            _ => "Other Rate",
        }
    }

    /// Check if this is a debit rate
    pub fn is_debit_rate(&self) -> bool {
        self.rate_indicator == 'D'
    }

    /// Check if this is a credit rate
    pub fn is_credit_rate(&self) -> bool {
        self.rate_indicator == 'C'
    }

    /// Check if this is a penalty rate
    pub fn is_penalty_rate(&self) -> bool {
        self.rate_indicator == 'P'
    }

    /// Check if this is a high interest rate
    pub fn is_high_rate(&self) -> bool {
        self.rate.abs() >= 10.0 // 10% or higher
    }

    /// Check if this is a zero rate
    pub fn is_zero_rate(&self) -> bool {
        self.rate.abs() < 0.001 // Effectively zero
    }

    /// Format rate for SWIFT output (with comma as decimal separator)
    pub fn format_rate(rate: f64) -> String {
        format!("{:.3}", rate).replace('.', ",")
    }

    /// Parse rate from string (handles both comma and dot as decimal separator)
    fn parse_rate(rate_str: &str) -> Result<f64, ParseError> {
        let normalized_rate = rate_str.replace(',', ".");

        normalized_rate
            .parse::<f64>()
            .map_err(|_| ParseError::InvalidFieldFormat {
                field_tag: "37H".to_string(),
                message: "Invalid rate format".to_string(),
            })
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        let sign = if self.is_negative { "-" } else { "" };
        format!("{}: {}{}%", self.rate_type(), sign, self.raw_rate)
    }

    /// Get formatted rate with percentage sign
    pub fn formatted_rate(&self) -> String {
        let sign = if self.is_negative { "-" } else { "" };
        format!("{}{}%", sign, self.raw_rate)
    }

    /// Calculate interest amount for a given principal
    pub fn calculate_interest(&self, principal: f64, days: u32) -> f64 {
        let annual_rate = self.effective_rate() / 100.0;
        let daily_rate = annual_rate / 365.0;
        principal * daily_rate * days as f64
    }
}

impl SwiftField for Field37H {
    fn parse(content: &str) -> Result<Self, ParseError> {
        let content = content.trim();
        if content.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "37H".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Remove field tag prefix if present
        let content = if let Some(stripped) = content.strip_prefix(":37H:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("37H:") {
            stripped
        } else {
            content
        };

        if content.len() < 2 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "37H".to_string(),
                message: "Field content too short (minimum 2 characters)".to_string(),
            });
        }

        // Parse components
        let rate_indicator = content.chars().next().unwrap();
        let remaining = &content[1..];

        // Check for negative indicator
        let (is_negative, rate_str) = if let Some(stripped) = remaining.strip_prefix('N') {
            (true, stripped)
        } else {
            (false, remaining)
        };

        if rate_str.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "37H".to_string(),
                message: "Rate value is required".to_string(),
            });
        }

        Self::from_raw(rate_indicator, is_negative, rate_str)
    }

    fn to_swift_string(&self) -> String {
        let negative_part = if self.is_negative { "N" } else { "" };
        format!(
            ":37H:{}{}{}",
            self.rate_indicator, negative_part, self.raw_rate
        )
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Validate rate indicator
        if !self.rate_indicator.is_alphabetic() || !self.rate_indicator.is_ascii() {
            errors.push(ValidationError::FormatValidation {
                field_tag: "37H".to_string(),
                message: "Rate indicator must be a single alphabetic character".to_string(),
            });
        }

        // Validate rate range
        if self.rate < -100.0 || self.rate > 100.0 {
            errors.push(ValidationError::ValueValidation {
                field_tag: "37H".to_string(),
                message: "Rate must be between -100.00% and +100.00%".to_string(),
            });
        }

        // Business validation warnings
        if self.is_high_rate() {
            warnings.push(format!(
                "High interest rate detected: {}% - please verify",
                self.rate
            ));
        }

        if self.is_zero_rate() {
            warnings.push("Zero interest rate detected".to_string());
        }

        // Validate raw rate format
        if self.raw_rate.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "37H".to_string(),
                message: "Rate value cannot be empty".to_string(),
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    fn format_spec() -> &'static str {
        "1!a[N]12d"
    }
}

impl std::fmt::Display for Field37H {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.formatted_rate(), self.rate_type())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field37h_creation() {
        let field = Field37H::new('D', false, 3.25).unwrap();
        assert_eq!(field.rate_indicator(), 'D');
        assert!(!field.is_negative());
        assert_eq!(field.rate(), 3.25);
        assert!(field.is_debit_rate());
        assert_eq!(field.rate_type(), "Debit Rate");
    }

    #[test]
    fn test_field37h_negative_rate() {
        let field = Field37H::new('C', true, 2.50).unwrap();
        assert_eq!(field.rate_indicator(), 'C');
        assert!(field.is_negative());
        assert_eq!(field.rate(), 2.50);
        assert_eq!(field.effective_rate(), -2.50);
        assert!(field.is_credit_rate());
    }

    #[test]
    fn test_field37h_from_raw() {
        let field = Field37H::from_raw('P', false, "5,000").unwrap();
        assert_eq!(field.rate_indicator(), 'P');
        assert!(!field.is_negative());
        assert_eq!(field.rate(), 5.0);
        assert_eq!(field.raw_rate(), "5,000");
        assert!(field.is_penalty_rate());
    }

    #[test]
    fn test_field37h_parse() {
        let field = Field37H::parse("D3,250").unwrap();
        assert_eq!(field.rate_indicator(), 'D');
        assert!(!field.is_negative());
        assert_eq!(field.rate(), 3.25);
    }

    #[test]
    fn test_field37h_parse_negative() {
        let field = Field37H::parse("CN2,500").unwrap();
        assert_eq!(field.rate_indicator(), 'C');
        assert!(field.is_negative());
        assert_eq!(field.rate(), 2.50);
        assert_eq!(field.effective_rate(), -2.50);
    }

    #[test]
    fn test_field37h_parse_with_field_tag() {
        let field = Field37H::parse(":37H:P5,000").unwrap();
        assert_eq!(field.rate_indicator(), 'P');
        assert!(!field.is_negative());
        assert_eq!(field.rate(), 5.0);
    }

    #[test]
    fn test_field37h_to_swift_string() {
        let field1 = Field37H::new('D', false, 3.25).unwrap();
        assert_eq!(field1.to_swift_string(), ":37H:D3,250");

        let field2 = Field37H::new('C', true, 2.50).unwrap();
        assert_eq!(field2.to_swift_string(), ":37H:CN2,500");
    }

    #[test]
    fn test_field37h_display() {
        let field1 = Field37H::new('D', false, 3.25).unwrap();
        assert_eq!(format!("{}", field1), "3,250% (Debit Rate)");

        let field2 = Field37H::new('C', true, 2.50).unwrap();
        assert_eq!(format!("{}", field2), "-2,500% (Credit Rate)");
    }

    #[test]
    fn test_field37h_validation_errors() {
        // Invalid rate indicator
        let result = Field37H::new('1', false, 3.0);
        assert!(result.is_err());

        // Rate too high
        let result = Field37H::new('D', false, 150.0);
        assert!(result.is_err());

        // Rate too low
        let result = Field37H::new('D', false, -150.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_field37h_rate_types() {
        let debit_field = Field37H::new('D', false, 3.0).unwrap();
        assert!(debit_field.is_debit_rate());
        assert!(!debit_field.is_credit_rate());
        assert!(!debit_field.is_penalty_rate());

        let credit_field = Field37H::new('C', false, 2.0).unwrap();
        assert!(!credit_field.is_debit_rate());
        assert!(credit_field.is_credit_rate());
        assert!(!credit_field.is_penalty_rate());

        let penalty_field = Field37H::new('P', false, 8.0).unwrap();
        assert!(!penalty_field.is_debit_rate());
        assert!(!penalty_field.is_credit_rate());
        assert!(penalty_field.is_penalty_rate());
    }

    #[test]
    fn test_field37h_high_rate() {
        let high_field = Field37H::new('D', false, 15.0).unwrap();
        assert!(high_field.is_high_rate());

        let normal_field = Field37H::new('D', false, 5.0).unwrap();
        assert!(!normal_field.is_high_rate());
    }

    #[test]
    fn test_field37h_zero_rate() {
        let zero_field = Field37H::new('D', false, 0.0).unwrap();
        assert!(zero_field.is_zero_rate());

        let nonzero_field = Field37H::new('D', false, 1.0).unwrap();
        assert!(!nonzero_field.is_zero_rate());
    }

    #[test]
    fn test_field37h_calculate_interest() {
        let field = Field37H::new('D', false, 5.0).unwrap(); // 5% annual rate
        let interest = field.calculate_interest(10000.0, 30); // 30 days on $10,000
        assert!((interest - 41.096).abs() < 0.01); // Approximately $41.10
    }

    #[test]
    fn test_field37h_description() {
        let field1 = Field37H::new('D', false, 3.25).unwrap();
        assert_eq!(field1.description(), "Debit Rate: 3,250%");

        let field2 = Field37H::new('C', true, 2.50).unwrap();
        assert_eq!(field2.description(), "Credit Rate: -2,500%");
    }

    #[test]
    fn test_field37h_parse_dot_decimal() {
        let field = Field37H::parse("D3.25").unwrap();
        assert_eq!(field.rate(), 3.25);
        assert_eq!(field.raw_rate(), "3.25");
    }

    #[test]
    fn test_field37h_format_rate() {
        let formatted = Field37H::format_rate(3.25);
        assert_eq!(formatted, "3,250");
    }

    #[test]
    fn test_field37h_validation() {
        let field = Field37H::new('D', false, 3.25).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field37h_parse_errors() {
        // Empty content
        let result = Field37H::parse("");
        assert!(result.is_err());

        // Too short
        let result = Field37H::parse("D");
        assert!(result.is_err());

        // Invalid rate indicator
        let result = Field37H::parse("13,25");
        assert!(result.is_err());

        // Missing rate
        let result = Field37H::parse("DN");
        assert!(result.is_err());
    }
}
