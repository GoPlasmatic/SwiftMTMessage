use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT910: Confirmation of Credit
///
/// ## Purpose
/// Used to confirm that a credit entry has been posted to an account. This message serves
/// as notification to the account holder that their account has been credited with the
/// specified amount and provides complete details of the transaction.
///
/// ## Scope
/// This message is:
/// - Sent by the account servicing institution to the account holder
/// - Used to confirm that a credit has been processed and posted
/// - Applied to various types of credits including incoming transfers, deposits, and reversals
/// - Essential for account reconciliation and cash management
/// - Part of real-time liquidity monitoring and cash flow management
///
/// ## Key Features
/// - **Credit Confirmation**: Official confirmation that account has been credited
/// - **Transaction Details**: Complete information about the credit transaction
/// - **Originator Information**: Details about who initiated the credit (field 50 or 52)
/// - **Timing Information**: Optional date/time indication for processing details
/// - **Reference Tracking**: Links to original payment instructions or related transactions
/// - **Intermediary Details**: Optional information about intermediary institutions
///
/// ## Common Use Cases
/// - Confirming incoming payment transfers to customer accounts
/// - Notifying of investment proceeds and settlements
/// - Confirming foreign exchange transaction proceeds
/// - Trade settlement credit confirmations
/// - Interest credit confirmations
/// - Reversal and correction credit confirmations
/// - Deposit and funding confirmations
/// - Loan disbursement confirmations
///
/// ## Field Structure
/// - **20**: Transaction Reference (mandatory) - Unique reference for this confirmation
/// - **21**: Related Reference (mandatory) - Reference to original transaction/instruction
/// - **25**: Account Identification (mandatory) - Account that has been credited
/// - **13D**: Date/Time Indication (optional) - Processing timing details
/// - **32A**: Value Date/Currency/Amount (mandatory) - Credit details
/// - **50**: Ordering Customer (optional) - Customer who initiated the credit
/// - **52**: Ordering Institution (optional) - Institution that initiated the credit
/// - **56**: Intermediary Institution (optional) - Intermediary in the payment chain
/// - **72**: Sender to Receiver Information (optional) - Additional transaction details
///
/// ## Network Validation Rules
/// - **C1 Rule**: Either field 50 (Ordering Customer) or field 52 (Ordering Institution) must be present, but not both
/// - **Reference Format**: Transaction references must follow SWIFT formatting standards
/// - **Amount Validation**: Credit amounts must be positive
/// - **Account Validation**: Account identification must be valid and properly formatted
/// - **Date Validation**: Date/time indications must be valid when present
/// - **Currency Validation**: Currency codes must be valid ISO 4217 codes
///
/// ## Processing Context
/// ### Credit Processing Workflow
/// 1. Incoming payment received (e.g., MT103, MT202, wire transfer)
/// 2. Account credited by servicing institution
/// 3. MT910 sent to confirm credit execution
/// 4. Account holder updates records and cash position
///
/// ### Cash Management Integration
/// - Real-time balance updates
/// - Liquidity position management
/// - Cash flow forecasting support
/// - Working capital optimization
///
/// ## Originator Identification
/// The message must identify the originator through either:
/// - **Field 50**: When the credit originates from a customer
/// - **Field 52**: When the credit originates from a financial institution
///
/// This distinction is important for:
/// - Compliance and regulatory reporting
/// - Know Your Customer (KYC) requirements
/// - Anti-money laundering (AML) monitoring
/// - Transaction categorization and analysis
///
/// ## SRG2025 Status
/// - **No Structural Changes**: MT910 format remains unchanged in SRG2025
/// - **Enhanced Validation**: Additional validation rules for improved transaction integrity
/// - **Digital Banking Integration**: Better support for digital banking platforms
/// - **Real-time Processing**: Enhanced capabilities for instant payment confirmations
///
/// ## Integration Considerations
/// - **Banking Systems**: Direct integration with core banking and account management systems
/// - **Treasury Systems**: Essential input for treasury and cash management platforms
/// - **ERP Integration**: Critical for enterprise resource planning and financial reporting
/// - **Reconciliation**: Automated matching with expected receipts and cash flow forecasts
///
/// ## Relationship to Other Messages
/// - **Responds to**: MT103, MT202, MT205 and other payment instructions
/// - **Complements**: MT900 (Confirmation of Debit) for complete transaction visibility
/// - **Supports**: Cash management, liquidity monitoring, and reconciliation processes
/// - **Integrates with**: Statement messages (MT940, MT950) for comprehensive account reporting

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT910_VALIDATION_RULES)]
pub struct MT910 {
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

    #[field("50")]
    pub field_50: Option<Field50OrderingCustomerAFK>,

    #[field("52")]
    pub field_52: Option<Field52OrderingInstitution>,

    #[field("56")]
    pub field_56: Option<Field56Intermediary>,

    #[field("72")]
    pub field_72: Option<Field72>,
}

/// Validation rules for MT910 - Confirmation of Credit
const MT910_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Either field 50a or field 52a must be present",
      "condition": {
        "or": [
          {"exists": ["fields", "50"]},
          {"exists": ["fields", "52"]}
        ]
      }
    }
  ]
}"#;
