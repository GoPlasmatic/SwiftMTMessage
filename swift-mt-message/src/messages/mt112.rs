use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT112: Status of Request for Stop Payment of a Cheque
///
/// This message is used by financial institutions to communicate the status of a stop payment
/// request that was previously submitted via MT111. It provides confirmation, rejection, or
/// status updates regarding the processing of the stop payment request.
///
/// ## Structure
/// Simple flat structure with no repeating sequences - all fields are at message level.
///
/// ## Key Features
/// - Status response to MT111 stop payment requests
/// - References original stop payment request details  
/// - Provides detailed status information and reasons
/// - Support for partial processing scenarios
/// - Optional additional correspondence information
/// - Maintains audit trail for stop payment lifecycle
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT112_VALIDATION_RULES)]
pub struct MT112 {
    // Mandatory Fields
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField, // Transaction Reference Number

    #[field("21", mandatory)]
    pub field_21: GenericReferenceField, // Cheque Number

    #[field("30", mandatory)]
    pub field_30: GenericTextField, // Date of Issue (YYMMDD)

    #[field("32A", mandatory)]
    pub field_32a: GenericCurrencyAmountField, // Amount

    #[field("76", mandatory)]
    pub field_76: GenericMultiLine6x35, // Answers (Status Information)

    // Optional Fields
    #[field("52A", optional)]
    pub field_52a: Option<GenericBicField>, // Drawer Bank A

    #[field("52B", optional)]
    pub field_52b: Option<GenericPartyField>, // Drawer Bank B

    #[field("52D", optional)]
    pub field_52d: Option<GenericNameAddressField>, // Drawer Bank D

    #[field("59", optional)]
    pub field_59: Option<Field59>, // Payee (without account number)
}

/// Validation rules for MT112 - Status of Request for Stop Payment of a Cheque
const MT112_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Date of Issue (30) must be valid YYMMDD format",
      "condition": {
        "matches": [
          {"var": "field_30.value"},
          "^[0-9]{6}$"
        ]
      }
    },
    {
      "id": "C2", 
      "description": "Transaction Reference (20) must not start or end with '/' and must not contain '//'",
      "condition": {
        "and": [
          {"!": {"matches": [{"var": "field_20.value"}, "^/"]}},
          {"!": {"matches": [{"var": "field_20.value"}, "/$"]}},
          {"!": {"matches": [{"var": "field_20.value"}, "//"]}}
        ]
      }
    },
    {
      "id": "C3",
      "description": "Cheque Number (21) must not start or end with '/' and must not contain '//'", 
      "condition": {
        "and": [
          {"!": {"matches": [{"var": "field_21.value"}, "^/"]}},
          {"!": {"matches": [{"var": "field_21.value"}, "/$"]}},
          {"!": {"matches": [{"var": "field_21.value"}, "//"]}}
        ]
      }
    },
    {
      "id": "C4",
      "description": "Only one option of Drawer Bank (52A/B/D) may be present",
      "condition": {
        "<=": [
          {"+": [
            {"if": [{"var": "field_52a.is_some"}, 1, 0]},
            {"if": [{"var": "field_52b.is_some"}, 1, 0]},
            {"if": [{"var": "field_52d.is_some"}, 1, 0]}
          ]},
          1
        ]
      }
    },
    {
      "id": "C5",
      "description": "Payee field (59) must not contain account number in first line",
      "condition": {
        "if": [
          {"var": "field_59.is_some"},
          {"!": {"matches": [{"var": "field_59.lines.0"}, "^/"]}},
          true
        ]
      }
    },
    {
      "id": "C6",
      "description": "Answers field (76) should contain predefined status codes when applicable",
      "condition": {
        "or": [
          {"matches": [{"var": "field_76.lines.0"}, "STOP PAYMENT"]},
          {"matches": [{"var": "field_76.lines.0"}, "REQUEST"]},
          {"matches": [{"var": "field_76.lines.0"}, "PROCESSING"]},
          {"matches": [{"var": "field_76.lines.0"}, "/[0-9]{2}/"]},
          true
        ]
      }
    }
  ]
}"#;
