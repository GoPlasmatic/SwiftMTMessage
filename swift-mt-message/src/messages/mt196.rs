use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT196: Answers
///
/// ## Purpose
/// Used to provide comprehensive answers and responses to various queries and requests related to customer
/// payments and transactions. This flexible message format serves as the standard response mechanism for
/// payment inquiries, cancellation requests, and status updates in the SWIFT payment ecosystem.
///
/// ## Scope
/// This message is:
/// - Used for responses to queries about payment status, cancellation requests, and transaction inquiries
/// - Applicable for structured answers with detailed information and resolution status
/// - Designed for flexible narrative content supporting various inquiry types
/// - Compatible with automated and manual response generation systems
/// - Subject to validation rules for proper reference tracking and response formatting
/// - Integrated with customer service and payment processing workflow systems
///
/// ## Key Features
/// - **Comprehensive Query Response**: Structured response to various types of payment inquiries and requests
/// - **Reference Tracking System**: Direct links to original query or request messages
/// - **Detailed Narrative Content**: Field 76 for comprehensive explanatory information and answers
/// - **Status Resolution Information**: Clear indication of query resolution and processing outcomes
/// - **Flexible Response Format**: Adaptable to different types of customer payment inquiries and scenarios
/// - **Audit Trail Support**: Complete documentation of inquiry resolution for compliance and tracking
///
/// ## Common Use Cases
/// - Response to MT192 cancellation requests with approval or rejection status
/// - Status updates on payment processing and execution outcomes
/// - Detailed inquiry responses about transaction details and processing steps
/// - Error resolution and clarification messages for payment issues
/// - Customer service communications for payment-related questions
/// - Regulatory inquiry responses for compliance and audit purposes
/// - Technical problem resolution and system status communications
///
/// ## Message Structure
/// - **Field 20**: Sender's Reference (mandatory) - Unique reference for response message
/// - **Field 21**: Related Reference (mandatory) - Reference to original query or request message
/// - **Field 76**: Answers (mandatory) - Detailed response content and status information
/// - **Field 77A**: Proprietary Message (optional) - Additional proprietary information for specific scenarios
/// - **Field 11**: Message Type and Date (optional) - Reference to original message type and processing date
/// - **Field 79**: Narrative (optional) - Additional explanatory text and detailed information
///
/// ## Network Validation Rules
/// - **Reference Format Validation**: Reference fields must not start/end with '/' or contain '//'
/// - **Mutual Exclusivity**: Field 79 and original message field copies cannot both be present
/// - **Field 11 Format**: Field 11 must have proper format when present (minimum 8 characters)
/// - **Answer Content Validation**: Field 76 must contain valid answer codes and non-empty content
/// - **Reference Consistency**: All references must be consistent with original inquiry message
/// - **Response Completeness**: All mandatory fields must be present with valid content
/// - **Format Compliance**: All fields must comply with SWIFT format specifications
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT196 format remains stable for answer and response processing
/// - **Validation Updates**: Enhanced validation for response completeness and reference accuracy
/// - **Processing Improvements**: Improved handling of automated response generation
/// - **Compliance Notes**: Strengthened requirements for audit trail and regulatory response documentation
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with customer service systems and payment processing platforms
/// - **API Integration**: RESTful API support for modern digital banking response systems
/// - **Processing Requirements**: Supports both automated and manual response generation workflows
/// - **Compliance Integration**: Built-in validation for regulatory response requirements and documentation
///
/// ## Relationship to Other Messages
/// - **Triggers**: Directly triggered by MT192 cancellation requests and various payment inquiry messages
/// - **Responses**: Provides definitive responses to inquiries, completing request-response workflows
/// - **Related**: Works with payment messages, customer service systems, and audit platforms
/// - **Alternatives**: Direct system notifications for internal processing status updates
/// - **Status Updates**: Final response message in inquiry and request resolution lifecycle
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT196_VALIDATION_RULES)]
pub struct MT196 {
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

/// Enhanced validation rules for MT196
const MT196_VALIDATION_RULES: &str = r#"{
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
                {"!!": {"var": "field_23b"}},
                {"!!": {"var": "field_32a"}},
                {"!!": {"var": "field_50"}},
                {"!!": {"var": "field_59"}},
                {"!!": {"var": "field_71a"}}
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
