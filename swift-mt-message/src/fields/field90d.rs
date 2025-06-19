use crate::{SwiftField, ValidationError, ValidationResult, errors::ParseError};
use regex::Regex;
use serde::{Deserialize, Serialize};

/// # Field 90D - Sum of Debits
///
/// ## Overview
/// Field 90D represents the sum of all debit entries in a statement period.
/// It contains the number of entries and the total amount of debits.
/// This field is used in MT940 and MT942 messages for reconciliation
/// and summary reporting purposes.
///
/// ## Format Specification
/// **Format**: `3!n3!a15d`
/// - **3!n**: Number of debit entries (up to 3 digits)
/// - **3!a**: Currency code (ISO 4217)
/// - **15d**: Total amount with up to 15 digits including decimal places
///
/// ## Usage Context
/// Used in MT940 (Customer Statement Message) and MT942 (Interim Transaction Report) for:
/// - **Debit Summary**: Total of all debit transactions
/// - **Reconciliation**: Verification of debit transaction totals
/// - **Reporting**: Summary statistics for the statement period
///
/// ## Usage Examples
/// ```text
/// 035USD987654,32
/// └─── 35 debit entries totaling USD 987,654.32
///
/// 125EUR750000,00
/// └─── 125 debit entries totaling EUR 750,000.00
/// ```

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field90D {
    /// Number of debit entries
    entry_count: u16,
    /// Currency code
    currency: String,
    /// Total amount of debits
    amount: f64,
    /// Raw amount string as received
    raw_amount: String,
}

impl Field90D {
    /// Create a new Field90D with validation
    pub fn new(
        entry_count: u16,
        currency: impl Into<String>,
        amount: f64,
    ) -> Result<Self, ParseError> {
        let currency = currency.into();

        // Validate entry count
        if entry_count > 999 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "90D".to_string(),
                message: "Entry count cannot exceed 999".to_string(),
            });
        }

        // Validate currency code
        if currency.len() != 3 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "90D".to_string(),
                message: "Currency code must be exactly 3 characters".to_string(),
            });
        }

        // Validate amount
        if amount < 0.0 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "90D".to_string(),
                message: "Debit sum amount cannot be negative".to_string(),
            });
        }

        // Format amount to raw string
        let raw_amount = if amount.fract() == 0.0 {
            format!("{:.0}", amount)
        } else {
            format!("{:.2}", amount).replace('.', ",")
        };

        Ok(Field90D {
            entry_count,
            currency,
            amount,
            raw_amount,
        })
    }

    /// Create from raw amount string
    pub fn from_raw(
        entry_count: u16,
        currency: impl Into<String>,
        raw_amount: impl Into<String>,
    ) -> Result<Self, ParseError> {
        let currency = currency.into();
        let raw_amount = raw_amount.into();

        // Validate entry count
        if entry_count > 999 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "90D".to_string(),
                message: "Entry count cannot exceed 999".to_string(),
            });
        }

        // Validate currency code
        if currency.len() != 3 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "90D".to_string(),
                message: "Currency code must be exactly 3 characters".to_string(),
            });
        }

        // Parse amount from raw string
        let amount_str = raw_amount.replace(',', ".");
        let amount = amount_str
            .parse::<f64>()
            .map_err(|_| ParseError::InvalidFieldFormat {
                field_tag: "90D".to_string(),
                message: format!("Invalid amount format: {}", raw_amount),
            })?;

        if amount < 0.0 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "90D".to_string(),
                message: "Debit sum amount cannot be negative".to_string(),
            });
        }

        Ok(Field90D {
            entry_count,
            currency,
            amount,
            raw_amount,
        })
    }

    /// Get the number of debit entries
    pub fn entry_count(&self) -> u16 {
        self.entry_count
    }

    /// Get the currency code
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Get the total amount
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Get the raw amount string
    pub fn raw_amount(&self) -> &str {
        &self.raw_amount
    }

    /// Check if this represents a high volume of debit entries
    pub fn is_high_volume(&self) -> bool {
        self.entry_count >= 100
    }

    /// Check if this represents a high-value debit sum
    pub fn is_high_value_sum(&self) -> bool {
        self.amount >= 1000000.0
    }

    /// Check if this represents no debit activity
    pub fn is_no_debit_activity(&self) -> bool {
        self.entry_count == 0 && self.amount == 0.0
    }

    /// Get average debit amount per entry
    pub fn average_debit_amount(&self) -> f64 {
        if self.entry_count == 0 {
            0.0
        } else {
            self.amount / self.entry_count as f64
        }
    }

    /// Check if the average debit amount is unusually high
    pub fn has_unusually_high_average(&self) -> bool {
        let avg = self.average_debit_amount();
        avg >= 50000.0 && self.entry_count > 0
    }

    /// Check if this represents excessive debit activity
    pub fn indicates_excessive_debit_activity(&self) -> bool {
        self.entry_count >= 200 || self.amount >= 5000000.0
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!(
            "Sum of {} debit entries: {} {}",
            self.entry_count, self.currency, self.raw_amount
        )
    }
}

