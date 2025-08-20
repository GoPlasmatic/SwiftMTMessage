use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

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
pub struct MT942StatementLine {
    #[field("61")]
    pub field_61: Option<Field61>,

    #[field("86")]
    pub field_86: Option<Field86>,
}

/// Validation rules for MT942 - Interim Transaction Report
const MT942_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "The first two characters of the three-character currency code in fields 34F, 61, 90D, and 90C must be the same for all occurrences",
      "condition": {
        "and": [
          {"!!": {"var": "fields.34F#1"}},
          {
            "if": [
              {"!!": {"var": "fields.34F#2"}},
              {"==": [
                {"substr": [{"var": "fields.34F#1.currency"}, 0, 2]},
                {"substr": [{"var": "fields.34F#2.currency"}, 0, 2]}
              ]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.90D"}},
              {"==": [
                {"substr": [{"var": "fields.34F#1.currency"}, 0, 2]},
                {"substr": [{"var": "fields.90D.currency"}, 0, 2]}
              ]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.90C"}},
              {"==": [
                {"substr": [{"var": "fields.34F#1.currency"}, 0, 2]},
                {"substr": [{"var": "fields.90C.currency"}, 0, 2]}
              ]},
              true
            ]
          },
          {
            "if": [
              {">=": [{"length": {"var": "fields.#"}}, 1]},
              {
                "all": [
                  {"var": "fields.#"},
                  {
                    "if": [
                      {"!!": {"var": "61"}},
                      {"==": [
                        {"substr": [{"var": "fields.34F#1.currency"}, 0, 2]},
                        {"substr": [{"var": "61.currency"}, 0, 2]}
                      ]},
                      true
                    ]
                  }
                ]
              },
              true
            ]
          }
        ]
      }
    }
  ]
}"#;
