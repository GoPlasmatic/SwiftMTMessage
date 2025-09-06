use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT204: Financial Markets Direct Debit Message
///
/// ## Purpose
/// Used for direct debit transactions in financial markets, particularly for settlement of trades
/// and market-related obligations. This message enables efficient collection of funds from multiple
/// debtors in financial market transactions, supporting both single and bulk debit operations.
///
/// ## Scope
/// This message is:
/// - Used between financial institutions for market-related direct debits
/// - Designed for settlement of securities trades and derivatives transactions
/// - Applicable for margin calls and collateral management
/// - Used for collecting clearing house obligations
/// - Compatible with central counterparty (CCP) settlement systems
/// - Limited to a maximum of 10 transactions per message
///
/// ## Key Features
/// - **Two-Sequence Structure**: Common reimbursement details and individual transaction details
/// - **Bulk Processing**: Processes up to 10 direct debit transactions in one message
/// - **Sum Validation**: Built-in validation that sum of transactions equals total amount
/// - **Currency Consistency**: All transactions must use the same currency
/// - **Efficient Settlement**: Optimized for financial markets settlement processes
/// - **Reimbursement Focus**: Streamlined for financial institution reimbursements
///
/// ## Common Use Cases
/// - Securities trade settlement collections
/// - Derivatives margin call collections
/// - Clearing house member collections
/// - Central securities depository (CSD) fee collections
/// - Exchange membership fee collections
/// - Market maker obligation settlements
/// - Cross-border financial market settlements
///
/// ## Message Structure
/// ### Sequence A (Common Elements - Reimbursement Details - Mandatory, Single)
/// - **Field 20**: Transaction Reference Number (mandatory) - Unique message identifier
/// - **Field 19**: Sum of Amounts (mandatory) - Total amount to be collected
/// - **Field 30**: Value Date (mandatory) - Settlement date for all transactions
/// - **Field 57**: Account With Institution (optional) - Institution holding the account
/// - **Field 58**: Beneficiary Institution (optional) - Final beneficiary institution
/// - **Field 72**: Sender to Receiver Information (optional) - Additional instructions
///
/// ### Sequence B (Transaction Details - Mandatory, Repetitive, Max 10)
/// - **Field 20**: Transaction Reference Number (mandatory) - Individual transaction reference
/// - **Field 21**: Related Reference (optional) - Reference to related transaction/trade
/// - **Field 32B**: Transaction Amount (mandatory) - Individual debit amount
/// - **Field 53**: Debit Institution (mandatory) - Institution to be debited
/// - **Field 72**: Sender to Receiver Information (optional) - Transaction-specific instructions
///
/// ## Network Validation Rules
/// - **C1**: Field 19 amount must equal sum of all Field 32B amounts
/// - **C2**: Currency in all Field 32B occurrences must be identical
/// - **C3**: Maximum 10 occurrences of Sequence B allowed (T10 error if exceeded)
///
/// ## Integration Considerations
/// - **Trading Systems**: Direct integration with trading and settlement platforms
/// - **CCP Systems**: Compatible with central counterparty clearing systems
/// - **Risk Management**: Integration with margin and collateral management systems
/// - **Regulatory Reporting**: Supports transaction reporting requirements
///
/// ## Relationship to Other Messages
/// - **Triggers**: Often triggered by trade confirmations or margin calculations
/// - **Related**: Works with MT202 for cover payments and MT210 for pre-notifications
/// - **Confirmations**: May generate MT900 (debit) confirmations
/// - **Status**: May receive MT296 for cancellation responses
/// - **Reporting**: Reflected in MT940/MT950 account statements
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT204_VALIDATION_RULES)]
pub struct MT204 {
    // Sequence A: Common Elements - Reimbursement Details
    #[field("20")]
    pub field_20: Field20,

    #[field("19")]
    pub field_19: Field19,

    #[field("30")]
    pub field_30: Field30,

    #[field("57")]
    pub field_57_seq_a: Option<Field57DebtInstitution>,

    #[field("58")]
    pub field_58: Option<Field58>,

    #[field("72")]
    pub field_72_seq_a: Option<Field72>,

    // Sequence B: Transaction Details (repetitive)
    #[field("#")]
    pub transactions: Vec<MT204Transaction>,
}

/// Individual transaction details for MT204 Sequence B
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT204Transaction {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Option<Field21NoOption>,

    #[field("32B")]
    pub field_32b: Field32B,

    #[field("53")]
    pub field_53: Field53SenderCorrespondent,

    #[field("72")]
    pub field_72: Option<Field72>,
}

// Validation rules for MT204 using JSONLogic
pub const MT204_VALIDATION_RULES: &str = r##"{
    "rules": [
        {
            "id": "C1",
            "description": "The amount in field 19 must equal the sum of the amounts in all occurrences of field 32B",
            "condition": {
                "==": [
                    {"var": "fields.19.amount"},
                    {"reduce": [
                        {"var": "fields.#"},
                        {"+": [{"var": "accumulator"}, {"var": "current.32B.amount"}]},
                        0
                    ]}
                ]
            }
        },
        {
            "id": "C2", 
            "description": "The currency code in the amount field 32B must be the same for all occurrences of this field in the message",
            "condition": {
                "all": [
                    {"var": "fields.#"},
                    {
                        "==": [
                            {"var": "32B.currency"},
                            {"val": [[-3], "fields", "#", 0, "32B", "currency"]}
                        ]
                    }
                ]
            }
        },
        {
            "id": "C3",
            "description": "The repetitive sequence must not appear more than ten times",
            "condition": {
                "if": [
                    {"exists": ["fields", "#"]},
                    {"<=": [{"length": {"var": "fields.#"}}, 10]},
                    true
                ]
            }
        }
    ]
}"##;
