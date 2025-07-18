use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT950_VALIDATION_RULES)]
pub struct MT950 {
    #[field("20")]
    pub field_20: Field20,

    #[field("25")]
    pub field_25: Field25NoOption,

    #[field("28C")]
    pub field_28c: Field28C,

    #[field("60")]
    pub field_60: Field60,

    #[field("61")]
    pub field_61: Vec<Field61>,

    #[field("62")]
    pub field_62: Field62,

    #[field("64")]
    pub field_64: Option<Field64>,
}

/// Enhanced validation rules for MT950
const MT950_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "CURRENCY_CONSISTENCY",
      "description": "Opening and closing balances must have consistent currency",
      "condition": {
        "==": [
          {"var": "field_60.currency"},
          {"var": "field_62.currency"}
        ]
      }
    },
    {
      "id": "AVAILABLE_BALANCE_CURRENCY",
      "description": "Available balances must use same currency as main balances",
      "condition": {
        "and": [
          {
            "if": [
              {"var": "field_64.is_some"},
              {"==": [
                {"var": "field_60.currency"},
                {"var": "field_64.currency"}
              ]},
              true
            ]
          },
          {
            "if": [
              {"var": "field_65.is_some"},
              {"==": [
                {"var": "field_60.currency"},
                {"var": "field_65.currency"}
              ]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "REF_FORMAT",
      "description": "Transaction reference must not have invalid slash patterns",
      "condition": {
        "and": [
          {"!": {"startsWith": [{"var": "field_20.value"}, "/"]}},
          {"!": {"endsWith": [{"var": "field_20.value"}, "/"]}},
          {"!": {"includes": [{"var": "field_20.value"}, "//"]}}
        ]
      }
    },
    {
      "id": "REQUIRED_FIELDS",
      "description": "All mandatory fields must be present and non-empty",
      "condition": {
        "and": [
          {"!=": [{"var": "field_20.value"}, ""]},
          {"!=": [{"var": "field_25.value"}, ""]},
          {"var": "field_28c.is_valid"},
          {"var": "field_60.is_valid"},
          {"var": "field_62.is_valid"}
        ]
      }
    }
  ]
}"#;
