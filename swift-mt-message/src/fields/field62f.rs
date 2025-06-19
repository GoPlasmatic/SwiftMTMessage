use crate::fields::common::GenericBalanceField;
use crate::{SwiftField, ValidationError, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Field 62F - Closing Balance (Final)
///
/// ## Overview
/// Field 62F represents the closing balance of an account for statement reporting.
/// It follows the standard balance field format with debit/credit mark, date,
/// currency, and amount. This field is used in MT940 and MT942 messages to
/// indicate the final balance at the end of the reporting period.
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
/// - **Closing Balance**: Final balance at the end of the statement period
/// - **End-of-Day Balance**: Balance at the close of business
/// - **Statement Reconciliation**: Final balance for reconciliation purposes
///
/// ## Usage Examples
/// ```text
/// C240315USD1534567,89
/// └─── Credit closing balance of USD 1,534,567.89 on March 15, 2024
///
/// D240315EUR250000,00
/// └─── Debit closing balance of EUR 250,000.00 on March 15, 2024
/// ```

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field62F {
    /// The underlying generic balance field
    balance: GenericBalanceField,
}

impl Field62F {
    /// Create a new Field62F with validation
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
                        field_tag: "62F".to_string(),
                        message,
                    }
                } else {
                    e
                }
            })?;

        Ok(Field62F { balance })
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
                        field_tag: "62F".to_string(),
                        message,
                    }
                } else {
                    e
                }
            })?;

        Ok(Field62F { balance })
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

    /// Check if this is a debit closing balance
    pub fn is_debit(&self) -> bool {
        self.balance.is_debit()
    }

    /// Check if this is a credit closing balance
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

    /// Check if this is a high-value closing balance
    pub fn is_high_value_balance(&self) -> bool {
        self.balance.is_high_value_balance()
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        self.balance.description("Closing Balance")
    }

    /// Check if this closing balance indicates account closure
    pub fn indicates_account_closure(&self) -> bool {
        self.balance.amount() == 0.0
    }

    /// Check if this closing balance indicates overdraft
    pub fn indicates_overdraft(&self) -> bool {
        self.is_debit() && self.amount() > 0.0
    }

    /// Check if this closing balance indicates positive cash flow
    pub fn indicates_positive_cash_flow(&self) -> bool {
        self.is_credit() && self.amount() > 0.0
    }

    /// Get the underlying generic balance field
    pub fn balance(&self) -> &GenericBalanceField {
        &self.balance
    }
}

impl SwiftField for Field62F {
    fn parse(content: &str) -> Result<Self, ParseError> {
        let balance = GenericBalanceField::parse_with_tag(content, "62F")?;
        Ok(Field62F { balance })
    }

    fn to_swift_string(&self) -> String {
        self.balance.to_swift_string_with_tag("62F")
    }

    fn validate(&self) -> ValidationResult {
        let mut result = self.balance.validate();

        // Update field tag in errors
        for error in &mut result.errors {
            match error {
                ValidationError::LengthValidation { field_tag, .. } => {
                    *field_tag = "62F".to_string();
                }
                ValidationError::FormatValidation { field_tag, .. } => {
                    *field_tag = "62F".to_string();
                }
                ValidationError::ValueValidation { field_tag, .. } => {
                    *field_tag = "62F".to_string();
                }
                _ => {}
            }
        }

        // Add specific business validations for closing balance
        if self.indicates_account_closure() {
            result
                .warnings
                .push("Zero closing balance may indicate account closure".to_string());
        }

        if self.indicates_overdraft() {
            result
                .warnings
                .push("Debit closing balance indicates overdraft situation".to_string());
        }

        if self.is_high_value_balance() {
            result.warnings.push(format!(
                "High-value closing balance detected: {} {} - please verify",
                self.currency(),
                self.raw_amount()
            ));
        }

        if self.indicates_positive_cash_flow() {
            result
                .warnings
                .push("Positive closing balance indicates healthy cash position".to_string());
        }

        result
    }

    fn format_spec() -> &'static str {
        "1!a6!n3!a15d"
    }
}

impl std::fmt::Display for Field62F {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Closing Balance: {}", self.balance)
    }
}
