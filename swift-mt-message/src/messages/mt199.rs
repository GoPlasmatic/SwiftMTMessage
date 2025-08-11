use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT199: Free Format Message
///
/// ## Purpose
/// Used for free format communication between financial institutions regarding customer payments and related
/// matters. This message provides the maximum flexibility for various types of communication that don't fit into
/// other structured message formats, enabling efficient bilateral communication in the payment ecosystem.
///
/// ## Scope
/// This message is:
/// - Used for customer payment-related communications between financial institutions
/// - Applicable for inquiries, clarifications, and general information exchange
/// - Designed for flexible narrative text supporting various communication needs
/// - Compatible with both automated and manual message generation systems
/// - Subject to minimal validation rules for maximum communication flexibility
/// - Integrated with customer service and payment processing communication workflows
///
/// ## Key Features
/// - **Maximum Communication Flexibility**: Field 79 for completely free format narrative text
/// - **Payment Context Integration**: Specifically related to customer payments and transactions
/// - **Bilateral Communication Support**: Facilitates direct bank-to-bank communication
/// - **Reference Tracking System**: Links to related payment messages or transactions
/// - **Minimal Structure Requirements**: Minimal mandatory fields for maximum messaging flexibility
/// - **Multi-Purpose Usage**: Adaptable to various communication scenarios and business needs
///
/// ## Common Use Cases
/// - Payment inquiry messages for transaction status and details
/// - Status update communications for processing milestones
/// - Clarification requests for payment instructions and requirements
/// - Special instruction messages for unique processing needs
/// - Problem resolution communications for payment issues
/// - Customer service related messages for account and payment support
/// - Reject and return notifications with detailed explanatory information
///
/// ## Message Structure
/// - **Field 20**: Sender's Reference (mandatory) - Unique message reference identifier
/// - **Field 21**: Related Reference (optional) - Reference to related message or transaction
/// - **Field 79**: Narrative (mandatory) - Free format text content for communication
///
/// ## Network Validation Rules
/// - **Reference Format Validation**: Sender's reference must follow standard SWIFT reference format rules
/// - **Narrative Content Requirements**: Field 79 must contain meaningful communication content
/// - **Related Reference Format**: Field 21 must follow proper reference format when present
/// - **Reject/Return Guidelines**: If narrative starts with /REJT/ or /RETN/, must follow Payments Guidelines
/// - **Content Length Validation**: Narrative content must be within specified field length limits
/// - **Character Set Compliance**: All text content must use valid SWIFT character sets
/// - **Mandatory Field Validation**: All mandatory fields must be present with valid content
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT199 format remains stable for free format communication
/// - **Validation Updates**: Enhanced validation for reject/return guidelines compliance
/// - **Processing Improvements**: Improved handling of structured reject and return notifications
/// - **Compliance Notes**: Maintained flexibility while ensuring compliance with payment guidelines
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with customer service systems and payment communication platforms
/// - **API Integration**: RESTful API support for modern digital banking communication systems
/// - **Processing Requirements**: Supports both automated and manual message generation and processing
/// - **Compliance Integration**: Built-in validation for regulatory communication requirements
///
/// ## Relationship to Other Messages
/// - **Triggers**: Triggered by various payment scenarios requiring flexible communication
/// - **Responses**: May generate response messages or trigger follow-up communications
/// - **Related**: Works with all payment message types for supporting communication
/// - **Alternatives**: Structured messages for specific communication scenarios with defined formats
/// - **Status Updates**: Provides flexible status updates and notifications for payment processes
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT199 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Option<Field21NoOption>,

    #[field("79")]
    pub field_79: Field79,
}
