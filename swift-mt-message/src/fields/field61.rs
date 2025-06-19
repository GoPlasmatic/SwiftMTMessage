use crate::{SwiftField, ValidationError, ValidationResult, errors::ParseError};
use regex::Regex;
use serde::{Deserialize, Serialize};

/// # Field 61 - Statement Line
///
/// ## Overview
/// Field 61 represents an individual transaction line in a bank statement.
/// This is the most complex field in MT940/MT942 messages, containing detailed
/// transaction information including dates, amounts, transaction types, and references.
///
/// ## Format Specification
/// **Format**: `6!n[4!n]1!a[1!a]15d1!a3!c16x[//16x][34x]`
/// - **6!n**: Value date (YYMMDD)
/// - **[4!n]**: Entry date (MMDD) - optional
/// - **1!a**: Debit/Credit mark (D=Debit, C=Credit, RD=Reverse Debit, RC=Reverse Credit)
/// - **[1!a]**: Funds code - optional
/// - **15d**: Amount with up to 15 digits including decimal places
/// - **1!a**: Transaction type identification
/// - **3!c**: Customer reference
/// - **16x**: Bank reference
/// - **[//16x]**: Supplementary details - optional
/// - **[34x]**: Information to account owner - optional
///
/// ## Usage Context
/// Used in MT940 (Customer Statement Message) and MT942 (Interim Transaction Report) for:
/// - **Transaction Details**: Individual transaction entries
/// - **Statement Lines**: Each line represents one transaction
/// - **Reconciliation**: Detailed transaction information for matching
///
/// ## Usage Examples
/// ```text
/// 2403150315D125000,00NTRFNONREF//SALARY PAYMENT
/// └─── Debit transaction on 2024-03-15, amount 125,000.00, reference NONREF
///
/// 240316C50000,00FMSCCHGS123456//INCOMING TRANSFER
/// └─── Credit transaction on 2024-03-16, amount 50,000.00, charges reference
/// ```

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field61 {
    /// Value date (YYMMDD)
    value_date: String,
    /// Entry date (MMDD) - optional
    entry_date: Option<String>,
    /// Debit/Credit mark and reverse indicator
    debit_credit_mark: String,
    /// Funds code - optional
    funds_code: Option<char>,
    /// Transaction amount
    amount: f64,
    /// Raw amount string as received
    raw_amount: String,
    /// Transaction type identification
    transaction_type: char,
    /// Customer reference
    customer_reference: String,
    /// Bank reference
    bank_reference: String,
    /// Supplementary details - optional
    supplementary_details: Option<String>,
    /// Information to account owner - optional
    information_to_account_owner: Option<String>,
}

