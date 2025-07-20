use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// **Field 11: MT Reference**
///
/// ## Purpose
/// Identifies the original message being referenced in acknowledgment, cancellation, or status inquiry messages.
/// This field establishes critical traceability links between original transactions and their related responses,
/// enabling proper transaction lifecycle management and exception handling in the SWIFT network.
///
/// ## Options Overview
/// - **Option R**: Used in certain acknowledgment and response contexts
/// - **Option S**: Used in cancellation requests and status inquiry messages
///
/// ## Usage by Message Type
/// - **MT110**: Advice of Cheque(s) - references original payment instruction
/// - **MT111**: Request for Stop Payment - references cheque to be stopped
/// - **MT112**: Status of Stop Payment Request - references original stop request
/// - **MT192**: Request for Cancellation (Customer) - references message to cancel
/// - **MT196**: Client Side Query - references message being queried
/// - **MT292**: Request for Cancellation (Treasury) - references treasury message to cancel
/// - **MT296**: Answer to Client Side Query - references original query
///
/// ## Network Validation Rules
/// - **Message Type**: Must be valid 3-digit SWIFT message type
/// - **Date Validation**: Must be valid calendar date in YYMMDD format
/// - **Sequence Validation**: Session and sequence numbers must be numeric when present
/// - **Reference Integrity**: Referenced message should exist and be accessible
///
/// ## Format Specifications per Option
/// Both options follow identical format patterns but are used in different message contexts.

/// **Field 11R: MT Reference (Response Context)**
///
/// ## Purpose
/// Used in acknowledgment and response messages to reference the original message.
/// Commonly found in confirmation and status response messages.
///
/// ## Format
/// - **Swift Format**: `3!n6!n[4!n][6!n]`
/// - **Components**:
///   - `3!n`: Message type of original message (103, 202, etc.)
///   - `6!n`: Date of original message (YYMMDD format)
///   - `[4!n]`: Optional session number (4 digits)
///   - `[6!n]`: Optional Input Sequence Number (6 digits)
///
/// ## Usage Rules
/// - **Message Type**: Must match the type of the original message being referenced
/// - **Date Accuracy**: Must be the exact date from the original message header
/// - **Session Context**: Include session number for proper message identification
/// - **Sequence Tracing**: Sequence number enables precise message location in logs
///
/// ## Examples
/// ```logic
/// :11R:103250125          // MT103 dated January 25, 2025
/// :11R:2022512251234      // MT202 dated December 25, 2025, session 1234
/// :11R:205250125123456789 // MT205 with full session and sequence
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field11R {
    /// Message type of the original message being referenced
    ///
    /// Format: 3!n - Exactly 3 numeric digits
    /// Examples: "103", "202", "205", "940"
    #[component("3!n")]
    pub message_type: String,

    /// Date of the original message
    ///
    /// Format: 6!n (YYMMDD) - Must be valid calendar date
    /// Used to locate the original message in daily processing
    #[component("6!n")]
    pub date: NaiveDate,

    /// Session number of the original message
    ///
    /// Format: \[4!n\] - Optional 4-digit session identifier
    /// Helps identify message within specific processing session
    #[component("[4!n]")]
    pub session_number: Option<String>,

    /// Input Sequence Number of the original message
    ///
    /// Format: \[6!n\] - Optional 6-digit sequence identifier
    /// Provides precise message location within session logs
    #[component("[6!n]")]
    pub input_sequence_number: Option<String>,
}

