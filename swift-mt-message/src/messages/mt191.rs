use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT191: Request for Payment of Charges, Interest and Other Expenses
///
/// ## Purpose
/// Used to request payment of charges, interest and other expenses from another financial institution.
/// This message enables banks to claim reimbursement for costs incurred on behalf of another institution,
/// ensuring proper cost recovery and transparent inter-bank expense management.
///
/// ## Scope
/// This message is:
/// - Sent between financial institutions to request expense reimbursement
/// - Used to claim charges, interest, and other costs incurred
/// - Applied to various types of inter-bank expenses and fee recovery
/// - Essential for correspondent banking cost management
/// - Part of the comprehensive inter-bank settlement process
///
/// ## Key Features
/// - **Expense Request**: Formal request for payment of incurred expenses
/// - **Detailed Charges**: Comprehensive breakdown of charges being claimed
/// - **Currency Specification**: Clear identification of currency and amount
/// - **Reference Tracking**: Links to related transactions or services
/// - **Institution Identification**: Clear identification of parties involved
/// - **Settlement Instructions**: Optional routing for payment settlement
///
/// ## Common Use Cases
/// - Correspondent banking charge recovery
/// - Interest payment requests on nostro accounts
/// - Service fee reimbursement requests
/// - Transaction processing cost recovery
/// - Foreign exchange handling charges
/// - Investigation and inquiry fee recovery
/// - Amendment processing charge requests
/// - Compliance and regulatory cost recovery
///
/// ## Message Structure
/// - **Field 20**: Transaction Reference Number (mandatory) - Unique reference for this request
/// - **Field 21**: Related Reference (mandatory) - Reference to related transaction or period
/// - **Field 32B**: Currency Code, Amount (mandatory) - Amount being requested
/// - **Field 52a**: Ordering Institution (optional) - Institution initiating request (A or D)
/// - **Field 57a**: Account With Institution (optional) - Settlement account details (A, B, or D)
/// - **Field 71B**: Details of Charges (mandatory) - Detailed breakdown of expenses
/// - **Field 72**: Sender to Receiver Information (optional) - Additional information
///
/// ## Network Validation Rules
/// - **Reference Format**: Transaction references must follow SWIFT standards
/// - **Amount Validation**: Requested amounts must be properly formatted
/// - **Institution Validation**: BIC codes must be valid SWIFT participants
/// - **Charge Details**: Field 71B must contain meaningful expense information
/// - **Currency Validation**: Currency codes must be valid ISO 4217 codes
///
/// ## Processing Context
/// ### Request Processing Workflow
/// 1. Expenses incurred and documented
/// 2. MT191 request prepared with details
/// 3. Message sent to responsible institution
/// 4. Receiving institution validates claim
/// 5. Payment processed via appropriate channel
///
/// ### Expense Management
/// - Cost tracking and allocation
/// - Inter-bank billing processes
/// - Service level agreements
/// - Reconciliation support
///
/// ## SRG2025 Status
/// - **No Structural Changes**: MT191 format remains stable
/// - **Enhanced Processing**: Improved integration with expense management systems
/// - **Validation Updates**: Enhanced validation rules for expense claims
/// - **Real-time Capability**: Support for immediate expense notifications
///
/// ## Integration Considerations
/// - **Correspondent Banking**: Core component of correspondent relationships
/// - **Expense Systems**: Integration with bank expense management platforms
/// - **Settlement Networks**: Links to payment and settlement systems
/// - **Reporting**: Essential for inter-bank cost reporting
///
/// ## Relationship to Other Messages
/// - **May trigger**: MT103/MT202 for actual payment
/// - **Related to**: MT192 for payment request cancellation
/// - **Complements**: MT199 for free format messages
/// - **Alternative**: MT291 for cancellation of this request
///
/// ## Best Practices
/// - Provide detailed charge breakdown in Field 71B
/// - Include clear reference to services or transactions
/// - Specify preferred settlement instructions when applicable
/// - Ensure timely submission of expense claims
/// - Maintain supporting documentation for all charges

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT191 {
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
