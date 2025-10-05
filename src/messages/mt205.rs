use crate::errors::{ParseError, SwiftValidationError};
use crate::fields::*;
use crate::parser::MessageParser;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// **MT205: Financial Institution Transfer Execution**
///
/// Advises execution of transfer previously initiated by MT200 or MT202.
///
/// **Usage:** Cover payments, transfer execution advice, additional transfer details
/// **Category:** Category 2 (Financial Institution Transfers)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT205 {
    /// Transaction Reference Number (Field 20)
    #[serde(rename = "20")]
    pub transaction_reference: Field20,

    /// Related Reference (Field 21)
    #[serde(rename = "21")]
    pub related_reference: Field21NoOption,

    /// Time Indication (Field 13C)
    #[serde(rename = "13C", skip_serializing_if = "Option::is_none")]
    pub time_indication: Option<Vec<Field13C>>,

    /// Bank Operation Code (Field 23B)
    #[serde(rename = "23B", skip_serializing_if = "Option::is_none")]
    pub bank_operation_code: Option<Field23B>,

    /// Value Date, Currency Code, Amount (Field 32A)
    #[serde(rename = "32A")]
    pub value_date_amount: Field32A,

    /// Currency Code, Instructed Amount (Field 33B)
    #[serde(rename = "33B", skip_serializing_if = "Option::is_none")]
    pub instructed_amount: Option<Field33B>,

    /// Ordering Institution (Field 52)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub ordering_institution: Option<Field52OrderingInstitution>,

    /// Sender's Correspondent (Field 53)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub senders_correspondent: Option<Field53>,

    /// Receiver's Correspondent (Field 54)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub receivers_correspondent: Option<Field54>,

    /// Intermediary Institution (Field 56)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub intermediary: Option<Field56>,

    /// Account With Institution (Field 57)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub account_with_institution: Option<Field57>,

    /// Beneficiary Institution (Field 58)
    #[serde(flatten)]
    pub beneficiary_institution: Field58,

    /// Sender to Receiver Information (Field 72)
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

        // Parse optional Field 13C (can be repeated) - enable duplicates mode
        parser = parser.with_duplicates(true);
        let mut time_indications = Vec::new();
        while let Ok(field) = parser.parse_field::<Field13C>("13C") {
            time_indications.push(field);
        }
        parser = parser.with_duplicates(false);

        let time_indication = if time_indications.is_empty() {
            None
        } else {
            Some(time_indications)
        };

        // Parse optional Field 23B
        let bank_operation_code = parser.parse_optional_field::<Field23B>("23B")?;

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
            bank_operation_code,
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

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT205)
    // ========================================================================

    // ========================================================================
    // VALIDATION RULES (C1)
    // ========================================================================

    /// C1: Intermediary and Account With Institution Dependency (Error code: C81)
    /// If field 56a is present, then field 57a must also be present
    fn validate_c1_intermediary_account_with(&self) -> Option<SwiftValidationError> {
        if self.intermediary.is_some() && self.account_with_institution.is_none() {
            return Some(SwiftValidationError::content_error(
                "C81",
                "57a",
                "",
                "Field 57a (Account With Institution) is mandatory when field 56a (Intermediary) is present",
                "If field 56a is present, then field 57a must also be present",
            ));
        }

        None
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Intermediary and Account With Institution Dependency
        if let Some(error) = self.validate_c1_intermediary_account_with() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        all_errors
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
        append_optional_field(&mut result, &self.bank_operation_code);
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

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT205::validate_network_rules(self, stop_on_first_error)
    }
}