impl Field61 {
    /// Create a new Field61 with validation
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        value_date: impl Into<String>,
        entry_date: Option<String>,
        debit_credit_mark: impl Into<String>,
        funds_code: Option<char>,
        amount: f64,
        transaction_type: char,
        customer_reference: impl Into<String>,
        bank_reference: impl Into<String>,
        supplementary_details: Option<String>,
        information_to_account_owner: Option<String>,
    ) -> Result<Self, ParseError> {
        let value_date = value_date.into();
        let debit_credit_mark = debit_credit_mark.into();
        let customer_reference = customer_reference.into();
        let bank_reference = bank_reference.into();

        // Validate value date format (YYMMDD)
        if value_date.len() != 6 || !value_date.chars().all(|c| c.is_ascii_digit()) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "61".to_string(),
                message: "Value date must be 6 digits (YYMMDD)".to_string(),
            });
        }

        // Validate entry date format (MMDD) if provided
        if let Some(ref ed) = entry_date {
            if ed.len() != 4 || !ed.chars().all(|c| c.is_ascii_digit()) {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "61".to_string(),
                    message: "Entry date must be 4 digits (MMDD)".to_string(),
                });
            }
        }

        // Validate debit/credit mark
        if !matches!(debit_credit_mark.as_str(), "D" | "C" | "RD" | "RC") {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "61".to_string(),
                message: "Debit/Credit mark must be D, C, RD, or RC".to_string(),
            });
        }

        // Validate amount
        if amount < 0.0 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "61".to_string(),
                message: "Transaction amount cannot be negative".to_string(),
            });
        }

        // Validate customer reference length
        if customer_reference.len() > 3 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "61".to_string(),
                message: "Customer reference cannot exceed 3 characters".to_string(),
            });
        }

        // Validate bank reference length
        if bank_reference.len() > 16 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "61".to_string(),
                message: "Bank reference cannot exceed 16 characters".to_string(),
            });
        }

        // Format amount to raw string
        let raw_amount = if amount.fract() == 0.0 {
            format!("{:.0}", amount)
        } else {
            format!("{:.2}", amount).replace('.', ",")
        };

        Ok(Field61 {
            value_date,
            entry_date,
            debit_credit_mark,
            funds_code,
            amount,
            raw_amount,
            transaction_type,
            customer_reference,
            bank_reference,
            supplementary_details,
            information_to_account_owner,
        })
    }

    /// Get the value date
    pub fn value_date(&self) -> &str {
        &self.value_date
    }

    /// Get the entry date
    pub fn entry_date(&self) -> Option<&str> {
        self.entry_date.as_deref()
    }

    /// Get the debit/credit mark
    pub fn debit_credit_mark(&self) -> &str {
        &self.debit_credit_mark
    }

    /// Get the funds code
    pub fn funds_code(&self) -> Option<char> {
        self.funds_code
    }

    /// Get the transaction amount
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Get the raw amount string
    pub fn raw_amount(&self) -> &str {
        &self.raw_amount
    }

    /// Get the transaction type
    pub fn transaction_type(&self) -> char {
        self.transaction_type
    }

    /// Get the customer reference
    pub fn customer_reference(&self) -> &str {
        &self.customer_reference
    }

    /// Get the bank reference
    pub fn bank_reference(&self) -> &str {
        &self.bank_reference
    }

    /// Get the supplementary details
    pub fn supplementary_details(&self) -> Option<&str> {
        self.supplementary_details.as_deref()
    }

    /// Get the information to account owner
    pub fn information_to_account_owner(&self) -> Option<&str> {
        self.information_to_account_owner.as_deref()
    }

    /// Check if this is a debit transaction
    pub fn is_debit(&self) -> bool {
        self.debit_credit_mark.starts_with('D')
    }

    /// Check if this is a credit transaction
    pub fn is_credit(&self) -> bool {
        self.debit_credit_mark.starts_with('C')
    }

    /// Check if this is a reversal transaction
    pub fn is_reversal(&self) -> bool {
        self.debit_credit_mark.starts_with('R')
    }

    /// Check if this is a high-value transaction
    pub fn is_high_value_transaction(&self) -> bool {
        self.amount >= 100000.0
    }

    /// Get formatted value date (DD/MM/YYYY)
    pub fn formatted_value_date(&self) -> String {
        if self.value_date.len() == 6 {
            let year = format!("20{}", &self.value_date[0..2]);
            let month = &self.value_date[2..4];
            let day = &self.value_date[4..6];
            format!("{}/{}/{}", day, month, year)
        } else {
            self.value_date.clone()
        }
    }

    /// Get formatted entry date (DD/MM) if available
    pub fn formatted_entry_date(&self) -> Option<String> {
        self.entry_date.as_ref().map(|ed| {
            if ed.len() == 4 {
                let month = &ed[0..2];
                let day = &ed[2..4];
                format!("{}/{}", day, month)
            } else {
                ed.clone()
            }
        })
    }

    /// Get transaction direction description
    pub fn transaction_direction(&self) -> &'static str {
        match self.debit_credit_mark.as_str() {
            "D" => "Debit",
            "C" => "Credit",
            "RD" => "Reverse Debit",
            "RC" => "Reverse Credit",
            _ => "Unknown",
        }
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!(
            "{} transaction of {} on {} ({})",
            self.transaction_direction(),
            self.raw_amount,
            self.formatted_value_date(),
            self.bank_reference
        )
    }
}

