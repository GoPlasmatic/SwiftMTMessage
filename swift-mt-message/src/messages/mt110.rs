use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT110_VALIDATION_RULES)]
pub struct MT110 {
    #[field("20")]
    pub field_20: Field20,

    #[field("53A")]
    pub field_53a: Option<Field53SenderCorrespondent>,

    #[field("54A")]
    pub field_54a: Option<Field54ReceiverCorrespondent>,

    #[field("72")]
    pub field_72: Option<Field72>,

    #[field("#")]
    pub cheques: Vec<MT110Cheque>,
}

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT110_CHEQUE_VALIDATION_RULES)]
pub struct MT110Cheque {
    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("30")]
    pub field_30: Field30,

    #[field("32")]
    pub field_32a: Field32,

    #[field("50")]
    pub field_50a: Option<Field50OrderingCustomerAFK>,

    #[field("52")]
    pub field_52a: Option<Field52DrawerBank>,

    #[field("59")]
    pub field_59a: Field59,
}

/// Enhanced validation rules with forEach support for repetitive sequences
const MT110_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Maximum 10 cheques per message",
      "condition": {
        "<=": [{"length": {"var": "cheques"}}, 10]
      }
    },
    {
      "id": "C2",
      "description": "All cheques must have the same currency",
      "condition": {
        "if": [
          {">": [{"length": {"var": "cheques"}}, 0]},
          {
            "allEqual": {
              "map": ["cheques", "field_32a.currency"]
            }
          },
          true
        ]
      }
    },
    {
      "id": "CHQ_MIN",
      "description": "At least one cheque required",
      "condition": {
        ">=": [{"length": {"var": "cheques"}}, 1]
      }
    }
  ]
}"#;

/// Validation rules specific to MT110 cheques
const MT110_CHEQUE_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "CHQ_REF",
      "description": "Cheque reference must be unique and not contain '/' or '//'",
      "condition": {
        "and": [
          {"!=": [{"var": "field_21.value"}, ""]},
          {"!": [{"in": ["/", {"var": "field_21.value"}]}]},
          {"!": [{"in": ["//", {"var": "field_21.value"}]}]}
        ]
      }
    },
    {
      "id": "CHQ_DATE",
      "description": "Date of issue must be a valid date",
      "condition": {
        "!=": [{"var": "field_30.value"}, ""]
      }
    }
  ]
}"#;
