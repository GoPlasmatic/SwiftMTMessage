use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT210: Notice to Receive
///
/// ## Purpose
/// Used to inform a financial institution that funds will be credited to their account.
/// This message serves as advance notification of incoming funds, allowing the receiving
/// institution to prepare for and reconcile expected payments.
///
/// ## Scope
/// This message is:
/// - Sent by a financial institution to notify another institution of pending credits
/// - Used for liquidity management and cash flow planning
/// - Applicable to various payment scenarios requiring advance notification
/// - Essential for correspondent banking relationships
/// - Used in settlement systems requiring pre-advice of incoming funds
///
/// ## Key Features
/// - **Pre-Advice Functionality**: Advance notification of incoming payments
/// - **Repetitive Sequence Structure**: Multiple payment notifications in single message
/// - **Flexible Identification**: Either ordering customer (50) or ordering institution (52) required
/// - **Account Management**: Optional account identification for specific crediting
/// - **Multi-Currency Support**: Individual currency and amount per sequence
/// - **Correspondent Banking**: Support for intermediary institution chains
///
/// ## Common Use Cases
/// - Correspondent bank payment notifications
/// - Treasury department cash flow management
/// - Settlement system pre-advice messages
/// - Liquidity management notifications
/// - Expected payment confirmations
/// - Cross-border payment notifications
/// - Multi-currency portfolio funding advice
///
/// ## Message Structure
/// ### Header Section
/// - **20**: Transaction Reference (mandatory) - Unique message identifier
/// - **25**: Account Identification (optional) - Specific account to be credited
/// - **30**: Value Date (mandatory) - Date when funds will be available
///
/// ### Repetitive Sequence (Multiple entries allowed)
/// - **21**: Related Reference (mandatory) - Reference to related payment/transaction
/// - **32B**: Currency and Amount (mandatory) - Currency code and amount to be credited
/// - **50**: Ordering Customer (optional) - Customer initiating the payment
/// - **52**: Ordering Institution (optional) - Institution initiating the payment
/// - **56**: Intermediary Institution (optional) - Intermediary in payment chain
///
/// ## Network Validation Rules
/// - **C2 Rule**: Either field 50 (Ordering Customer) or field 52 (Ordering Institution) must be present, but not both
/// - **Currency Consistency**: All 32B fields should use consistent currency (when multiple sequences)
/// - **Reference Format**: Transaction and related references must follow SWIFT format rules
/// - **Date Validation**: Value date must be valid and properly formatted (YYMMDD)
/// - **Amount Validation**: All amounts must be positive and within acceptable ranges
/// - **Institution Codes**: BIC codes must be valid when institution fields are used
///
/// ## MT210Sequence Details
/// Each sequence within the message represents a separate payment notification and contains:
/// - Individual payment reference (field 21)
/// - Specific currency and amount (field 32B)
/// - Payment originator identification (either field 50 or 52)
/// - Optional intermediary institution details (field 56)
///
/// ## Processing Considerations
/// - **Advance Notice**: Typically sent before the actual payment message
/// - **Reconciliation**: Used for matching expected vs. actual payments
/// - **Liquidity Planning**: Enables receiving institution to plan cash positions
/// - **Settlement Timing**: Coordinates with payment system settlement cycles
/// - **Exception Handling**: Facilitates investigation of missing payments
///
/// ## SRG2025 Status
/// - **No Structural Changes**: MT210 format remains unchanged in SRG2025
/// - **Enhanced Validation**: Additional validation rules for payment notification accuracy
/// - **Cross-Border Compliance**: Enhanced validation for international payment notifications
/// - **Settlement Integration**: Improved integration with modern settlement systems
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with liquidity management and treasury systems
/// - **API Integration**: RESTful API support for modern cash management platforms
/// - **Processing Requirements**: Supports real-time notification and cash flow planning
/// - **Compliance Integration**: Built-in validation for correspondent banking requirements
///
/// ## Relationship to Other Messages
/// - **Triggers**: Often precedes actual payment messages like MT202, MT103, or MT205
/// - **Responses**: May generate acknowledgment or status confirmation messages
/// - **Related**: Works with account reporting messages and settlement confirmations
/// - **Alternatives**: Direct payment messages for immediate transfer without pre-advice
/// - **Status Updates**: Enables reconciliation of expected vs. actual payments received

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT210_VALIDATION_RULES)]
pub struct MT210 {
    #[field("20")]
    pub field_20: Field20, // Transaction Reference Number

    #[field("25")]
    pub field_25: Option<Field25NoOption>, // Account Identification

    #[field("30")]
    pub field_30: Field30, // Value Date (YYMMDD)

    #[field("#")]
    pub sequence: Vec<MT210Sequence>, // Sequence of Fields
}

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT210_SEQUENCE_VALIDATION_RULES)]
pub struct MT210Sequence {
    #[field("21")]
    pub field_21: Field21NoOption, // Related Reference

    #[field("32B")]
    pub field_32b: Field32B, // Currency and Amount

    #[field("50")]
    pub field_50: Option<Field50OrderingCustomerNCF>, // Ordering Customer (C, F options)

    #[field("52")]
    pub field_52: Option<Field52OrderingInstitution>, // Ordering Institution (A, D options)

    #[field("56")]
    pub field_56: Option<Field56Intermediary>, // Intermediary Institution (A, D options)
}

/// Validation rules for MT210 - Notice to Receive
const MT210_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "The repetitive sequence (fields 50a-32B) must not appear more than 10 times",
      "condition": {
        "if": [
          {"!!": {"var": "fields.#"}},
          {"<=": [{"length": {"var": "fields.#"}}, 10]},
          true
        ]
      }
    }
  ]
}"#;

/// Validation rules for MT210Sequence - Individual sequence validation
const MT210_SEQUENCE_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C2",
      "description": "Either field 50a or field 52a must be present, but not both",
      "condition": {
        "or": [
          {
            "and": [
              {"!!": {"var": "fields.50"}},
              {"!": {"!!": {"var": "fields.52"}}}
            ]
          },
          {
            "and": [
              {"!": {"!!": {"var": "fields.50"}}},
              {"!!": {"var": "fields.52"}}
            ]
          }
        ]
      }
    }
  ]
}"#;
