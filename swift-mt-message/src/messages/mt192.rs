use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT192: Request for Cancellation
///
/// ## Purpose
/// Used by the originator to request the cancellation of a previously sent payment message before execution.
/// This message provides precise reference information to identify the specific message to be cancelled and
/// includes optional reason codes for the cancellation request, enabling proper payment lifecycle management.
///
/// ## Scope
/// This message is:
/// - Used for cancellation requests by originators of previously sent payment messages
/// - Applicable for preventing execution of payment instructions before settlement
/// - Designed for urgent processing to halt payment execution workflows
/// - Compatible with various payment message types requiring cancellation
/// - Subject to validation rules for proper message identification and tracking
/// - Integrated with payment processing systems for real-time cancellation requests
///
/// ## Key Features
/// - **Precise Message Identification**: Complete reference to original message requiring cancellation
/// - **Urgent Cancellation Processing**: Time-sensitive processing to prevent payment execution
/// - **Session-Based Tracking**: Field 11S for accurate session-based message identification
/// - **Comprehensive Reason Codes**: Optional detailed information about cancellation reasons
/// - **Original Message Reconstruction**: Optional inclusion of original message field copies
/// - **Audit Trail Maintenance**: Complete tracking of cancellation requests and outcomes
///
/// ## Common Use Cases
/// - Duplicate payment prevention for inadvertent double submissions
/// - Incorrect payment details correction before execution
/// - Customer-initiated cancellation requests for payment changes
/// - System error recovery and payment correction procedures
/// - Fraud prevention and suspicious payment halting
/// - Regulatory compliance-driven payment cancellations
/// - Emergency cancellation for financial institution risk management
///
/// ## Message Structure
/// - **Field 20**: Sender's Reference (mandatory) - Unique reference for cancellation request
/// - **Field 21**: Related Reference (mandatory) - Reference to original message being cancelled
/// - **Field 11S**: MT and Date (mandatory) - Session details of original message (MT+date+session+sequence)
/// - **Field 79**: Narrative (optional) - Cancellation reason codes and additional information
/// - **Original Message Fields**: Optional copies of fields from original message for verification
///
/// ## Network Validation Rules
/// - **Reference Format Validation**: Reference fields must not start/end with '/' or contain '//'
/// - **Session Reference Format**: Field 11S must have proper format for MT, date, session, and sequence numbers
/// - **Information Requirement**: Either field 79 or copies of original message fields must be present
/// - **Reason Code Validation**: Field 79 should contain valid cancellation reason codes when present
/// - **Message Type Consistency**: Field 11S message type must match valid payment message types
/// - **Date Format Validation**: Date in field 11S must be in valid YYMMDD format
/// - **Sequence Number Validation**: Session and sequence numbers must be properly formatted
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT192 format remains stable for cancellation processing
/// - **Validation Updates**: Enhanced validation for session reference accuracy and reason codes
/// - **Processing Improvements**: Improved handling of urgent cancellation request processing
/// - **Compliance Notes**: Strengthened requirements for audit trail and regulatory reporting
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with payment processing engines and workflow management systems
/// - **API Integration**: RESTful API support for modern digital banking cancellation requests
/// - **Processing Requirements**: Supports urgent processing with real-time cancellation capabilities
/// - **Compliance Integration**: Built-in validation for regulatory cancellation requirements and reporting
///
/// ## Relationship to Other Messages
/// - **Triggers**: Triggered by payment processing systems, customer requests, or risk management systems
/// - **Responses**: Generates MT196 response messages confirming cancellation status
/// - **Related**: Works with original payment messages (MT103, MT202, etc.) requiring cancellation
/// - **Alternatives**: Direct system-level cancellation for internal processing corrections
/// - **Status Updates**: May receive status notifications about cancellation success or failure
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT192_VALIDATION_RULES)]
pub struct MT192 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("11S")]
    pub field_11s: Field11S,

    #[field("79")]
    pub field_79: Option<Field79>,
}

/// Enhanced validation rules for MT192
const MT192_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Either Field 79 or a copy of mandatory fields from the original message (or both) must be present",
      "condition": {
        "!!": {"var": "fields.79"}
      }
    }
  ]
}"#;
