use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT900: Confirmation of Debit
///
/// ## Purpose
/// Used to confirm that a debit entry has been posted to an account. This message serves
/// as notification to the account holder that their account has been debited with the
/// specified amount and provides details of the transaction.
///
/// ## Scope
/// This message is:
/// - Sent by the account servicing institution to the account holder
/// - Used to confirm that a debit has been processed and posted
/// - Applied to various types of debits including transfers, fees, and other charges
/// - Essential for account reconciliation and transaction tracking
/// - Part of the cash management and liquidity monitoring process
///
/// ## Key Features
/// - **Debit Confirmation**: Official confirmation that account has been debited
/// - **Transaction Details**: Complete information about the debit transaction
/// - **Timing Information**: Optional date/time indication for processing details
/// - **Reference Tracking**: Links to original payment instructions or requests
/// - **Account Identification**: Clear identification of the debited account
/// - **Ordering Institution**: Optional details about the institution that initiated the debit
///
/// ## Common Use Cases
/// - Confirming payment transfers from customer accounts
/// - Notifying of fee debits and charges
/// - Confirming investment transfers and settlements
/// - Trade settlement confirmations
/// - Standing order execution confirmations
/// - Direct debit processing confirmations
/// - Foreign exchange transaction confirmations
/// - Account closure and transfer confirmations
///
/// ## Field Structure
/// - **20**: Transaction Reference (mandatory) - Unique reference for this confirmation
/// - **21**: Related Reference (mandatory) - Reference to original transaction/instruction
/// - **25**: Account Identification (mandatory) - Account that has been debited
/// - **13D**: Date/Time Indication (optional) - Processing timing details
/// - **32A**: Value Date/Currency/Amount (mandatory) - Debit details
/// - **52**: Ordering Institution (optional) - Institution that initiated the debit
/// - **72**: Sender to Receiver Information (optional) - Additional transaction details
///
/// ## Processing Context
/// ### Debit Processing Workflow
/// 1. Original payment instruction received (e.g., MT103, MT202)
/// 2. Account debited by servicing institution
/// 3. MT900 sent to confirm debit execution
/// 4. Account holder updates records based on confirmation
///
/// ### Account Management
/// - Real-time account balance updates
/// - Transaction history maintenance
/// - Reconciliation support
/// - Liquidity monitoring
///
/// ## Network Validation Rules
/// - **Reference Format**: Transaction references must follow SWIFT standards
/// - **Amount Validation**: Debit amounts must be positive
/// - **Account Validation**: Account identification must be valid and properly formatted
/// - **Date Validation**: Date/time indications must be valid when present
/// - **Currency Validation**: Currency codes must be valid ISO 4217 codes
///
/// ## SRG2025 Status
/// - **No Structural Changes**: MT900 format remains unchanged in SRG2025
/// - **Enhanced Validation**: Additional validation rules for improved accuracy
/// - **Digital Integration**: Better integration with digital banking platforms
/// - **Real-time Processing**: Enhanced support for real-time transaction confirmation
///
/// ## Integration Considerations
/// - **Banking Systems**: Direct integration with core banking systems
/// - **Cash Management**: Part of comprehensive cash management solutions
/// - **Reconciliation**: Essential input for automated reconciliation processes
/// - **Reporting**: Key component of transaction reporting and audit trails
///
/// ## Relationship to Other Messages
/// - **Responds to**: MT103, MT202, MT205 and other payment instructions
/// - **Complements**: MT910 (Confirmation of Credit) for complete transaction lifecycle
/// - **Supports**: Cash management and account reconciliation processes
/// - **Integrates with**: Statement messages (MT940, MT950) for comprehensive account reporting

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT900 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("25")]
    pub field_25: Field25AccountIdentification,

    #[field("13D")]
    pub field_13d: Option<Field13D>,

    #[field("32A")]
    pub field_32a: Field32A,

    #[field("52")]
    pub field_52: Option<Field52OrderingInstitution>,

    #[field("72")]
    pub field_72: Option<Field72>,
}
