use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT190: Advice of Charges, Interest and Other Adjustments
///
/// ## Purpose
/// Used to advise charges, interest and other adjustments that have been debited or credited
/// to an account. This message provides detailed information about adjustments made to accounts,
/// enabling proper reconciliation and transparency of fees, interest, and other financial adjustments.
///
/// ## Scope
/// This message is:
/// - Sent by the account servicing institution to the account holder
/// - Used to advise charges, interest calculations, and account adjustments
/// - Applied to various types of adjustments including fees, interest, and corrections
/// - Essential for account reconciliation and financial transparency
/// - Part of the comprehensive account management and reporting process
///
/// ## Key Features
/// - **Adjustment Notification**: Official notification of account adjustments
/// - **Detailed Charges**: Comprehensive breakdown of charges and fees
/// - **Interest Information**: Details of interest calculations and postings
/// - **Reference Tracking**: Links to related transactions or periods
/// - **Account Identification**: Clear identification of the affected account
/// - **Flexible Amount Fields**: Supports both debit and credit adjustments
///
/// ## Common Use Cases
/// - Periodic interest posting notifications
/// - Bank fee and charge advisories
/// - Account maintenance charge notifications
/// - Overdraft interest calculations
/// - Investment management fee notifications
/// - Foreign exchange adjustment advisories
/// - Error corrections and adjustments
/// - Service charge notifications
///
/// ## Message Structure
/// - **Field 20**: Transaction Reference Number (mandatory) - Unique reference for this advice
/// - **Field 21**: Related Reference (mandatory) - Reference to related transaction or period
/// - **Field 25**: Account Identification (mandatory) - Account being adjusted
/// - **Field 32a**: Value Date, Currency Code, Amount (mandatory) - Adjustment details (C or D)
/// - **Field 52a**: Ordering Institution (optional) - Institution initiating adjustment (A or D)
/// - **Field 71B**: Details of Charges (mandatory) - Detailed breakdown of charges/adjustments
/// - **Field 72**: Sender to Receiver Information (optional) - Additional information
///
/// ## Network Validation Rules
/// - **Reference Format**: Transaction references must follow SWIFT standards
/// - **Amount Validation**: Adjustment amounts must be properly formatted
/// - **Account Validation**: Account identification must be valid and properly formatted
/// - **Charge Details**: Field 71B must contain meaningful adjustment information
/// - **Currency Validation**: Currency codes must be valid ISO 4217 codes
///
/// ## Processing Context
/// ### Adjustment Processing Workflow
/// 1. Adjustment calculation or determination
/// 2. Account posting (debit or credit)
/// 3. MT190 sent to advise adjustment
/// 4. Account holder updates records
///
/// ### Account Management
/// - Periodic adjustment processing
/// - Fee calculation and posting
/// - Interest computation
/// - Account reconciliation support
///
/// ## SRG2025 Status
/// - **No Structural Changes**: MT190 format remains stable
/// - **Enhanced Processing**: Improved integration with digital banking
/// - **Validation Updates**: Enhanced validation rules for better accuracy
/// - **Real-time Capability**: Support for immediate adjustment notifications
///
/// ## Integration Considerations
/// - **Banking Systems**: Direct integration with core banking systems
/// - **Account Management**: Part of comprehensive account servicing
/// - **Reconciliation**: Essential for automated reconciliation processes
/// - **Reporting**: Key component of account reporting and statements
///
/// ## Relationship to Other Messages
/// - **Related to**: MT900/910 for transaction confirmations
/// - **Complements**: MT940/950 statement messages
/// - **Supports**: Account reconciliation and management processes
/// - **Alternative**: MT290 for cancellation of adjustments

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT190 {
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