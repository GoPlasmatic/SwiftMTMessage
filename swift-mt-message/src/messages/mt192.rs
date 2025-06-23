use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT192: Request for Cancellation
///
/// This message is sent by a financial institution to request the cancellation
/// of a previously sent message. MT192 is used for cancellation requests related
/// to customer credit transfers and other payment instructions.
///
/// ## Key Features
/// - **Cancellation request**: Official request to cancel a previously sent message
/// - **Reference tracking**: Links to the original message through multiple reference fields
/// - **Conditional structure**: Either narrative (field 79) or copy of original message fields
/// - **Audit trail**: Maintains complete cancellation audit records
/// - **Reason codes**: Standardized cancellation reason codes for processing
///
/// ## Field Structure
/// The message follows a conditional structure where either field 79 (narrative description)
/// or a copy of the mandatory fields from the original message must be present, or both.
///
/// ## Cancellation Process
/// Used when a sender needs to request cancellation of a previously sent message,
/// typically due to errors, fraud, customer request, or technical problems.
/// The receiver processes the request and may accept or reject the cancellation.
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT192_VALIDATION_RULES)]
pub struct MT192 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique reference assigned by the sender for this cancellation request.
    /// This reference is used throughout the cancellation lifecycle for tracking,
    /// acknowledgment, and audit purposes. Must be unique within sender's system per business day.
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    /// **Related Reference** - Field 21
    ///
    /// Contains the reference from field 20 of the message to be cancelled.
    /// This creates a direct link between the cancellation request and the original
    /// message, enabling the receiver to identify exactly which message to cancel.
    #[field("21", mandatory)]
    pub field_21: GenericReferenceField,

    /// **MT and Date of the Original Message** - Field 11S
    ///
    /// Specifies the message type, date, session number, and Input Sequence Number (ISN)
    /// of the original message to be cancelled. Format: 3!n6!n4!n/4!n
    /// Example: 103231215001/0123 (MT103 dated 2023-12-15, session 0001, ISN 0123)
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
    /// When present, contains a copy of at least the mandatory fields from the original message.
    /// This helps the receiver identify the exact transaction to be cancelled.
    /// The specific fields depend on the original message type referenced in field 11S.
    ///
    /// For MT103: Would include fields 23B, 32A, 50, 59, 71A
    ///
    /// Note: This is represented as optional structured content that can contain
    /// various field combinations depending on the original message type.

    #[field("23B", optional)]
    pub field_23b: Option<GenericTextField>,

    #[field("32A", optional)]
    pub field_32a: Option<Field32A>,

    #[field("50", optional)]
    pub field_50: Option<Field50>,

    #[field("59", optional)]
    pub field_59: Option<Field59>,

    #[field("71A", optional)]
    pub field_71a: Option<GenericTextField>,
}

/// Enhanced validation rules for MT192
const MT192_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "CONDITIONAL_C1",
      "description": "Either field 79 or copy of original message fields must be present, or both",
      "condition": {
        "or": [
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
