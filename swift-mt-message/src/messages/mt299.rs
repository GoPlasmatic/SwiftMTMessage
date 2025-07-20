use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT299: Free Format Message (Category 2 - Financial Institution Transfers)
///
/// ## Purpose
/// Used for free format communication between financial institutions regarding institutional
/// transfers and treasury operations. Provides a flexible structure for various types of
/// communication that don't fit into other structured Category 2 message formats.
///
/// ## Scope
/// This message is:
/// - Sent between financial institutions for treasury and institutional transfer communications
/// - Used for inquiries, clarifications, and general information exchange about Category 2 operations
/// - Contains free format narrative text for flexible institutional communication
/// - Supports various treasury scenarios requiring custom messaging
/// - Essential for operational communications in correspondent banking
///
/// ## Key Features
/// - **Free Format Content**: Field 79 for flexible narrative text
/// - **Treasury Context**: Related to institutional transfers and treasury operations
/// - **Bilateral Communication**: Facilitates bank-to-bank operational communication
/// - **Reference Tracking**: Links to related institutional transfer messages or operations
/// - **Flexible Structure**: Minimal mandatory fields for maximum operational flexibility
/// - **Operational Focus**: Designed for treasury and institutional transfer operations
///
/// ## Common Use Cases
/// - Treasury operation inquiry messages
/// - Institutional transfer status update communications
/// - Settlement clarification requests
/// - Liquidity management communications
/// - Special instruction messages for institutional transfers
/// - Problem resolution communications for Category 2 operations
/// - Correspondent banking operational messages
/// - Cross-border institutional transfer clarifications
/// - Central bank communication messages
/// - Settlement system operational updates
///
/// ## Field Structure
/// - **20**: Sender's Reference (mandatory) - Message reference for this communication
/// - **21**: Related Reference (optional) - Reference to related message/transaction/operation
/// - **79**: Narrative (mandatory) - Free format text content for institutional communication
///
/// ## Treasury and Institutional Applications
/// ### Settlement Communications
/// - Real-time gross settlement (RTGS) operational messages
/// - Net settlement system communications
/// - Central bank settlement clarifications
/// - Cross-currency settlement coordination
///
/// ### Correspondent Banking
/// - Nostro/vostro account operational messages
/// - Credit line communications
/// - Account reconciliation inquiries
/// - Liquidity management discussions
///
/// ### Treasury Operations
/// - Foreign exchange settlement communications
/// - Money market operation messages
/// - Institutional funding communications
/// - Treasury position management messages
///
/// ## Network Validation Rules
/// - **Narrative Format**: If narrative starts with /REJT/ or /RETN/, must follow Payments Reject/Return Guidelines
/// - **Reference Format**: Transaction references must follow SWIFT formatting standards
/// - **Content Guidelines**: Free format content should be relevant to institutional transfers
/// - **Professional Communication**: Content should maintain institutional communication standards
///
/// ## Special Format Handling
/// ### Reject/Return Messages
/// When the narrative begins with /REJT/ or /RETN/, the message should follow specific guidelines:
/// - Structured format for payment exception handling
/// - Clear indication of reject/return reasons
/// - Reference to original institutional transfer
/// - Appropriate reason codes where applicable
///
/// ### Operational Messages
/// - Settlement system status updates
/// - Technical issue communications
/// - Operational procedure clarifications
/// - Emergency communication protocols
///
/// ## Processing Considerations
/// - **Operational Priority**: Often used for time-sensitive institutional communications
/// - **Clarity Required**: Free format content should be clear and unambiguous
/// - **Follow-up**: May require structured message responses
/// - **Audit Trail**: Maintains record of institutional operational communications
/// - **Escalation Path**: Part of operational issue resolution procedures
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT299 format remains unchanged in SRG2025
/// - **Validation Updates**: Enhanced guidelines for institutional transfer communications
/// - **Processing Improvements**: Better integration with modern settlement systems
/// - **Compliance Notes**: Enhanced support for operational communication workflows
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with treasury management and settlement communication systems
/// - **API Integration**: RESTful API support for modern institutional banking communication platforms
/// - **Processing Requirements**: Supports both automated and manual message generation for operational needs
/// - **Compliance Integration**: Built-in validation for regulatory institutional communication requirements
///
/// ## Relationship to Other Messages
/// - **Triggers**: Triggered by operational needs related to Category 2 institutional transfers
/// - **Responses**: May generate response messages or trigger follow-up operational actions
/// - **Related**: Works with all Category 2 messages (MT200-series) for supporting communication
/// - **Alternatives**: Structured messages for specific scenarios with defined communication formats
/// - **Status Updates**: Provides flexible operational updates for institutional transfer processes

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