impl SwiftField for Field61 {
    fn parse(content: &str) -> Result<Self, ParseError> {
        // Complex regex for parsing Field 61
        let re = Regex::new(
            r"^(\d{6})(\d{4})?(D|C|RD|RC)([A-Z])?([0-9,]+)([A-Z])([A-Z0-9]{1,3})([A-Z0-9]{1,16})(?://([^/]{1,16}))?(?:/(.{1,34}))?$"
        ).unwrap();

        let captures = re
            .captures(content)
            .ok_or_else(|| ParseError::InvalidFieldFormat {
                field_tag: "61".to_string(),
                message: "Invalid format for Field 61".to_string(),
            })?;

        let value_date = captures[1].to_string();
        let entry_date = captures.get(2).map(|m| m.as_str().to_string());
        let debit_credit_mark = captures[3].to_string();
        let funds_code = captures.get(4).map(|m| m.as_str().chars().next().unwrap());
        let raw_amount = captures[5].to_string();
        let transaction_type = captures[6].chars().next().unwrap();
        let customer_reference = captures[7].to_string();
        let bank_reference = captures[8].to_string();
        let supplementary_details = captures.get(9).map(|m| m.as_str().to_string());
        let information_to_account_owner = captures.get(10).map(|m| m.as_str().to_string());

        // Parse amount from raw string
        let amount_str = raw_amount.replace(',', ".");
        let amount = amount_str
            .parse::<f64>()
            .map_err(|_| ParseError::InvalidFieldFormat {
                field_tag: "61".to_string(),
                message: format!("Invalid amount format: {}", raw_amount),
            })?;

        let mut field = Self::new(
            value_date,
            entry_date,
            debit_credit_mark,
            funds_code,
            amount,
            transaction_type,
            customer_reference,
            bank_reference,
            supplementary_details,
            information_to_account_owner,
        )?;

        // Preserve the original raw amount
        field.raw_amount = raw_amount;

        Ok(field)
    }

    fn to_swift_string(&self) -> String {
        let mut result = format!(
            "{}{}{}{}{}{}{}",
            self.value_date,
            self.entry_date.as_deref().unwrap_or(""),
            self.debit_credit_mark,
            self.funds_code.map(|c| c.to_string()).unwrap_or_default(),
            self.raw_amount,
            self.transaction_type,
            self.customer_reference
        );

        result.push_str(&self.bank_reference);

        if let Some(ref details) = self.supplementary_details {
            result.push_str("//");
            result.push_str(details);
        }

        if let Some(ref info) = self.information_to_account_owner {
            result.push('/');
            result.push_str(info);
        }

        result
    }

    fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::valid();

        // Validate value date format
        if self.value_date.len() != 6 || !self.value_date.chars().all(|c| c.is_ascii_digit()) {
            result.errors.push(ValidationError::FormatValidation {
                field_tag: "61".to_string(),
                message: "Value date must be 6 digits (YYMMDD)".to_string(),
            });
        }

        // Validate entry date format if provided
        if let Some(ref ed) = self.entry_date {
            if ed.len() != 4 || !ed.chars().all(|c| c.is_ascii_digit()) {
                result.errors.push(ValidationError::FormatValidation {
                    field_tag: "61".to_string(),
                    message: "Entry date must be 4 digits (MMDD)".to_string(),
                });
            }
        }

        // Validate debit/credit mark
        if !matches!(self.debit_credit_mark.as_str(), "D" | "C" | "RD" | "RC") {
            result.errors.push(ValidationError::FormatValidation {
                field_tag: "61".to_string(),
                message: "Debit/Credit mark must be D, C, RD, or RC".to_string(),
            });
        }

        // Validate amount
        if self.amount < 0.0 {
            result.errors.push(ValidationError::ValueValidation {
                field_tag: "61".to_string(),
                message: "Transaction amount cannot be negative".to_string(),
            });
        }

        // Business logic validations
        if self.is_reversal() {
            result
                .warnings
                .push("Reversal transaction detected".to_string());
        }

        if self.is_high_value_transaction() {
            result.warnings.push(format!(
                "High-value transaction detected: {} - please verify",
                self.raw_amount
            ));
        }

        if self.customer_reference.is_empty() {
            result
                .warnings
                .push("Customer reference is empty".to_string());
        }

        if self.bank_reference.is_empty() {
            result.warnings.push("Bank reference is empty".to_string());
        }

        result
    }

    fn format_spec() -> &'static str {
        "6!n[4!n]1!a[1!a]15d1!a3!c16x[//16x][34x]"
    }
}

impl std::fmt::Display for Field61 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Statement Line: {}", self.description())
    }
}
