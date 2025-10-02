use crate::errors::{ParseError, ParseResult, ParserConfig};
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MT290 - Advice of Charges, Interest and Other Adjustments
///
/// Used by financial institutions to advise charges, interest and other adjustments
/// that have been debited/credited to an account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT290 {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub transaction_reference: Field20,

    /// Field 21 - Related Reference (Mandatory)
    #[serde(rename = "21")]
    pub related_reference: Field21NoOption,

    /// Field 25 - Account Identification (Mandatory)
    #[serde(rename = "25")]
    pub account_identification: Field25AccountIdentification,

    /// Field 32 - Value Date, Currency Code, Amount (Mandatory)
    /// Can be 32C (credit) or 32D (debit)
    #[serde(flatten)]
    pub value_date_amount: Field32AmountCD,

    /// Field 52 - Ordering Institution (Optional)
    /// Can be 52A or 52D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub ordering_institution: Option<Field52OrderingInstitution>,

    /// Field 71B - Details of Charges (Mandatory)
    #[serde(rename = "71B")]
    pub details_of_charges: Field71B,

    /// Field 72 - Sender to Receiver Information (Optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub sender_to_receiver: Option<Field72>,
}

impl MT290 {
    /// Parse MT290 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "290");

        // Parse mandatory fields
        let transaction_reference = parser.parse_field::<Field20>("20")?;
        let related_reference = parser.parse_field::<Field21NoOption>("21")?;

        // Parse Field 25 - Account Identification
        let account_identification = parser.parse_variant_field::<Field25AccountIdentification>("25")?;

        // Parse Field 32 - check for variant (32C or 32D)
        let value_date_amount = if parser.detect_field("32C") {
            Field32AmountCD::C(parser.parse_field::<Field32C>("32C")?)
        } else if parser.detect_field("32D") {
            Field32AmountCD::D(parser.parse_field::<Field32D>("32D")?)
        } else {
            return Err(ParseError::InvalidFormat {
                message: "MT290: Either field 32C or 32D is required".to_string(),
            });
        };

        // Parse optional Field 52 - Ordering Institution
        let ordering_institution = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;

        // Parse mandatory Field 71B
        let details_of_charges = parser.parse_field::<Field71B>("71B")?;

        // Parse optional Field 72
        let sender_to_receiver = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT290 {
            transaction_reference,
            related_reference,
            account_identification,
            value_date_amount,
            ordering_institution,
            details_of_charges,
            sender_to_receiver,
        })
    }

    /// Static validation rules for MT290
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }
}

impl crate::traits::SwiftMessageBody for MT290 {
    fn message_type() -> &'static str {
        "290"
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
            Err(e) => Err(e)
        }
    }

    fn to_fields(&self) -> HashMap<String, Vec<String>> {
        let mut fields = HashMap::new();

        fields.insert("20".to_string(), vec![self.transaction_reference.to_swift_string()]);
        fields.insert("21".to_string(), vec![self.related_reference.to_swift_string()]);
        match &self.account_identification {
            Field25AccountIdentification::NoOption(f) => {
                fields.insert("25".to_string(), vec![f.to_swift_string()]);
            }
            Field25AccountIdentification::P(f) => {
                fields.insert("25P".to_string(), vec![f.to_swift_string()]);
            }
        }

        match &self.value_date_amount {
            Field32AmountCD::C(f) => {
                fields.insert("32C".to_string(), vec![f.to_swift_string()]);
            }
            Field32AmountCD::D(f) => {
                fields.insert("32D".to_string(), vec![f.to_swift_string()]);
            }
        }

        if let Some(ref ord_inst) = self.ordering_institution {
            match ord_inst {
                Field52OrderingInstitution::A(f) => {
                    fields.insert("52A".to_string(), vec![f.to_swift_string()]);
                }
                Field52OrderingInstitution::D(f) => {
                    fields.insert("52D".to_string(), vec![f.to_swift_string()]);
                }
            }
        }

        fields.insert("71B".to_string(), vec![self.details_of_charges.to_swift_string()]);

        if let Some(ref sender_info) = self.sender_to_receiver {
            fields.insert("72".to_string(), vec![sender_info.to_swift_string()]);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "21", "25", "32", "71B"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["52", "72"]
    }
}