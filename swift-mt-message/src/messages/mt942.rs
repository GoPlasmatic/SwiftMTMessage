use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, SwiftField, serde_swift_fields};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT942_VALIDATION_RULES)]
pub struct MT942 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Option<Field21NoOption>,

    #[field("25")]
    pub field_25: Field25AccountIdentification,

    #[field("28C")]
    pub field_28c: Field28C,

    #[field("34F")]
    pub field_34f_debit_limit: Field34F,

    #[field("34F")]
    pub field_34f_credit_limit: Option<Field34F>,

    #[field("13D")]
    pub field_13d: Field13D,

    #[field("#")]
    pub statement_lines: Vec<MT942StatementLine>,

    #[field("90D")]
    pub field_90d: Option<Field90D>,

    #[field("90C")]
    pub field_90c: Option<Field90C>,

    #[field("86")]
    pub field_86: Option<Field86>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct MT942StatementLine {
    #[component("61")]
    pub field_61: Option<Field61>,

    #[component("86")]
    pub field_86: Option<Field86>,
}

/// Enhanced validation rules for MT942
const MT942_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "CURRENCY_CONSISTENCY",
      "description": "All balance fields must use the same currency",
      "condition": {
        "and": [
          {"==": [
            {"var": "field_60f.currency"},
            {"var": "field_62f.currency"}
          ]},
          {
            "if": [
              {"var": "field_64.is_some"},
              {"==": [
                {"var": "field_60f.currency"},
                {"var": "field_64.currency"}
              ]},
              true
            ]
          },
          {
            "if": [
              {"var": "field_65.is_some"},
              {"==": [
                {"var": "field_60f.currency"},
                {"var": "field_65.currency"}
              ]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "ENTRY_CURRENCY_CONSISTENCY",
      "description": "Entry summaries must use same currency as balances",
      "condition": {
        "and": [
          {
            "if": [
              {"var": "field_90d.is_some"},
              {"==": [
                {"var": "field_60f.currency"},
                {"var": "field_90d.currency"}
              ]},
              true
            ]
          },
          {
            "if": [
              {"var": "field_90c.is_some"},
              {"==": [
                {"var": "field_60f.currency"},
                {"var": "field_90c.currency"}
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
          {"var": "field_60f.is_valid"},
          {"var": "field_62f.is_valid"}
        ]
      }
    }
  ]
}"#;

/// Validation rules specific to MT942 statement lines
const MT942_STATEMENT_LINE_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "STATEMENT_LINE_VALID",
      "description": "Statement line must be valid",
      "condition": {
        "var": "field_61.is_valid"
      }
    }
  ]
}"#;
