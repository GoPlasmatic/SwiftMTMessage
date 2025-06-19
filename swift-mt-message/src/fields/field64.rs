use crate::fields::common::GenericBalanceField;
use crate::{SwiftField, ValidationError, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Field 64 - Closing Available Balance
///
/// ## Overview
/// Field 64 represents the closing available balance of an account for statement reporting.
/// It follows the standard balance field format with debit/credit mark, date,
/// currency, and amount. This field is used in MT940 and MT942 messages to
/// indicate the available balance at the end of the reporting period.
///
/// ## Format Specification
/// **Format**: `1!a6!n3!a15d`
/// - **1!a**: Debit/Credit mark (D=Debit, C=Credit)
/// - **6!n**: Date in YYMMDD format
/// - **3!a**: Currency code (ISO 4217)
/// - **15d**: Amount with up to 15 digits including decimal places
///
/// ## Usage Context
/// Used in MT940 (Customer Statement Message) and MT942 (Interim Transaction Report) for:
/// - **Available Balance**: Balance available for withdrawal/use
/// - **Liquidity Reporting**: Available funds after holds and restrictions
/// - **Credit Facility**: Available credit limit information
///
/// ## Usage Examples
/// ```text
/// C240315USD1234567,89
/// └─── Credit available balance of USD 1,234,567.89 on March 15, 2024
///
/// D240315EUR250000,00
/// └─── Debit available balance of EUR 250,000.00 on March 15, 2024
/// ```

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field64 {
    /// The underlying generic balance field
    balance: GenericBalanceField,
}

impl Field64 {
    /// Create a new Field64 with validation
    pub fn new(
        debit_credit_mark: char,
        date: impl Into<String>,
        currency: impl Into<String>,
        amount: f64,
    ) -> Result<Self, ParseError> {
        let balance =
            GenericBalanceField::new(debit_credit_mark, date, currency, amount).map_err(|e| {
                if let ParseError::InvalidFieldFormat {
                    field_tag: _,
                    message,
                } = e
                {
                    ParseError::InvalidFieldFormat {
                        field_tag: "64".to_string(),
                        message,
                    }
                } else {
                    e
                }
            })?;

        Ok(Field64 { balance })
    }

    /// Create from raw amount string
    pub fn from_raw(
        debit_credit_mark: char,
        date: impl Into<String>,
        currency: impl Into<String>,
        raw_amount: impl Into<String>,
    ) -> Result<Self, ParseError> {
        let balance = GenericBalanceField::from_raw(debit_credit_mark, date, currency, raw_amount)
            .map_err(|e| {
                if let ParseError::InvalidFieldFormat {
                    field_tag: _,
                    message,
                } = e
                {
                    ParseError::InvalidFieldFormat {
                        field_tag: "64".to_string(),
                        message,
                    }
                } else {
                    e
                }
            })?;

        Ok(Field64 { balance })
    }

    /// Get the debit/credit mark
    pub fn debit_credit_mark(&self) -> char {
        self.balance.debit_credit_mark()
    }

    /// Get the date
    pub fn date(&self) -> &str {
        self.balance.date()
    }

    /// Get the currency code
    pub fn currency(&self) -> &str {
        self.balance.currency()
    }

    /// Get the amount value
    pub fn amount(&self) -> f64 {
        self.balance.amount()
    }

    /// Get the raw amount string
    pub fn raw_amount(&self) -> &str {
        self.balance.raw_amount()
    }

    /// Check if this is a debit available balance
    pub fn is_debit(&self) -> bool {
        self.balance.is_debit()
    }

    /// Check if this is a credit available balance
    pub fn is_credit(&self) -> bool {
        self.balance.is_credit()
    }

    /// Get the balance type description
    pub fn balance_type(&self) -> &'static str {
        self.balance.balance_type()
    }

    /// Get the year from the date (20YY format)
    pub fn year(&self) -> u16 {
        self.balance.year()
    }

    /// Get the month from the date
    pub fn month(&self) -> u8 {
        self.balance.month()
    }

    /// Get the day from the date
    pub fn day(&self) -> u8 {
        self.balance.day()
    }

    /// Get formatted date string (DD/MM/YYYY)
    pub fn formatted_date(&self) -> String {
        self.balance.formatted_date()
    }

    /// Check if this is a high-value available balance
    pub fn is_high_value_balance(&self) -> bool {
        self.balance.is_high_value_balance()
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        self.balance.description("Closing Available Balance")
    }

    /// Check if this available balance indicates no funds available
    pub fn indicates_no_funds_available(&self) -> bool {
        self.balance.amount() == 0.0
    }

    /// Check if this available balance indicates overdraft facility
    pub fn indicates_overdraft_facility(&self) -> bool {
        self.is_debit() && self.amount() > 0.0
    }

    /// Check if this available balance indicates positive liquidity
    pub fn indicates_positive_liquidity(&self) -> bool {
        self.is_credit() && self.amount() > 0.0
    }

    /// Check if this available balance is sufficient for large transactions
    pub fn is_sufficient_for_large_transactions(&self) -> bool {
        self.is_credit() && self.amount() >= 100000.0
    }

    /// Get the underlying generic balance field
    pub fn balance(&self) -> &GenericBalanceField {
        &self.balance
    }
}

impl SwiftField for Field64 {
    fn parse(content: &str) -> Result<Self, ParseError> {
        let balance = GenericBalanceField::parse_with_tag(content, "64")?;
        Ok(Field64 { balance })
    }

    fn to_swift_string(&self) -> String {
        self.balance.to_swift_string_with_tag("64")
    }

    fn validate(&self) -> ValidationResult {
        let mut result = self.balance.validate();

        // Update field tag in errors
        for error in &mut result.errors {
            match error {
                ValidationError::LengthValidation { field_tag, .. } => {
                    *field_tag = "64".to_string();
                }
                ValidationError::FormatValidation { field_tag, .. } => {
                    *field_tag = "64".to_string();
                }
                ValidationError::ValueValidation { field_tag, .. } => {
                    *field_tag = "64".to_string();
                }
                _ => {}
            }
        }

        // Add specific business validations for available balance
        if self.indicates_no_funds_available() {
            result
                .warnings
                .push("Zero available balance - no funds available for withdrawal".to_string());
        }

        if self.indicates_overdraft_facility() {
            result
                .warnings
                .push("Debit available balance indicates overdraft facility available".to_string());
        }

        if self.indicates_positive_liquidity() {
            result
                .warnings
                .push("Positive available balance indicates good liquidity position".to_string());
        }

        if self.is_high_value_balance() {
            result.warnings.push(format!(
                "High-value available balance detected: {} {} - please verify",
                self.currency(),
                self.raw_amount()
            ));
        }

        if self.is_sufficient_for_large_transactions() {
            result
                .warnings
                .push("Available balance is sufficient for large transactions".to_string());
        }

        result
    }

    fn format_spec() -> &'static str {
        "1!a6!n3!a15d"
    }
}

impl std::fmt::Display for Field64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Closing Available Balance: {}", self.balance)
    }
}
