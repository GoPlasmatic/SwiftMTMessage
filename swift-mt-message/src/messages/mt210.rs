use crate::errors::{ParseError, ParseResult, ParserConfig};
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

        if let Some(ref account) = self.account_identification {
            fields.insert("25".to_string(), vec![account.to_swift_string()]);
        }

        fields.insert("30".to_string(), vec![self.value_date.to_swift_string()]);

        // Add transaction fields
        for transaction in &self.transactions {
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

            if let Some(ref ord_cust) = transaction.ordering_customer {
                match ord_cust {
                    Field50::NoOption(f) => {
                        fields
                            .entry("50".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_string());
                    }
                    Field50::C(f) => {
                        fields
                            .entry("50C".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_string());
                    }
                    Field50::F(f) => {
                        fields
                            .entry("50F".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_string());
                    }
                }
            }

            if let Some(ref ord_inst) = transaction.ordering_institution {
                match ord_inst {
                    Field52OrderingInstitution::A(f) => {
                        fields
                            .entry("52A".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_string());
                    }
                    Field52OrderingInstitution::D(f) => {
                        fields
                            .entry("52D".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_string());
                    }
                }
            }

            if let Some(ref inter) = transaction.intermediary {
                match inter {
                    Field56::A(f) => {
                        fields
                            .entry("56A".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_string());
                    }
                    Field56::C(f) => {
                        fields
                            .entry("56C".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_string());
                    }
                    Field56::D(f) => {
                        fields
                            .entry("56D".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_string());
                    }
                }
            }
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "30"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["25", "21", "32B", "50", "52", "56"]
    }
}
