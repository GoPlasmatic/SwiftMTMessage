use crate::errors::ParseError;
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};

/// MT210 - Notice to Receive
///
/// Used to advise the receiver that funds will be coming and should be credited
/// to the account specified. Typically precedes the actual transfer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT210 {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub transaction_reference: Field20,

    /// Field 25 - Account Identification (Optional)
    #[serde(rename = "25", skip_serializing_if = "Option::is_none")]
    pub account_identification: Option<Field25NoOption>,

    /// Field 30 - Value Date (Mandatory)
    #[serde(rename = "30")]
    pub value_date: Field30,

    /// Transactions (Repeatable)
    #[serde(rename = "#", default)]
    pub transactions: Vec<MT210Transaction>,
}

/// Individual transaction within an MT210 message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT210Transaction {
    /// Field 21 - Related Reference (Optional)
    #[serde(rename = "21", skip_serializing_if = "Option::is_none")]
    pub related_reference: Option<Field21NoOption>,

    /// Field 32B - Currency Code, Amount (Mandatory)
    #[serde(rename = "32B")]
    pub currency_amount: Field32B,

    /// Field 50 - Ordering Customer (Optional)
    /// Can be 50, 50C, 50F, or 50K
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub ordering_customer: Option<Field50>,

    /// Field 52 - Ordering Institution (Optional)
    /// Can be 52A or 52D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub ordering_institution: Option<Field52OrderingInstitution>,

    /// Field 56 - Intermediary Institution (Optional)
    /// Can be 56A, 56C, or 56D
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

        // Parse transactions
        // For now, we'll create an empty vector as transaction parsing requires special handling
        let transactions = Vec::new();

        Ok(MT210 {
            transaction_reference,
            account_identification,
            value_date,
            transactions,
        })
    }

    /// Static validation rules for MT210
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
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
}
