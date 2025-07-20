use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

///   **Field 20: Sender's Reference**
///
/// ## Purpose
/// Specifies the reference assigned by the Sender to unambiguously identify the message.
/// This reference must be unique within the sender's system and is used to link related
/// messages and confirmations throughout the payment chain.
///
/// ## Format
/// - **Swift Format**: `16x`
/// - **Description**: Up to 16 alphanumeric characters
/// - **Pattern**: May include letters, digits, and limited special characters
/// - **Restrictions**: Must not start or end with `/` and must not contain `//`
///
/// ## Presence
/// - **Status**: Mandatory in all MT messages containing this field
/// - **Swift Error Codes**: T26 (invalid format), T13 (field too long)
/// - **Referenced in Rules**: Network validation rules across multiple message types
///
/// ## Network Validation Rules
/// - **Format Validation**: Must conform to 16x pattern (alphanumeric, max 16 chars)
/// - **Content Validation**: Cannot start/end with `/` or contain consecutive slashes `//`
/// - **Uniqueness**: Should be unique within sender's daily operations for audit purposes
///
/// ## Usage Rules
/// - **Confirmations**: Must be quoted unchanged in related MT900/MT910/MT950 messages
/// - **Cover Payments**: When using cover method, copy to field 21 of associated MT202 COV
/// - **Audit Trail**: Used by institutions to track payment lifecycle and exceptions
/// - **Reference Format**: Common patterns include transaction IDs, invoice numbers, or internal references
///
/// ## Examples
/// ```logic
/// :20:PAYMENT123456
/// :20:INV2024001234
/// :20:TXN20240719001
/// :20:URGPAY240719
/// ```
///
/// ## Related Fields
/// - **Field 21**: Transaction Reference Number (often contains Field 20 value in cover payments)
/// - **Field 61**: Statement Line Reference (in account statements)
/// - **Block 3 {108}**: Message User Reference (system-level tracking)
///
/// ## STP Compliance
/// Field 20 has no specific STP restrictions but must meet standard format requirements.
/// STP processing relies on this field for automated matching and exception handling.
///
/// ## See Also
/// - Swift FIN User Handbook: Message Structure and Field Specifications
/// - MT103 Specification: Customer Credit Transfer requirements
/// - Cover Payment Guidelines: Field 20/21 relationship rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field20 {
    /// The sender's reference string (max 16 characters)
    ///
    /// Format: 16x - Up to 16 alphanumeric characters
    /// Validation: No leading/trailing slashes, no consecutive slashes
    #[component("16x")]
    pub reference: String,
}
