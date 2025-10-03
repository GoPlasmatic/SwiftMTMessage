use crate::errors::ParseError;
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};

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

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.transaction_reference);
        append_field(&mut result, &self.related_reference);
        append_vec_field(&mut result, &self.time_indication);
        append_field(&mut result, &self.value_date_amount);
        append_optional_field(&mut result, &self.instructed_amount);
        append_optional_field(&mut result, &self.ordering_institution);
        append_optional_field(&mut result, &self.senders_correspondent);
        append_optional_field(&mut result, &self.receivers_correspondent);
        append_optional_field(&mut result, &self.intermediary);
        append_optional_field(&mut result, &self.account_with_institution);
        append_field(&mut result, &self.beneficiary_institution);
        append_optional_field(&mut result, &self.sender_to_receiver);

        finalize_mt_string(result, false)
    }
}
