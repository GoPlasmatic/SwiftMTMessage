use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT941: Balance Report Message
///
/// ## Purpose
/// Used to report account balance information with summary details for a specific period.
/// This message provides balance reporting with optional transaction summaries and is typically
/// used for balance monitoring and cash management without detailed transaction information.
///
/// ## Scope
/// This message is:
/// - Sent by account servicing institutions for balance reporting
/// - Used for periodic balance reporting (daily, weekly, monthly)
/// - Applied when detailed transaction information is not required
/// - Essential for cash position monitoring and liquidity management
/// - Part of streamlined cash management and treasury operations
///
/// ## Key Features
/// - **Balance Focus**: Emphasis on balance information rather than transaction detail
/// - **Summary Information**: Optional transaction summaries without individual entries
/// - **Period Reporting**: Statement numbering and period identification
/// - **Available Balance**: Forward available balance information for cash planning
/// - **Simplified Structure**: Streamlined format for efficient balance reporting
/// - **Cash Management**: Optimized for automated cash management systems
///
/// ## Common Use Cases
/// - Daily balance reporting for cash management
/// - Automated liquidity monitoring
/// - Treasury position reporting
/// - Balance verification and confirmation
/// - Cash forecasting and planning support
/// - Correspondent banking balance monitoring
/// - Investment account balance reporting
/// - Multi-currency position reporting
///
/// ## Field Structure
/// - **20**: Transaction Reference (mandatory) - Unique report reference
/// - **21**: Related Reference (optional) - Reference to related period or statement
/// - **25**: Account Identification (mandatory) - Account being reported
/// - **28**: Statement Number (mandatory) - Report numbering and sequence
/// - **13D**: Date/Time Indication (optional) - Report timing information
/// - **60F**: Opening Balance (mandatory) - Starting balance for reporting period
/// - **90D**: Number/Sum of Debit Entries (optional) - Debit transaction summary
/// - **90C**: Number/Sum of Credit Entries (optional) - Credit transaction summary
/// - **62F**: Closing Balance (mandatory) - Ending balance for reporting period
/// - **64**: Available Balance (optional) - Available balance information
/// - **65**: Forward Available Balance (optional, repetitive) - Future balance projections
/// - **86**: Information to Account Owner (optional) - Additional balance information
///
/// ## Network Validation Rules
/// - **Currency Consistency**: All balance fields must use the same currency code
/// - **Reference Format**: Transaction references must follow SWIFT formatting standards
/// - **Required Fields**: All mandatory fields must be present and properly formatted
/// - **Date Validation**: All dates must be valid and properly sequenced
/// - **Amount Validation**: All amounts must be properly formatted with currency precision
/// - **Summary Consistency**: Entry summaries must be consistent with balance calculations
/// - **Account Validation**: Account identification must be valid and properly formatted
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT941 format remains unchanged in SRG2025
/// - **Validation Updates**: Additional validation for balance accuracy and completeness
/// - **Processing Improvements**: Improved support for automated balance reporting
/// - **Compliance Notes**: Enhanced capabilities for real-time balance delivery
///
/// ## Integration Considerations
/// - **Banking Systems**: Core component of balance reporting and cash management systems
/// - **Treasury Systems**: Primary input for automated treasury and liquidity management
/// - **API Integration**: Essential for modern digital banking and cash management platforms
/// - **Regulatory Reporting**: Critical for compliance and audit trail requirements
///
/// ## Relationship to Other Messages
/// - **Triggered by**: MT920 (Request Message) for on-demand balance reporting
/// - **Complements**: MT940 (detailed statements) and MT942 (interim reports)
/// - **Supports**: Cash management, treasury operations, and balance monitoring workflows
/// - **Integrates with**: Customer communication and digital banking platforms

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT941_VALIDATION_RULES)]
pub struct MT941 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Option<Field21NoOption>,

    #[field("25")]
    pub field_25: Field25AccountIdentification,

    #[field("28")]
    pub field_28: Field28,

    #[field("13D")]
    pub field_13d: Option<Field13D>,

    #[field("60F")]
    pub field_60f: Field60F,

    #[field("90D")]
    pub field_90d: Option<Field90D>,

    #[field("90C")]
    pub field_90c: Option<Field90C>,

    #[field("62F")]
    pub field_62f: Field62F,

    #[field("64")]
    pub field_64: Option<Field64>,

    #[field("65")]
    pub field_65: Vec<Field65>,

    #[field("86")]
    pub field_86: Option<Field86>,
}

/// Validation rules for MT941 - Balance Report Message
const MT941_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "The first two characters of the three-character currency code in fields 60F, 90D, 90C, 62F, 64, and 65 must be the same for all occurrences of these fields",
      "condition": {
        "and": [
          {"!!": {"var": "fields.60F"}},
          {"!!": {"var": "fields.62F"}},
          {"==": [
            {"substr": [{"var": "fields.60F.currency"}, 0, 2]},
            {"substr": [{"var": "fields.62F.currency"}, 0, 2]}
          ]},
          {
            "if": [
              {"!!": {"var": "fields.90D"}},
              {"==": [
                {"substr": [{"var": "fields.60F.currency"}, 0, 2]},
                {"substr": [{"var": "fields.90D.currency"}, 0, 2]}
              ]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.90C"}},
              {"==": [
                {"substr": [{"var": "fields.60F.currency"}, 0, 2]},
                {"substr": [{"var": "fields.90C.currency"}, 0, 2]}
              ]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.64"}},
              {"==": [
                {"substr": [{"var": "fields.60F.currency"}, 0, 2]},
                {"substr": [{"var": "fields.64.currency"}, 0, 2]}
              ]},
              true
            ]
          },
          {
            "if": [
              {">=": [{"length": {"var": "fields.65"}}, 1]},
              {
                "reduce": [
                  {"var": "fields.65"},
                  {
                    "and": [
                      {"var": "accumulator"},
                      {"==": [
                        {"substr": [{"var": "fields.60F.currency"}, 0, 2]},
                        {"substr": [{"var": "current.currency"}, 0, 2]}
                      ]}
                    ]
                  },
                  true
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
