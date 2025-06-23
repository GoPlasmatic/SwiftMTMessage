use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT292: Request for Cancellation (Treasury)
///
/// This message is sent by a financial institution to request the cancellation
/// of a previously sent treasury-related message. MT292 is used for cancellation 
/// requests related to treasury operations, interbank transfers, and institutional transactions.
///
/// ## Key Features
/// - **Treasury cancellation**: Official request to cancel treasury/institutional messages
/// - **Reference tracking**: Links to the original message through multiple reference fields
/// - **Conditional structure**: Either narrative (field 79) or copy of original message fields
/// - **Audit trail**: Maintains complete cancellation audit records for treasury operations
/// - **Reason codes**: Standardized cancellation reason codes for institutional processing
///
/// ## Field Structure
/// The message follows a conditional structure where either field 79 (narrative description)
/// or a copy of the mandatory fields from the original message must be present, or both.
/// This structure is identical to MT192 but used for treasury/institutional contexts.
///
/// ## Cancellation Process
/// Used when a treasury department or institutional sender needs to request cancellation
/// of a previously sent message, typically MT2xx series messages for interbank transfers,
/// due to errors, regulatory requirements, or operational changes.
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT292_VALIDATION_RULES)]
pub struct MT292 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique reference assigned by the sender for this treasury cancellation request.
    /// This reference is used throughout the cancellation lifecycle for tracking,
    /// acknowledgment, and audit purposes. Must be unique within sender's system per business day.
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    /// **Related Reference** - Field 21
    ///
    /// Contains the reference from field 20 of the treasury message to be cancelled.
    /// This creates a direct link between the cancellation request and the original
    /// treasury message, enabling the receiver to identify exactly which message to cancel.
    #[field("21", mandatory)]
    pub field_21: GenericReferenceField,

    /// **MT and Date of the Original Message** - Field 11S
    ///
    /// Specifies the message type, date, session number, and Input Sequence Number (ISN)
    /// of the original treasury message to be cancelled. Format: 3!n6!n4!n/4!n
    /// Example: 202231215001/0123 (MT202 dated 2023-12-15, session 0001, ISN 0123)
    #[field("11S", mandatory)]
    pub field_11s: Field11S,

    /// **Narrative Description of the Original Message** - Field 79 (Conditional)
    ///
    /// Contains cancellation reason codes and free-form text explaining the cancellation.
    /// Must be present if copy of original message fields is not included, or both may be present.
    /// Common reason codes: AGNT, AM09, COVR, CURR, CUST, CUTA, DUPL, FRAD, TECH, UPAY
    #[field("79", optional)]
    pub field_79: Option<GenericMultiLine6x35>,

    /// **Copy of Mandatory Fields from Original Message** - Multiple Fields (Conditional)
    ///
    /// When present, contains a copy of at least the mandatory fields from the original treasury message.
    /// This helps the receiver identify the exact treasury transaction to be cancelled.
    /// The specific fields depend on the original message type referenced in field 11S.
    /// 
    /// For MT202: Would include fields 32A, 53, 58
    /// For MT205: Would include fields 32A, 53, 56, 57, 58  
    /// For MT210: Would include fields 32A, 53
    /// 
    /// Note: This is represented as optional structured content that can contain
    /// various field combinations depending on the original treasury message type.

    #[field("32A", optional)]
    pub field_32a: Option<Field32A>,

    #[field("58A", optional)]
    pub field_58a: Option<GenericBicField>,

    #[field("52A", optional)]
    pub field_52a: Option<GenericBicField>,

    #[field("53A", optional)]
    pub field_53a: Option<GenericBicField>,

    #[field("56A", optional)]
    pub field_56a: Option<GenericBicField>,

    #[field("57A", optional)]
    pub field_57a: Option<GenericBicField>,

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