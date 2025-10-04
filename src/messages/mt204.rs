use crate::errors::ParseError;
use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::MessageParser;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// MT204 - Financial Markets Direct Debit Message
///
/// Used for direct debit transactions in financial markets,
/// typically for clearing and settlement of multiple transactions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT204 {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub transaction_reference: Field20,

    /// Field 19 - Sum of Amounts (Mandatory)
    #[serde(rename = "19")]
    pub sum_of_amounts: Field19,

    /// Field 30 - Execution Date (Mandatory)
    #[serde(rename = "30")]
    pub execution_date: Field30,

    /// Field 57 - Account With Institution (Optional)
    /// Can be 57A, 57B, or 57D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub account_with_institution: Option<Field57>,

    /// Field 72 - Sender to Receiver Information (Optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub sender_to_receiver: Option<Field72>,

    /// Transactions (Repeatable)
    #[serde(rename = "#", default)]
    pub transactions: Vec<MT204Transaction>,
}

/// Individual transaction within an MT204 message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT204Transaction {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub transaction_reference: Field20,

    /// Field 21 - Related Reference (Optional)
    #[serde(rename = "21", skip_serializing_if = "Option::is_none")]
    pub related_reference: Option<Field21NoOption>,

    /// Field 32B - Currency Code, Amount (Mandatory)
    #[serde(rename = "32B")]
    pub currency_amount: Field32B,

    /// Field 53 - Sender's Correspondent (Optional)
    /// Can be 53A or 53B
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub senders_correspondent: Option<Field53>,

    /// Field 72 - Sender to Receiver Information (Optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub sender_to_receiver: Option<Field72>,
}

impl MT204 {
    /// Parse MT204 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "204");

        // Parse header fields in the correct order
        let sum_of_amounts = parser.parse_field::<Field19>("19")?;
        let transaction_reference = parser.parse_field::<Field20>("20")?;
        let execution_date = parser.parse_field::<Field30>("30")?;

        // Parse optional Field 57 - Account With Institution
        let account_with_institution = parser.parse_optional_variant_field::<Field57>("57")?;

        // Parse optional Field 72 at message level
        let sender_to_receiver = parser.parse_optional_field::<Field72>("72")?;

        // Parse transactions
        // For now, we'll create an empty vector as transaction parsing requires special handling
        let transactions = Vec::new();

        Ok(MT204 {
            transaction_reference,
            sum_of_amounts,
            execution_date,
            account_with_institution,
            sender_to_receiver,
            transactions,
        })
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT204)
    // ========================================================================

    /// Maximum number of repetitive sequences allowed
    const MAX_SEQUENCE_B_OCCURRENCES: usize = 10;

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Get the sum of all transaction amounts in Sequence B
    fn calculate_sum_of_transactions(&self) -> f64 {
        self.transactions
            .iter()
            .map(|tx| tx.currency_amount.amount)
            .sum()
    }

    /// Get all unique currency codes from Sequence B transactions
    fn get_transaction_currencies(&self) -> HashSet<String> {
        self.transactions
            .iter()
            .map(|tx| tx.currency_amount.currency.clone())
            .collect()
    }

    // ========================================================================
    // VALIDATION RULES (C1-C3, T10)
    // ========================================================================

    /// C1: Sum of Amounts Must Equal Total of Transaction Amounts (Error code: C01)
    /// The amount in field 19 must equal the sum of amounts in all occurrences of field 32B
    fn validate_c1_sum_of_amounts(&self) -> Option<SwiftValidationError> {
        if self.transactions.is_empty() {
            return None; // No transactions to validate
        }

        let sum_of_transactions = self.calculate_sum_of_transactions();
        let field_19_amount = self.sum_of_amounts.amount;

        // Use a small epsilon for floating-point comparison (0.01 = 1 cent)
        let difference = (field_19_amount - sum_of_transactions).abs();

        if difference > 0.01 {
            return Some(SwiftValidationError::content_error(
                "C01",
                "19",
                &field_19_amount.to_string(),
                &format!(
                    "Sum of amounts in field 19 ({:.2}) must equal the sum of all field 32B amounts ({:.2}). Difference: {:.2}",
                    field_19_amount, sum_of_transactions, difference
                ),
                "The amount in field 19 must equal the sum of the amounts in all occurrences of field 32B",
            ));
        }

        None
    }

    /// C2: Currency Code Consistency Across All Transactions (Error code: C02)
    /// The currency code in field 32B must be the same for all occurrences
    fn validate_c2_currency_consistency(&self) -> Option<SwiftValidationError> {
        if self.transactions.is_empty() {
            return None;
        }

        let currencies = self.get_transaction_currencies();

        if currencies.len() > 1 {
            let currency_list: Vec<String> = currencies.into_iter().collect();
            return Some(SwiftValidationError::content_error(
                "C02",
                "32B",
                &currency_list.join(", "),
                &format!(
                    "All occurrences of field 32B must have the same currency code. Found currencies: {}",
                    currency_list.join(", ")
                ),
                "The currency code in the amount field 32B must be the same for all occurrences of this field in the message",
            ));
        }

        None
    }

    /// C3/T10: Maximum Number of Repetitive Sequences (Error code: T10)
    /// Sequence B must not appear more than ten times
    fn validate_c3_max_sequences(&self) -> Option<SwiftValidationError> {
        let count = self.transactions.len();

        if count > Self::MAX_SEQUENCE_B_OCCURRENCES {
            return Some(SwiftValidationError::content_error(
                "T10",
                "Sequence B",
                &count.to_string(),
                &format!(
                    "The repetitive sequence B appears {} times, which exceeds the maximum of {} occurrences",
                    count,
                    Self::MAX_SEQUENCE_B_OCCURRENCES
                ),
                "The repetitive sequence must not appear more than ten times",
            ));
        }

        None
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Sum of Amounts
        if let Some(error) = self.validate_c1_sum_of_amounts() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C2: Currency Consistency
        if let Some(error) = self.validate_c2_currency_consistency() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C3/T10: Maximum Sequences
        if let Some(error) = self.validate_c3_max_sequences() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        all_errors
    }
}

impl crate::traits::SwiftMessageBody for MT204 {
    fn message_type() -> &'static str {
        "204"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT204::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        let mut result = String::new();

        append_field(&mut result, &self.sum_of_amounts);
        append_field(&mut result, &self.transaction_reference);
        append_field(&mut result, &self.execution_date);
        append_optional_field(&mut result, &self.account_with_institution);
        append_optional_field(&mut result, &self.sender_to_receiver);

        // Transactions
        for txn in &self.transactions {
            append_field(&mut result, &txn.transaction_reference);
            append_optional_field(&mut result, &txn.related_reference);
            append_field(&mut result, &txn.currency_amount);
            append_optional_field(&mut result, &txn.senders_correspondent);
            append_optional_field(&mut result, &txn.sender_to_receiver);
        }

        finalize_mt_string(result, false)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT204::validate_network_rules(self, stop_on_first_error)
    }
}
