use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT941_VALIDATION_RULES)]
pub struct MT941 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Option<Field21NoOption>,

    #[field("25")]
    pub field_25: Field25AccountIdentification,

    #[field("28")]
    pub field_28: Field28,

    #[field("13D")]
    pub field_13d: Option<Field13D>,

    #[field("60F")]
    pub field_60f: Field60F,

    #[field("90D")]
    pub field_90d: Option<Field90D>,

    #[field("90C")]
    pub field_90c: Option<Field90C>,

    #[field("62F")]
    pub field_62f: Field62F,

    #[field("64")]
    pub field_64: Option<Field64>,

    #[field("65")]
    pub field_65: Vec<Field65>,

    #[field("86")]
    pub field_86: Option<Field86>,
}

/// Enhanced validation rules for MT941
const MT941_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "CURRENCY_CONSISTENCY",
      "description": "All balance fields must use the same currency",
      "condition": {
        "and": [
          {"==": [
            {"var": "field_60f.currency"},
            {"var": "field_62f.currency"}
          ]}
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
          {"var": "field_28d.is_valid"},
          {"var": "field_60f.is_valid"},
          {"var": "field_62f.is_valid"}
        ]
      }
    }
  ]
}"#;
