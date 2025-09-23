use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

///   **Field 72: Sender to Receiver Information**
///
/// ## Purpose
/// Specifies additional information for the Receiver or other specified party in
/// financial messages. This field provides structured communication between financial
/// institutions, enabling additional instructions, clarifications, and institutional
/// coordination that supplements the main transaction details.
///
/// ## Format Specification
/// - **Swift Format**: `6*35x`
/// - **Structure**: Up to 6 lines of 35 characters each
/// - **Content**: Structured narrative format with specific codes
/// - **Line Format**: `/8c/[additional information]` (Code)(Narrative)
///
/// ## Business Context Applications
/// - **Institutional Communication**: Additional instructions between banks
/// - **Processing Instructions**: Specific handling requirements
/// - **Regulatory Information**: Compliance-related communications
/// - **Operational Coordination**: Coordination between correspondent banks
///
/// ## Network Validation Requirements
/// - **Line Structure**: Each code must be between slashes at line beginning
/// - **Continuation**: Continuation text starts with '//'  
/// - **Prohibited Codes**: /REJT/ and /RETN/ codes not allowed (Error T81)
/// - **ERI Exclusion**: Must not include ERI (Error T82)
/// - **Character Set**: Must use valid SWIFT character set
///
/// ## Structured Code Requirements
/// ### Mandatory Code Format
/// - **Line 1**: `/8c/[additional information]` - Code followed by narrative
/// - **Lines 2-6**: Continuation with '//' or new codes
/// - **Code Uniqueness**: Each code should appear only once
/// - **Format Compliance**: Exact adherence to code structure required
///
/// ### Primary Code: INS (Instructing Institution)
/// - **Purpose**: Identifies instructing institution
/// - **Format**: /INS/[BIC code]
/// - **Validation**: Must be followed by valid BIC
/// - **Uniqueness**: Must be unique within message
/// - **Usage**: Critical for institutional identification
///
/// ## Regional Considerations
/// - **European Networks**: SEPA and TARGET2 institutional communications
/// - **US Systems**: Federal Reserve and commercial bank coordination
/// - **Asian Markets**: Regional institutional communication requirements
/// - **Cross-Border**: International institutional coordination
///
/// ## Error Prevention Guidelines
/// - **Code Validation**: Verify all codes are properly formatted
/// - **BIC Verification**: Confirm BIC codes are valid and registered
/// - **Continuation Format**: Ensure continuation lines use '//' prefix
/// - **Prohibited Content**: Avoid prohibited codes and content
///
/// ## Related Fields Integration
/// - **Field 53A**: Sender's Correspondent (institutional relationships)
/// - **Field 54A**: Receiver's Correspondent (receiving institutions)
/// - **Field 70**: Remittance Information (payment details)
/// - **Field 77A**: Narrative (extended narrative information)
///
/// ## Compliance Framework
/// - **Regulatory Communication**: Institutional regulatory information exchange
/// - **Audit Documentation**: Complete institutional communication trail
/// - **Risk Management**: Institutional risk communication
/// - **Operational Compliance**: Processing instruction compliance
///
/// ## See Also
/// - Swift FIN User Handbook: Sender to Receiver Information Specifications
/// - Institutional Communication: Banking Institution Coordination
/// - Processing Instructions: Financial Institution Guidelines
/// - Regulatory Communication: Banking Regulatory Requirements

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field72 {
    /// Sender to receiver information
    ///
    /// Format: 6*35x - Up to 6 lines of 35 characters each
    /// Contains structured institutional communications with codes and narrative
    /// Line 1: /8c/[additional information], subsequent lines: continuation or new codes
    #[component("6*35x")]
    pub information: Vec<String>,
}
