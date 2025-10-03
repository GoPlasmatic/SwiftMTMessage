use crate::errors::ParseError;
use crate::fields::*;
use crate::message_parser::MessageParser;
use serde::{Deserialize, Serialize};

/// MT202 - General Financial Institution Transfer
///
/// Used for bank-to-bank transfers on behalf of a customer or another financial institution.
/// Can be used for both direct transfers and cover payments (MT202 COV).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT202 {
    // Sequence A - Basic Transfer Details
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Field 21 - Related Reference (Mandatory)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Field 13C - Time Indication (Optional, Repetitive)
    #[serde(rename = "13C", skip_serializing_if = "Option::is_none")]
    pub field_13c: Option<Vec<Field13C>>,

    /// Field 32A - Value Date, Currency Code, Amount (Mandatory)
    #[serde(rename = "32A")]
    pub field_32a: Field32A,

    /// Field 52 - Ordering Institution (Optional)
    /// Can be 52A or 52D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    /// Field 53 - Sender's Correspondent (Optional)
    /// Can be 53A, 53B, or 53D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_53: Option<Field53SenderCorrespondent>,

    /// Field 54 - Receiver's Correspondent (Optional)
    /// Can be 54A, 54B, or 54D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_54: Option<Field54ReceiverCorrespondent>,

    /// Field 56 - Intermediary Institution (Optional)
    /// Can be 56A or 56D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_56: Option<Field56Intermediary>,

    /// Field 57 - Account With Institution (Optional)
    /// Can be 57A, 57B, or 57D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_57: Option<Field57AccountWithInstitution>,

    /// Field 58 - Beneficiary Institution (Mandatory)
    /// Can be 58A or 58D
    #[serde(flatten)]
    pub field_58: Field58,

    /// Field 72 - Sender to Receiver Information (Optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,

    // Sequence B - Cover Payment Details (MT202 COV)
    /// Field 50 - Ordering Customer (Optional, COV only)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub ordering_customer_b: Option<Field50OrderingCustomerAFK>,

    /// Field 52 - Ordering Institution (Optional, COV Sequence B)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub ordering_institution_b: Option<Field52OrderingInstitution>,

    /// Field 56 - Intermediary (Optional, COV Sequence B)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub intermediary_b: Option<Field56Intermediary>,

    /// Field 57 - Account With Institution (Optional, COV Sequence B)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub account_with_institution_b: Option<Field57AccountWithInstitution>,

    /// Field 59 - Beneficiary Customer (Optional, COV only)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub beneficiary_customer_b: Option<Field59>,

    /// Field 70 - Remittance Information (Optional, COV only)
    #[serde(rename = "70", skip_serializing_if = "Option::is_none")]
    pub remittance_information_b: Option<Field70>,

    /// Field 72 - Sender to Receiver Information (Optional, COV Sequence B)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_to_receiver_information_b: Option<Field72>,

    /// Field 33B - Currency/Instructed Amount (Optional, COV only)
    #[serde(rename = "33B", skip_serializing_if = "Option::is_none")]
    pub currency_amount_b: Option<Field33B>,
}

impl MT202 {
    /// Parse MT202 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "202");

        // Sequence A - Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;

