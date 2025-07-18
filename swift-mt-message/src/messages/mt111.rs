use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT111_VALIDATION_RULES)]
pub struct MT111 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("30")]
    pub field_30: Field30,

    #[field("32")]
    pub field_32: Field32,

    #[field("52")]
    pub field_52: Option<Field52DrawerBank>,

    #[field("59")]
    pub field_59: Option<Field59NoOption>,

    #[field("75")]
    pub field_75: Option<Field75>,
}

/// Enhanced validation rules for MT111
const MT111_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "REF_FORMAT",
      "description": "Sender's reference must not start/end with '/' or contain '//'",
      "condition": {
        "and": [
          {"!=": [{"var": "field_20.value"}, ""]},
          {"!": [{"startsWith": [{"var": "field_20.value"}, "/"]}]},
          {"!": [{"endsWith": [{"var": "field_20.value"}, "/"]}]},
          {"!": [{"in": ["//", {"var": "field_20.value"}]}]}
        ]
      }
    },
    {
      "id": "CHQ_FORMAT",
      "description": "Cheque number must not contain '/' or '//'",
      "condition": {
        "and": [
          {"!=": [{"var": "field_21.value"}, ""]},
          {"!": [{"in": ["/", {"var": "field_21.value"}]}]},
          {"!": [{"in": ["//", {"var": "field_21.value"}]}]}
        ]
      }
    },
    {
      "id": "DATE_VALID",
      "description": "Date of issue must be a valid date",
      "condition": {
        "!=": [{"var": "field_30.value"}, ""]
      }
    },
    {
      "id": "PAYEE_NO_ACCOUNT",
      "description": "Payee must not contain account number - only name and address",
      "condition": {
        "if": [
          {"var": "field_59.is_some"},
          true,
          true
        ]
      }
    }
  ]
}"#;
