use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT292_VALIDATION_RULES)]
pub struct MT292 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("11S")]
    pub field_11s: Field11S,

    #[field("79")]
    pub field_79: Option<Field79>,
}

/// Enhanced validation rules for MT292
const MT292_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "CONDITIONAL_C1",
      "description": "Either field 79 or copy of original message fields must be present, or both",
      "condition": {
        "or": [
          {"!!": {"var": "field_79"}},
          {
            "or": [
              {"!!": {"var": "field_32a"}},
              {"!!": {"var": "field_58a"}},
              {"!!": {"var": "field_52a"}},
              {"!!": {"var": "field_53a"}},
              {"!!": {"var": "field_56a"}},
              {"!!": {"var": "field_57a"}}
            ]
          }
        ]
      }
    },
    {
      "id": "REFERENCE_FORMAT",
      "description": "Reference fields must not have invalid slash patterns",
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
      "id": "FIELD_11S_FORMAT",
      "description": "Field 11S must have proper format for MT and date reference",
      "condition": {
        "and": [
          {"==": [{"strlen": {"var": "field_11s.message_type"}}, 3]},
          {"==": [{"strlen": {"var": "field_11s.date"}}, 6]},
          {"==": [{"strlen": {"var": "field_11s.session_number"}}, 4]},
          {"==": [{"strlen": {"var": "field_11s.input_sequence_number"}}, 4]}
        ]
      }
    },
    {
      "id": "TREASURY_MESSAGE_TYPE",
      "description": "Field 11S should reference valid treasury message types",
      "condition": {
        "in": [
          {"var": "field_11s.message_type"},
          ["200", "202", "205", "210", "256", "299"]
        ]
      }
    },
    {
      "id": "REQUIRED_FIELDS",
      "description": "All mandatory fields must be present and non-empty",
      "condition": {
        "and": [
          {"!=": [{"var": "field_20.value"}, ""]},
          {"!=": [{"var": "field_21.value"}, ""]},
          {"!=": [{"var": "field_11s.message_type"}, ""]},
          {"!=": [{"var": "field_11s.date"}, ""]}
        ]
      }
    },
    {
      "id": "REASON_CODE_VALIDATION",
      "description": "If field 79 is present, it should contain valid cancellation reason codes",
      "condition": {
        "if": [
          {"!!": {"var": "field_79"}},
          {
            "or": [
              {"includes": [{"var": "field_79.lines.0"}, "AGNT"]},
              {"includes": [{"var": "field_79.lines.0"}, "AM09"]},
              {"includes": [{"var": "field_79.lines.0"}, "COVR"]},
              {"includes": [{"var": "field_79.lines.0"}, "CURR"]},
              {"includes": [{"var": "field_79.lines.0"}, "CUST"]},
              {"includes": [{"var": "field_79.lines.0"}, "CUTA"]},
              {"includes": [{"var": "field_79.lines.0"}, "DUPL"]},
              {"includes": [{"var": "field_79.lines.0"}, "FRAD"]},
              {"includes": [{"var": "field_79.lines.0"}, "TECH"]},
              {"includes": [{"var": "field_79.lines.0"}, "UPAY"]}
            ]
          },
          true
        ]
      }
    }
  ]
}"#;
