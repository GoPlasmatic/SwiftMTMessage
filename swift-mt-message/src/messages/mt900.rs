use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT900: Confirmation of Debit
///
/// This message is used by a financial institution to confirm to another financial institution
/// that a debit has been made to the sender's account held with the receiver, or that
/// the sender's account held with a third party has been debited. This message serves
/// as official confirmation of debit transactions and facilitates reconciliation between
/// financial institutions.
///
/// ## Key Features
/// - **Debit confirmation**: Official confirmation of debit transactions
/// - **Account reconciliation**: Facilitates reconciliation between institutions
/// - **Audit trail**: Creates audit records for debit transactions
/// - **Settlement confirmation**: Confirms settlement debits
/// - **Liquidity management**: Account balance change notifications
///
/// ## Field Structure
/// All fields follow the enhanced macro system with proper validation rules.
/// The message provides comprehensive debit transaction confirmation capabilities.
///
/// ## Usage Guidelines
/// Used for ad-hoc confirmations of significant debit transactions requiring confirmation,
/// exception cases for problem resolution, and when audit trail documentation is required.
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT900_VALIDATION_RULES)]
pub struct MT900 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique sender's reference identifying this specific debit confirmation.
    /// Used throughout the confirmation lifecycle for tracking, reconciliation, and audit.
    /// Must be unique within the sender's system per business day.
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    /// **Related Reference** - Field 21
    ///
    /// Reference to the original transaction or message that resulted in this debit.
    /// Critical for linking the confirmation back to the initiating transaction
    /// and maintaining complete audit trails.
    #[field("21", mandatory)]
    pub field_21: GenericReferenceField,

    /// **Account Identification** - Field 25
    ///
    /// Identifies the specific account that has been debited. This account
    /// is typically held by the sender with the receiver, or with a third party
    /// as specified in the original transaction.
    #[field("25", mandatory)]
    pub field_25: GenericTextField,

    /// **Value Date, Currency, Amount** - Field 32A
    ///
    /// Core debit details specifying when the debit was effective, in what currency,
    /// and for what amount. The value date indicates when the debit actually
    /// took effect on the account.
    #[field("32A", mandatory)]
    pub field_32a: Field32A,

    /// **Date/Time Indication** - Field 13D (Optional)
    ///
    /// Provides precise timing information for when the debit was processed,
    /// including UTC offset for accurate time coordination across time zones.
    #[field("13D", optional)]
    pub field_13d: Option<Field13D>,

    /// **Ordering Institution** - Field 52a (Optional)
    ///
    /// Identifies the financial institution that ordered or initiated the
    /// transaction that resulted in this debit. May include additional
    /// clearing or routing information.
    #[field("52", optional)]
    pub field_52a: Option<GenericBicField>,

    /// **Sender to Receiver Information** - Field 72 (Optional)
    ///
    /// Free-format field for additional information about the debit transaction.
    /// May contain structured codes, exchange rate information, or narrative
    /// details relevant to the debit confirmation.
    #[field("72", optional)]
    pub field_72: Option<GenericMultiLineTextField<6, 35>>,
}

/// Enhanced validation rules for MT900
const MT900_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "REF_FORMAT",
      "description": "Transaction and related references must not have invalid slash patterns",
      "condition": {
        "and": [
          {"!": {"startsWith": [{"var": "field_20.value"}, "/"]}},
          {"!": {"endsWith": [{"var": "field_20.value"}, "/"]}},
          {"!": {"includes": [{"var": "field_20.value"}, "//"]}},
          {"!": {"startsWith": [{"var": "field_21.value"}, "/"]}},
          {"!": {"endsWith": [{"var": "field_21.value"}, "/"]}},
          {"!": {"includes": [{"var": "field_21.value"}, "//"]}}
        ]
      }
    },
    {
      "id": "AMOUNT_POSITIVE",
      "description": "Debit amount must be positive",
      "condition": {
        ">": [{"var": "field_32a.amount"}, 0]
      }
    },
    {
      "id": "REQUIRED_FIELDS",
      "description": "All mandatory fields must be present and non-empty",
      "condition": {
        "and": [
          {"!=": [{"var": "field_20.value"}, ""]},
          {"!=": [{"var": "field_21.value"}, ""]},
          {"!=": [{"var": "field_25.value"}, ""]}
        ]
      }
    }
  ]
}"#;
