use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT292: Request for Cancellation (Category 2 - Financial Institution Transfers)
///
/// ## Purpose
/// Used to request the cancellation of a previously sent financial institution transfer message
/// (Category 2). This message provides reference information to identify the specific message
/// to be cancelled and includes reasons for the cancellation request.
///
/// ## Scope
/// This message is:
/// - Sent by the originator of a Category 2 financial institution transfer to request its cancellation
/// - Used for MT200, MT202, MT205, MT210, and other Category 2 messages
/// - Applied when an institutional transfer needs to be cancelled before execution
/// - Contains precise identifying information of the original message
/// - May include structured reason codes for the cancellation
///
/// ## Key Features
/// - **Message Identification**: Field 11S provides precise reference to original message
/// - **Treasury Focus**: Specifically designed for financial institution transfer cancellations
/// - **Cancellation Control**: Request processing before payment settlement
/// - **Reference Tracking**: Links to original message through transaction references
/// - **Reason Documentation**: Optional structured cancellation reasons in field 79
/// - **Session-Based Tracking**: Field 11S includes session and sequence number details
///
/// ## Common Use Cases
/// - Cancellation of MT202/MT205 institutional transfers
/// - Prevention of duplicate institutional payments
/// - Correction of erroneous transfer instructions
/// - Liquidity management adjustments
/// - Settlement system error recovery
/// - Treasury operation corrections
/// - Cross-border institutional payment cancellations
///
/// ## Field Structure
/// - **20**: Sender's Reference (mandatory) - Message reference for this cancellation request
/// - **21**: Related Reference (mandatory) - Reference to the original message being cancelled
/// - **11S**: MT and Date Reference (mandatory) - Precise identification of original message
/// - **79**: Narrative (optional) - Cancellation reasons and additional information
///
/// ## Field 11S Structure
/// The Field 11S contains critical information for identifying the original message:
/// - **Message Type**: 3-digit MT number (200, 202, 205, 210, etc.)
/// - **Date**: 6-digit date (YYMMDD) when original message was sent
/// - **Session Number**: 4-digit session identifier
/// - **Input Sequence Number**: 4-digit sequence number within the session
///
/// ## Network Validation Rules
/// - **C1 Rule**: Either field 79 or copy of original message fields must be present
/// - **Reference Format**: All reference fields must follow SWIFT formatting rules
/// - **Field 11S Format**: Must contain valid MT type, date, session, and sequence numbers
/// - **Treasury Message Types**: Field 11S should reference valid Category 2 message types
/// - **Mandatory Fields**: All required fields must be present and properly formatted
/// - **Reason Codes**: If field 79 present, should contain valid cancellation reason codes
///
/// ## Cancellation Reason Codes
/// When field 79 is used, it may contain standardized reason codes such as:
/// - **AGNT**: Agent/Institution Error
/// - **AM09**: Wrong Amount
/// - **COVR**: Cover Payment Issue
/// - **CURR**: Currency Error
/// - **CUST**: Customer Request
/// - **CUTA**: Cut-off Time
/// - **DUPL**: Duplicate Payment
/// - **FRAD**: Fraud
/// - **TECH**: Technical Problem
/// - **UPAY**: Undue Payment
///
/// ## Processing Considerations
/// - **Timing Critical**: Should be sent as soon as possible after error detection
/// - **Settlement Impact**: Cancellation success depends on settlement timing
/// - **Institution Coordination**: May require coordination between multiple institutions
/// - **Audit Trail**: Maintains complete record of cancellation requests
/// - **Response Required**: Typically followed by MT296 (Answers) message
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT292 format remains unchanged in SRG2025
/// - **Validation Updates**: Enhanced validation for institutional transfer cancellations
/// - **Processing Improvements**: Improved integration with modern settlement systems
/// - **Compliance Notes**: Enhanced validation for cross-border and international cancellations
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with treasury management and settlement systems
/// - **API Integration**: RESTful API support for modern institutional transfer platforms
/// - **Processing Requirements**: Supports urgent processing with time-sensitive cancellation capabilities
/// - **Compliance Integration**: Built-in validation for regulatory cancellation requirements
///
/// ## Relationship to Other Messages
/// - **Triggers**: Triggered by treasury systems or institutional transfer processing errors
/// - **Responses**: Generates MT296 (Answers) response messages with cancellation status
/// - **Related**: Works with original Category 2 messages (MT200, MT202, MT205, MT210, etc.)
/// - **Alternatives**: Direct system-level cancellation for internal processing corrections
/// - **Status Updates**: May be followed by replacement transfer if correction needed

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
