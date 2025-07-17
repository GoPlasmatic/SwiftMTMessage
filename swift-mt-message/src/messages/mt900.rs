use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT900_VALIDATION_RULES)]
pub struct MT900 {
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

    #[field("52")]
    pub field_52: Option<Field52OrderingInstitution>,

    #[field("72")]
    pub field_72: Option<Field72>,
}

/// Enhanced validation rules for MT900
const MT900_VALIDATION_RULES: &str = r#"{
  "rules": [
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
      "description": "Debit amount must be positive",
      "condition": {
        ">": [{"var": "field_32a.amount"}, 0]
      }
    },
    {
      "id": "REQUIRED_FIELDS",
      "description": "All mandatory fields must be present and non-empty",
      "condition": {
        "and": [
          {"!=": [{"var": "field_20.value"}, ""]},
          {"!=": [{"var": "field_21.value"}, ""]},
          {"!=": [{"var": "field_25.value"}, ""]}
        ]
      }
    }
  ]
}"#;
