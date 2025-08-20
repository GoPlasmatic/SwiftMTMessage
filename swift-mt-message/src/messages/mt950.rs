use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT950: Statement Message
///
/// ## Purpose
/// Used to transmit account statement information with a simplified structure focusing
/// on balance information and essential transaction data. This message provides streamlined
/// account reporting for efficient processing and communication.
///
/// ## Scope
/// This message is:
/// - Sent by account servicing institutions for streamlined statement delivery
/// - Used for simplified account reporting with essential information
/// - Applied when detailed narrative information is not required
/// - Essential for automated processing and high-volume account reporting
/// - Part of efficient account management and customer communication systems
///
/// ## Key Features
/// - **Simplified Structure**: Streamlined format for efficient processing
/// - **Essential Information**: Focus on key balance and transaction data
/// - **Multiple Transactions**: Support for multiple statement line entries
/// - **Balance Information**: Opening and closing balance with currency consistency
/// - **Available Balance**: Optional available balance information
/// - **Automated Processing**: Optimized for automated statement processing systems
///
/// ## Common Use Cases
/// - High-volume account statement processing
/// - Automated statement delivery systems
/// - Simplified account reporting for operational accounts
/// - Batch processing of multiple account statements
/// - System-to-system account information exchange
/// - Streamlined cash management reporting
/// - Efficient correspondent banking statement delivery
/// - Simplified regulatory reporting requirements
///
/// ## Field Structure
/// - **20**: Transaction Reference (mandatory) - Unique statement reference
/// - **25**: Account Identification (mandatory) - Account being reported
/// - **28C**: Statement Number/Sequence (mandatory) - Statement numbering
/// - **60**: Opening Balance (mandatory) - Starting balance for statement period
/// - **61**: Statement Line (mandatory, repetitive) - Individual transaction entries
/// - **62**: Closing Balance (mandatory) - Ending balance for statement period
/// - **64**: Available Balance (optional) - Available balance information
///
/// ## Field Details
/// ### Field 61 - Statement Line
/// Multiple statement lines can be included, each containing:
/// - **Value Date**: Date when transaction becomes effective
/// - **Entry Date**: Date when transaction was posted (optional)
/// - **Credit/Debit Mark**: C (Credit) or D (Debit) entry
/// - **Amount**: Transaction amount
/// - **Transaction Type**: SWIFT transaction type identification
/// - **Reference**: Transaction reference number
///
/// ## Network Validation Rules
/// - **Currency Consistency**: Opening and closing balances must use the same currency
/// - **Available Balance Currency**: Available balances must use same currency as main balances
/// - **Reference Format**: Transaction references must follow SWIFT formatting standards
/// - **Required Fields**: All mandatory fields must be present and properly formatted
/// - **Balance Logic**: Closing balance should reflect opening balance plus/minus transactions
/// - **Date Validation**: All dates must be valid and properly sequenced
///
/// ## Processing Context
/// ### Simplified Statement Generation
/// 1. Account activity summarized for statement period
/// 2. Essential transactions selected for reporting
/// 3. Opening balance carried forward from previous period
/// 4. MT950 generated with streamlined transaction detail
/// 5. Closing balance calculated and validated
///
/// ### Automated Processing
/// - High-volume statement batch processing
/// - Automated account reconciliation
/// - System integration and data exchange
/// - Efficient customer communication
/// - Streamlined compliance reporting
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT950 format remains unchanged in SRG2025
/// - **Validation Updates**: Additional validation for statement accuracy and completeness
/// - **Processing Improvements**: Improved support for digital banking integration
/// - **Compliance Notes**: Enhanced support for high-volume automated processing
///
/// ## Integration Considerations
/// - **Banking Systems**: Efficient integration with core banking platforms and statement processing
/// - **Customer Systems**: Streamlined input for customer financial management systems
/// - **API Integration**: Optimized for modern API-based banking services and digital platforms
/// - **Compliance Integration**: Simplified compliance and audit trail maintenance requirements
///
/// ## Relationship to Other Messages
/// - **Triggers**: Often triggered by MT920 (Request Message) for streamlined statement delivery
/// - **Responses**: Provides simplified alternative to MT940 when detailed information is not required
/// - **Related**: Works with other cash management and account reporting messages
/// - **Alternatives**: MT940 for detailed transaction information when comprehensive reporting is needed
/// - **Status Updates**: Supports efficient account management and customer communication workflows

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT950_VALIDATION_RULES)]
pub struct MT950 {
    #[field("20")]
    pub field_20: Field20,

    #[field("25")]
    pub field_25: Field25NoOption,

    #[field("28C")]
    pub field_28c: Field28C,

    #[field("60")]
    pub field_60: Field60,

    #[field("61")]
    pub field_61: Option<Vec<Field61>>,

    #[field("62")]
    pub field_62: Field62,

    #[field("64")]
    pub field_64: Option<Field64>,
}

/// Validation rules for MT950 - Statement Message
const MT950_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "The first two characters of the three-character currency code in fields 60a, 62a, and 64 must be the same",
      "condition": {
        "and": [
          {"==": [
            {"substr": [
              {"if": [
                {"!!": {"var": "fields.60.F"}},
                {"var": "fields.60.F.currency"},
                {"var": "fields.60.M.currency"}
              ]}, 0, 2]},
            {"substr": [
              {"if": [
                {"!!": {"var": "fields.62.F"}},
                {"var": "fields.62.F.currency"},
                {"var": "fields.62.M.currency"}
              ]}, 0, 2]}
          ]},
          {
            "if": [
              {"!!": {"var": "fields.64"}},
              {"==": [
                {"substr": [
                  {"if": [
                    {"!!": {"var": "fields.60.F"}},
                    {"var": "fields.60.F.currency"},
                    {"var": "fields.60.M.currency"}
                  ]}, 0, 2]},
                {"substr": [{"var": "fields.64.currency"}, 0, 2]}
              ]},
              true
            ]
          }
        ]
      }
    }
  ]
}"#;
