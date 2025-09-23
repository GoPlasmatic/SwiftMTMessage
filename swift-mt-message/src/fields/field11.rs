//! # Field 11: MT Reference
//!
//! ## Purpose
//! Identifies the original message being referenced in acknowledgment, cancellation, or status inquiry messages.
//! This field establishes critical traceability links between original transactions and their related responses,
//! enabling proper transaction lifecycle management and exception handling in the SWIFT network.
//!
//! ## Options Overview
//! - **Option R**: Used in acknowledgment and response contexts
//! - **Option S**: Used in cancellation requests and status inquiry messages
//!
//! ## Format Specification
//! Both options follow identical format: `3!n6!n[4!n][6!n]`
//! - **Message Type**: 3 numeric digits (103, 202, etc.)
//! - **Date**: 6 numeric digits (YYMMDD format)
//! - **Session Number**: Optional 4 numeric digits
//! - **Input Sequence Number**: Optional 6 numeric digits
//!
//! ## Usage by Message Type
//! - **MT110**: Advice of Cheque(s) - references original payment instruction
//! - **MT111**: Request for Stop Payment - references cheque to be stopped
//! - **MT112**: Status of Stop Payment Request - references original stop request
//! - **MT192**: Request for Cancellation (Customer) - references message to cancel
//! - **MT196**: Client Side Query - references message being queried
//! - **MT292**: Request for Cancellation (Treasury) - references treasury message to cancel
//! - **MT296**: Answer to Client Side Query - references original query
//!
//! ## Option Selection Guidelines
//! ### When to Use Option R (Response Context)
//! - **Acknowledgments**: Confirming receipt of original message
//! - **Status Responses**: Providing status updates on original transactions
//! - **Confirmation Messages**: Confirming execution of original instructions
//! - **Response Messages**: Any response to an original message
//!
//! ### When to Use Option S (Status/Cancellation Context)
//! - **Cancellation Requests**: Requesting cancellation of original message
//! - **Status Inquiries**: Inquiring about status of original message
//! - **Stop Payment Requests**: Stopping cheque or payment instructions
//! - **Transaction Control**: Any control operation on original message
//!
//! ## Network Validation Rules
//! - **Message Type**: Must be valid 3-digit SWIFT message type
//! - **Date Validation**: Must be valid calendar date in YYMMDD format
//! - **Sequence Validation**: Session and sequence numbers must be numeric when present
//! - **Reference Integrity**: Referenced message should exist and be accessible
//! - **Authority Validation**: Sender must have authority to reference/cancel message
//!
//! ## Message Traceability Benefits
//! ### Transaction Lifecycle Tracking
//! - **Payment Chain**: Link all related messages in payment processing
//! - **Exception Handling**: Identify problematic transactions for investigation
//! - **Audit Trails**: Maintain complete transaction history for compliance
//! - **Customer Service**: Provide accurate status updates to customers
//!
//! ### Operational Efficiency
//! - **Automated Processing**: Enable STP for acknowledgments and cancellations
//! - **Risk Management**: Proper identification prevents erroneous operations
//! - **Dispute Resolution**: Clear audit trail for transaction disputes
//! - **System Integration**: Support end-to-end transaction processing
//!
//! ## Cancellation Windows and Timing
//! - **Customer Payments (MT192)**: Typically cancellable before beneficiary credit
//! - **Treasury Messages (MT292)**: Cancellation windows vary by market practice
//! - **Stop Payment (MT111)**: Must reference valid cheque payment instruction
//! - **Real-time Processing**: Immediate processing reduces cancellation windows
//!
//! ## Best Practices
//! - **Complete References**: Include session and sequence numbers when available
//! - **Timely Processing**: Process cancellation requests promptly within business windows
//! - **Validation**: Verify referenced message exists and is in appropriate state
//! - **Documentation**: Maintain clear records of all reference relationships
//!
//! ## Error Prevention Guidelines
//! - **Verify Message Type**: Ensure referenced message type is correct
//! - **Validate Date**: Confirm date matches original message exactly
//! - **Check Authority**: Verify sender has rights to reference/cancel message
//! - **Avoid Duplicates**: Prevent multiple references to same original message
//! - **Sequence Accuracy**: Use precise sequence numbers to avoid wrong message targeting
//!
//! ## Related Fields Integration
//! - **Field 20**: Sender's Reference (original message identifier)
//! - **Field 21**: Related Reference (cover payment relationships)
//! - **Block 1**: Basic Header (contains actual session and sequence numbers)
//! - **Block 3**: User Header (additional reference information)
//!
//! ## Compliance and Regulatory Aspects
//! - **Audit Requirements**: Maintain complete reference chains for audit purposes
//! - **Regulatory Reporting**: Support regulatory transaction reporting requirements
//! - **Data Retention**: Preserve reference relationships for required retention periods
//! - **Investigation Support**: Enable transaction investigation and dispute resolution
//!
//! ## See Also
//! - Swift FIN User Handbook: Message Reference Standards
//! - Cancellation Guidelines: Timing and Authority Requirements
//! - Exception Handling Guide: Proper Reference Usage
//! - Network Rules: Message Identification Standards

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

/// **Field 11R: MT Reference (Option R)**
///
/// Response context variant of [Field 11 module](index.html). Used in acknowledgment
/// and response messages to reference the original message.
///
/// **Components:**
/// - Message type (3!n)
/// - Date (6!n, YYMMDD format)
/// - Session number (optional, \[4!n\])
/// - Input sequence number (optional, \[6!n\])
///
/// For complete documentation, see the [Field 11 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
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

/// **Field 11S: MT Reference (Option S)**
///
/// Status/Cancellation context variant of [Field 11 module](index.html). Used in cancellation
/// requests and status inquiry messages for transaction control.
///
/// **Components:**
/// - Message type (3!n)
/// - Date (6!n, YYMMDD format)
/// - Session number (optional, \[4!n\])
/// - Input sequence number (optional, \[6!n\])
///
/// For complete documentation, see the [Field 11 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
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

/// **Field 11 Enum: MT Reference Options**
///
/// Enum wrapper for [Field 11 module](index.html) variants providing message referencing
/// capability for acknowledgment, cancellation, and status inquiry scenarios.
///
/// For complete documentation, see the [Field 11 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field11 {
    /// Option R: Used in acknowledgment and response contexts
    /// Common in confirmation messages and status responses
    R(Field11R),

    /// Option S: Used in cancellation and status inquiry contexts
    /// Essential for transaction control and exception handling
    S(Field11S),
}
