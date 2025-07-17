use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT210_VALIDATION_RULES)]
pub struct MT210 {
    #[field("20")]
    pub field_20: Field20, // Transaction Reference Number

    #[field("25")]
    pub field_25: Option<Field25NoOption>, // Account Identification

    #[field("30")]
    pub field_30: Field30, // Value Date (YYMMDD)

    #[field("#")]
    pub sequence: Vec<MT210Sequence>, // Sequence of Fields
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT210Sequence {
    #[field("21")]
    pub field_21: Field21NoOption, // Related Reference

    #[field("32B")]
    pub field_32b: Field32B, // Currency and Amount

    #[field("50")]
    pub field_50: Option<Field50OrderingCustomerNCF>, // Ordering Customer (C, F options)

    #[field("52")]
    pub field_52a: Option<Field52OrderingInstitution>, // Ordering Institution (A, D options)

    #[field("56")]
    pub field_56a: Option<Field56Intermediary>, // Intermediary Institution (A, D options)
}

/// Validation rules for MT210 - Notice to Receive
const MT210_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
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
      "id": "C2_RULE",
      "description": "Rule C2: Either Field 50a or Field 52a must be present, not both",
      "condition": {
        "xor": [
          {"var": "field_50a.is_some"},
          {"var": "field_52a.is_some"}
        ]
      }
    },
    {
      "id": "C3",
      "description": "Rule C3: Currency must be consistent in all 32B fields (single instance validation)",
      "condition": {
        "!=": [{"var": "field_32b.currency"}, ""]
      }
    },
    {
      "id": "C4",
      "description": "Related Reference (21) must not start or end with '/' and must not contain '//'",
      "condition": {
        "and": [
          {"!": {"matches": [{"var": "field_21.value"}, "^/"]}},
          {"!": {"matches": [{"var": "field_21.value"}, "/$"]}},
          {"!": {"matches": [{"var": "field_21.value"}, "//"]}}
        ]
      }
    },
    {
      "id": "C5",
      "description": "Value Date (30) must be valid YYMMDD format",
      "condition": {
        "matches": [
          {"var": "field_30.value"},
          "^[0-9]{6}$"
        ]
      }
    },
    {
      "id": "C6",
      "description": "Amount must be positive",
      "condition": {
        ">": [{"var": "field_32b.amount"}, 0]
      }
    },
    {
      "id": "C7",
      "description": "Commodity currencies (XAU, XAG, XPD, XPT) must not be used",
      "condition": {
        "!": {
          "in": [
            {"var": "field_32b.currency"},
            ["XAU", "XAG", "XPD", "XPT"]
          ]
        }
      }
    },
    {
      "id": "C8",
      "description": "Account identification format validation when present",
      "condition": {
        "if": [
          {"var": "field_25.is_some"},
          {"<=": [{"length": {"var": "field_25.value"}}, 35]},
          true
        ]
      }
    },
    {
      "id": "C9",
      "description": "Option F for Field 50a requires structured identity details",
      "condition": {
        "if": [
          {"var": "field_50a.is_some"},
          {"or": [
            {"var": "field_50a.is_option_c"},
            {"var": "field_50a.is_option_f"}
          ]},
          true
        ]
      }
    },
    {
      "id": "C10",
      "description": "Option D for Fields 52a/56a may include national clearing codes",
      "condition": {
        "if": [
          {"or": [
            {"var": "field_52a.is_some"},
            {"var": "field_56a.is_some"}
          ]},
          {"or": [
            {"var": "field_52a.is_option_a"},
            {"var": "field_52a.is_option_d"},
            {"var": "field_56a.is_option_a"},
            {"var": "field_56a.is_option_d"}
          ]},
          true
        ]
      }
    }
  ]
}"#;
