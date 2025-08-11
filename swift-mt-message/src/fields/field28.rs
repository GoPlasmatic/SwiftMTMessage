use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

///   **Field 28: Statement Number / Sequence Number / Message Index**
///
/// ## Purpose
/// Provides statement numbering, sequence identification, and message indexing capabilities
/// for financial statements, batch operations, and multi-part message communication.
/// This field enables proper ordering, continuation tracking, and completeness verification
/// of related messages or statement sequences.
///
/// ## Format Variants
/// - **Field 28**: `5n[/2n]` - Statement number with optional sequence
/// - **Field 28C**: `5n[/5n]` - Statement number with extended sequence
/// - **Field 28D**: `5n/5n` - Message index and total count
///
/// ## Presence
/// - **Status**: Mandatory in statement messages and batch operations
/// - **Swift Error Codes**: T40 (invalid number), T51 (format violation)
/// - **Usage Context**: Statement numbering and message sequencing
///
/// ## Usage Rules
/// - **Sequential Numbering**: Numbers must follow logical sequence for statements
/// - **Index Validation**: Message index must not exceed total count in Field 28D
/// - **Completeness**: Enables verification that all messages/statements received
/// - **Ordering**: Facilitates proper chronological ordering of related messages
///
/// ## Network Validation Rules
/// - **Positive Numbers**: All numeric values must be greater than zero
/// - **Range Validation**: Numbers must be within reasonable business limits
/// - **Format Compliance**: Must follow exact numeric format specifications
/// - **Logic Validation**: Index must not exceed total in indexed variants
///
/// ## Field Variants and Usage
///
/// ### Field 28 - Basic Statement/Sequence Number
/// - **Format**: `5n[/2n]`
/// - **Usage**: Account statements with optional sequence numbers
/// - **Statement Number**: Primary identifier for statement period
/// - **Sequence Number**: Sub-sequence within statement period
///
/// ### Field 28C - Extended Statement/Sequence Number
/// - **Format**: `5n[/5n]`
/// - **Usage**: Extended numbering for complex statement structures
/// - **Enhanced Range**: Larger sequence number capacity
/// - **Complex Statements**: Multi-part statements with extensive sequences
///
/// ### Field 28D - Message Index/Total
/// - **Format**: `5n/5n`
/// - **Usage**: Batch message indexing for completeness verification
/// - **Index Number**: Current message position in sequence
/// - **Total Count**: Complete count of messages in batch
/// - **Verification**: Enables receiver to verify all messages received
///
/// ## Business Context
/// - **Statement Management**: Systematic numbering of account statements
/// - **Batch Processing**: Sequencing multiple related transactions
/// - **Audit Trail**: Maintaining proper sequence records for compliance
/// - **Message Integrity**: Ensuring complete message set delivery
///
/// ## Examples
/// ```logic
/// :28:12345              // Statement 12345 (no sequence)
/// :28:12345/01           // Statement 12345, sequence 1
/// :28C:98765/00123       // Extended statement with sequence
/// :28D:001/010           // Message 1 of 10 total
/// :28D:010/010           // Final message (10 of 10)
/// ```
///
/// ## Statement Sequencing Logic
/// - **Daily Statements**: Incremental numbering by business day
/// - **Monthly Statements**: Period-based numbering with daily sequences
/// - **Special Statements**: Ad-hoc numbering for specific requirements
/// - **Continuation**: Sequence numbers for multi-part statements
///
/// ## Batch Message Processing
/// - **Transmission Order**: Sequential transmission of indexed messages
/// - **Completeness Check**: Verification all messages received
/// - **Error Recovery**: Re-transmission of missing message indices
/// - **Processing Logic**: Ordered processing based on index sequence
///
/// ## Regional Considerations
/// - **European Standards**: SEPA statement numbering requirements
/// - **US Banking**: Federal Reserve statement sequence standards
/// - **Asian Markets**: Local statement numbering conventions
/// - **International**: Cross-border statement coordination
///
/// ## Error Prevention
/// - **Number Validation**: Verify numbers are positive and within range
/// - **Sequence Logic**: Ensure logical progression of sequence numbers
/// - **Index Validation**: Confirm message index does not exceed total
/// - **Completeness Check**: Verify all expected messages received
///
/// ## Related Fields
/// - **Field 60**: Opening Balance (statement start information)
/// - **Field 62**: Closing Balance (statement end information)
/// - **Field 64**: Closing Available Balance (additional statement data)
/// - **Block Headers**: Message timestamps and references
///
/// ## Processing Applications
/// - **MT940**: Customer Statement Message (Field 28)
/// - **MT942**: Interim Transaction Report (Field 28C)
/// - **MT101**: Request for Transfer (Field 28D)
/// - **MT102**: Multiple Customer Credit Transfer (Field 28D)
///
/// ## STP Compliance
/// - **Automated Sequencing**: System-generated sequence numbers for STP
/// - **Integrity Validation**: Automated completeness checking
/// - **Exception Handling**: Missing sequence detection and alerts
/// - **Quality Control**: Real-time sequence validation
///
/// ## Compliance and Audit
/// - **Regulatory Reporting**: Sequential statement reporting requirements
/// - **Audit Trail**: Maintaining complete sequence records
/// - **Record Keeping**: Statement number preservation for regulatory periods
/// - **Investigation Support**: Sequence-based transaction reconstruction
///
/// ## See Also
/// - Swift FIN User Handbook: Statement Numbering Standards
/// - MT940/942 Guidelines: Statement Sequence Requirements
/// - Batch Processing Standards: Message Indexing Specifications
/// - Regulatory Guidelines: Statement Numbering Compliance
///   **Field 28: Basic Statement Number/Sequence Number**
///
/// Basic statement numbering with optional sequence for account statements
/// and transaction reports.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field28 {
    /// Statement number
    ///
    /// Format: 5n - Up to 5 digits (1-99999)
    /// Primary identifier for statement period or report
    #[component("5n")]
    pub statement_number: u32,

    /// Optional sequence number
    ///
    /// Format: [/2n] - Optional 1-2 digits after slash (1-99)
    /// Used for multi-part statements within same period
    #[component("[/2n]")]
    pub sequence_number: Option<u8>,
}

///   **Field 28C: Extended Statement Number/Sequence Number**
///
/// Extended statement numbering with larger sequence capacity for
/// complex statement structures and detailed transaction reports.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field28C {
    /// Statement number
    ///
    /// Format: 5n - Up to 5 digits (1-99999)
    /// Primary identifier for statement period
    #[component("5n")]
    pub statement_number: u32,

    /// Optional extended sequence number
    ///
    /// Format: [/5n] - Optional 1-5 digits after slash (1-99999)
    /// Enhanced sequence capacity for complex multi-part statements
    #[component("[/5n]")]
    pub sequence_number: Option<u32>,
}

///   **Field 28D: Message Index/Total**
///
/// Message indexing for batch operations enabling completeness verification
/// and proper sequencing of related messages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field28D {
    /// Message index (current position)
    ///
    /// Format: 5n - Current message number in sequence (1-99999)
    /// Must not exceed total count, enables ordering verification
    #[component("5n")]
    pub index: u32,

    /// Total message count
    ///
    /// Format: /5n - Complete count of messages in batch (1-99999)
    /// Enables receiver to verify all messages received
    #[component("/5n")]
    pub total: u32,
}
