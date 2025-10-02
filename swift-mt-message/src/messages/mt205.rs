use crate::errors::{ParseError, ParseResult, ParserConfig};
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MT205 - Financial Institution Transfer Execution
///
/// Used to advise the execution of a transfer previously initiated by an MT200 or MT202.
/// Often used for cover payments and to provide additional details about a transfer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT205 {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub transaction_reference: Field20,

    /// Field 21 - Related Reference (Mandatory)
    #[serde(rename = "21")]
    pub related_reference: Field21NoOption,

    /// Field 13C - Time Indication (Optional, Repetitive)
    #[serde(rename = "13C", skip_serializing_if = "Option::is_none")]
    pub time_indication: Option<Vec<Field13C>>,

    /// Field 32A - Value Date, Currency Code, Amount (Mandatory)
    #[serde(rename = "32A")]
    pub value_date_amount: Field32A,

    /// Field 33B - Currency Code, Instructed Amount (Optional)
    #[serde(rename = "33B", skip_serializing_if = "Option::is_none")]
    pub instructed_amount: Option<Field33B>,

    /// Field 52 - Ordering Institution (Optional)
    /// Can be 52A or 52D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub ordering_institution: Option<Field52OrderingInstitution>,

    /// Field 53 - Sender's Correspondent (Optional)
    /// Can be 53A, 53B, or 53D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub senders_correspondent: Option<Field53>,

    /// Field 54 - Receiver's Correspondent (Optional)
    /// Can be 54A, 54B, or 54D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub receivers_correspondent: Option<Field54>,

    /// Field 56 - Intermediary Institution (Optional)
    /// Can be 56A or 56D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub intermediary: Option<Field56>,

    /// Field 57 - Account With Institution (Optional)
    /// Can be 57A, 57B, or 57D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub account_with_institution: Option<Field57>,

    /// Field 58 - Beneficiary Institution (Mandatory)
    /// Can be 58A or 58D
    #[serde(flatten)]
    pub beneficiary_institution: Field58,

    /// Field 72 - Sender to Receiver Information (Optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub sender_to_receiver: Option<Field72>,
}

impl MT205 {
    /// Parse MT205 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "205");

        // Parse mandatory fields
        let transaction_reference = parser.parse_field::<Field20>("20")?;
        let related_reference = parser.parse_field::<Field21NoOption>("21")?;

        // Parse optional Field 13C (can be repeated)
        let mut time_indications = Vec::new();
        while let Ok(field) = parser.parse_field::<Field13C>("13C") {
            time_indications.push(field);
        }
        let time_indication = if time_indications.is_empty() {
            None
        } else {
            Some(time_indications)
        };

        // Parse mandatory Field 32A
        let value_date_amount = parser.parse_field::<Field32A>("32A")?;

        // Parse optional Field 33B
        let instructed_amount = parser.parse_optional_field::<Field33B>("33B")?;

        // Parse optional fields
        let ordering_institution =
            parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let senders_correspondent = parser.parse_optional_variant_field::<Field53>("53")?;
        let receivers_correspondent = parser.parse_optional_variant_field::<Field54>("54")?;
        let intermediary = parser.parse_optional_variant_field::<Field56>("56")?;
        let account_with_institution = parser.parse_optional_variant_field::<Field57>("57")?;

        // Parse mandatory Field 58 - Beneficiary Institution
        let beneficiary_institution = parser.parse_variant_field::<Field58>("58")?;

        // Parse optional Field 72
        let sender_to_receiver = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT205 {
            transaction_reference,
            related_reference,
            time_indication,
            value_date_amount,
            instructed_amount,
            ordering_institution,
            senders_correspondent,
            receivers_correspondent,
            intermediary,
            account_with_institution,
            beneficiary_institution,
            sender_to_receiver,
        })
    }

    /// Static validation rules for MT205
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }

    /// Check if this message has reject codes
    pub fn has_reject_codes(&self) -> bool {
        if let Some(ref info) = self.sender_to_receiver {
            info.information
                .iter()
                .any(|line| line.contains("/REJT/") || line.contains("/RJT/"))
        } else {
            false
        }
    }

    /// Check if this message has return codes
    pub fn has_return_codes(&self) -> bool {
        if let Some(ref info) = self.sender_to_receiver {
            info.information
                .iter()
                .any(|line| line.contains("/RETN/") || line.contains("/RET/"))
        } else {
            false
        }
    }

    /// Check if this is a cover message
    pub fn is_cover_message(&self) -> bool {
        if let Some(ref info) = self.sender_to_receiver {
            info.information
                .iter()
                .any(|line| line.contains("/COV/") || line.contains("/COVER/"))
        } else {
            false
        }
    }
}

impl crate::traits::SwiftMessageBody for MT205 {
    fn message_type() -> &'static str {
        "205"
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
            "21".to_string(),
            vec![self.related_reference.to_swift_string()],
        );

        if let Some(ref times) = self.time_indication {
            let time_strings: Vec<String> = times.iter().map(|t| t.to_swift_string()).collect();
            fields.insert("13C".to_string(), time_strings);
        }

        fields.insert(
            "32A".to_string(),
            vec![self.value_date_amount.to_swift_string()],
        );

        if let Some(ref inst_amt) = self.instructed_amount {
            fields.insert("33B".to_string(), vec![inst_amt.to_swift_string()]);
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

        if let Some(ref corr) = self.senders_correspondent {
            match corr {
                Field53::A(f) => {
                    fields.insert("53A".to_string(), vec![f.to_swift_string()]);
                }
                Field53::B(f) => {
                    fields.insert("53B".to_string(), vec![f.to_swift_string()]);
                }
                Field53::D(f) => {
                    fields.insert("53D".to_string(), vec![f.to_swift_string()]);
                }
            }
        }

        if let Some(ref rec_corr) = self.receivers_correspondent {
            match rec_corr {
                Field54::A(f) => {
                    fields.insert("54A".to_string(), vec![f.to_swift_string()]);
                }
                Field54::B(f) => {
                    fields.insert("54B".to_string(), vec![f.to_swift_string()]);
                }
                Field54::D(f) => {
                    fields.insert("54D".to_string(), vec![f.to_swift_string()]);
                }
            }
        }

        if let Some(ref inter) = self.intermediary {
            match inter {
                Field56::A(f) => {
                    fields.insert("56A".to_string(), vec![f.to_swift_string()]);
                }
                Field56::C(f) => {
                    fields.insert("56C".to_string(), vec![f.to_swift_string()]);
                }
                Field56::D(f) => {
                    fields.insert("56D".to_string(), vec![f.to_swift_string()]);
                }
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
                Field57::C(f) => {
                    fields.insert("57C".to_string(), vec![f.to_swift_string()]);
                }
                Field57::D(f) => {
                    fields.insert("57D".to_string(), vec![f.to_swift_string()]);
                }
            }
        }

        match &self.beneficiary_institution {
            Field58::A(f) => {
                fields.insert("58A".to_string(), vec![f.to_swift_string()]);
            }
            Field58::D(f) => {
                fields.insert("58D".to_string(), vec![f.to_swift_string()]);
            }
        }

        if let Some(ref sender_info) = self.sender_to_receiver {
            fields.insert("72".to_string(), vec![sender_info.to_swift_string()]);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "21", "32A", "58"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["13C", "33B", "52", "53", "54", "56", "57", "72"]
    }
}