use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT202 COV: General Financial Institution Transfer (Cover)
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT202_COV_VALIDATION_RULES)]
pub struct MT202COV {
    // Sequence A: General Information (Cover Payment Routing)
    // Mandatory Fields
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    #[field("21", mandatory)]
    pub field_21: GenericReferenceField,

    #[field("32A", mandatory)]
    pub field_32a: Field32A,

    #[field("58A", mandatory)]
    pub field_58a: GenericBicField,

    // Optional Fields - Sequence A
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

    // Sequence B: Underlying Customer Credit Transfer Details
    // Mandatory Fields
    #[field("50A", mandatory)]
    pub field_50a_seq_b: Field50,

    #[field("59A", mandatory)]
    pub field_59a_seq_b: Field59,

    // Optional Fields - Sequence B
    #[field("52A_SEQ_B", optional)]
    pub field_52a_seq_b: Option<GenericBicField>,

    #[field("52D_SEQ_B", optional)]
    pub field_52d_seq_b: Option<GenericNameAddressField>,

    #[field("56A_SEQ_B", optional)]
    pub field_56a_seq_b: Option<GenericBicField>,

    #[field("56C_SEQ_B", optional)]
    pub field_56c_seq_b: Option<GenericAccountField>,

    #[field("56D_SEQ_B", optional)]
    pub field_56d_seq_b: Option<GenericNameAddressField>,

    #[field("57A_SEQ_B", optional)]
    pub field_57a_seq_b: Option<GenericBicField>,

    #[field("57B_SEQ_B", optional)]
    pub field_57b_seq_b: Option<GenericPartyField>,

    #[field("57C_SEQ_B", optional)]
    pub field_57c_seq_b: Option<GenericAccountField>,

    #[field("57D_SEQ_B", optional)]
    pub field_57d_seq_b: Option<GenericNameAddressField>,

    #[field("70", optional)]
    pub field_70: Option<GenericMultiLine4x35>,

    #[field("72_SEQ_B", optional)]
    pub field_72_seq_b: Option<GenericMultiLine6x35>,

    #[field("33B", optional)]
    pub field_33b: Option<GenericCurrencyAmountField>,
}

/// MT202 COV validation rules
const MT202_COV_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Sequence A: If 56a is present, 57a becomes mandatory",
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
    },
    {
      "id": "C2",
      "description": "Sequence B: If 56a is present, 57a becomes mandatory",
      "condition": {
        "if": [
          {"or": [
            {"var": "field_56a_seq_b.is_some"},
            {"var": "field_56c_seq_b.is_some"},
            {"var": "field_56d_seq_b.is_some"}
          ]},
          {"or": [
            {"var": "field_57a_seq_b.is_some"},
            {"var": "field_57b_seq_b.is_some"},
            {"var": "field_57c_seq_b.is_some"},
            {"var": "field_57d_seq_b.is_some"}
          ]},
          true
        ]
      }
    }
  ],
  "constants": {
    "VALID_TIME_CODES": ["CLS", "RNC", "SND"],
    "VALID_INSTRUCTION_CODES": ["/ACC/", "/INS/", "/INT/", "/COV/", "/REIMBURSEMENT/", "/SETTLEMENT/", "/SDVA/", "/RETN/", "/REJT/"],
    "VALID_CUSTOMER_OPTIONS": ["A", "F", "K"],
    "VALID_INSTITUTION_OPTIONS": ["A", "B", "C", "D"]
  }
}"#;
