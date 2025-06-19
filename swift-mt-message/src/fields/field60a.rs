use crate::fields::common::GenericBalanceField;
use crate::{SwiftField, ValidationError, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Field 60A - Opening Balance
///
/// ## Overview
/// Field 60A represents the opening balance of an account for statement reporting.
/// It follows the standard balance field format with debit/credit mark, date,
/// currency, and amount. This field is used in MT940 and MT942 messages to
/// indicate the starting balance for the reporting period.
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
/// - **Opening Balance**: Starting balance for the statement period
/// - **Account Balance**: Current balance information
/// - **Balance Reporting**: Historical balance information
///
/// ## Usage Examples
/// ```text
/// C240315USD1234567,89
/// └─── Credit opening balance of USD 1,234,567.89 on March 15, 2024
///
/// D240315EUR500000,00
/// └─── Debit opening balance of EUR 500,000.00 on March 15, 2024
/// ```

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field60A {
    /// The underlying generic balance field
    balance: GenericBalanceField,
}

impl Field60A {
    /// Create a new Field60A with validation
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
                        field_tag: "60A".to_string(),
                        message,
                    }
                } else {
                    e
                }
            })?;

        Ok(Field60A { balance })
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
                        field_tag: "60A".to_string(),
                        message,
                    }
                } else {
                    e
                }
            })?;

        Ok(Field60A { balance })
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

    /// Check if this is a debit opening balance
    pub fn is_debit(&self) -> bool {
        self.balance.is_debit()
    }

    /// Check if this is a credit opening balance
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

    /// Check if this is a high-value opening balance
    pub fn is_high_value_balance(&self) -> bool {
        self.balance.is_high_value_balance()
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        self.balance.description("Opening Balance")
    }

    /// Check if this opening balance indicates account closure
    pub fn indicates_account_closure(&self) -> bool {
        self.balance.amount() == 0.0
    }

    /// Check if this opening balance indicates overdraft
    pub fn indicates_overdraft(&self) -> bool {
        self.is_debit() && self.amount() > 0.0
    }

    /// Get the underlying generic balance field
    pub fn balance(&self) -> &GenericBalanceField {
        &self.balance
    }
}

impl SwiftField for Field60A {
    fn parse(content: &str) -> Result<Self, ParseError> {
        let balance = GenericBalanceField::parse_with_tag(content, "60A")?;
        Ok(Field60A { balance })
    }

    fn to_swift_string(&self) -> String {
        self.balance.to_swift_string_with_tag("60A")
    }

    fn validate(&self) -> ValidationResult {
        let mut result = self.balance.validate();

        // Update field tag in errors
        for error in &mut result.errors {
            match error {
                ValidationError::LengthValidation { field_tag, .. } => {
                    *field_tag = "60A".to_string();
                }
                ValidationError::FormatValidation { field_tag, .. } => {
                    *field_tag = "60A".to_string();
                }
                ValidationError::ValueValidation { field_tag, .. } => {
                    *field_tag = "60A".to_string();
                }
                _ => {}
            }
        }

        // Add specific business validations for opening balance
        if self.indicates_account_closure() {
            result
                .warnings
                .push("Zero opening balance may indicate account closure".to_string());
        }

        if self.indicates_overdraft() {
            result
                .warnings
                .push("Debit opening balance indicates overdraft situation".to_string());
        }

        if self.is_high_value_balance() {
            result.warnings.push(format!(
                "High-value opening balance detected: {} {} - please verify",
                self.currency(),
                self.raw_amount()
            ));
        }

        result
    }

    fn format_spec() -> &'static str {
        "1!a6!n3!a15d"
    }
}

impl std::fmt::Display for Field60A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Opening Balance: {}", self.balance)
    }
}
