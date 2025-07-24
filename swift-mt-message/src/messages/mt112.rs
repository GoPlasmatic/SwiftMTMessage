use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT112: Status of a Request for Stop Payment of a Cheque
///
/// ## Purpose
/// Used by financial institutions to respond to an MT111 stop payment request with definitive status information.
/// This message provides clear confirmation of whether the stop payment was accepted, rejected, or requires additional
/// processing, ensuring proper closure of the stop payment request workflow.
///
/// ## Scope
/// This message is:
/// - Used for status responses to MT111 stop payment requests from financial institutions
/// - Applicable for confirming stop payment processing outcomes and decisions
/// - Designed for maintaining audit trails and regulatory compliance for stop payments
/// - Compatible with both automated and manual stop payment processing systems
/// - Subject to validation rules for proper status reporting and reference tracking
/// - Integrated with cheque processing systems for real-time status updates
///
/// ## Key Features
/// - **Comprehensive Status Reporting**: Clear indication of stop payment request processing status
/// - **Reference Linkage System**: Direct links to original MT111 stop payment request
/// - **Detailed Reason Codes**: Comprehensive information for rejected or failed requests
/// - **Transaction Audit Trail**: Maintains complete audit trail for stop payment requests
/// - **Regulatory Compliance**: Structured reporting for regulatory and internal compliance
/// - **Processing Confirmation**: Definitive confirmation of stop payment effectiveness
///
/// ## Common Use Cases
/// - Confirmation of successful stop payment implementation
/// - Rejection notification for invalid or late stop payment requests
/// - Status updates for stop payments requiring manual review
/// - Compliance reporting for regulatory stop payment tracking
/// - Audit trail maintenance for customer service and dispute resolution
/// - Integration with fraud prevention systems for security confirmation
/// - Notification of stop payment effectiveness periods and limitations
///
/// ## Message Structure
/// - **Field 20**: Transaction Reference (mandatory) - Links to original MT111 request reference
/// - **Field 21**: Cheque Number (mandatory) - Confirms specific cheque number from original request
/// - **Field 30**: Date of Issue (mandatory) - Confirms original cheque issue date (YYMMDD)
/// - **Field 32**: Currency/Amount (mandatory) - Confirms original cheque amount and currency
/// - **Field 52**: Drawer Bank (optional) - Confirms bank information from original request
/// - **Field 59**: Payee (optional) - Confirms payee information (no account number)
/// - **Field 76**: Answers (mandatory) - Status information and processing results
///
/// ## Network Validation Rules
/// - **Date Format Validation**: Date of issue must be in valid YYMMDD format
/// - **Reference Consistency**: Transaction reference must match original MT111 request format
/// - **Cheque Number Format**: Cheque number must not contain '/' or '//' characters
/// - **Bank Option Validation**: Only one option of Drawer Bank (52A/B/D) may be present
/// - **Payee Format Validation**: Payee field must not contain account number in first line
/// - **Status Code Validation**: Answers field should contain predefined status codes
/// - **Reference Linking**: All fields must be consistent with original MT111 request
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT112 format remains stable for status reporting
/// - **Validation Updates**: Enhanced validation for status code compliance and consistency
/// - **Processing Improvements**: Improved handling of automated status determination
/// - **Compliance Notes**: Strengthened regulatory reporting requirements for stop payment outcomes
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with cheque processing and fraud prevention systems
/// - **API Integration**: RESTful API support for modern digital banking status reporting
/// - **Processing Requirements**: Supports real-time status reporting with audit capabilities
/// - **Compliance Integration**: Built-in validation for regulatory stop payment status requirements
///
/// ## Relationship to Other Messages
/// - **Triggers**: Directly triggered by MT111 stop payment requests requiring status response
/// - **Responses**: Provides definitive response to MT111, completing stop payment workflow
/// - **Related**: Works with cheque processing systems and customer notification platforms
/// - **Alternatives**: Electronic payment status messages for digital transaction cancellations
/// - **Status Updates**: Final status message in stop payment request lifecycle
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT112 {
    #[field("20")]
    pub field_20: Field20, // Transaction Reference Number

    #[field("21")]
    pub field_21: Field21NoOption, // Cheque Number

    #[field("30")]
    pub field_30: Field30, // Date of Issue (YYMMDD)

    #[field("32")]
    pub field_32: Field32, // Amount

    #[field("52")]
    pub field_52: Option<Field52DrawerBank>, // Drawer Bank A

    #[field("59")]
    pub field_59: Option<Field59NoOption>, // Payee (without account number)

    #[field("76")]
    pub field_76: Field76, // Answers (Status Information)
}
