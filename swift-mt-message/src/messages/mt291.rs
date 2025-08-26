use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT291: Request for Payment of Charges, Interest and Other Expenses Cancellation
///
/// ## Purpose
/// Used to cancel or withdraw a previously sent MT191 request for payment of charges.
/// This message notifies the receiving institution that a previous expense claim should be
/// disregarded, ensuring proper handling of erroneous or superseded charge requests.
///
/// ## Scope
/// This message is:
/// - Sent between financial institutions to cancel expense reimbursement requests
/// - Used to withdraw previously claimed charges, interest, or expenses
/// - Applied when errors are discovered or claims need withdrawal
/// - Essential for maintaining accurate inter-bank expense records
/// - Part of the error correction and expense management process
///
/// ## Key Features
/// - **Cancellation Notification**: Official cancellation of previous MT191 request
/// - **Reference Linking**: Clear reference to the original MT191 being cancelled
/// - **Amount Details**: Complete information about the claim being withdrawn
/// - **Institution Identification**: Clear identification of parties involved
/// - **Charge Details**: Explanation of the cancellation reason
/// - **Audit Trail**: Maintains complete documentation of requests and cancellations
///
/// ## Common Use Cases
/// - Cancelling erroneously submitted expense claims
/// - Withdrawing duplicate charge requests
/// - Correcting miscalculated interest claims
/// - Cancelling provisional expense requests
/// - Withdrawing charges due to billing errors
/// - Correcting service fee claims
/// - Cancelling requests due to system errors
/// - Withdrawing claims per bilateral agreement changes
///
/// ## Message Structure
/// - **Field 20**: Transaction Reference Number (mandatory) - Unique reference for this cancellation
/// - **Field 21**: Related Reference (mandatory) - Reference to original MT191 being cancelled
/// - **Field 32B**: Currency Code, Amount (mandatory) - Original requested amount
/// - **Field 52a**: Ordering Institution (optional) - Institution initiating cancellation (A or D)
/// - **Field 57a**: Account With Institution (optional) - Settlement account details (A, B, or D)
/// - **Field 71B**: Details of Charges (mandatory) - Explanation of cancellation/withdrawal
/// - **Field 72**: Sender to Receiver Information (optional) - Additional cancellation details
///
/// ## Network Validation Rules
/// - **Reference Matching**: Field 21 must reference a valid MT191 transaction
/// - **Amount Consistency**: Amount should match the original MT191 request
/// - **Institution Validation**: Institutions must match the original MT191
/// - **Cancellation Details**: Field 71B must explain the cancellation reason
/// - **Timing Rules**: Cancellation typically before payment processing
///
/// ## Processing Context
/// ### Cancellation Processing Workflow
/// 1. Error or withdrawal requirement identified
/// 2. Original MT191 request marked for cancellation
/// 3. MT291 sent to advise cancellation
/// 4. Receiving institution cancels pending claim
/// 5. Reconciliation updated accordingly
///
/// ### Claim Management
/// - Automatic claim withdrawal
/// - Audit trail maintenance
/// - Expense tracking updates
/// - Billing system adjustments
///
/// ## SRG2025 Status
/// - **No Structural Changes**: MT291 format remains stable
/// - **Enhanced Processing**: Improved cancellation workflow support
/// - **Validation Updates**: Stricter reference matching validation
/// - **Real-time Capability**: Support for immediate cancellation notifications
///
/// ## Integration Considerations
/// - **Expense Systems**: Integrated with expense management platforms
/// - **Audit Systems**: Complete audit trail for compliance
/// - **Reconciliation**: Automatic expense reconciliation adjustment
/// - **Reporting**: Reflected in inter-bank expense reports
///
/// ## Relationship to Other Messages
/// - **Cancels**: MT191 request messages
/// - **May trigger**: New MT191 with corrected information
/// - **Related to**: MT292 for payment request cancellation
/// - **Supports**: Error correction and expense management processes
///
/// ## Best Practices
/// - Send MT291 promptly upon discovering errors
/// - Provide clear cancellation reasons in Field 71B
/// - Ensure amount and institution details match original MT191
/// - Consider sending corrected MT191 if claim still valid
/// - Maintain complete audit trail of requests and cancellations
/// - Notify affected departments of claim withdrawal

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT291 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("32B")]
    pub field_32b: Field32B,

    #[field("52")]
    pub field_52: Option<Field52OrderingInstitution>,

    #[field("57")]
    pub field_57: Option<Field57AccountWithInstitution>,

    #[field("71B")]
    pub field_71b: Field71B,

    #[field("72")]
    pub field_72: Option<Field72>,
}
