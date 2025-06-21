use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT940: Customer Statement Message
///
/// This message is used by financial institutions to send customer account statements
/// containing transaction details and balance information. This message provides
/// a detailed view of account activity over a specific period.
///
/// ## Key Features
/// - **Account statements**: Detailed transaction history for customer accounts
/// - **Balance information**: Opening and closing balance details
/// - **Transaction details**: Individual transaction lines with narrative
/// - **Multi-part statements**: Support for statement sequencing
/// - **Available balance**: Optional closing available balance reporting
/// - **Reconciliation support**: Comprehensive data for account reconciliation
///
/// ## Field Structure
/// All fields follow the enhanced macro system with proper validation rules.
/// The message supports repetitive statement lines for multiple transactions.
///
/// ## Business Rules
/// - Opening balance (60F) and closing balance (62F) must be in consistent currency
/// - Each Field 61 (transaction line) may be followed by optional Field 86
/// - Balances use comma as decimal separator
/// - Statement supports multi-part statements via Field 28C
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT940_VALIDATION_RULES)]
pub struct MT940 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique ID for this statement, no leading/trailing slashes.
    /// Used for tracking and referencing this specific statement.
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    /// **Related Reference** - Field 21 (Optional)
    ///
    /// Links to MT920 if applicable.
    /// Provides connection to statement request that triggered this response.
    #[field("21", optional)]
    pub field_21: Option<GenericReferenceField>,

    /// **Account Identification** - Field 25
    ///
    /// IBAN or account identifier, BIC optional.
    /// Identifies the account for which this statement is provided.
    #[field("25", mandatory)]
    pub field_25: GenericTextField,

    /// **Statement/Sequence Number** - Field 28C
    ///
    /// Statement and sub-sequence numbers for multi-part statements.
    /// Enables tracking of statement parts and sequencing.
    #[field("28C", mandatory)]
    pub field_28c: Field28C,

    /// **Opening Balance** - Field 60F
    ///
    /// Booked opening balance at start of statement period.
    /// Must be consistent with currency used in closing balance.
    #[field("60F", mandatory)]
    pub field_60f: GenericBalanceField,

    /// **Statement Lines** (Repetitive)
    ///
    /// Transaction lines with optional accompanying Field 86.
    /// Each statement line represents one transaction with optional narrative.
    #[field("STATEMENT_LINES", repetitive)]
    pub statement_lines: Vec<MT940StatementLine>,

    /// **Closing Balance** - Field 62F
    ///
    /// Booked closing balance at end of statement period.
    /// Must be consistent with currency used in opening balance.
    #[field("62F", mandatory)]
    pub field_62f: GenericBalanceField,

    /// **Closing Available Balance** - Field 64 (Optional)
    ///
    /// Cash availability balance showing funds available for use.
    /// Provides additional liquidity information beyond booked balance.
    #[field("64", optional)]
    pub field_64: Option<GenericBalanceField>,

    /// **Forward Available Balance** - Field 65 (Optional)
    ///
    /// Value-dated available funds for future periods.
    /// Shows expected available balance considering future value dates.
    #[field("65", optional)]
    pub field_65: Option<GenericBalanceField>,
}

/// # MT940 Statement Line
///
/// Represents a single transaction line (Field 61) with optional
/// accompanying information (Field 86).
/// Enhanced with SwiftMessage derive for automatic parsing and validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT940_STATEMENT_LINE_VALIDATION_RULES)]
pub struct MT940StatementLine {
    /// **Statement Line** - Field 61
    ///
    /// Transaction details including value date, amount, and transaction type.
    /// Contains the core transaction information.
    #[field("61", mandatory)]
    pub field_61: Field61,

    /// **Info to Account Owner** - Field 86 (Optional)
    ///
    /// Narrative details for the transaction.
    /// Provides additional context and description for the transaction.
    #[field("86", optional)]
    pub field_86: Option<GenericMultiLineTextField<6, 65>>,
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
