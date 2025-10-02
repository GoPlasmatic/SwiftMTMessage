use crate::errors::{ParseError, ParseResult, ParserConfig};
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MT200 - Financial Institution Transfer for Own Account
///
/// Used by financial institutions to transfer funds for their own account,
/// typically for nostro account funding, liquidity management, or internal transfers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT200 {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub transaction_reference: Field20,

    /// Field 32A - Value Date, Currency Code, Amount (Mandatory)
    #[serde(rename = "32A")]
    pub value_date_amount: Field32A,

    /// Field 53 - Sender's Correspondent (Optional)
    /// Can be 53A or 53B
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub senders_correspondent: Option<Field53>,

    /// Field 56 - Intermediary Institution (Optional)
    /// Can be 56A or 56D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub intermediary: Option<Field56>,

    /// Field 57 - Account With Institution (Optional)
    /// Can be 57A, 57B, or 57D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub account_with_institution: Option<Field57>,

    /// Field 72 - Sender to Receiver Information (Optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub sender_to_receiver: Option<Field72>,
}

impl MT200 {
    /// Parse MT200 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "200");

        // Parse mandatory fields
        let transaction_reference = parser.parse_field::<Field20>("20")?;
        let value_date_amount = parser.parse_field::<Field32A>("32A")?;

        // Parse optional Field 53 - Sender's Correspondent
        let senders_correspondent = parser.parse_optional_variant_field::<Field53>("53")?;

        // Parse optional Field 56 - Intermediary Institution
        let intermediary = parser.parse_optional_variant_field::<Field56>("56")?;

        // Parse optional Field 57 - Account With Institution
        let account_with_institution = parser.parse_optional_variant_field::<Field57>("57")?;

        // Parse optional Field 72
        let sender_to_receiver = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT200 {
            transaction_reference,
            value_date_amount,
            senders_correspondent,
            intermediary,
            account_with_institution,
            sender_to_receiver,
        })
    }

    /// Static validation rules for MT200
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }
}

impl crate::traits::SwiftMessageBody for MT200 {
    fn message_type() -> &'static str {
        "200"
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
            "32A".to_string(),
            vec![self.value_date_amount.to_swift_string()],
        );

        if let Some(ref corr) = self.senders_correspondent {
            match corr {
                Field53::A(f) => {
                    fields.insert("53A".to_string(), vec![f.to_swift_string()]);
                }
                Field53::B(f) => {
                    fields.insert("53B".to_string(), vec![f.to_swift_string()]);
                }
                _ => {}
            }
        }

        if let Some(ref inter) = self.intermediary {
            match inter {
                Field56::A(f) => {
                    fields.insert("56A".to_string(), vec![f.to_swift_string()]);
                }
                Field56::D(f) => {
                    fields.insert("56D".to_string(), vec![f.to_swift_string()]);
                }
                _ => {}
            }
        }

        if let Some(ref acc_with) = self.account_with_institution {
            match acc_with {
                Field57::A(f) => {
                    fields.insert("57A".to_string(), vec![f.to_swift_string()]);
                }
                Field57::B(f) => {
                    fields.insert("57B".to_string(), vec![f.to_swift_string()]);
                }
                Field57::D(f) => {
                    fields.insert("57D".to_string(), vec![f.to_swift_string()]);
                }
                _ => {}
            }
        }

        if let Some(ref sender_info) = self.sender_to_receiver {
            fields.insert("72".to_string(), vec![sender_info.to_swift_string()]);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "32A"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["53", "56", "57", "72"]
    }
}