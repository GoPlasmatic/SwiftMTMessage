use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT296: Answers (Treasury Messages)
///
/// This message is sent by a financial institution to provide answers, confirmations,
/// or status information in response to queries or requests related to treasury messages.
/// MT296 is used for answering treasury transfer queries and other institutional inquiries.
///
/// ## Key Features
/// - **Treasury answer provision**: Official responses to treasury message queries
/// - **Reference tracking**: Links to the original message through reference fields
/// - **Conditional structure**: Either narrative (field 79) or copy of original message fields
/// - **Answer codes**: Standardized answer codes with supplementary data
/// - **Narrative support**: Additional narrative description capability
///
/// ## Field Structure
/// The message follows a conditional structure where either field 79 (narrative description)
/// or a copy of the mandatory fields from the original message may be present, but not both.
/// This structure is identical to MT196 but used for treasury/institutional contexts.
///
/// ## Answer Process
/// Used when a treasury department or institutional receiver needs to provide answers,
/// confirmations, or status updates regarding previously received treasury messages,
/// including confirmations of institutional transfers, cover payments, or responses to treasury queries.
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT296_VALIDATION_RULES)]
pub struct MT296 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique reference assigned by the sender for this treasury answer message.
    /// This reference is used throughout the answer lifecycle for tracking,
    /// acknowledgment, and audit purposes. Must be unique within sender's system per business day.
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    /// **Related Reference** - Field 21
    ///
    /// Contains the reference from field 20 of the treasury message being answered.
    /// This creates a direct link between the answer and the original
    /// treasury message, enabling complete audit trails and transaction tracking.
    #[field("21", mandatory)]
    pub field_21: GenericReferenceField,

    /// **Answers** - Field 76
    ///
    /// Contains response codes, narratives, and supplementary data.
    /// Includes confirmation codes (1-33), cancellation codes (CNCL, PDCR, RJCR),
    /// and reason codes with optional supplementary details in parentheses.
    #[field("76", mandatory)]
    pub field_76: GenericMultiLine6x35,

    /// **Narrative** - Field 77A (Optional)
    ///
    /// Free-form narrative description that supplements the answer codes in field 76.
    /// Used for providing additional context, explanations, or details about the treasury answers.
    #[field("77A", optional)]
    pub field_77a: Option<GenericMultiLine20x35>,

    /// **MT and Date of the Original Message** - Field 11a (Optional)
    ///
    /// Specifies the message type and date of the original treasury message being answered.
    /// Can be in Option R format (with session/ISN) or Option S format (date only).
    #[field("11a", optional)]
    pub field_11a: Option<GenericTextField>,

    /// **Narrative Description of Original Message** - Field 79 (Conditional)
    ///
    /// Contains narrative description of the original treasury message being answered.
    /// Must be present if copy of original message fields is not included.
    /// Cannot be used together with copy of original message fields.
    #[field("79", optional)]
    pub field_79: Option<GenericMultiLine6x35>,

    /// **Copy of Mandatory Fields from Original Message** - Multiple Fields (Conditional)
    ///
    /// When present, contains a copy of at least the mandatory fields from the original treasury message.
    /// This helps identify the exact treasury transaction being answered.
    /// Cannot be used together with field 79 according to conditional rule C1.
    ///
    /// For treasury messages:
    /// - MT202: Would include fields 32A, 53, 58
    /// - MT205: Would include fields 32A, 53, 56, 57, 58
    /// - MT210: Would include fields 32A, 53
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

/// Enhanced validation rules for MT296
const MT296_VALIDATION_RULES: &str = r#"{
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