use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT942: Interim Transaction Report
///
/// This message is used by financial institutions to send periodic interim
/// transaction reports containing summary information about account activity
/// within a specified period. Unlike MT940, this message focuses on transaction
/// summaries rather than detailed transaction lines.
///
/// ## Key Features
/// - **Interim reporting**: Regular transaction summaries between full statements
/// - **Transaction counts**: Summary of debit and credit transaction volumes
/// - **Floor limits**: Threshold-based reporting for significant transactions
/// - **Balance progression**: Opening and closing balance information
/// - **High-volume accounts**: Efficient reporting for accounts with many transactions
/// - **Cash management**: Regular monitoring of account activity
///
/// ## Field Structure
/// All fields follow the enhanced macro system with proper validation rules.
/// The message supports optional floor limit information and transaction summaries.
///
/// ## Business Rules
/// - All balance fields must use the same currency
/// - Transaction counts represent actual processed transactions
/// - Floor limits determine which transactions are included in summaries
/// - Entry counts should match the sum of individual transaction counts
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT942_VALIDATION_RULES)]
pub struct MT942 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique reference for this interim transaction report.
    /// Used for tracking and referencing this specific report.
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    /// **Related Reference** - Field 21 (Optional)
    ///
    /// Links to MT920 request if applicable.
    /// Provides connection to report request that triggered this response.
    #[field("21", optional)]
    pub field_21: Option<GenericReferenceField>,

    /// **Account Identification** - Field 25
    ///
    /// IBAN or account identifier for the reported account.
    /// Identifies the account for which transaction summary is provided.
    #[field("25", mandatory)]
    pub field_25: GenericTextField,

    /// **Statement Number** - Field 28C
    ///
    /// Statement sequence number and optional page number.
    /// Enables proper sequencing of interim reports.
    #[field("28C", mandatory)]
    pub field_28c: Field28C,

    /// **Floor Limit Indicator** - Field 34F (Optional)
    ///
    /// Minimum transaction amount for inclusion in the report.
    /// Transactions below this threshold may be excluded from summaries.
    #[field("34F", optional)]
    pub field_34f: Option<Field34F>,

    /// **Date/Time Indication** - Field 13D (Optional)
    ///
    /// Date and time when the report was generated.
    /// Provides timestamp for report generation context.
    #[field("13D", optional)]
    pub field_13d: Option<Field13D>,

    /// **Opening Balance** - Field 60F
    ///
    /// Booked opening balance at start of reporting period.
    /// Reference point for transaction summaries during the period.
    #[field("60F", mandatory)]
    pub field_60f: GenericBalanceField,

    /// **Sum of Debit Entries** - Field 90D (Optional)
    ///
    /// Total amount and count of debit transactions.
    /// Summarizes all debit activity during the reporting period.
    #[field("90D", optional)]
    pub field_90d: Option<Field90D>,

    /// **Sum of Credit Entries** - Field 90C (Optional)
    ///
    /// Total amount and count of credit transactions.
    /// Summarizes all credit activity during the reporting period.
    #[field("90C", optional)]
    pub field_90c: Option<Field90C>,

    /// **Closing Balance** - Field 62F
    ///
    /// Booked closing balance at end of reporting period.
    /// Final balance after all transactions during the period.
    #[field("62F", mandatory)]
    pub field_62f: GenericBalanceField,

    /// **Closing Available Balance** - Field 64 (Optional)
    ///
    /// Available funds at close of reporting period.
    /// Shows actual spendable balance after reserves and holds.
    #[field("64", optional)]
    pub field_64: Option<GenericBalanceField>,

    /// **Forward Available Balance** - Field 65 (Optional)
    ///
    /// Value-dated available balance for future periods.
    /// Shows projected available funds considering pending transactions.
    #[field("65", optional)]
    pub field_65: Option<GenericBalanceField>,

    /// **Info to Account Owner** - Field 86 (Optional)
    ///
    /// Additional narrative information about the report.
    /// Provides context or explanatory details for the transaction summary.
    #[field("86", optional)]
    pub field_86: Option<GenericMultiLineTextField<6, 65>>,
}

/// Enhanced validation rules for MT942
const MT942_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "CURRENCY_CONSISTENCY",
      "description": "All balance fields must use the same currency",
      "condition": {
        "and": [
          {"==": [
            {"var": "field_60f.currency"},
            {"var": "field_62f.currency"}
          ]},
          {
            "if": [
              {"var": "field_64.is_some"},
              {"==": [
                {"var": "field_60f.currency"},
                {"var": "field_64.currency"}
              ]},
              true
            ]
          },
          {
            "if": [
              {"var": "field_65.is_some"},
              {"==": [
                {"var": "field_60f.currency"},
                {"var": "field_65.currency"}
              ]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "ENTRY_CURRENCY_CONSISTENCY",
      "description": "Entry summaries must use same currency as balances",
      "condition": {
        "and": [
          {
            "if": [
              {"var": "field_90d.is_some"},
              {"==": [
                {"var": "field_60f.currency"},
                {"var": "field_90d.currency"}
              ]},
              true
            ]
          },
          {
            "if": [
              {"var": "field_90c.is_some"},
              {"==": [
                {"var": "field_60f.currency"},
                {"var": "field_90c.currency"}
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
          {"var": "field_60f.is_valid"},
          {"var": "field_62f.is_valid"}
        ]
      }
    }
  ]
}"#;
