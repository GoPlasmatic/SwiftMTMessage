use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT935: Rate Change Advice
///
/// This message is used by a financial institution to advise another financial institution
/// of a change in interest rates. This message is critical for managing interest
/// rate exposure, updating pricing models, and ensuring accurate interest calculations
/// across correspondent banking relationships and customer accounts.
///
/// ## Key Features
/// - **Interest rate updates**: Notifying changes in deposit or lending rates
/// - **Base rate changes**: Communicating central bank rate adjustments
/// - **Account-specific rates**: Updating rates for specific customer accounts
/// - **Product rate changes**: Modifying rates for specific banking products
/// - **Regulatory compliance**: Meeting rate disclosure requirements
/// - **Risk management**: Coordinating rate changes across institutions
///
/// ## Field Structure
/// All fields follow the enhanced macro system with proper validation rules.
/// The message supports repetitive rate change sequences for bulk updates.
///
/// ## Conditional Rules
/// - **C1**: The repeating sequence of fields 23/25/30/37H must occur at least once and at most 10 times
/// - **C2**: Either Field 23 or Field 25 must be present in each sequence, but not both
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT935_VALIDATION_RULES)]
pub struct MT935 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique sender's reference for this rate change advice.
    /// Used for tracking and auditing rate change communications.
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    /// **Rate Change Sequences** (Repetitive)
    ///
    /// Each sequence represents one rate change with either rate type
    /// identification (Field 23) or account identification (Field 25),
    /// along with the effective date and new rate.
    #[field("RATE_CHANGES", repetitive)]
    pub rate_changes: Vec<MT935RateChange>,

    /// **Sender to Receiver Information** - Field 72 (Optional)
    ///
    /// Additional information about the rate changes.
    /// Can include structured text or narrative details.
    #[field("72", optional)]
    pub field_72: Option<GenericMultiLineTextField<6, 35>>,
}

/// # MT935 Rate Change Sequence
///
/// Represents a single rate change within an MT935 message.
/// Each sequence must have either Field 23 (rate type) OR Field 25 (account), but not both.
/// Enhanced with SwiftMessage derive for automatic parsing and validation as a sub-message structure.
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT935_RATE_CHANGE_VALIDATION_RULES)]
pub struct MT935RateChange {
    /// **Further Identification** - Field 23 (Conditional C2)
    ///
    /// Identifies the type of rate being changed (BASE, CALL, COMMERCIAL, etc.)
    /// Function codes: BASE, CALL, COMMERCIAL, CURRENT, DEPOSIT, NOTICE, PRIME
    #[field("23", optional)]
    pub field_23: Option<Field23>,

    /// **Account Identification** - Field 25 (Conditional C2)
    ///
    /// Identifies specific account for account-specific rate changes.
    /// Alternative to Field 23 when rate applies to individual account.
    #[field("25", optional)]
    pub field_25: Option<GenericTextField>,

    /// **Effective Date of New Rate** - Field 30 (Mandatory)
    ///
    /// When the new rate becomes effective (YYMMDD format).
    /// Must be a valid calendar date.
    #[field("30", mandatory)]
    pub field_30: GenericTextField,

    /// **New Interest Rate** - Field 37H (Mandatory)
    ///
    /// The new interest rate value with C/D indicator.
    /// Indicator: 'C' for credit rate, 'D' for debit rate.
    #[field("37H", mandatory)]
    pub field_37h: Field37H,
}

/// Enhanced validation rules for MT935
const MT935_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Rate change sequences must occur 1-10 times",
      "condition": {
        "and": [
          {">=": [{"length": {"var": "rate_changes"}}, 1]},
          {"<=": [{"length": {"var": "rate_changes"}}, 10]}
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
        "!=": [{"var": "field_20.value"}, ""]
      }
    }
  ]
}"#;

/// Validation rules specific to MT935 rate change sequences
const MT935_RATE_CHANGE_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C2",
      "description": "Either Field 23 or Field 25 must be present, but not both",
      "condition": {
        "or": [
          {
            "and": [
              {"var": "field_23.is_some"},
              {"var": "field_25.is_none"}
            ]
          },
          {
            "and": [
              {"var": "field_23.is_none"},
              {"var": "field_25.is_some"}
            ]
          }
        ]
      }
    },
    {
      "id": "REQUIRED_SEQUENCE_FIELDS",
      "description": "Effective date and new rate must be present and non-empty",
      "condition": {
        "and": [
          {"!=": [{"var": "field_30.value"}, ""]},
          {"var": "field_37h.is_valid"}
        ]
      }
    }
  ]
}"#;
