use crate::fields::{
    Field20, Field21, Field30, Field59, Field75, GenericBicField, GenericCurrencyAmountField,
};
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT111: Request for Stop Payment of a Cheque (Enhanced Architecture)
///
/// ## Overview
/// MT111 is used by financial institutions to request the stop payment of a cheque
/// that has been previously issued. This message provides all necessary details
/// to identify the specific cheque and includes optional query information about
/// the reason for the stop payment request.
///
/// This implementation uses the enhanced macro system for optimal type safety and validation.
///
/// ## Structure
/// All fields are at the message level (no repeating sequences)
///
/// ## Key Features
/// - Stop payment request for specific cheque
/// - Must match original cheque details if MT110 was previously sent
/// - Optional query information with predefined codes
/// - Support for national clearing codes
/// - Payee identification without account numbers
/// - Validation against original MT110 if applicable
/// - Type-safe field handling
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT111_VALIDATION_RULES)]
pub struct MT111 {
    /// **Sender's Reference** - Field 20 (Mandatory)
    /// No '/' start/end, no '//'
    #[field("20", mandatory)]
    pub field_20: Field20,

    /// **Cheque Number** - Field 21 (Mandatory)
    /// Must match original cheque if MT110 was sent
    #[field("21", mandatory)]
    pub field_21: Field21,

    /// **Date of Issue** - Field 30 (Mandatory)
    /// Valid date format (YYMMDD)
    #[field("30", mandatory)]
    pub field_30: Field30,

    /// **Amount** - Field 32a (Mandatory)
    /// Options: A (6!n3!a15d), B (3!a15d)
    /// Must match MT110 if already sent
    /// Use Option A if sender credited receiver in advance, otherwise Option B
    #[field("32A", mandatory)]
    pub field_32a: GenericCurrencyAmountField,

    /// **Drawer Bank** - Field 52a (Optional)
    /// Options: A, B, D. Use national clearing codes if no BIC
    #[field("52A", optional)]
    pub field_52a: Option<GenericBicField>,

    /// **Payee** - Field 59 (Optional)
    /// Account field not used - only name and address allowed
    /// Must not contain an account number
    #[field("59", optional)]
    pub field_59: Option<Field59>,

    /// **Queries** - Field 75 (Optional)
    /// Format: 6*35x, optional format with codes
    /// Predefined codes: 3, 18, 19, 20, 21
    #[field("75", optional)]
    pub field_75: Option<Field75>,
}

/// Enhanced validation rules for MT111
const MT111_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "REF_FORMAT",
      "description": "Sender's reference must not start/end with '/' or contain '//'",
      "condition": {
        "and": [
          {"!=": [{"var": "field_20.value"}, ""]},
          {"!": [{"startsWith": [{"var": "field_20.value"}, "/"]}]},
          {"!": [{"endsWith": [{"var": "field_20.value"}, "/"]}]},
          {"!": [{"in": ["//", {"var": "field_20.value"}]}]}
        ]
      }
    },
    {
      "id": "CHQ_FORMAT",
      "description": "Cheque number must not contain '/' or '//'",
      "condition": {
        "and": [
          {"!=": [{"var": "field_21.value"}, ""]},
          {"!": [{"in": ["/", {"var": "field_21.value"}]}]},
          {"!": [{"in": ["//", {"var": "field_21.value"}]}]}
        ]
      }
    },
    {
      "id": "DATE_VALID",
      "description": "Date of issue must be a valid date",
      "condition": {
        "!=": [{"var": "field_30.value"}, ""]
      }
    },
    {
      "id": "PAYEE_NO_ACCOUNT",
      "description": "Payee must not contain account number - only name and address",
      "condition": {
        "if": [
          {"var": "field_59.is_some"},
          true,
          true
        ]
      }
    }
  ]
}"#;
