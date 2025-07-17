use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT910_VALIDATION_RULES)]
pub struct MT910 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("25")]
    pub field_25: Field25AccountIdentification,

    #[field("13D")]
    pub field_13d: Option<Field13D>,

    #[field("32A")]
    pub field_32a: Field32A,

    #[field("50")]
    pub field_50a: Option<Field50OrderingCustomerAFK>,

    #[field("52")]
    pub field_52a: Option<Field52OrderingInstitution>,

    #[field("56")]
    pub field_56a: Option<Field56Intermediary>,

    #[field("72")]
    pub field_72: Option<Field72>,
}

/// Enhanced validation rules for MT910
const MT910_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Either Field 50a or Field 52a must be present (not both)",
      "condition": {
        "or": [
          {
            "and": [
              {"var": "field_50a.is_some"},
              {"var": "field_52a.is_none"}
            ]
          },
          {
            "and": [
              {"var": "field_50a.is_none"},
              {"var": "field_52a.is_some"}
            ]
          }
        ]
      }
    },
    {
      "id": "REF_FORMAT",
      "description": "Transaction and related references must not have invalid slash patterns",
      "condition": {
        "and": [
          {"!": {"startsWith": [{"var": "field_20.value"}, "/"]}},
          {"!": {"endsWith": [{"var": "field_20.value"}, "/"]}},
          {"!": {"includes": [{"var": "field_20.value"}, "//"]}},
          {"!": {"startsWith": [{"var": "field_21.value"}, "/"]}},
          {"!": {"endsWith": [{"var": "field_21.value"}, "/"]}},
          {"!": {"includes": [{"var": "field_21.value"}, "//"]}}
        ]
      }
    },
    {
      "id": "AMOUNT_POSITIVE",
      "description": "Credit amount must be positive",
      "condition": {
        ">": [{"var": "field_32a.amount"}, 0]
      }
    }
  ]
}"#;
