use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT941: Balance Report
///
/// This message is used by financial institutions to report account balance
/// information to their customers or correspondent banks. It provides a summary
/// of account balances at specific value dates without detailed transaction
/// information, making it ideal for balance monitoring and cash management.
///
/// ## Key Features
/// - **Balance reporting**: Summary of account balances at specific dates
/// - **Multi-date balances**: Forward value dates for liquidity planning
/// - **Cash management**: Real-time balance monitoring capabilities
/// - **Correspondent banking**: Inter-bank balance reporting
/// - **Treasury operations**: Daily balance reconciliation
/// - **Liquidity management**: Available funds tracking
///
/// ## Field Structure
/// All fields follow the enhanced macro system with proper validation rules.
/// The message supports repetitive balance lines for multiple value dates.
///
/// ## Business Rules
/// - All balance fields must use the same currency
/// - Forward balances must have value dates in the future
/// - Available balances reflect actual spendable funds
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT941_VALIDATION_RULES)]
pub struct MT941 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique reference for this balance report.
    /// Used for tracking and referencing this specific report.
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    /// **Related Reference** - Field 21 (Optional)
    ///
    /// Links to MT920 request if applicable.
    /// Provides connection to balance request that triggered this report.
    #[field("21", optional)]
    pub field_21: Option<GenericReferenceField>,

    /// **Account Identification** - Field 25
    ///
    /// IBAN or account identifier.
    /// Identifies the account for which balances are reported.
    #[field("25", mandatory)]
    pub field_25: GenericTextField,

    /// **Statement Number** - Field 28D
    ///
    /// Statement sequence number for tracking.
    /// Enables proper sequencing of balance reports.
    #[field("28D", mandatory)]
    pub field_28d: Field28D,

    /// **Opening Balance** - Field 60F
    ///
    /// Booked opening balance for the reporting period.
    /// Reference point for balance changes during the period.
    #[field("60F", mandatory)]
    pub field_60f: GenericBalanceField,

    /// **Balance Lines** (Repetitive)
    ///
    /// Forward balances at different value dates.
    /// Each line represents balance projection for specific dates.
    #[field("BALANCE_LINES", repetitive)]
    pub balance_lines: Vec<MT941BalanceLine>,

    /// **Closing Balance** - Field 62F
    ///
    /// Booked closing balance at end of reporting period.
    /// Final balance after all transactions for the period.
    #[field("62F", mandatory)]
    pub field_62f: GenericBalanceField,

    /// **Closing Available Balance** - Field 64 (Optional)
    ///
    /// Available funds at close of business.
    /// Shows actual spendable balance after reserves and holds.
    #[field("64", optional)]
    pub field_64: Option<GenericBalanceField>,
}

/// # MT941 Balance Line
///
/// Represents a forward balance at a specific value date.
/// Enhanced with SwiftMessage derive for automatic parsing and validation.
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT941_BALANCE_LINE_VALIDATION_RULES)]
pub struct MT941BalanceLine {
    /// **Forward Available Balance** - Field 65
    ///
    /// Available balance at specific future value date.
    /// Shows projected available funds considering pending transactions.
    #[field("65", mandatory)]
    pub field_65: GenericBalanceField,
}

/// Enhanced validation rules for MT941
const MT941_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "CURRENCY_CONSISTENCY",
      "description": "All balance fields must use the same currency",
      "condition": {
        "and": [
          {"==": [
            {"var": "field_60f.currency"},
            {"var": "field_62f.currency"}
          ]}
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
          {"var": "field_28d.is_valid"},
          {"var": "field_60f.is_valid"},
          {"var": "field_62f.is_valid"}
        ]
      }
    }
  ]
}"#;

/// Validation rules specific to MT941 balance lines
const MT941_BALANCE_LINE_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "BALANCE_LINE_VALID",
      "description": "Balance line must be valid",
      "condition": {
        "var": "field_65.is_valid"
      }
    }
  ]
}"#;
