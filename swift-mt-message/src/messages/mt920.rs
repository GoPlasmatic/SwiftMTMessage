use crate::fields::{field34::Field34F, *};
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT920_VALIDATION_RULES)]
pub struct MT920 {
    #[field("20")]
    pub field_20: Field20,

    #[field("#")]
    pub sequence: Vec<MT920Sequence>, // Sequence of Fields
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT920Sequence {
    #[field("12")]
    pub field_12: Field12,

    #[field("25")]
    pub field_25: Field25NoOption,

    #[field("34F")]
    pub field_34f_debit: Option<Field34F>,

    #[field("34F")]
    pub field_34f_credit: Option<Field34F>,
}

/// Enhanced validation rules for MT920
const MT920_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "If message requested is 942, Field 34F for debit must be present",
      "condition": {
        "if": [
          {"==": [{"var": "field_12.value"}, "942"]},
          {"var": "field_34f_debit.is_some"},
          true
        ]
      }
    },
    {
      "id": "C2",
      "description": "When both 34F fields present: first must be 'D', second must be 'C'",
      "condition": {
        "if": [
          {
            "and": [
              {"var": "field_34f_debit.is_some"},
              {"var": "field_34f_credit.is_some"}
            ]
          },
          {
            "and": [
              {"==": [{"var": "field_34f_debit.sign"}, "D"]},
              {"==": [{"var": "field_34f_credit.sign"}, "C"]}
            ]
          },
          true
        ]
      }
    },
    {
      "id": "C3",
      "description": "Currency code must be same across all 34F entries",
      "condition": {
        "if": [
          {
            "and": [
              {"var": "field_34f_debit.is_some"},
              {"var": "field_34f_credit.is_some"}
            ]
          },
          {"==": [
            {"var": "field_34f_debit.currency"},
            {"var": "field_34f_credit.currency"}
          ]},
          true
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
      "id": "MESSAGE_TYPE_VALID",
      "description": "Message requested must be valid SWIFT MT type",
      "condition": {
        "in": [
          {"var": "field_12.value"},
          ["940", "941", "942", "950"]
        ]
      }
    },
    {
      "id": "REQUIRED_FIELDS",
      "description": "All mandatory fields must be present and non-empty",
      "condition": {
        "and": [
          {"!=": [{"var": "field_20.value"}, ""]},
          {"!=": [{"var": "field_12.value"}, ""]},
          {"!=": [{"var": "field_25.value"}, ""]}
        ]
      }
    }
  ]
}"#;
