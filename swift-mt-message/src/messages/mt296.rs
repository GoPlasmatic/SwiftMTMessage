use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT296: Answers (Category 2 - Financial Institution Transfers)
///
/// ## Purpose
/// Used to provide answers or responses to various queries and requests related to Category 2
/// financial institution transfers. This message responds to different types of inquiries,
/// including cancellation requests (MT292) and other operational queries.
///
/// ## Scope
/// This message is:
/// - Sent in response to MT292 cancellation requests and other Category 2 inquiries
/// - Used to provide structured answers with detailed status information
/// - Contains response codes and explanatory text for institutional transfer queries
/// - Supports various inquiry types with flexible narrative content
/// - Essential for treasury operations and institutional transfer management
///
/// ## Key Features
/// - **Query Response**: Structured response to various types of institutional transfer inquiries
/// - **Reference Tracking**: Links to original query or request message through field 21
/// - **Answer Codes**: Field 76 provides structured answers with specific codes
/// - **Status Information**: Clear indication of query resolution and outcome
/// - **Flexible Format**: Adaptable to different types of institutional transfer inquiries
/// - **Optional Narrative**: Field 79 for additional explanatory information
///
/// ## Common Use Cases
/// - Response to MT292 cancellation requests (accept/reject/partial)
/// - Status updates on institutional transfer processing
/// - Inquiry responses about transaction details and settlement status
/// - Error resolution and clarification messages for treasury operations
/// - Settlement system status communications
/// - Correspondent banking operational responses
/// - Cross-border institutional transfer confirmations
///
/// ## Field Structure
/// - **20**: Sender's Reference (mandatory) - Reference for this answer message
/// - **21**: Related Reference (mandatory) - Reference to original inquiry/request
/// - **76**: Answers (mandatory) - Structured answer codes and information
/// - **77A**: Optional Query Section (optional) - Additional query details
/// - **11**: MT and Date Reference (optional) - Reference to specific original message
/// - **79**: Narrative (optional) - Additional explanatory text
///
/// ## Field 76 Answer Codes
/// The Field 76 contains structured answer codes that may include:
/// - **ACCEPTED**: Request has been accepted and processed
/// - **REJECTED**: Request has been rejected with reason
/// - **PARTIAL**: Partial acceptance/processing of request
/// - **PENDING**: Request is under review/processing
/// - **COMPLETED**: Processing has been completed
/// - **ERROR**: Error encountered during processing
/// - **TIMEOUT**: Request timed out or expired
/// - **DUPLICATE**: Duplicate request detected
/// - **INVALID**: Invalid request format or content
///
/// ## Network Validation Rules
/// - **C1 Rule**: Field 79 or copy of original message fields may be present, but not both
/// - **Reference Format**: All reference fields must follow SWIFT formatting conventions
/// - **Field 11A Format**: When present, must have proper format with valid MT reference
/// - **Required Fields**: All mandatory fields must be present and non-empty
/// - **Answer Code Validation**: Field 76 must contain valid, recognizable answer codes
/// - **Conditional Fields**: Optional fields must follow proper conditional logic
///
/// ## Answer Processing Types
/// ### Cancellation Responses (to MT292)
/// - **CANC**: Cancellation accepted and processed
/// - **RJCT**: Cancellation rejected (payment already processed)
/// - **PART**: Partial cancellation (only some transactions cancelled)
/// - **NPAY**: No payment found matching cancellation request
///
/// ### Status Responses
/// - **ACPT**: Message accepted for processing
/// - **PROC**: Currently processing
/// - **SETT**: Settlement completed
/// - **FAIL**: Processing failed
///
/// ### Information Responses
/// - **INFO**: Informational response provided
/// - **CONF**: Confirmation of status or details
/// - **NFND**: Requested information not found
/// - **RSTR**: Restricted information (access denied)
///
/// ## Processing Considerations
/// - **Timely Response**: Should be sent promptly after receiving inquiry
/// - **Accurate Status**: Must reflect current and accurate status information
/// - **Clear Communication**: Answer codes should be unambiguous
/// - **Audit Trail**: Maintains record of all query-response interactions
/// - **Follow-up**: May trigger additional operational actions
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT296 format remains unchanged in SRG2025
/// - **Validation Updates**: Enhanced validation for institutional transfer responses
/// - **Processing Improvements**: Improved validation for answer code consistency
/// - **Compliance Notes**: Better integration with modern settlement systems
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with treasury management and customer service systems
/// - **API Integration**: RESTful API support for modern institutional transfer response platforms
/// - **Processing Requirements**: Supports real-time response generation with audit capabilities
/// - **Compliance Integration**: Built-in validation for regulatory response requirements
///
/// ## Relationship to Other Messages
/// - **Triggers**: Directly triggered by MT292 cancellation requests and Category 2 inquiries
/// - **Responses**: Provides definitive responses to institutional transfer requests
/// - **Related**: Works with Category 2 messages and operational workflow systems
/// - **Alternatives**: Direct system notifications for internal processing status updates
/// - **Status Updates**: Final response message in institutional transfer inquiry lifecycle

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT296_VALIDATION_RULES)]
pub struct MT296 {
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

/// Enhanced validation rules for MT296
const MT296_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Only one of the following may be present: Field 79, or a copy of mandatory fields of the original message",
      "condition": {
        "!": {"exists": ["fields", "79"]}
      }
    }
  ]
}"#;
