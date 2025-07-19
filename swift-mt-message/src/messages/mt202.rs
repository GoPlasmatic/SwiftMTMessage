use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT202_VALIDATION_RULES)]
pub struct MT202 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("13C")]
    pub field_13c: Option<Vec<Field13C>>,

    #[field("32A")]
    pub field_32a: Field32A,

    #[field("52")]
    pub field_52: Option<Field52OrderingInstitution>,

    #[field("53")]
    pub field_53: Option<Field53SenderCorrespondent>,

    #[field("54")]
    pub field_54: Option<Field54ReceiverCorrespondent>,

    #[field("56")]
    pub field_56: Option<Field56Intermediary>,

    #[field("57")]
    pub field_57: Option<Field57AccountWithInstitution>,

    #[field("58")]
    pub field_58: Field58,

    #[field("72")]
    pub field_72: Option<Field72>,

    #[field("50")]
    pub field_50_seq_b: Option<Field50OrderingCustomerAFK>,

    #[field("52")]
    pub field_52_seq_b: Option<Field52OrderingInstitution>,

    #[field("56")]
    pub field_56_seq_b: Option<Field56Intermediary>,

    #[field("57")]
    pub field_57_seq_b: Option<Field57AccountWithInstitution>,

    #[field("59")]
    pub field_59_seq_b: Option<Field59>,

    #[field("70")]
    pub field_70_seq_b: Option<Field70>,

    #[field("72")]
    pub field_72_seq_b: Option<Field72>,

    #[field("33B")]
    pub field_33b_seq_b: Option<Field33B>,
}

impl MT202 {
    /// Check if this MT202 message contains reject codes
    ///
    /// Reject messages are identified by checking:
    /// 1. Field 20 (Transaction Reference) for "REJT" prefix or content
    /// 2. Field 72 (Sender to Receiver Information) containing `/REJT/` codes
    /// 3. Additional structured reject information in field 72
    pub fn has_reject_codes(&self) -> bool {
        // Check field 20 (transaction reference)
        if self.field_20.reference.to_uppercase().contains("REJT") {
            return true;
        }

        // Check field 72 for structured reject codes
        if let Some(field_72) = &self.field_72 {
            let content = field_72.information.join(" ").to_uppercase();
            if content.contains("/REJT/") || content.contains("REJT") {
                return true;
            }
        }

        false
    }

    /// Check if this MT202 message contains return codes
    ///
    /// Return messages are identified by checking:
    /// 1. Field 20 (Transaction Reference) for "RETN" prefix or content
    /// 2. Field 72 (Sender to Receiver Information) containing `/RETN/` codes
    /// 3. Additional structured return information in field 72
    pub fn has_return_codes(&self) -> bool {
        // Check field 20 (transaction reference)
        if self.field_20.reference.to_uppercase().contains("RETN") {
            return true;
        }

        // Check field 72 for structured return codes
        if let Some(field_72) = &self.field_72 {
            let content = field_72.information.join(" ").to_uppercase();
            if content.contains("/RETN/") || content.contains("RETN") {
                return true;
            }
        }

        false
    }

    /// Check if this MT202 message is a Cover (COV) message
    ///
    /// COV messages are distinguished by:
    /// - Presence of customer fields (50A/50 and 59A/59) indicating underlying customer details
    /// - Field 121 (UETR) in Block 3 is typically mandatory for COV messages
    pub fn is_cover_message(&self) -> bool {
        // COV messages contain customer fields that indicate underlying customer credit transfer details
        self.field_50_seq_b.is_some() && (self.field_59_seq_b.is_some())
    }
}

const MT202_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "If 56a is present, 57a becomes mandatory",
      "condition": {
        "if": [
          {"or": [
            {"var": "field_56a.is_some"},
            {"var": "field_56d.is_some"}
          ]},
          {"or": [
            {"var": "field_57a.is_some"},
            {"var": "field_57b.is_some"},
            {"var": "field_57d.is_some"}
          ]},
          true
        ]
      }
    }
  ],
  "constants": {
    "VALID_TIME_CODES": ["CLS", "RNC", "SND"],
    "VALID_INSTRUCTION_CODES": ["/INT/", "/COV/", "/REIMBURSEMENT/", "/SETTLEMENT/", "/SDVA/", "/RETN/", "/REJT/"]
  }
}"#;
