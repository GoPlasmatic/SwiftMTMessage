use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT935_VALIDATION_RULES)]
pub struct MT935 {
    #[field("20")]
    pub field_20: Field20,

    #[field("#")]
    pub rate_changes: Vec<MT935RateChange>,

    #[field("72")]
    pub field_72: Option<Field72>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT935_RATE_CHANGE_VALIDATION_RULES)]
pub struct MT935RateChange {
    #[field("23")]
    pub field_23: Option<Field23>,

    #[field("25")]
    pub field_25: Option<Field25NoOption>,

    #[field("30")]
    pub field_30: Field30,

    #[field("37H")]
    pub field_37h: Vec<Field37H>,
}

/// Enhanced validation rules for MT935
const MT935_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Rate change sequences must occur 1-10 times",
      "condition": {
        "and": [
          {">=": [{"length": {"var": "rate_changes"}}, 1]},
          {"<=": [{"length": {"var": "rate_changes"}}, 10]}
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
        "!=": [{"var": "field_20.value"}, ""]
      }
    }
  ]
}"#;

/// Validation rules specific to MT935 rate change sequences
const MT935_RATE_CHANGE_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C2",
      "description": "Either Field 23 or Field 25 must be present, but not both",
      "condition": {
        "or": [
          {
            "and": [
              {"var": "field_23.is_some"},
              {"var": "field_25.is_none"}
            ]
          },
          {
            "and": [
              {"var": "field_23.is_none"},
              {"var": "field_25.is_some"}
            ]
          }
        ]
      }
    },
    {
      "id": "REQUIRED_SEQUENCE_FIELDS",
      "description": "Effective date and new rate must be present and non-empty",
      "condition": {
        "and": [
          {"!=": [{"var": "field_30.value"}, ""]},
          {"var": "field_37h.is_valid"}
        ]
      }
    }
  ]
}"#;
