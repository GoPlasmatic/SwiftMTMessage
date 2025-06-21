use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT920: Request Message
///
/// This message is used by a financial institution to request specific types of statements
/// or reports from another financial institution. This message enables automated
/// request processing for account statements, balance reports, and transaction
/// reports, facilitating efficient cash management and reconciliation processes.
///
/// ## Key Features
/// - **Statement requests**: Requesting MT940 (customer statement) or MT950 (statement message)
/// - **Balance reports**: Requesting MT941 (balance report)
/// - **Interim reports**: Requesting MT942 (interim transaction report)
/// - **Automated reporting**: Scheduled statement and report generation
/// - **Cash management**: Regular balance and transaction monitoring
/// - **Reconciliation**: Obtaining statements for reconciliation purposes
///
/// ## Field Structure
/// All fields follow the enhanced macro system with proper validation rules.
/// The message supports floor limit specification for MT942 requests.
///
/// ## Conditional Rules
/// - **C1**: If Field 12 = '942', Field 34F for debit or debit/credit must be present
/// - **C2**: When both Field 34F fields are present: first must have sign 'D', second must have sign 'C'
/// - **C3**: Currency code must be same across all Field 34F entries in a message
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT920_VALIDATION_RULES)]
pub struct MT920 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique sender's reference identifying this specific request message.
    /// Used throughout the request lifecycle for tracking, correlation with
    /// response messages, and audit purposes.
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    /// **Message Requested** - Field 12
    ///
    /// Specifies the type of SWIFT message being requested. This determines
    /// the format and content of the response message that will be generated.
    /// Valid values: 940, 941, 942, 950
    #[field("12", mandatory)]
    pub field_12: GenericTextField,

    /// **Account Identification** - Field 25
    ///
    /// Identifies the specific account for which the statement or report
    /// is being requested. Must be a valid account identifier that the
    /// receiver can process and generate reports for.
    #[field("25", mandatory)]
    pub field_25: GenericTextField,

    /// **Debit or Debit/Credit Floor Limit** - Field 34F (Optional, Conditional C1)
    ///
    /// Specifies the floor limit for debit transactions or combined debit/credit
    /// transactions when requesting MT942 interim transaction reports. Transactions
    /// above this limit will be included in the report.
    #[field("34F_DEBIT", optional)]
    pub field_34f_debit: Option<Field34F>,

    /// **Credit Floor Limit Indicator** - Field 34F (Optional, Conditional C2)
    ///
    /// Specifies the floor limit for credit transactions when requesting MT942
    /// interim transaction reports. Used in conjunction with debit floor limit
    /// to provide comprehensive transaction filtering.
    #[field("34F_CREDIT", optional)]
    pub field_34f_credit: Option<Field34F>,
}

/// Enhanced validation rules for MT920
const MT920_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "If message requested is 942, Field 34F for debit must be present",
      "condition": {
        "if": [
          {"==": [{"var": "field_12.value"}, "942"]},
          {"var": "field_34f_debit.is_some"},
          true
        ]
      }
    },
    {
      "id": "C2",
      "description": "When both 34F fields present: first must be 'D', second must be 'C'",
      "condition": {
        "if": [
          {
            "and": [
              {"var": "field_34f_debit.is_some"},
              {"var": "field_34f_credit.is_some"}
            ]
          },
          {
            "and": [
              {"==": [{"var": "field_34f_debit.sign"}, "D"]},
              {"==": [{"var": "field_34f_credit.sign"}, "C"]}
            ]
          },
          true
        ]
      }
    },
    {
      "id": "C3",
      "description": "Currency code must be same across all 34F entries",
      "condition": {
        "if": [
          {
            "and": [
              {"var": "field_34f_debit.is_some"},
              {"var": "field_34f_credit.is_some"}
            ]
          },
          {"==": [
            {"var": "field_34f_debit.currency"},
            {"var": "field_34f_credit.currency"}
          ]},
          true
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
      "id": "MESSAGE_TYPE_VALID",
      "description": "Message requested must be valid SWIFT MT type",
      "condition": {
        "in": [
          {"var": "field_12.value"},
          ["940", "941", "942", "950"]
        ]
      }
    },
    {
      "id": "REQUIRED_FIELDS",
      "description": "All mandatory fields must be present and non-empty",
      "condition": {
        "and": [
          {"!=": [{"var": "field_20.value"}, ""]},
          {"!=": [{"var": "field_12.value"}, ""]},
          {"!=": [{"var": "field_25.value"}, ""]}
        ]
      }
    }
  ]
}"#;
