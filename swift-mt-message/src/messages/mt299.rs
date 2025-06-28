use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT299: Free Format Message (Category 2)
///
/// MT299 is used for information where no specific message type exists.
/// This message type is used in Category 2 (financial institution transfers).
///
/// ## Key Features:
/// - Used for free format information exchange
/// - Category 2 routing (financial institution transfers)
/// - Can contain reject/return information when narrative starts with /REJT/ or /RETN/
///
/// ## Fields:
/// - **20**: Transaction Reference Number (Mandatory) - 16x
/// - **21**: Related Reference (Optional) - 16x  
/// - **79**: Narrative (Mandatory) - 50*35x
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT299_VALIDATION_RULES)]
pub struct MT299 {
    // Mandatory Fields
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    #[field("79", mandatory)]
    pub field_79: Field79,

    // Optional Fields
    #[field("21", optional)]
    pub field_21: Option<GenericReferenceField>,
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
