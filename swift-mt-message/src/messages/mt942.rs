use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT942: Interim Transaction Report
///
/// ## Purpose
/// Used to report interim account information including real-time or intraday transaction
/// details and balance updates. This message provides timely account information between
/// regular statement periods for enhanced cash management and liquidity monitoring.
///
/// ## Scope
/// This message is:
/// - Sent for real-time or intraday account reporting
/// - Used when immediate transaction visibility is required
/// - Applied for active cash management and treasury operations
/// - Essential for intraday liquidity management and position monitoring
/// - Part of real-time cash management and payment system integration
///
/// ## Key Features
/// - **Real-time Reporting**: Immediate transaction and balance information
/// - **Intraday Updates**: Multiple reports possible within a single business day
/// - **Balance Limits**: Credit and debit limit information for account management
/// - **Transaction Details**: Individual transaction entries with real-time processing
/// - **Summary Information**: Debit and credit entry summaries for quick analysis
/// - **Available Balance**: Current available balance for immediate decision making
///
/// ## Common Use Cases
/// - Intraday liquidity monitoring
/// - Real-time cash position management
/// - Payment system integration
/// - Overdraft and credit limit monitoring
/// - High-frequency trading account management
/// - Treasury operations requiring immediate visibility
/// - Risk management and exposure monitoring
/// - Automated cash sweeping and positioning
///
/// ## Field Structure
/// - **20**: Transaction Reference (mandatory) - Unique report reference
/// - **21**: Related Reference (optional) - Reference to related period or statement
/// - **25**: Account Identification (mandatory) - Account being reported
/// - **28C**: Statement Number/Sequence (mandatory) - Report numbering
/// - **34F**: Debit Floor Limit (mandatory) - Minimum debit amount for reporting
/// - **34F**: Credit Ceiling Limit (optional) - Maximum credit limit information
/// - **13D**: Date/Time Indication (mandatory) - Precise timing of report
/// - **Statement Lines**: Repetitive sequence of transaction details
/// - **90D**: Number/Sum of Debit Entries (optional) - Debit transaction summary
/// - **90C**: Number/Sum of Credit Entries (optional) - Credit transaction summary
/// - **86**: Information to Account Owner (optional) - Additional transaction information
///
/// ## Network Validation Rules
/// - **Currency Consistency**: All balance and limit fields must use consistent currency
/// - **Entry Currency Consistency**: Entry summaries must use same currency as balances
/// - **Reference Format**: Transaction references must follow SWIFT standards
/// - **Required Fields**: All mandatory fields must be present and properly formatted
/// - **Real-time Constraints**: Timing information must reflect current processing
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT942 format remains unchanged in SRG2025
/// - **Validation Updates**: Additional validation for real-time reporting accuracy
/// - **Processing Improvements**: Improved support for real-time banking platforms
/// - **Compliance Notes**: Enhanced support for modern payment system APIs
///
/// ## Integration Considerations
/// - **Banking Systems**: Real-time integration with payment processing and account management systems
/// - **Treasury Systems**: Critical input for intraday liquidity management and cash positioning
/// - **API Integration**: Essential for modern real-time banking and payment system integration
/// - **Risk Management**: Key component for real-time exposure monitoring and limit management
///
/// ## Relationship to Other Messages
/// - **Triggered by**: MT920 (Request Message) for real-time account information requests
/// - **Complements**: MT940 (daily statements) and MT941 (balance reports) with real-time updates
/// - **Supports**: Intraday liquidity management, payment processing, and real-time cash management
/// - **Integrates with**: Real-time payment systems, treasury platforms, and risk management systems

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT942_VALIDATION_RULES)]
pub struct MT942 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Option<Field21NoOption>,

    #[field("25")]
    pub field_25: Field25AccountIdentification,

    #[field("28C")]
    pub field_28c: Field28C,

    #[field("34F#1")]
    pub field_34f_debit_limit: Field34F,

    #[field("34F#2")]
    pub field_34f_credit_limit: Option<Field34F>,

    #[field("13D")]
    pub field_13d: Field13D,

    #[field("#")]
    pub statement_lines: Vec<MT942StatementLine>,

    #[field("90D")]
    pub field_90d: Option<Field90D>,

    #[field("90C")]
    pub field_90c: Option<Field90C>,

    #[field("86")]
    pub field_86: Option<Field86>,
}

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT942_STATEMENT_LINE_VALIDATION_RULES)]
pub struct MT942StatementLine {
    #[field("61")]
    pub field_61: Option<Field61>,

    #[field("86")]
    pub field_86: Option<Field86>,
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

/// Validation rules specific to MT942 statement lines
const MT942_STATEMENT_LINE_VALIDATION_RULES: &str = r#"{
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
