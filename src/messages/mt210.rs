use crate::errors::{ParseError, SwiftValidationError};
use crate::fields::*;
use crate::parser::MessageParser;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// **MT210: Notice to Receive**
///
/// Advises correspondent that funds have been/will be deposited to account.
///
/// **Usage:** Deposit notifications, account funding notices
/// **Category:** Category 2 (Financial Institution Transfers)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT210 {
    /// Transaction Reference Number (Field 20)
    #[serde(rename = "20")]
    pub transaction_reference: Field20,

    /// Account Identification (Field 25)
    #[serde(rename = "25", skip_serializing_if = "Option::is_none")]
    pub account_identification: Option<Field25NoOption>,

    /// Value Date (Field 30)
    #[serde(rename = "30")]
    pub value_date: Field30,

    /// Transactions (repeatable)
    #[serde(rename = "#", default)]
    pub transactions: Vec<MT210Transaction>,
}

/// Individual transaction within MT210
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT210Transaction {
    /// Related Reference (Field 21)
    #[serde(rename = "21", skip_serializing_if = "Option::is_none")]
    pub related_reference: Option<Field21NoOption>,

    /// Currency Code, Amount (Field 32B)
    #[serde(rename = "32B")]
    pub currency_amount: Field32B,

    /// Ordering Customer (Field 50)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub ordering_customer: Option<Field50>,

    /// Ordering Institution (Field 52)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub ordering_institution: Option<Field52OrderingInstitution>,

    /// Intermediary Institution (Field 56)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub intermediary: Option<Field56>,
}

impl MT210 {
    /// Parse MT210 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "210");

        // Parse mandatory fields
        let transaction_reference = parser.parse_field::<Field20>("20")?;

        // Parse optional Field 25
        let account_identification = parser.parse_optional_field::<Field25NoOption>("25")?;

        // Parse mandatory Field 30
        let value_date = parser.parse_field::<Field30>("30")?;

        // Parse repeating transaction sequences - enable duplicates mode
        parser = parser.with_duplicates(true);
        let mut transactions = Vec::new();

        while parser.detect_field("21") || parser.detect_field("32B") {
            // Parse optional Field 21 - Related Reference
            let related_reference = parser.parse_optional_field::<Field21NoOption>("21")?;

            // Parse mandatory Field 32B - Currency Code, Amount
            let currency_amount = parser.parse_field::<Field32B>("32B")?;

            // Parse optional Field 50 - Ordering Customer
            let ordering_customer = parser.parse_optional_variant_field::<Field50>("50")?;

            // Parse optional Field 52 - Ordering Institution
            let ordering_institution =
                parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;

            // Parse optional Field 56 - Intermediary Institution
            let intermediary = parser.parse_optional_variant_field::<Field56>("56")?;

            transactions.push(MT210Transaction {
                related_reference,
                currency_amount,
                ordering_customer,
                ordering_institution,
                intermediary,
            });

            // Limit to 10 sequences
            if transactions.len() >= 10 {
                break;
            }
        }

        Ok(MT210 {
            transaction_reference,
            account_identification,
            value_date,
            transactions,
        })
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT210)
    // ========================================================================

    /// Maximum number of repetitive sequences allowed
    const MAX_REPETITIVE_SEQUENCES: usize = 10;

    // ========================================================================
    // VALIDATION RULES (C1-C3)
    // ========================================================================

    /// C1: Repetitive Sequence Count (Error code: T10)
    /// The repetitive sequence must not appear more than ten times
    fn validate_c1_repetitive_sequence_count(&self) -> Option<SwiftValidationError> {
        if self.transactions.len() > Self::MAX_REPETITIVE_SEQUENCES {
            return Some(SwiftValidationError::format_error(
                "T10",
                "21",
                &self.transactions.len().to_string(),
                &format!("Max {} occurrences", Self::MAX_REPETITIVE_SEQUENCES),
                &format!(
                    "The repetitive sequence must not appear more than {} times. Found {} occurrences",
                    Self::MAX_REPETITIVE_SEQUENCES,
                    self.transactions.len()
                ),
            ));
        }

        None
    }

    /// C2: Ordering Customer and Ordering Institution Mutual Exclusivity (Error code: C06)
    /// Either field 50a or field 52a, but not both, must be present in a repetitive sequence
    fn validate_c2_mutual_exclusivity(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, transaction) in self.transactions.iter().enumerate() {
            let has_ordering_customer = transaction.ordering_customer.is_some();
            let has_ordering_institution = transaction.ordering_institution.is_some();

            if has_ordering_customer && has_ordering_institution {
                // Both present - NOT ALLOWED
                errors.push(SwiftValidationError::content_error(
                    "C06",
                    "50a/52a",
                    "",
                    &format!(
                        "Transaction {}: Either field 50a (Ordering Customer) or field 52a (Ordering Institution), but not both, must be present",
                        idx + 1
                    ),
                    "Field 50a and field 52a are mutually exclusive. Only one may be present in each repetitive sequence",
                ));
            } else if !has_ordering_customer && !has_ordering_institution {
                // Neither present - NOT ALLOWED
                errors.push(SwiftValidationError::content_error(
                    "C06",
                    "50a/52a",
                    "",
                    &format!(
                        "Transaction {}: Either field 50a (Ordering Customer) or field 52a (Ordering Institution) must be present",
                        idx + 1
                    ),
                    "At least one of field 50a or field 52a must be present in each repetitive sequence",
                ));
            }
        }

        errors
    }

    /// C3: Currency Code Consistency (Error code: C02)
    /// The currency code must be the same for all occurrences of field 32B
    fn validate_c3_currency_consistency(&self) -> Option<SwiftValidationError> {
        if self.transactions.is_empty() {
            return None;
        }

        // Get the currency from the first transaction
        let first_currency = &self.transactions[0].currency_amount.currency;

        // Check if all transactions have the same currency
        for (idx, transaction) in self.transactions.iter().enumerate().skip(1) {
            if &transaction.currency_amount.currency != first_currency {
                return Some(SwiftValidationError::content_error(
                    "C02",
                    "32B",
                    &transaction.currency_amount.currency,
                    &format!(
                        "Transaction {}: Currency code in field 32B ({}) must be the same as in the first transaction ({})",
                        idx + 1,
                        transaction.currency_amount.currency,
                        first_currency
                    ),
                    "The currency code must be the same for all occurrences of field 32B in the message",
                ));
            }
        }

        None
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Repetitive Sequence Count
        if let Some(error) = self.validate_c1_repetitive_sequence_count() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C2: Mutual Exclusivity of 50a and 52a
        let c2_errors = self.validate_c2_mutual_exclusivity();
        all_errors.extend(c2_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C3: Currency Consistency
        if let Some(error) = self.validate_c3_currency_consistency() {
            all_errors.push(error);
        }

        all_errors
    }
}

impl crate::traits::SwiftMessageBody for MT210 {
    fn message_type() -> &'static str {
        "210"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.transaction_reference);
        append_optional_field(&mut result, &self.account_identification);
        append_field(&mut result, &self.value_date);

        // Transactions
        for txn in &self.transactions {
            append_optional_field(&mut result, &txn.related_reference);
            append_field(&mut result, &txn.currency_amount);
            append_optional_field(&mut result, &txn.ordering_customer);
            append_optional_field(&mut result, &txn.ordering_institution);
            append_optional_field(&mut result, &txn.intermediary);
        }

        finalize_mt_string(result, false)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT210::validate_network_rules(self, stop_on_first_error)
    }
}
