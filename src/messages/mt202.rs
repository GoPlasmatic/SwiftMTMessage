use crate::errors::{ParseError, SwiftValidationError};
use crate::fields::*;
use crate::parser::MessageParser;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// Sequence B - Cover Payment Details (MT202 COV)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MT202SequenceB {
    /// Ordering Customer (Field 50)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub ordering_customer: Option<Field50OrderingCustomerAFK>,

    /// Ordering Institution (Field 52)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub ordering_institution: Option<Field52OrderingInstitution>,

    /// Intermediary (Field 56)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub intermediary: Option<Field56Intermediary>,

    /// Account With Institution (Field 57)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub account_with_institution: Option<Field57AccountWithInstitution>,

    /// Beneficiary Customer (Field 59)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub beneficiary_customer: Option<Field59>,

    /// Remittance Information (Field 70)
    #[serde(rename = "70", skip_serializing_if = "Option::is_none")]
    pub remittance_information: Option<Field70>,

    /// Sender to Receiver Information (Field 72)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub sender_to_receiver_information: Option<Field72>,

    /// Currency/Instructed Amount (Field 33B)
    #[serde(rename = "33B", skip_serializing_if = "Option::is_none")]
    pub currency_amount: Option<Field33B>,
}

/// **MT202: General Financial Institution Transfer**
///
/// Bank-to-bank transfer on behalf of customer or financial institution.
/// Supports both direct transfers and cover payments (MT202 COV).
///
/// **Usage:** Interbank transfers, cover payments, correspondent banking
/// **Category:** Category 2 (Financial Institution Transfers)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MT202 {
    /// Transaction Reference Number (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Related Reference (Field 21)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Time Indication (Field 13C)
    #[serde(rename = "13C", skip_serializing_if = "Option::is_none")]
    pub field_13c: Option<Vec<Field13C>>,

    /// Value Date, Currency Code, Amount (Field 32A)
    #[serde(rename = "32A")]
    pub field_32a: Field32A,

    /// Ordering Institution (Field 52)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    /// Sender's Correspondent (Field 53)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_53: Option<Field53SenderCorrespondent>,

    /// Receiver's Correspondent (Field 54)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_54: Option<Field54ReceiverCorrespondent>,

    /// Intermediary Institution (Field 56)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_56: Option<Field56Intermediary>,

    /// Account With Institution (Field 57)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_57: Option<Field57AccountWithInstitution>,

    /// Beneficiary Institution (Field 58)
    #[serde(flatten)]
    pub field_58: Field58,

    /// Sender to Receiver Information (Field 72)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,

    /// Sequence B - Cover Payment Details
    #[serde(rename = "#", skip_serializing_if = "Option::is_none")]
    pub sequence_b: Option<MT202SequenceB>,
}

impl MT202 {
    /// Parse MT202 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "202");

        // Sequence A - Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;

        // Parse optional Field 13C (can be repeated) - enable duplicates mode
        parser = parser.with_duplicates(true);
        let mut time_indications = Vec::new();
        while let Ok(field) = parser.parse_field::<Field13C>("13C") {
            time_indications.push(field);
        }
        parser = parser.with_duplicates(false);

        let field_13c = if time_indications.is_empty() {
            None
        } else {
            Some(time_indications)
        };

        // Parse mandatory Field 32A
        let field_32a = parser.parse_field::<Field32A>("32A")?;

        // Parse optional Sequence A fields
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_53 = parser.parse_optional_variant_field::<Field53SenderCorrespondent>("53")?;
        let field_54 = parser.parse_optional_variant_field::<Field54ReceiverCorrespondent>("54")?;
        let field_56 = parser.parse_optional_variant_field::<Field56Intermediary>("56")?;
        let field_57 =
            parser.parse_optional_variant_field::<Field57AccountWithInstitution>("57")?;

        // Parse mandatory Field 58 - Beneficiary Institution
        let field_58 = parser.parse_variant_field::<Field58>("58")?;

        // Parse optional Field 72
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        // Sequence B - Parse COV fields (optional, for MT202 COV variant)
        // Enable duplicates for Sequence B as it may have fields 52, 56, 57, 72 again
        parser = parser.with_duplicates(true);

        let ordering_customer =
            parser.parse_optional_variant_field::<Field50OrderingCustomerAFK>("50")?;
        let ordering_institution =
            parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let intermediary = parser.parse_optional_variant_field::<Field56Intermediary>("56")?;
        let account_with_institution =
            parser.parse_optional_variant_field::<Field57AccountWithInstitution>("57")?;
        let beneficiary_customer = parser.parse_optional_variant_field::<Field59>("59")?;
        let remittance_information = parser.parse_optional_field::<Field70>("70")?;
        let sender_to_receiver_information = parser.parse_optional_field::<Field72>("72")?;
        let currency_amount = parser.parse_optional_field::<Field33B>("33B")?;

        // Build Sequence B only if any COV fields are present
        let sequence_b = if ordering_customer.is_some()
            || ordering_institution.is_some()
            || intermediary.is_some()
            || account_with_institution.is_some()
            || beneficiary_customer.is_some()
            || remittance_information.is_some()
            || sender_to_receiver_information.is_some()
            || currency_amount.is_some()
        {
            Some(MT202SequenceB {
                ordering_customer,
                ordering_institution,
                intermediary,
                account_with_institution,
                beneficiary_customer,
                remittance_information,
                sender_to_receiver_information,
                currency_amount,
            })
        } else {
            None
        };

