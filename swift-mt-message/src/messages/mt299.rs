use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT299_VALIDATION_RULES)]
pub struct MT299 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Option<Field21NoOption>,

    #[field("79")]
    pub field_79: Field79,
}

const MT299_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "NARRATIVE_FORMAT",
      "description": "If narrative starts with /REJT/ or /RETN/, it must follow Payments Reject/Return Guidelines",
      "condition": true
    }
  ]
}"#;
