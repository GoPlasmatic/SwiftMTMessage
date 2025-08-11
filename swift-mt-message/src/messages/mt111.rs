use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT111: Stop Payment of a Cheque
///
/// ## Purpose
/// Used to request the stop payment of a previously issued cheque from the account holder to their bank.
/// This message provides precise identification of the cheque to be stopped and includes supporting information
/// for immediate processing to prevent unauthorized or problematic cheque payments.
///
/// ## Scope
/// This message is:
/// - Used for stop payment requests from account holders to their financial institutions
/// - Applicable for preventing payment of specific cheques before clearing
/// - Designed for urgent processing to halt cheque payment authorization
/// - Compatible with both domestic and international cheque clearing systems
/// - Subject to validation rules for proper cheque identification
/// - Integrated with fraud prevention and account security systems
///
/// ## Key Features
/// - **Precise Cheque Identification**: Complete cheque details for accurate identification
/// - **Immediate Stop Payment Control**: Urgent processing to prevent cheque payment
/// - **Reference Tracking System**: Links to original cheque issue and account references
/// - **Reason Code Support**: Optional information about why stop payment is requested
/// - **Payee Information**: Optional payee details for additional verification
/// - **Bank Integration**: Seamless integration with bank's cheque processing systems
///
/// ## Common Use Cases
/// - Stolen or lost cheque stop payment requests
/// - Fraudulent cheque prevention and security measures
/// - Duplicate cheque issuance corrections
/// - Post-dated cheque cancellation requests
/// - Dispute resolution for unauthorized cheque issuance
/// - Account closure preparation with outstanding cheques
/// - Emergency stop payments for financial protection
///
/// ## Message Structure
/// - **Field 20**: Transaction Reference (mandatory) - Unique stop payment request identifier
/// - **Field 21**: Cheque Number (mandatory) - Specific cheque number to be stopped
/// - **Field 30**: Date of Issue (mandatory) - Date when cheque was originally issued (YYMMDD)
/// - **Field 32**: Currency/Amount (mandatory) - Original cheque amount and currency
/// - **Field 52**: Drawer Bank (optional) - Bank on which the cheque was drawn
/// - **Field 59**: Payee (optional) - Name and address of cheque payee (no account number)
/// - **Field 75**: Queries (optional) - Additional information or reason for stop payment
///
/// ## Network Validation Rules
/// - **Reference Format**: Transaction reference must not start/end with '/' or contain '//'
/// - **Cheque Number Format**: Cheque number must not contain '/' or '//' characters
/// - **Date Validation**: Date of issue must be in valid YYMMDD format
/// - **Payee Information**: Payee field must not contain account number information
/// - **Amount Validation**: Currency and amount must match original cheque details
/// - **Bank Identification**: Proper validation of drawer bank information when present
/// - **Query Information**: Proper formatting of reason codes and additional information
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT111 format remains stable for stop payment processing
/// - **Validation Updates**: Enhanced validation for fraud prevention and security
/// - **Processing Improvements**: Improved handling of urgent stop payment requests
/// - **Compliance Notes**: Maintained compatibility with regulatory requirements for stop payments
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with cheque processing and fraud prevention systems
/// - **API Integration**: RESTful API support for modern digital banking platforms
/// - **Processing Requirements**: Supports urgent processing with immediate effect
/// - **Compliance Integration**: Built-in validation for regulatory stop payment requirements
///
/// ## Relationship to Other Messages
/// - **Triggers**: Often triggered by customer requests through digital banking or branch systems
/// - **Responses**: Generates MT112 status response messages for stop payment confirmation
/// - **Related**: Works with cheque processing systems and account management platforms
/// - **Alternatives**: Electronic payment cancellation messages for digital transactions
/// - **Status Updates**: May receive status updates about stop payment effectiveness
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT111 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("30")]
    pub field_30: Field30,

    #[field("32")]
    pub field_32: Field32,

    #[field("52")]
    pub field_52: Option<Field52DrawerBank>,

    #[field("59")]
    pub field_59: Option<Field59NoOption>,

    #[field("75")]
    pub field_75: Option<Field75>,
}
