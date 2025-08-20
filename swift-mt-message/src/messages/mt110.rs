use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT110: Advice of Cheque(s)
///
/// ## Purpose
/// Used to advise the receipt of cheque(s) for collection or deposit between financial institutions.
/// This message provides detailed information about each cheque including amounts, dates, and processing
/// instructions to facilitate efficient cheque clearing and collection processes.
///
/// ## Scope
/// This message is:
/// - Used for cheque collection advice between financial institutions
/// - Applicable for bulk cheque processing with individual cheque identification
/// - Designed for correspondent banking relationships in cheque clearing
/// - Compatible with domestic and cross-border cheque collection schemes
/// - Subject to cheque validation rules for proper clearing procedures
/// - Integrated with traditional paper-based and electronic cheque processing systems
///
/// ## Key Features
/// - **Comprehensive Cheque Details**: Individual cheque information with amounts, dates, and references
/// - **Correspondent Banking Support**: Fields for sender/receiver correspondents (53A/54A)
/// - **Collection Processing Framework**: Structured information for cheque collection and clearing
/// - **Bulk Operations Support**: Multiple cheques can be advised in a single message
/// - **Reference Tracking**: Unique identification for each cheque in the collection
/// - **Currency Validation**: Ensures all cheques in a message use the same currency
///
/// ## Common Use Cases
/// - Bank-to-bank cheque collection advice for deposited cheques
/// - Correspondent banking cheque clearing operations
/// - Cross-border cheque collection for foreign currency cheques
/// - Bulk cheque deposit processing for corporate customers
/// - Cheque truncation and electronic presentment advice
/// - Collection of returned or dishonored cheques
/// - High-value cheque collection requiring special handling
///
/// ## Message Structure
/// ### Message Header
/// - **Field 20**: Sender's Reference (mandatory) - Unique message identifier
/// - **Field 53A**: Sender's Correspondent (optional) - Sender's correspondent bank
/// - **Field 54A**: Receiver's Correspondent (optional) - Receiver's correspondent bank  
/// - **Field 72**: Sender to Receiver Information (optional) - Additional processing instructions
///
/// ### Cheque Details (Repetitive Sequence)
/// - **Field 21**: Cheque Reference (mandatory) - Unique reference for each cheque
/// - **Field 30**: Date of Issue (mandatory) - Date when cheque was issued (YYMMDD)
/// - **Field 32**: Currency/Amount (mandatory) - Cheque amount and currency code
/// - **Field 50**: Ordering Customer (optional) - Drawer/issuer of the cheque
/// - **Field 52**: Drawer Bank (optional) - Bank on which cheque is drawn
/// - **Field 59**: Beneficiary Customer (mandatory) - Payee of the cheque
///
/// ## Network Validation Rules
/// - **Cheque Quantity Limits**: Maximum 10 cheques per message for processing efficiency
/// - **Currency Consistency**: All cheques must have the same currency within a single message
/// - **Reference Uniqueness**: Each cheque reference must be unique within the message
/// - **Date Validation**: Date of issue must be in valid YYMMDD format
/// - **Reference Format**: Cheque references must not contain '/' or '//' characters
/// - **Minimum Requirements**: At least one cheque required per advice message
/// - **Correspondent Validation**: Proper BIC validation for correspondent bank fields
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT110 format remains stable for cheque processing
/// - **Validation Updates**: Enhanced validation rules for electronic cheque processing
/// - **Processing Improvements**: Improved handling of cheque truncation scenarios
/// - **Compliance Notes**: Maintained compatibility with traditional cheque clearing systems
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with cheque processing systems and clearing houses
/// - **API Integration**: RESTful API support for modern cheque collection platforms
/// - **Processing Requirements**: Supports both batch and real-time cheque collection processing
/// - **Compliance Integration**: Built-in validation for regulatory cheque processing requirements
///
/// ## Relationship to Other Messages
/// - **Triggers**: Often triggered by cheque deposit systems or collection processing
/// - **Responses**: May generate status messages or collection confirmation messages
/// - **Related**: Works with cheque clearing messages and account reporting systems
/// - **Alternatives**: Electronic payment messages for digital payment alternatives
/// - **Status Updates**: May receive notifications about cheque clearing success or failure
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT110_VALIDATION_RULES)]
pub struct MT110 {
    #[field("20")]
    pub field_20: Field20,

    #[field("53A")]
    pub field_53a: Option<Field53SenderCorrespondent>,

    #[field("54A")]
    pub field_54a: Option<Field54ReceiverCorrespondent>,

    #[field("72")]
    pub field_72: Option<Field72>,

    #[field("#")]
    pub cheques: Vec<MT110Cheque>,
}

/// MT110 Cheque (Repetitive Sequence)
///
/// ## Purpose
/// Represents individual cheque details within an MT110 advice message.
/// Each occurrence provides complete information for one cheque being advised.
///
/// ## Field Details
/// - **21**: Cheque Reference (mandatory) - Unique reference for this cheque
/// - **32B**: Currency/Amount - Cheque amount and currency
/// - **30**: Date - Cheque date or value date
/// - **25**: Account - Related account information
///
/// ## Validation Notes
/// - Each cheque must have a unique reference (21)
/// - Amount (32B) must be positive for valid cheque advice
/// - Date information (30) required for proper clearing
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT110Cheque {
    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("30")]
    pub field_30: Field30,

    #[field("32")]
    pub field_32: Field32,

    #[field("50")]
    pub field_50: Option<Field50OrderingCustomerAFK>,

    #[field("52")]
    pub field_52: Option<Field52DrawerBank>,

    #[field("59")]
    pub field_59: Field59,
}

/// Enhanced validation rules with forEach support for repetitive sequences
const MT110_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "The repetitive sequence (Cheque details) must not occur more than 10 times",
      "condition": {
        "<=": [{"length": {"var": "fields.#"}}, 10]
      }
    },
    {
      "id": "C2",
      "description": "The currency code in field 32a must be the same in all occurrences of that field",
      "condition": {
        "if": [
          {">=": [{"length": {"var": "fields.#"}}, 2]},
          {
            "all": [
              {"var": "fields.#"},
              {
                "==": [
                  {"var": "32.currency"},
                  {"var": "fields.#.0.32.currency"}
                ]
              }
            ]
          },
          true
        ]
      }
    },
    {
      "id": "CHQ_MIN",
      "description": "At least one cheque required",
      "condition": {
        ">=": [{"length": {"var": "fields.#"}}, 1]
      }
    }
  ]
}"#;
