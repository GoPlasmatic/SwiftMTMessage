use crate::errors::{ParseError, ParseResult, ParserConfig};
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    fn from_fields(fields: HashMap<String, Vec<(String, usize)>>) -> crate::SwiftResult<Self> {
        // Reconstruct block4 from fields
        let mut all_fields: Vec<(String, String, usize)> = Vec::new();
        for (tag, values) in fields {
            for (value, position) in values {
                all_fields.push((tag.clone(), value, position));
            }
        }

        // Sort by position
        all_fields.sort_by_key(|f| f.2);

        // Build block4
        let mut block4 = String::new();
        for (tag, value, _) in all_fields {
            block4.push_str(&format!(":{}:{}\n", tag, value));
        }

        Self::parse_from_block4(&block4)
    }

    fn from_fields_with_config(
        fields: HashMap<String, Vec<(String, usize)>>,
        _config: &ParserConfig,
    ) -> Result<ParseResult<Self>, ParseError> {
        match Self::from_fields(fields) {
            Ok(msg) => Ok(ParseResult::Success(msg)),
            Err(e) => Err(e),
        }
    }

    fn to_fields(&self) -> HashMap<String, Vec<String>> {
        let mut fields = HashMap::new();

        fields.insert(
            "20".to_string(),
            vec![self.transaction_reference.to_swift_string()],
        );
        fields.insert(
            "19".to_string(),
            vec![self.sum_of_amounts.to_swift_string()],
        );
        fields.insert(
            "30".to_string(),
            vec![self.execution_date.to_swift_string()],
        );

        if let Some(ref acc_with) = self.account_with_institution {
            match acc_with {
                Field57::A(f) => {
                    fields.insert("57A".to_string(), vec![f.to_swift_string()]);
                }
                Field57::B(f) => {
                    fields.insert("57B".to_string(), vec![f.to_swift_string()]);
                }
                Field57::C(f) => {
                    fields.insert("57C".to_string(), vec![f.to_swift_string()]);
                }
                Field57::D(f) => {
                    fields.insert("57D".to_string(), vec![f.to_swift_string()]);
                }
            }
        }

        if let Some(ref sender_info) = self.sender_to_receiver {
            fields.insert("72".to_string(), vec![sender_info.to_swift_string()]);
        }

        // Add transaction fields
        for transaction in &self.transactions {
            // Note: This is simplified - actual implementation would need to handle
            // transaction sequence numbers and proper field ordering
            fields
                .entry("20".to_string())
                .or_insert_with(Vec::new)
                .push(transaction.transaction_reference.to_swift_string());

            if let Some(ref related) = transaction.related_reference {
                fields
                    .entry("21".to_string())
                    .or_insert_with(Vec::new)
                    .push(related.to_swift_string());
            }

            fields
                .entry("32B".to_string())
                .or_insert_with(Vec::new)
                .push(transaction.currency_amount.to_swift_string());
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "19", "30"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["57", "72", "21", "32B", "53"]
    }
}