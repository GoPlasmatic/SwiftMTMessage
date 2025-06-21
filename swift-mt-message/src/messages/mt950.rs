use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT950: Statement Message
///
/// This message is used by financial institutions to send account statements
/// to correspondent banks or financial institutions for nostro account management.
/// Unlike MT940 which is used for customer statements, MT950 is specifically
/// designed for inter-bank statement reporting and nostro account reconciliation.
///
/// ## Key Features
/// - **Nostro account statements**: Inter-bank account statement reporting
/// - **Correspondent banking**: Statement exchange between financial institutions
/// - **Account reconciliation**: Detailed transaction history for reconciliation
/// - **Multi-currency support**: Statement reporting in various currencies
/// - **Transaction details**: Complete transaction information with narrative
/// - **Balance tracking**: Opening and closing balance information
///
/// ## Field Structure
/// All fields follow the enhanced macro system with proper validation rules.
/// The message supports repetitive statement lines for multiple transactions.
///
/// ## Business Rules
/// - All balance fields must use the same currency
/// - Each transaction line (Field 61) may have accompanying narrative (Field 86)
/// - Statement supports multi-part statements via Field 28C
/// - Balances use comma as decimal separator
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT950_VALIDATION_RULES)]
pub struct MT950 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique reference for this statement message.
    /// Used for tracking and referencing this specific statement.
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    /// **Related Reference** - Field 21 (Optional)
    ///
    /// Links to MT920 request if applicable.
    /// Provides connection to statement request that triggered this response.
    #[field("21", optional)]
    pub field_21: Option<GenericReferenceField>,

    /// **Account Identification** - Field 25
    ///
    /// IBAN or nostro account identifier.
    /// Identifies the correspondent account for which statement is provided.
    #[field("25", mandatory)]
    pub field_25: GenericTextField,

    /// **Statement/Sequence Number** - Field 28C
    ///
    /// Statement sequence number and optional page number.
    /// Enables proper sequencing of multi-part statements.
    #[field("28C", mandatory)]
    pub field_28c: Field28C,

    /// **Opening Balance** - Field 60F or 60M
    ///
    /// Opening balance at start of statement period.
    /// May be booked (60F) or interim (60M) balance.
    #[field("60", mandatory)]
    pub field_60: GenericBalanceField,

    /// **Statement Lines** (Repetitive)
    ///
    /// Transaction lines with optional accompanying narrative.
    /// Each line represents one transaction with optional Field 86.
    #[field("STATEMENT_LINES", repetitive)]
    pub statement_lines: Vec<MT950StatementLine>,

    /// **Closing Balance** - Field 62F or 62M
    ///
    /// Closing balance at end of statement period.
    /// May be booked (62F) or interim (62M) balance.
    #[field("62", mandatory)]
    pub field_62: GenericBalanceField,

    /// **Closing Available Balance** - Field 64 (Optional)
    ///
    /// Available funds at close of statement period.
    /// Shows actual spendable balance for the nostro account.
    #[field("64", optional)]
    pub field_64: Option<GenericBalanceField>,

    /// **Forward Available Balance** - Field 65 (Optional)
    ///
    /// Value-dated available balance for future periods.
    /// Shows projected available funds considering pending transactions.
    #[field("65", optional)]
    pub field_65: Option<GenericBalanceField>,
}

/// # MT950 Statement Line
///
/// Represents a single transaction line (Field 61) with optional
/// accompanying information (Field 86) for nostro account statements.
/// Enhanced with SwiftMessage derive for automatic parsing and validation.
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT950_STATEMENT_LINE_VALIDATION_RULES)]
pub struct MT950StatementLine {
    /// **Statement Line** - Field 61
    ///
    /// Transaction details including value date, amount, and transaction type.
    /// Contains the core transaction information for nostro account activity.
    #[field("61", mandatory)]
    pub field_61: Field61,

    /// **Info to Account Owner** - Field 86 (Optional)
    ///
    /// Narrative details for the transaction.
    /// Provides additional context and description for nostro transactions.
    #[field("86", optional)]
    pub field_86: Option<GenericMultiLineTextField<6, 65>>,
}

/// Enhanced validation rules for MT950
const MT950_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "CURRENCY_CONSISTENCY",
      "description": "Opening and closing balances must have consistent currency",
      "condition": {
        "==": [
          {"var": "field_60.currency"},
          {"var": "field_62.currency"}
        ]
      }
    },
    {
      "id": "AVAILABLE_BALANCE_CURRENCY",
      "description": "Available balances must use same currency as main balances",
      "condition": {
        "and": [
          {
            "if": [
              {"var": "field_64.is_some"},
              {"==": [
                {"var": "field_60.currency"},
                {"var": "field_64.currency"}
              ]},
              true
            ]
          },
          {
            "if": [
              {"var": "field_65.is_some"},
              {"==": [
                {"var": "field_60.currency"},
                {"var": "field_65.currency"}
              ]},
              true
            ]
          }
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
          {"var": "field_60.is_valid"},
          {"var": "field_62.is_valid"}
        ]
      }
    }
  ]
}"#;

/// Validation rules specific to MT950 statement lines
const MT950_STATEMENT_LINE_VALIDATION_RULES: &str = r#"{
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
