use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

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

    // Optional Fields - Standard MT202 Sequence A
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

    #[field("72", optional)]
    pub field_72: Option<GenericMultiLine6x35>,

    // COV Sequence B Fields - Customer Credit Transfer Details
    // These fields are present only in MT202 COV messages
    #[field("50A", optional)]
    pub field_50a: Option<Field50>,

    #[field("50", optional)]
    pub field_50: Option<Field50>,

    #[field("59A", optional)]
    pub field_59a: Option<Field59>,

    #[field("59", optional)]
    pub field_59: Option<Field59>,

    #[field("70", optional)]
    pub field_70: Option<GenericMultiLine4x35>,

    #[field("33B", optional)]
    pub field_33b: Option<GenericCurrencyAmountField>,
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
    /// - Presence of customer fields (50A/50 and 59A/59) indicating underlying customer details
    /// - Field 121 (UETR) in Block 3 is typically mandatory for COV messages
    pub fn is_cover_message(&self) -> bool {
        // COV messages contain customer fields that indicate underlying customer credit transfer details
        (self.field_50a.is_some() || self.field_50.is_some())
            && (self.field_59a.is_some() || self.field_59.is_some())
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
