use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT935: Rate Change Advice
///
/// ## Purpose
/// Used to advise changes in interest rates, exchange rates, or other financial rates that
/// affect existing agreements, accounts, or financial instruments. This message provides
/// formal notification of rate changes with effective dates and detailed rate information.
///
/// ## Scope
/// This message is:
/// - Sent by financial institutions to notify customers or correspondents of rate changes
/// - Used for interest rate changes on deposits, loans, and credit facilities
/// - Applied to foreign exchange rate notifications and updates
/// - Essential for pricing transparency and regulatory compliance
/// - Part of relationship management and customer communication processes
///
/// ## Key Features
/// - **Rate Change Notification**: Formal advice of rate modifications
/// - **Multiple Rate Changes**: Support for up to 10 rate changes in a single message
/// - **Effective Dating**: Precise effective dates for each rate change
/// - **Flexible Identification**: Either function code (field 23) or account (field 25) identification
/// - **Detailed Rate Information**: Comprehensive rate details using field 37H
/// - **Additional Information**: Optional narrative for context and explanations
///
/// ## Common Use Cases
/// - Interest rate changes on deposit accounts
/// - Loan and credit facility rate adjustments
/// - Foreign exchange rate updates for currency accounts
/// - Investment product rate notifications
/// - Central bank rate change implementations
/// - Correspondent banking rate adjustments
/// - Treasury and money market rate updates
/// - Regulatory rate change compliance notifications
///
/// ## Message Structure
/// ### Header Section
/// - **20**: Transaction Reference (mandatory) - Unique reference for this rate change advice
/// - **Rate Changes**: Repetitive sequence (1-10 occurrences) of rate change details
/// - **72**: Sender to Receiver Information (optional) - Additional context and explanations
///
/// ### Rate Change Sequence (MT935RateChange)
/// Each rate change sequence contains:
/// - **23**: Function Code (optional) - Type of rate change or product code
/// - **25**: Account Identification (optional) - Specific account affected by rate change
/// - **30**: Effective Date (mandatory) - Date when new rate becomes effective
/// - **37H**: New Rate (mandatory, repetitive) - Detailed rate information
///
/// ## Network Validation Rules
/// - **C1 Rule**: Rate change sequences must occur 1-10 times
/// - **C2 Rule**: Either field 23 (Function Code) or field 25 (Account) must be present, but not both
/// - **Reference Format**: Transaction references must follow SWIFT formatting standards
/// - **Required Fields**: All mandatory fields must be present and properly formatted
/// - **Date Validation**: Effective dates must be valid and properly formatted
/// - **Rate Validation**: Rate information must be complete and valid
///
/// ## Field 23 - Function Codes
/// When used, field 23 may contain codes such as:
/// - **DEPOSIT**: Interest rates for deposit products
/// - **LOAN**: Interest rates for lending products
/// - **FX**: Foreign exchange rates
/// - **CREDIT**: Credit facility rates
/// - **INVEST**: Investment product rates
/// - **MONEY**: Money market rates
///
/// ## Field 37H - Rate Information
/// Provides detailed rate information including:
/// - Rate type and classification
/// - Percentage rates or basis point changes
/// - Spread information over reference rates
/// - Tier-based or graduated rate structures
/// - Minimum and maximum rate constraints
///
/// ## Processing Context
/// ### Rate Change Implementation
/// 1. Rate change decision made by institution
/// 2. MT935 prepared with effective date and rate details
/// 3. Message sent to affected customers/correspondents
/// 4. Recipients update systems and communicate changes
/// 5. New rates become effective on specified date
///
/// ### Regulatory Compliance
/// - Documentation of rate change notifications
/// - Audit trail for regulatory review
/// - Customer communication requirements
/// - Transparency and disclosure obligations
///
/// ## SRG2025 Status
/// - **No Structural Changes**: MT935 format remains unchanged in SRG2025
/// - **Enhanced Validation**: Additional validation for rate accuracy and completeness
/// - **Digital Integration**: Improved support for automated rate change processing
/// - **Regulatory Compliance**: Enhanced support for regulatory reporting requirements
///
/// ## Integration Considerations
/// - **Banking Systems**: Direct integration with rate management and pricing systems
/// - **Customer Systems**: Input for customer treasury and financial management systems
/// - **Compliance Systems**: Essential for regulatory reporting and audit trail maintenance
/// - **Communication Platforms**: Integration with multi-channel customer notification systems
///
/// ## Relationship to Other Messages
/// - **Supports**: Rate-sensitive account management and transaction processing
/// - **Complements**: Statement messages (MT940, MT950) that reflect rate changes
/// - **Integrates with**: Customer communication and relationship management processes
/// - **Documentation**: Provides formal record of rate change notifications for compliance

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT935_VALIDATION_RULES)]
pub struct MT935 {
    #[field("20")]
    pub field_20: Field20,

    #[field("#")]
    pub rate_changes: Vec<MT935RateChange>,

    #[field("72")]
    pub field_72: Option<Field72>,
}

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT935RateChange {
    #[field("23")]
    pub field_23: Option<Field23>,

    #[field("25")]
    pub field_25: Option<Field25NoOption>,

    #[field("30")]
    pub field_30: Field30,

    #[field("37H")]
    pub field_37h: Vec<Field37H>,
}

/// Validation rules for MT935 - Rate Change Advice
const MT935_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "The repetitive sequence (fields 23/25 to 37H) must appear at least once but no more than ten times",
      "condition": {
        "and": [
          {">=": [{"length": {"var": "fields.#"}}, 1]},
          {"<=": [{"length": {"var": "fields.#"}}, 10]}
        ]
      }
    },
    {
      "id": "C2",
      "description": "In each repetitive sequence, either field 23 or field 25, but not both, must be present",
      "condition": {
        "none": [
          {"var": "fields.#"},
          {
            "and": [
              {"exists": ["fields", "23"]},
              {"exists": ["fields", "25"]}
            ]
          }
        ]
      }
    }
  ]
}"#;
