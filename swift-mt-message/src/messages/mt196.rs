use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT196_VALIDATION_RULES)]
pub struct MT196 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("76")]
    pub field_76: Field76,

    #[field("77A")]
    pub field_77a: Option<Field77A>,

    #[field("11")]
    pub field_11: Option<Field11>,

    #[field("79")]
    pub field_79: Option<Field79>,
}

/// Enhanced validation rules for MT196
const MT196_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "CONDITIONAL_C1",
      "description": "Field 79 or copy of original message fields may be present, but not both",
      "condition": {
        "!": {
          "and": [
            {"!!": {"var": "field_79"}},
            {
              "or": [
                {"!!": {"var": "field_23b"}},
                {"!!": {"var": "field_32a"}},
                {"!!": {"var": "field_50"}},
                {"!!": {"var": "field_59"}},
                {"!!": {"var": "field_71a"}}
              ]
            }
          ]
        }
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
      "id": "FIELD_11A_FORMAT",
      "description": "Field 11a must have proper format when present",
      "condition": {
        "if": [
          {"!!": {"var": "field_11a"}},
          {">": [{"strlen": {"var": "field_11a.reference"}}, 8]},
          true
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
          {"!!": {"var": "field_76"}},
          {">": [{"count": {"var": "field_76.answer_lines"}}, 0]}
        ]
      }
    },
    {
      "id": "ANSWER_CODE_VALIDATION",
      "description": "Field 76 must contain valid answer codes",
      "condition": {
        "all": [
          {"var": "field_76.answer_lines"},
          {"!=": [{"var": ""}, ""]}
        ]
      }
    }
  ]
}"#;