impl SwiftField for Field90D {
    fn parse(content: &str) -> Result<Self, ParseError> {
        let re = Regex::new(r"^(\d{1,3})([A-Z]{3})(.+)$").unwrap();

        let captures = re
            .captures(content)
            .ok_or_else(|| ParseError::InvalidFieldFormat {
                field_tag: "90D".to_string(),
                message: "Invalid format for Field 90D".to_string(),
            })?;

        let entry_count =
            captures[1]
                .parse::<u16>()
                .map_err(|_| ParseError::InvalidFieldFormat {
                    field_tag: "90D".to_string(),
                    message: "Invalid entry count format".to_string(),
                })?;

        let currency = captures[2].to_string();
        let raw_amount = captures[3].to_string();

        Self::from_raw(entry_count, currency, raw_amount)
    }

    fn to_swift_string(&self) -> String {
        format!(
            "{:03}{}{}",
            self.entry_count, self.currency, self.raw_amount
        )
    }

    fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::valid();

        // Validate entry count
        if self.entry_count > 999 {
            result.errors.push(ValidationError::ValueValidation {
                field_tag: "90D".to_string(),
                message: "Entry count cannot exceed 999".to_string(),
            });
        }

        // Validate currency code
        if self.currency.len() != 3 {
            result.errors.push(ValidationError::FormatValidation {
                field_tag: "90D".to_string(),
                message: "Currency code must be exactly 3 characters".to_string(),
            });
        }

        // Validate amount
        if self.amount < 0.0 {
            result.errors.push(ValidationError::ValueValidation {
                field_tag: "90D".to_string(),
                message: "Debit sum amount cannot be negative".to_string(),
            });
        }

        // Business logic validations
        if self.is_no_debit_activity() {
            result
                .warnings
                .push("No debit activity recorded for this period".to_string());
        }

        if self.is_high_volume() {
            result.warnings.push(format!(
                "High volume of debit entries detected: {} entries",
                self.entry_count
            ));
        }

        if self.is_high_value_sum() {
            result.warnings.push(format!(
                "High-value debit sum detected: {} {} - please verify",
                self.currency, self.raw_amount
            ));
        }

        if self.has_unusually_high_average() {
            result.warnings.push(format!(
                "Unusually high average debit amount: {:.2} {} per entry",
                self.average_debit_amount(),
                self.currency
            ));
        }

        if self.indicates_excessive_debit_activity() {
            result.warnings.push(
                "Excessive debit activity detected - may require additional scrutiny".to_string(),
            );
        }

        // Consistency check
        if self.entry_count > 0 && self.amount == 0.0 {
            result
                .warnings
                .push("Entry count is positive but total amount is zero".to_string());
        }

        if self.entry_count == 0 && self.amount > 0.0 {
            result.errors.push(ValidationError::ValueValidation {
                field_tag: "90D".to_string(),
                message: "Cannot have zero entries with non-zero amount".to_string(),
            });
        }

        result
    }

    fn format_spec() -> &'static str {
        "3!n3!a15d"
    }
}

impl std::fmt::Display for Field90D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sum of Debits: {}", self.description())
    }
}
