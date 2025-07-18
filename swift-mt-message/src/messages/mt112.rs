use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT112_VALIDATION_RULES)]
pub struct MT112 {
    #[field("20")]
    pub field_20: Field20, // Transaction Reference Number

    #[field("21")]
    pub field_21: Field21NoOption, // Cheque Number

    #[field("30")]
    pub field_30: Field30, // Date of Issue (YYMMDD)

    #[field("32")]
    pub field_32: Field32, // Amount

    #[field("52")]
    pub field_52: Option<Field52DrawerBank>, // Drawer Bank A

    #[field("59")]
    pub field_59: Option<Field59NoOption>, // Payee (without account number)

    #[field("76")]
    pub field_76: Field76, // Answers (Status Information)
}

/// Validation rules for MT112 - Status of Request for Stop Payment of a Cheque
const MT112_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Date of Issue (30) must be valid YYMMDD format",
      "condition": {
        "matches": [
          {"var": "field_30.value"},
          "^[0-9]{6}$"
        ]
      }
    },
    {
      "id": "C2", 
      "description": "Transaction Reference (20) must not start or end with '/' and must not contain '//'",
      "condition": {
        "and": [
          {"!": {"matches": [{"var": "field_20.value"}, "^/"]}},
          {"!": {"matches": [{"var": "field_20.value"}, "/$"]}},
          {"!": {"matches": [{"var": "field_20.value"}, "//"]}}
        ]
      }
    },
    {
      "id": "C3",
      "description": "Cheque Number (21) must not start or end with '/' and must not contain '//'", 
      "condition": {
        "and": [
          {"!": {"matches": [{"var": "field_21.value"}, "^/"]}},
          {"!": {"matches": [{"var": "field_21.value"}, "/$"]}},
          {"!": {"matches": [{"var": "field_21.value"}, "//"]}}
        ]
      }
    },
    {
      "id": "C4",
      "description": "Only one option of Drawer Bank (52A/B/D) may be present",
      "condition": {
        "<=": [
          {"+": [
            {"if": [{"var": "field_52a.is_some"}, 1, 0]},
            {"if": [{"var": "field_52b.is_some"}, 1, 0]},
            {"if": [{"var": "field_52d.is_some"}, 1, 0]}
          ]},
          1
        ]
      }
    },
    {
      "id": "C5",
      "description": "Payee field (59) must not contain account number in first line",
      "condition": {
        "if": [
          {"var": "field_59.is_some"},
          {"!": {"matches": [{"var": "field_59.lines.0"}, "^/"]}},
          true
        ]
      }
    },
    {
      "id": "C6",
      "description": "Answers field (76) should contain predefined status codes when applicable",
      "condition": {
        "or": [
          {"matches": [{"var": "field_76.lines.0"}, "STOP PAYMENT"]},
          {"matches": [{"var": "field_76.lines.0"}, "REQUEST"]},
          {"matches": [{"var": "field_76.lines.0"}, "PROCESSING"]},
          {"matches": [{"var": "field_76.lines.0"}, "/[0-9]{2}/"]},
          true
        ]
      }
    }
  ]
}"#;
