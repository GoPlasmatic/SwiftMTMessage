use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT910: Confirmation of Credit
///
/// This message is used by a financial institution to confirm to another financial institution
/// that a credit has been made to the sender's account held with the receiver, or that
/// the sender's account held with a third party has been credited. This message serves
/// as official confirmation of credit transactions and facilitates reconciliation between
/// financial institutions.
///
/// ## Key Features
/// - **Credit confirmation**: Official confirmation of credit transactions
/// - **Account reconciliation**: Facilitates reconciliation between institutions
/// - **Audit trail**: Creates audit records for credit transactions
/// - **Settlement confirmation**: Confirms settlement credits
/// - **Liquidity management**: Account balance change notifications
///
/// ## Field Structure
/// All fields follow the enhanced macro system with proper validation rules.
/// The message supports both customer and institutional originator identification.
///
/// ## Conditional Rules
/// - **C1**: Either Field 50a or Field 52a must be present (not both)
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT910_VALIDATION_RULES)]
pub struct MT910 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique sender's reference identifying this specific credit confirmation.
    /// Used throughout the confirmation lifecycle for tracking, reconciliation, and audit.
    /// Must be unique within the sender's system per business day.
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    /// **Related Reference** - Field 21
    ///
    /// Reference to the original transaction or message that resulted in this credit.
    /// Should be copied unchanged from the original inward MT103/202 that triggered
    /// this credit confirmation.
    #[field("21", mandatory)]
    pub field_21: GenericReferenceField,

    /// **Account Identification** - Field 25a
    ///
    /// Identifies the specific account that has been credited. This account
    /// is typically held by the sender with the receiver, or with a third party
    /// as specified in the original transaction.
    #[field("25", mandatory)]
    pub field_25: GenericTextField,

    /// **Value Date, Currency, Amount** - Field 32A
    ///
    /// Core credit details specifying when the credit was effective, in what currency,
    /// and for what amount. The value date indicates when the credit actually
    /// took effect on the account.
    #[field("32A", mandatory)]
    pub field_32a: Field32A,

    /// **Date/Time Indication** - Field 13D (Optional)
    ///
    /// Provides precise timing information for when the credit was processed,
    /// including UTC offset for accurate time coordination across time zones.
    #[field("13D", optional)]
    pub field_13d: Option<Field13D>,

    /// **Ordering Customer** - Field 50a (Conditional C1)
    ///
    /// Identifies the customer who originated the transaction that resulted in this credit.
    /// This field provides customer-level traceability for the credit transaction.
    #[field("50", optional)]
    pub field_50a: Option<Field50>,

    /// **Ordering Institution** - Field 52a (Conditional C1)
    ///
    /// Identifies the financial institution of the ordering customer or the institution
    /// that ordered the transaction resulting in this credit. Alternative to Field 50a
    /// when institutional-level identification is more appropriate.
    #[field("52", optional)]
    pub field_52a: Option<GenericBicField>,

    /// **Intermediary** - Field 56a (Optional)
    ///
    /// Identifies the financial institution from which the sender received the funds
    /// that resulted in this credit. Used to document the routing chain and source
    /// of funds for audit and reconciliation purposes.
    #[field("56", optional)]
    pub field_56a: Option<GenericBicField>,

    /// **Sender to Receiver Information** - Field 72 (Optional)
    ///
    /// Free-format field for additional information about the credit transaction.
    /// Must contain narrative information only and may include structured codes
    /// for bilateral use or exchange rate information.
    #[field("72", optional)]
    pub field_72: Option<GenericMultiLineTextField<6, 35>>,
}

/// Enhanced validation rules for MT910
const MT910_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Either Field 50a or Field 52a must be present (not both)",
      "condition": {
        "or": [
          {
            "and": [
              {"var": "field_50a.is_some"},
              {"var": "field_52a.is_none"}
            ]
          },
          {
            "and": [
              {"var": "field_50a.is_none"},
              {"var": "field_52a.is_some"}
            ]
          }
        ]
      }
    },
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
      "description": "Credit amount must be positive",
      "condition": {
        ">": [{"var": "field_32a.amount"}, 0]
      }
    }
  ]
}"#;
