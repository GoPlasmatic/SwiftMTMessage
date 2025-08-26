use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT290: Advice of Charges, Interest and Other Adjustments Cancellation
///
/// ## Purpose
/// Used to cancel or reverse a previously sent MT190 advice. This message notifies the account
/// holder that a previous adjustment advice needs to be cancelled or reversed, ensuring accurate
/// account records and proper handling of erroneous or superseded adjustments.
///
/// ## Scope
/// This message is:
/// - Sent by the account servicing institution to the account holder
/// - Used to cancel previously advised charges, interest, or adjustments
/// - Applied when errors are discovered or adjustments need reversal
/// - Essential for maintaining accurate account records
/// - Part of the error correction and adjustment management process
///
/// ## Key Features
/// - **Cancellation Notification**: Official cancellation of previous MT190 advice
/// - **Reference Linking**: Clear reference to the original MT190 being cancelled
/// - **Reversal Details**: Complete information about the adjustment being reversed
/// - **Account Identification**: Clear identification of the affected account
/// - **Charge Details**: Detailed explanation of the cancellation reason
/// - **Audit Trail**: Maintains complete documentation of adjustments and cancellations
///
/// ## Common Use Cases
/// - Cancelling erroneously posted interest calculations
/// - Reversing incorrect fee charges
/// - Correcting misapplied account adjustments
/// - Cancelling duplicate charge advisories
/// - Reversing provisional adjustments
/// - Correcting calculation errors in fees
/// - Cancelling adjustments due to system errors
/// - Reversing adjustments per customer dispute resolution
///
/// ## Message Structure
/// - **Field 20**: Transaction Reference Number (mandatory) - Unique reference for this cancellation
/// - **Field 21**: Related Reference (mandatory) - Reference to original MT190 being cancelled
/// - **Field 25**: Account Identification (mandatory) - Account affected by cancellation
/// - **Field 32a**: Value Date, Currency Code, Amount (mandatory) - Original adjustment amount (C or D)
/// - **Field 52a**: Ordering Institution (optional) - Institution initiating cancellation (A or D)
/// - **Field 71B**: Details of Charges (mandatory) - Explanation of cancellation/reversal
/// - **Field 72**: Sender to Receiver Information (optional) - Additional cancellation details
///
/// ## Network Validation Rules
/// - **Reference Matching**: Field 21 must reference a valid MT190 transaction
/// - **Amount Consistency**: Amount should match the original MT190 adjustment
/// - **Account Validation**: Account must match the original MT190 account
/// - **Cancellation Details**: Field 71B must explain the cancellation reason
/// - **Timing Rules**: Cancellation typically within reasonable timeframe of original
///
/// ## Processing Context
/// ### Cancellation Processing Workflow
/// 1. Error or reversal requirement identified
/// 2. Original MT190 adjustment reversed in system
/// 3. MT290 sent to advise cancellation
/// 4. Account holder reverses original entry
/// 5. Reconciliation updated accordingly
///
/// ### Reversal Management
/// - Automatic reversal posting
/// - Audit trail maintenance
/// - Balance correction
/// - Statement adjustment
///
/// ## SRG2025 Status
/// - **No Structural Changes**: MT290 format remains stable
/// - **Enhanced Processing**: Improved cancellation workflow support
/// - **Validation Updates**: Stricter reference matching validation
/// - **Real-time Capability**: Support for immediate cancellation notifications
///
/// ## Integration Considerations
/// - **Banking Systems**: Integrated with core banking reversal processes
/// - **Audit Systems**: Complete audit trail for compliance
/// - **Reconciliation**: Automatic reconciliation adjustment
/// - **Reporting**: Reflected in account statements and reports
///
/// ## Relationship to Other Messages
/// - **Cancels**: MT190 advice messages
/// - **May trigger**: New MT190 with corrected information
/// - **Related to**: MT192/292 for payment cancellations
/// - **Supports**: Error correction and adjustment management processes
///
/// ## Best Practices
/// - Send MT290 promptly upon discovering errors
/// - Provide clear cancellation reasons in Field 71B
/// - Ensure amount and account details match original MT190
/// - Consider sending corrected MT190 if adjustment still valid
/// - Maintain complete audit trail of adjustments and cancellations

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT290 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("25")]
    pub field_25: Field25AccountIdentification,

    #[field("32")]
    pub field_32: Field32,

    #[field("52")]
    pub field_52: Option<Field52OrderingInstitution>,

    #[field("71B")]
    pub field_71b: Field71B,

    #[field("72")]
    pub field_72: Option<Field72>,
}