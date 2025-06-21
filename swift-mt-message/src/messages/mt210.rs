use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT210: Notice to Receive
///
/// This message is used by a financial institution to notify another financial institution
/// of an impending debit to the sender's account held with the receiver, or to request
/// the receiver to provide funds to cover the debit. This message serves as advance
/// notice of funds requirements and facilitates liquidity management between institutions.
///
/// ## Key Features
/// - **Liquidity management**: Advance notice of funding requirements
/// - **Correspondent banking**: Notice of impending debits to nostro accounts
/// - **Cash management**: Coordination of funds availability
/// - **Settlement preparation**: Pre-funding for settlement obligations
///
/// ## Business Rules
/// - **Rule C1**: Message may include up to 10 notice sequences (if repeated)
/// - **Rule C2**: Either Field 50a or Field 52a must be present, not both
/// - **Rule C3**: Currency must be consistent in all 32B fields
/// - **Commodity restriction**: XAU, XAG, XPD, XPT must not be used
///
/// ## Structure
/// Simple flat structure with conditional fields based on ordering party type.
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT210_VALIDATION_RULES)]
pub struct MT210 {
    // Mandatory Fields
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField, // Transaction Reference Number

    #[field("21", mandatory)]
    pub field_21: GenericReferenceField, // Related Reference

    #[field("30", mandatory)]
    pub field_30: GenericTextField, // Value Date (YYMMDD)

    #[field("32B", mandatory)]
    pub field_32b: GenericCurrencyAmountField, // Currency and Amount

    // Optional Fields
    #[field("25", optional)]
    pub field_25: Option<GenericTextField>, // Account Identification

    // Conditional Fields (Rule C2 - exactly one must be present)
    #[field("50A", optional)]
    pub field_50a: Option<Field50>, // Ordering Customer (C, F options)

    #[field("52A", optional)]
    pub field_52a: Option<GenericBicField>, // Ordering Institution (A, D options)

    #[field("56A", optional)]
    pub field_56a: Option<GenericBicField>, // Intermediary Institution (A, D options)
}

/// Validation rules for MT210 - Notice to Receive
const MT210_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
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
      "id": "C2_RULE",
      "description": "Rule C2: Either Field 50a or Field 52a must be present, not both",
      "condition": {
        "xor": [
          {"var": "field_50a.is_some"},
          {"var": "field_52a.is_some"}
        ]
      }
    },
    {
      "id": "C3",
      "description": "Rule C3: Currency must be consistent in all 32B fields (single instance validation)",
      "condition": {
        "!=": [{"var": "field_32b.currency"}, ""]
      }
    },
    {
      "id": "C4",
      "description": "Related Reference (21) must not start or end with '/' and must not contain '//'",
      "condition": {
        "and": [
          {"!": {"matches": [{"var": "field_21.value"}, "^/"]}},
          {"!": {"matches": [{"var": "field_21.value"}, "/$"]}},
          {"!": {"matches": [{"var": "field_21.value"}, "//"]}}
        ]
      }
    },
    {
      "id": "C5",
      "description": "Value Date (30) must be valid YYMMDD format",
      "condition": {
        "matches": [
          {"var": "field_30.value"},
          "^[0-9]{6}$"
        ]
      }
    },
    {
      "id": "C6",
      "description": "Amount must be positive",
      "condition": {
        ">": [{"var": "field_32b.amount"}, 0]
      }
    },
    {
      "id": "C7",
      "description": "Commodity currencies (XAU, XAG, XPD, XPT) must not be used",
      "condition": {
        "!": {
          "in": [
            {"var": "field_32b.currency"},
            ["XAU", "XAG", "XPD", "XPT"]
          ]
        }
      }
    },
    {
      "id": "C8",
      "description": "Account identification format validation when present",
      "condition": {
        "if": [
          {"var": "field_25.is_some"},
          {"<=": [{"length": {"var": "field_25.value"}}, 35]},
          true
        ]
      }
    },
    {
      "id": "C9",
      "description": "Option F for Field 50a requires structured identity details",
      "condition": {
        "if": [
          {"var": "field_50a.is_some"},
          {"or": [
            {"var": "field_50a.is_option_c"},
            {"var": "field_50a.is_option_f"}
          ]},
          true
        ]
      }
    },
    {
      "id": "C10",
      "description": "Option D for Fields 52a/56a may include national clearing codes",
      "condition": {
        "if": [
          {"or": [
            {"var": "field_52a.is_some"},
            {"var": "field_56a.is_some"}
          ]},
          {"or": [
            {"var": "field_52a.is_option_a"},
            {"var": "field_52a.is_option_d"},
            {"var": "field_56a.is_option_a"},
            {"var": "field_56a.is_option_d"}
          ]},
          true
        ]
      }
    }
  ]
}"#;
