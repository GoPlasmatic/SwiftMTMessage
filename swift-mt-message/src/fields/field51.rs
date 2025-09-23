use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

///   **Field 51A: Sending Institution**
///
/// ## Purpose
/// Identifies the Sender of the message, primarily used in FileAct messages
/// and specialized transaction contexts. This field provides explicit identification
/// of the message originator when additional clarity is required beyond the
/// message header information. Essential for proper message routing and accountability.
///
/// ## Format
/// - **Swift Format**: `[/1!a][/34x]4!a2!a2!c[3!c]`
/// - **Party Identifier**: `[/1!a][/34x]` - Optional clearing system identifier
/// - **BIC Component**: `4!a2!a2!c[3!c]` - Bank Identifier Code (8 or 11 characters)
/// - **Usage Context**: FileAct messages and institutional identification
///
/// ## Presence
/// - **Status**: Optional in most contexts, conditional for FileAct messages
/// - **Swift Error Codes**: T27/T28/T29 (invalid BIC), T45 (unregistered BIC), D63 (invalid context)
/// - **Usage Context**: Message sender identification and institutional routing
///
/// ## Usage Rules
/// - **FileAct Context**: Mandatory for FileAct message identification
/// - **BIC Validation**: Must be registered financial institution BIC
/// - **Originator Match**: First 8 characters must match message originator
/// - **Reference Coordination**: Works with Field 20 for transaction reference
///
/// ## Network Validation Rules
/// - **BIC Registration**: Must be valid and registered financial institution
/// - **Format Compliance**: Exact adherence to BIC format requirements
/// - **Originator Consistency**: BIC prefix must match sending institution
/// - **Context Validation**: Only valid in appropriate message types
/// - **Character Set**: Alphanumeric characters only in specified positions
///
/// ## Business Context
/// - **Message Identification**: Explicit sender identification for complex routing
/// - **FileAct Operations**: Essential component of file transfer messages
/// - **Institutional Clarity**: Removes ambiguity in multi-party transactions
/// - **Audit Trail**: Provides clear originator identification for compliance
///
/// ## Party Identifier Formats
/// - **Clearing System Codes**: Single character codes for domestic systems
/// - **Account References**: Up to 34 character institutional identifiers
/// - **Optional Usage**: May be omitted when not required for routing
/// - **System Specific**: Aligned with local clearing system requirements
///
/// ## Examples
/// ```logic
/// :51A:DEUTDEFFXXX        // Deutsche Bank Frankfurt (basic BIC)
/// :51A:/DCHAPSFFXXX       // With clearing system identifier
/// :51A:/12345678MIDLGB22  // With account reference and BIC
/// ```
///
/// ## FileAct Message Integration
/// - **Message Routing**: Critical for FileAct message delivery
/// - **Security Context**: Supports authentication and authorization
/// - **Service Integration**: Enables proper service endpoint identification
/// - **Error Handling**: Facilitates proper error message routing
///
/// ## Clearing System Codes
/// - **Domestic Systems**: Single character codes for national clearing
/// - **International Routing**: Multi-character codes for cross-border
/// - **Bilateral Agreements**: Custom codes for specific institution pairs
/// - **Regional Networks**: Codes for regional payment systems
///
/// ## Regional Considerations
/// - **European Networks**: TARGET2 and SEPA routing requirements
/// - **US Systems**: Fedwire and ACH routing considerations
/// - **Asian Markets**: Local clearing system integration requirements
/// - **Cross-Border**: International correspondent banking arrangements
///
/// ## Error Prevention
/// - **BIC Validation**: Verify BIC is registered and reachable
/// - **Context Checking**: Ensure appropriate message type usage
/// - **Originator Matching**: Confirm BIC alignment with sender
/// - **Format Verification**: Validate exact format compliance
///
/// ## Related Fields
/// - **Field 20**: Transaction Reference (coordination with sender ID)
/// - **Field 52A**: Ordering Institution (institution hierarchy)
/// - **Message Header**: Basic Application Header (sender information)
/// - **Field 53A**: Sender's Correspondent (relationship definition)
///
/// ## Institutional Hierarchy
/// - **Primary Sender**: Main institution originating message
/// - **Department/Branch**: Specific department within institution
/// - **Service Provider**: Third-party service acting for institution
/// - **Correspondent Network**: Institution acting through correspondent
///
/// ## STP Processing
/// - **Automated Routing**: System-driven message routing based on BIC
/// - **Validation Enhancement**: Real-time BIC validation and verification
/// - **Error Reduction**: Automated detection of routing inconsistencies
/// - **Processing Efficiency**: Streamlined handling of institutional identification
///
/// ## Compliance Framework
/// - **Regulatory Identification**: Clear sender identification for compliance
/// - **Audit Documentation**: Complete institutional identification trail
/// - **Risk Management**: Proper sender identification for risk assessment
/// - **Investigation Support**: Clear originator details for compliance reviews
///
/// ## FileAct Specific Applications
/// - **File Transfer Security**: Sender authentication for file operations
/// - **Service Discovery**: Proper routing to file handling services
/// - **Message Correlation**: Linking file operations to sending institution
/// - **Error Resolution**: Proper routing of file operation errors
///
/// ## See Also
/// - Swift FIN User Handbook: Sending Institution Specifications
/// - FileAct Message Standards: Sender Identification Requirements
/// - BIC Directory: Valid Financial Institution Codes
/// - Message Routing Guidelines: Institutional Identification Standards

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field51A {
    /// Optional party identifier for clearing system or account reference
    ///
    /// Format: [/1!a][/34x] - Single character code + up to 34 character identifier
    /// Used for domestic clearing systems and institutional account references
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Bank Identifier Code of the sending institution
    ///
    /// Format: 4!a2!a2!c\[3!c\] - 8 or 11 character BIC code
    /// Must be registered financial institution BIC matching message originator
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}