        Ok(MT202 {
            field_20,
            field_21,
            field_13c,
            field_32a,
            field_52,
            field_53,
            field_54,
            field_56,
            field_57,
            field_58,
            field_72,
            sequence_b,
        })
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT202)
    // ========================================================================

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Check if intermediary (56a) is present in Sequence A
    fn has_intermediary_in_seq_a(&self) -> bool {
        self.field_56.is_some()
    }

    /// Check if account with institution (57a) is present in Sequence A
    fn has_account_with_in_seq_a(&self) -> bool {
        self.field_57.is_some()
    }

    /// Check if intermediary (56a) is present in Sequence B (COV)
    fn has_intermediary_in_seq_b(&self) -> bool {
        self.sequence_b
            .as_ref()
            .map(|seq_b| seq_b.intermediary.is_some())
            .unwrap_or(false)
    }

    /// Check if account with institution (57a) is present in Sequence B (COV)
    fn has_account_with_in_seq_b(&self) -> bool {
        self.sequence_b
            .as_ref()
            .map(|seq_b| seq_b.account_with_institution.is_some())
            .unwrap_or(false)
    }

    // ========================================================================
    // VALIDATION RULES (C1-C2)
    // ========================================================================

    /// C1: Intermediary and Account With Institution (Sequence A) (Error code: C81)
    /// If field 56a is present in sequence A, then field 57a must also be present in sequence A
    fn validate_c1_intermediary_seq_a(&self) -> Option<SwiftValidationError> {
        if self.has_intermediary_in_seq_a() && !self.has_account_with_in_seq_a() {
            return Some(SwiftValidationError::business_error(
                "C81",
                "57a",
                vec!["56a".to_string()],
                "Field 57a (Account With Institution) is mandatory when field 56a (Intermediary) is present in Sequence A",
                "If field 56a is present in sequence A, then field 57a must also be present in sequence A",
            ));
        }
        None
    }

    /// C2: Intermediary and Account With Institution (Sequence B) (Error code: C68)
    /// If field 56a is present in sequence B, then field 57a must also be present in sequence B
    fn validate_c2_intermediary_seq_b(&self) -> Option<SwiftValidationError> {
        if self.has_intermediary_in_seq_b() && !self.has_account_with_in_seq_b() {
            return Some(SwiftValidationError::business_error(
                "C68",
                "57a",
                vec!["56a".to_string()],
                "Field 57a (Account With Institution) is mandatory when field 56a (Intermediary) is present in Sequence B",
                "If field 56a is present in sequence B, then field 57a must also be present in sequence B",
            ));
        }
        None
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Intermediary and Account With Institution (Sequence A)
        if let Some(error) = self.validate_c1_intermediary_seq_a() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C2: Intermediary and Account With Institution (Sequence B)
        if let Some(error) = self.validate_c2_intermediary_seq_b() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        all_errors
    }

    /// Check if this message has reject codes
    pub fn has_reject_codes(&self) -> bool {
        if let Some(ref info) = self.field_72 {
            info.information
                .iter()
                .any(|line| line.contains("/REJT/") || line.contains("/RJT/"))
        } else {
            false
        }
    }

    /// Check if this message has return codes
    pub fn has_return_codes(&self) -> bool {
        if let Some(ref info) = self.field_72 {
            info.information
                .iter()
                .any(|line| line.contains("/RETN/") || line.contains("/RET/"))
        } else {
            false
        }
    }

    /// Check if this is a cover message (MT202 COV)
    pub fn is_cover_message(&self) -> bool {
        // Check if Sequence B is present with COV fields
        self.sequence_b
            .as_ref()
            .map(|seq_b| seq_b.ordering_customer.is_some() || seq_b.beneficiary_customer.is_some())
            .unwrap_or(false)
    }
}

impl crate::traits::SwiftMessageBody for MT202 {
    fn message_type() -> &'static str {
        "202"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        // Sequence A - Basic Transfer Details
        append_field(&mut result, &self.field_20);
        append_field(&mut result, &self.field_21);
        append_vec_field(&mut result, &self.field_13c);
        append_field(&mut result, &self.field_32a);
        append_optional_field(&mut result, &self.field_52);
        append_optional_field(&mut result, &self.field_53);
        append_optional_field(&mut result, &self.field_54);
        append_optional_field(&mut result, &self.field_56);
        append_optional_field(&mut result, &self.field_57);
        append_field(&mut result, &self.field_58);
        append_optional_field(&mut result, &self.field_72);

        // Sequence B - Cover Payment Details (MT202 COV)
        if let Some(ref seq_b) = self.sequence_b {
            append_optional_field(&mut result, &seq_b.ordering_customer);
            append_optional_field(&mut result, &seq_b.ordering_institution);
            append_optional_field(&mut result, &seq_b.intermediary);
            append_optional_field(&mut result, &seq_b.account_with_institution);
            append_optional_field(&mut result, &seq_b.beneficiary_customer);
            append_optional_field(&mut result, &seq_b.remittance_information);
            append_optional_field(&mut result, &seq_b.sender_to_receiver_information);
            append_optional_field(&mut result, &seq_b.currency_amount);
        }

        finalize_mt_string(result, false)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT202::validate_network_rules(self, stop_on_first_error)
    }
}
