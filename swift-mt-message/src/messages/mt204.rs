use crate::errors::ParseError;
use crate::fields::*;
use crate::message_parser::MessageParser;
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
        use crate::traits::SwiftField;
        let mut result = String::new();

        result.push_str(&self.sum_of_amounts.to_swift_string());
        result.push_str("\r\n");

        result.push_str(&self.transaction_reference.to_swift_string());
        result.push_str("\r\n");

        result.push_str(&self.execution_date.to_swift_string());
        result.push_str("\r\n");

        if let Some(ref field) = self.account_with_institution {
            match field {
                Field57::A(f) => result.push_str(&f.to_swift_string()),
                Field57::B(f) => result.push_str(&f.to_swift_string()),
                Field57::C(f) => result.push_str(&f.to_swift_string()),
                Field57::D(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.sender_to_receiver {
            result.push_str(&field.to_swift_string());
            result.push_str("\r\n");
        }

        // Transactions (currently not parsed, but structure is in place)
        for txn in &self.transactions {
            result.push_str(&txn.transaction_reference.to_swift_string());
            result.push_str("\r\n");

            if let Some(ref field) = txn.related_reference {
                result.push_str(&field.to_swift_string());
                result.push_str("\r\n");
            }

            result.push_str(&txn.currency_amount.to_swift_string());
            result.push_str("\r\n");

            if let Some(ref field) = txn.senders_correspondent {
                match field {
                    Field53::A(f) => result.push_str(&f.to_swift_string()),
                    Field53::B(f) => result.push_str(&f.to_swift_string()),
                    Field53::D(f) => result.push_str(&f.to_swift_string()),
                }
                result.push_str("\r\n");
            }

            if let Some(ref field) = txn.sender_to_receiver {
                result.push_str(&field.to_swift_string());
                result.push_str("\r\n");
            }
        }

        // Remove trailing \r\n
        if result.ends_with("\r\n") {
            result.truncate(result.len() - 2);
        }

        result
    }
}
