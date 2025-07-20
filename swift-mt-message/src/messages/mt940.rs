use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT940: Customer Statement Message
///
/// ## Purpose
/// Used to transmit detailed account statement information from an account servicing institution
/// to the account holder. This message provides a complete transaction-by-transaction record
/// of account activity with opening and closing balances for a specific period.
///
/// ## Scope
/// This message is:
/// - Sent by account servicing institutions to account holders
/// - Used for regular account statement transmission (daily, weekly, monthly)
/// - Applied to various account types including current, savings, and foreign currency accounts
/// - Essential for account reconciliation and cash management
/// - Part of automated statement delivery and cash management systems
///
/// ## Key Features
/// - **Complete Transaction Detail**: Full transaction-by-transaction statement
/// - **Balance Information**: Opening and closing balances with currency consistency
/// - **Statement Line Details**: Individual transaction entries with references and descriptions
/// - **Value Dating**: Precise value dates for each transaction
/// - **Available Balance**: Optional available balance information for credit management
/// - **Information Lines**: Additional transaction details and narrative information
///
/// ## Common Use Cases
/// - Daily account statement delivery
/// - End-of-month statement transmission
/// - Real-time account activity reporting
/// - Cash management system integration
/// - Account reconciliation support
/// - Regulatory reporting and compliance
/// - Customer self-service portal integration
/// - Treasury management system feeds
///
/// ## Message Structure
/// ### Header Information
/// - **20**: Transaction Reference (mandatory) - Unique statement reference
/// - **21**: Related Reference (optional) - Reference to related statement or period
/// - **25**: Account Identification (mandatory) - Account being reported
/// - **28C**: Statement Number/Sequence (mandatory) - Statement numbering
/// - **60**: Opening Balance (mandatory) - Starting balance for statement period
///
/// ### Transaction Details
/// - **Statement Lines**: Repetitive sequence of individual transactions
/// - **62**: Closing Balance (mandatory) - Ending balance for statement period
/// - **64**: Available Balance (optional) - Available credit or debit balance
/// - **65**: Forward Available Balance (optional, repetitive) - Future available balances
/// - **86**: Information to Account Owner (optional) - Additional statement information
///
/// ### Statement Line Structure (MT940StatementLine)
/// Each statement line contains:
/// - **61**: Statement Line (optional) - Individual transaction details
/// - **86**: Information to Account Owner (optional) - Additional transaction information
///
/// ## Field Details
/// ### Field 60/62 - Balance Information
/// - **Currency**: ISO 4217 3-character currency code
/// - **Amount**: Balance amount with appropriate precision
/// - **Date**: Balance date (YYMMDD format)
/// - **Credit/Debit Indicator**: C (Credit) or D (Debit) balance
///
/// ### Field 61 - Statement Line
/// - **Value Date**: Date when transaction becomes effective
/// - **Entry Date**: Date when transaction was posted (optional)
/// - **Credit/Debit Mark**: C (Credit) or D (Debit) entry
/// - **Amount**: Transaction amount
/// - **Transaction Type**: SWIFT transaction type identification
/// - **Reference**: Transaction reference number
/// - **Account Servicing Institution Reference**: Bank's internal reference
///
/// ## Network Validation Rules
/// - **Currency Consistency**: Opening and closing balances must use the same currency
/// - **Reference Format**: Transaction references must follow SWIFT formatting standards
/// - **Required Fields**: All mandatory fields must be present and properly formatted
/// - **Balance Logic**: Closing balance should reflect opening balance plus/minus transactions
/// - **Date Validation**: All dates must be valid and in proper sequence
/// - **Amount Validation**: All amounts must be properly formatted with currency precision
///
/// ## Processing Context
/// ### Statement Generation
/// 1. Account activity accumulated over statement period
/// 2. Transactions sorted by value date and sequence
/// 3. Opening balance carried forward from previous statement
/// 4. MT940 generated with complete transaction detail
/// 5. Closing balance calculated and validated
///
/// ### Cash Management Integration
/// - Real-time balance updating
/// - Transaction categorization and analysis
/// - Cash flow forecasting input
/// - Reconciliation automation
/// - Exception reporting and investigation
///
/// ## SRG2025 Status
/// - **No Structural Changes**: MT940 format remains unchanged in SRG2025
/// - **Enhanced Validation**: Additional validation for statement accuracy and completeness
/// - **Digital Integration**: Improved support for digital banking and API integration
/// - **Real-time Capabilities**: Enhanced support for real-time statement delivery
///
/// ## Integration Considerations
/// - **Banking Systems**: Core component of account management and customer communication
/// - **Cash Management**: Primary input for automated cash management and forecasting
/// - **ERP Integration**: Critical for enterprise financial management and reconciliation
/// - **Regulatory Reporting**: Essential for compliance and audit trail requirements
///
/// ## Relationship to Other Messages
/// - **Triggered by**: MT920 (Request Message) for on-demand statement delivery
/// - **Complements**: MT900/MT910 confirmation messages for real-time transaction notification
/// - **Supports**: Complete account management and cash management workflows
/// - **Integrates with**: Customer communication and digital banking platforms

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT940_VALIDATION_RULES)]
pub struct MT940 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Option<Field21NoOption>,

    #[field("25")]
    pub field_25: Field25AccountIdentification,

    #[field("28C")]
    pub field_28c: Field28C,

    #[field("60")]
    pub field_60: Field60,

    #[field("#")]
    pub statement_lines: Vec<MT940StatementLine>,

    #[field("62")]
    pub field_62: Field62,

    #[field("64")]
    pub field_64: Option<Field64>,

    #[field("65")]
    pub field_65: Vec<Field65>,

    #[field("86")]
    pub field_86: Option<Field86>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT940_STATEMENT_LINE_VALIDATION_RULES)]
pub struct MT940StatementLine {
    #[field("61")]
    pub field_61: Option<Field61>,

    #[field("86")]
    pub field_86: Option<Field86>,
}

/// Enhanced validation rules for MT940
const MT940_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "CURRENCY_CONSISTENCY",
      "description": "Opening and closing balances must have consistent currency",
      "condition": {
        "==": [
          {"var": "field_60f.currency"},
          {"var": "field_62f.currency"}
        ]
      }
    },
    {
      "id": "REF_FORMAT",
      "description": "Transaction reference must not have invalid slash patterns",
      "condition": {
        "and": [
          {"!": {"startsWith": [{"var": "field_20.value"}, "/"]}},
          {"!": {"endsWith": [{"var": "field_20.value"}, "/"]}},
          {"!": {"includes": [{"var": "field_20.value"}, "//"]}}
        ]
      }
    },
    {
      "id": "REQUIRED_FIELDS",
      "description": "All mandatory fields must be present and non-empty",
      "condition": {
        "and": [
          {"!=": [{"var": "field_20.value"}, ""]},
          {"!=": [{"var": "field_25.value"}, ""]},
          {"var": "field_28c.is_valid"},
          {"var": "field_60f.is_valid"},
          {"var": "field_62f.is_valid"}
        ]
      }
    }
  ]
}"#;

/// Validation rules specific to MT940 statement lines
const MT940_STATEMENT_LINE_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "STATEMENT_LINE_VALID",
      "description": "Statement line must be valid",
      "condition": {
        "var": "field_61.is_valid"
      }
    }
  ]
}"#;
