use crate::errors::ParseError;
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};

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

    /// Static validation rules for MT204
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }
}

impl crate::traits::SwiftMessageBody for MT204 {
    fn message_type() -> &'static str {
        "204"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
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
}
