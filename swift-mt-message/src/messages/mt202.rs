use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT202 Sequence B: Underlying Customer Credit Transfer Details (COV variant)
///
/// This sequence contains the underlying customer credit transfer details
/// and is present only in MT202 COV messages.
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT202_SEQUENCE_B_VALIDATION_RULES)]
pub struct MT202SequenceB {
    // Mandatory Fields for COV Sequence B
    #[field("50A", mandatory)]
    pub field_50a: Field50,

    #[field("59A", mandatory)]
    pub field_59a: Field59,

    // Optional Fields for COV Sequence B
    #[field("52A", optional)]
    pub field_52a: Option<GenericBicField>,

    #[field("52D", optional)]
    pub field_52d: Option<GenericNameAddressField>,

    #[field("56A", optional)]
    pub field_56a: Option<GenericBicField>,

    #[field("56C", optional)]
    pub field_56c: Option<GenericAccountField>,

    #[field("56D", optional)]
    pub field_56d: Option<GenericNameAddressField>,

    #[field("57A", optional)]
    pub field_57a: Option<GenericBicField>,

    #[field("57B", optional)]
    pub field_57b: Option<GenericPartyField>,

    #[field("57C", optional)]
    pub field_57c: Option<GenericAccountField>,

    #[field("57D", optional)]
    pub field_57d: Option<GenericNameAddressField>,

    #[field("70", optional)]
    pub field_70: Option<GenericMultiLine4x35>,

    #[field("72", optional)]
    pub field_72: Option<GenericMultiLine6x35>,

    #[field("33B", optional)]
    pub field_33b: Option<GenericCurrencyAmountField>,
}

const MT202_SEQUENCE_B_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "If 56a is present, 57a becomes mandatory",
      "condition": true
    }
  ]
}"#;

/// # MT202: General Financial Institution Transfer (Standard and COV variants)
///
/// Unified structure supporting both standard MT202 and MT202 COV variants.
/// Use `is_cover_message()` to check if the message is a COV variant.
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT202_VALIDATION_RULES)]
pub struct MT202 {
    // Mandatory Fields
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    #[field("21", mandatory)]
    pub field_21: GenericReferenceField,

    #[field("32A", mandatory)]
    pub field_32a: Field32A,

    #[field("58A", mandatory)]
    pub field_58a: GenericBicField,

    // Optional Fields
    #[field("13C", optional)]
    pub field_13c: Option<Vec<Field13C>>,

    #[field("52A", optional)]
    pub field_52a: Option<GenericBicField>,

    #[field("52D", optional)]
    pub field_52d: Option<GenericNameAddressField>,

    #[field("53A", optional)]
    pub field_53a: Option<GenericBicField>,

    #[field("53B", optional)]
    pub field_53b: Option<GenericPartyField>,

    #[field("53D", optional)]
    pub field_53d: Option<GenericNameAddressField>,

    #[field("54A", optional)]
    pub field_54a: Option<GenericBicField>,

    #[field("54B", optional)]
    pub field_54b: Option<GenericPartyField>,

    #[field("54D", optional)]
    pub field_54d: Option<GenericNameAddressField>,

    #[field("56A", optional)]
    pub field_56a: Option<GenericBicField>,

    #[field("56D", optional)]
    pub field_56d: Option<GenericNameAddressField>,

    #[field("57A", optional)]
    pub field_57a: Option<GenericBicField>,

    #[field("57B", optional)]
    pub field_57b: Option<GenericPartyField>,

    #[field("57D", optional)]
    pub field_57d: Option<GenericNameAddressField>,

    #[field("72", optional)]
    pub field_72: Option<GenericMultiLine6x35>,

    // Sequence B: Underlying Customer Credit Transfer Details (COV variant)
    // This entire section is optional and present only in MT202 COV messages
    #[field("SEQUENCE_B", optional)]
    pub sequence_b: Option<MT202SequenceB>,
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
        if self.field_20.value.to_uppercase().contains("REJT") {
            return true;
        }

        // Check field 72 for structured reject codes
        if let Some(field_72) = &self.field_72 {
            let content = field_72.lines.join(" ").to_uppercase();
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
        if self.field_20.value.to_uppercase().contains("RETN") {
            return true;
        }

        // Check field 72 for structured return codes
        if let Some(field_72) = &self.field_72 {
            let content = field_72.lines.join(" ").to_uppercase();
            if content.contains("/RETN/") || content.contains("RETN") {
                return true;
            }
        }

        false
    }

    /// Check if this MT202 message is a Cover (COV) message
    ///
    /// COV messages are distinguished by:
    /// - Presence of Sequence B section for underlying customer credit transfer details
    /// - Field 121 (UETR) in Block 3 is typically mandatory for COV messages
    pub fn is_cover_message(&self) -> bool {
        // The key distinguishing feature of COV is the presence of Sequence B
        self.sequence_b.is_some()
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
