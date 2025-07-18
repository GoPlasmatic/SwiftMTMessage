use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT940_VALIDATION_RULES)]
pub struct MT940 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Option<Field21NoOption>,

    #[field("25")]
    pub field_25: Field25AccountIdentification,

    #[field("28C")]
    pub field_28c: Field28C,

    #[field("60")]
    pub field_60: Field60,

    #[field("#")]
    pub statement_lines: Vec<MT940StatementLine>,

    #[field("62")]
    pub field_62: Field62,

    #[field("64")]
    pub field_64: Option<Field64>,

    #[field("65")]
    pub field_65: Vec<Field65>,

    #[field("86")]
    pub field_86: Option<Field86>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT940_STATEMENT_LINE_VALIDATION_RULES)]
pub struct MT940StatementLine {
    #[field("61")]
    pub field_61: Option<Field61>,

    #[field("86")]
    pub field_86: Option<Field86>,
}

/// Enhanced validation rules for MT940
const MT940_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "CURRENCY_CONSISTENCY",
      "description": "Opening and closing balances must have consistent currency",
      "condition": {
        "==": [
          {"var": "field_60f.currency"},
          {"var": "field_62f.currency"}
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

/// Validation rules specific to MT940 statement lines
const MT940_STATEMENT_LINE_VALIDATION_RULES: &str = r#"{
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