        // Parse optional Field 13C (can be repeated)
        let mut time_indications = Vec::new();
        while let Ok(field) = parser.parse_field::<Field13C>("13C") {
            time_indications.push(field);
        }
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
        let ordering_customer_b =
            parser.parse_optional_variant_field::<Field50OrderingCustomerAFK>("50")?;
        let ordering_institution_b =
            parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let intermediary_b = parser.parse_optional_variant_field::<Field56Intermediary>("56")?;
        let account_with_institution_b =
            parser.parse_optional_variant_field::<Field57AccountWithInstitution>("57")?;
        let beneficiary_customer_b = parser.parse_optional_variant_field::<Field59>("59")?;
        let remittance_information_b = parser.parse_optional_field::<Field70>("70")?;
        let sender_to_receiver_information_b = parser.parse_optional_field::<Field72>("72")?;
        let currency_amount_b = parser.parse_optional_field::<Field33B>("33B")?;

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
            ordering_customer_b,
            ordering_institution_b,
            intermediary_b,
            account_with_institution_b,
            beneficiary_customer_b,
            remittance_information_b,
            sender_to_receiver_information_b,
            currency_amount_b,
        })
    }

    /// Static validation rules for MT202
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
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
        // Check if Sequence B COV fields are present
        self.ordering_customer_b.is_some() || self.beneficiary_customer_b.is_some()
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
        use crate::traits::SwiftField;
        let mut result = String::new();

        // Sequence A - Basic Transfer Details
        result.push_str(&self.field_20.to_swift_string());
        result.push_str("\r\n");

        result.push_str(&self.field_21.to_swift_string());
        result.push_str("\r\n");

        if let Some(ref field_13c_vec) = self.field_13c {
            for field in field_13c_vec {
                result.push_str(&field.to_swift_string());
                result.push_str("\r\n");
            }
        }

        result.push_str(&self.field_32a.to_swift_string());
        result.push_str("\r\n");

        if let Some(ref field) = self.field_52 {
            match field {
                Field52OrderingInstitution::A(f) => result.push_str(&f.to_swift_string()),
                Field52OrderingInstitution::D(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.field_53 {
            match field {
                Field53SenderCorrespondent::A(f) => result.push_str(&f.to_swift_string()),
                Field53SenderCorrespondent::B(f) => result.push_str(&f.to_swift_string()),
                Field53SenderCorrespondent::D(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.field_54 {
            match field {
                Field54ReceiverCorrespondent::A(f) => result.push_str(&f.to_swift_string()),
                Field54ReceiverCorrespondent::B(f) => result.push_str(&f.to_swift_string()),
                Field54ReceiverCorrespondent::D(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.field_56 {
            match field {
                Field56Intermediary::A(f) => result.push_str(&f.to_swift_string()),
                Field56Intermediary::C(f) => result.push_str(&f.to_swift_string()),
                Field56Intermediary::D(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.field_57 {
            match field {
                Field57AccountWithInstitution::A(f) => result.push_str(&f.to_swift_string()),
                Field57AccountWithInstitution::B(f) => result.push_str(&f.to_swift_string()),
                Field57AccountWithInstitution::C(f) => result.push_str(&f.to_swift_string()),
                Field57AccountWithInstitution::D(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        match &self.field_58 {
            Field58::A(f) => result.push_str(&f.to_swift_string()),
            Field58::D(f) => result.push_str(&f.to_swift_string()),
        }
        result.push_str("\r\n");

        if let Some(ref field) = self.field_72 {
            result.push_str(&field.to_swift_string());
            result.push_str("\r\n");
        }

        // Sequence B - Cover Payment Details (MT202 COV)
        if let Some(ref field) = self.ordering_customer_b {
            match field {
                Field50OrderingCustomerAFK::A(f) => result.push_str(&f.to_swift_string()),
                Field50OrderingCustomerAFK::F(f) => result.push_str(&f.to_swift_string()),
                Field50OrderingCustomerAFK::K(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.ordering_institution_b {
            match field {
                Field52OrderingInstitution::A(f) => result.push_str(&f.to_swift_string()),
                Field52OrderingInstitution::D(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.intermediary_b {
            match field {
                Field56Intermediary::A(f) => result.push_str(&f.to_swift_string()),
                Field56Intermediary::C(f) => result.push_str(&f.to_swift_string()),
                Field56Intermediary::D(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.account_with_institution_b {
            match field {
                Field57AccountWithInstitution::A(f) => result.push_str(&f.to_swift_string()),
                Field57AccountWithInstitution::B(f) => result.push_str(&f.to_swift_string()),
                Field57AccountWithInstitution::C(f) => result.push_str(&f.to_swift_string()),
                Field57AccountWithInstitution::D(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.beneficiary_customer_b {
            match field {
                Field59::NoOption(f) => result.push_str(&f.to_swift_string()),
                Field59::A(f) => result.push_str(&f.to_swift_string()),
                Field59::F(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.remittance_information_b {
            result.push_str(&field.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.sender_to_receiver_information_b {
            result.push_str(&field.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.currency_amount_b {
            result.push_str(&field.to_swift_string());
            result.push_str("\r\n");
        }

        // Remove trailing \r\n
        if result.ends_with("\r\n") {
            result.truncate(result.len() - 2);
        }

        result
    }
}