/// **Field 11S: MT Reference (Status/Cancellation Context)**
///
/// ## Purpose
/// Used in cancellation requests and status inquiry messages to identify the specific
/// message being cancelled or queried. Critical for transaction control and exception handling.
///
/// ## Format
/// - **Swift Format**: `3!n6!n[4!n][6!n]`
/// - **Components**: Identical to Field 11R
///
/// ## Usage Rules
/// - **Cancellation Timing**: Original message must be in cancellable state
/// - **Authority Validation**: Sender must have authority to cancel referenced message
/// - **Status Inquiry**: Referenced message must be accessible for status reporting
/// - **Duplicate Prevention**: Avoid multiple cancellation attempts for same message
///
/// ## Cancellation Windows
/// - **Customer Payments (MT192)**: Typically cancellable before beneficiary credit
/// - **Treasury Messages (MT292)**: Cancellation windows vary by market practice
/// - **Stop Payment (MT111)**: Must reference valid cheque payment instruction
///
/// ## Examples
/// ```logic
/// :11S:103250125          // Cancel MT103 from January 25, 2025
/// :11S:1922512251234      // Cancel with session context
/// :11S:202250125123456789 // Full reference with sequence number
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field11S {
    /// Message type of the original message being referenced
    ///
    /// Format: 3!n - Must be valid SWIFT message type
    /// For MT292: Valid types include 200, 202, 205, 210, 256, 299
    #[component("3!n")]
    pub message_type: String,

    /// Date of the original message
    ///
    /// Format: 6!n (YYMMDD) - Must match original message date exactly
    /// Used for precise message identification in cancellation requests
    #[component("6!n")]
    pub date: NaiveDate,

    /// Session number of the original message
    ///
    /// Format: \[4!n\] - Optional but recommended for accurate targeting
    /// Essential in high-volume processing environments
    #[component("[4!n]")]
    pub session_number: Option<String>,

    /// Input Sequence Number of the original message
    ///
    /// Format: \[6!n\] - Provides highest precision for message identification
    /// Critical for avoiding cancellation of wrong messages
    #[component("[6!n]")]
    pub input_sequence_number: Option<String>,
}

/// **Field 11: MT Reference (Multi-Option)**
///
/// ## Purpose
/// Provides flexible message referencing capability for various acknowledgment, cancellation,
/// and status inquiry scenarios. The option used depends on the specific message type and context.
///
/// ## Option Selection Guidelines
/// - **Option R**: Use in acknowledgment and response messages
/// - **Option S**: Use in cancellation requests and status inquiries
///
/// ## Message Traceability
/// Field 11 enables critical SWIFT network functions:
/// - **Transaction Lifecycle Tracking**: Link all related messages in payment chain
/// - **Exception Handling**: Identify problematic transactions for investigation
/// - **Regulatory Compliance**: Maintain audit trails for regulatory reporting
/// - **Customer Service**: Provide status updates and handle customer inquiries
///
/// ## Network Benefits
/// - **Operational Efficiency**: Automated processing of acknowledgments and cancellations
/// - **Risk Management**: Proper identification prevents erroneous operations
/// - **Dispute Resolution**: Clear audit trail for transaction disputes
/// - **System Integration**: Enables straight-through processing for exception handling
///
/// ## Best Practices
/// - **Complete References**: Include session and sequence numbers when available
/// - **Timely Processing**: Process cancellation requests promptly within business windows
/// - **Validation**: Verify referenced message exists and is in appropriate state
/// - **Documentation**: Maintain clear records of all reference relationships
///
/// ## Error Prevention
/// - **Verify Message Type**: Ensure referenced message type is correct
/// - **Validate Date**: Confirm date matches original message exactly
/// - **Check Authority**: Verify sender has rights to reference/cancel message
/// - **Avoid Duplicates**: Prevent multiple references to same original message
///
/// ## Related Fields
/// - **Field 20**: Sender's Reference (original message identifier)
/// - **Field 21**: Related Reference (cover payment relationships)
/// - **Block 1**: Basic Header (contains actual session and sequence numbers)
/// - **Block 3**: User Header (additional reference information)
///
/// ## See Also
/// - Swift FIN User Handbook: Message Reference Standards
/// - Cancellation Guidelines: Timing and Authority Requirements
/// - Exception Handling Guide: Proper Reference Usage
/// - Network Rules: Message Identification Standards
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field11 {
    /// Option R: Used in acknowledgment and response contexts
    /// Common in confirmation messages and status responses
    R(Field11R),

    /// Option S: Used in cancellation and status inquiry contexts
    /// Essential for transaction control and exception handling
    S(Field11S),
}
