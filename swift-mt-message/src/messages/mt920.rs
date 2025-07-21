use crate::fields::{field34::Field34F, *};
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT920: Request Message
///
/// ## Purpose
/// Used to request specific account information or statement messages from another financial
/// institution. This message allows institutions to request various types of account-related
/// data including balances, statements, and transaction details on a per-account basis.
///
/// ## Scope
/// This message is:
/// - Sent between financial institutions to request account information
/// - Used to request specific message types (MT940, MT941, MT942, MT950)
/// - Applied for correspondent banking and account servicing relationships
/// - Essential for account monitoring and reconciliation processes
/// - Part of automated cash management and reporting systems
///
/// ## Key Features
/// - **Message Type Specification**: Field 12 specifies the exact message type requested
/// - **Account-Specific Requests**: Individual account identification for targeted requests
/// - **Balance Requirements**: Specific balance information requirements using field 34F
/// - **Repetitive Structure**: Multiple account requests in a single message
/// - **Flexible Reporting**: Support for different statement and balance message types
/// - **Automated Processing**: Designed for systematic and automated information requests
///
/// ## Common Use Cases
/// - Requesting daily account statements (MT940)
/// - Obtaining balance and transaction reports (MT941)
/// - Requesting interim transaction statements (MT942)
/// - Getting periodic balance statements (MT950)
/// - Cash management system automation
/// - Correspondent banking account monitoring
/// - Regulatory reporting data collection
/// - Liquidity management and planning
///
/// ## Message Structure
/// ### Header Section
/// - **20**: Transaction Reference (mandatory) - Unique reference for this request
///
/// ### Repetitive Sequence (MT920Sequence)
/// Each sequence represents a request for a specific account and contains:
/// - **12**: Message Type Requested (mandatory) - MT940, MT941, MT942, or MT950
/// - **25**: Account Identification (mandatory) - Account for which information is requested
/// - **34F**: Amount Fields (optional) - Specific balance or amount requirements
///
/// ## Field 12 - Message Types Requested
/// Valid message types that can be requested:
/// - **940**: Customer Statement Message (detailed transaction statement)
/// - **941**: Balance Report Message (balance information with summary)
/// - **942**: Interim Transaction Report (interim statement with real-time updates)
/// - **950**: Statement Message (balance statement with transaction summary)
///
/// ## Field 34F - Amount Requirements
/// Optional field that can specify:
/// - **Debit Information**: When requesting debit balance details
/// - **Credit Information**: When requesting credit balance details
/// - **Currency Specification**: Specific currency for balance reporting
/// - **Threshold Amounts**: Minimum amounts for transaction reporting
///
/// ## Network Validation Rules
/// - **C1 Rule**: If message requested is 942, field 34F for debit must be present
/// - **C2 Rule**: When both 34F fields present, first must be 'D' (debit), second must be 'C' (credit)
/// - **C3 Rule**: Currency code must be consistent across all 34F entries
/// - **Message Type Validation**: Field 12 must contain valid SWIFT MT type (940, 941, 942, 950)
/// - **Reference Format**: Transaction references must follow SWIFT formatting standards
/// - **Required Fields**: All mandatory fields must be present and properly formatted
///
/// ## Processing Workflow
/// ### Request Processing
/// 1. MT920 sent with specific account and message type requests
/// 2. Receiving institution validates request parameters
/// 3. Requested information extracted from account systems
/// 4. Appropriate response message(s) generated and sent
/// 5. Requesting institution processes received information
///
/// ### Automated Integration
/// - Integration with cash management systems
/// - Scheduled automated requests for regular reporting
/// - Real-time balance monitoring capabilities
/// - Exception-based reporting triggers
///
/// ## SRG2025 Status
/// - **No Structural Changes**: MT920 format remains unchanged in SRG2025
/// - **Enhanced Validation**: Additional validation for request accuracy and completeness
/// - **Digital Integration**: Improved support for digital banking and API integration
/// - **Real-time Capabilities**: Enhanced support for real-time information requests
///
/// ## Integration Considerations
/// - **Banking Systems**: Direct integration with account management and core banking systems
/// - **Cash Management**: Essential component of comprehensive cash management solutions
/// - **API Gateway**: Often used in conjunction with modern API-based banking services
/// - **Reporting Systems**: Critical input for automated reporting and compliance systems
///
/// ## Relationship to Other Messages
/// - **Triggers**: MT940, MT941, MT942, MT950 response messages
/// - **Supports**: Account monitoring, cash management, and reconciliation processes
/// - **Complements**: Confirmation messages (MT900, MT910) for complete account visibility
/// - **Integrates with**: Broader cash management and treasury operation workflows

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT920_VALIDATION_RULES)]
pub struct MT920 {
    #[field("20")]
    pub field_20: Field20,

    #[field("#")]
    pub sequence: Vec<MT920Sequence>, // Sequence of Fields
}

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT920Sequence {
    #[field("12")]
    pub field_12: Field12,

    #[field("25")]
    pub field_25: Field25NoOption,

    #[field("34F")]
    pub field_34f_debit: Option<Field34F>,

    #[field("34F")]
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
