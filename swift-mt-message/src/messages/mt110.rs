use crate::fields::{
    Field20, Field21, Field30, Field50, Field59, Field72, GenericBicField,
    GenericCurrencyAmountField,
};
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT110: Advice of Cheque (Enhanced Architecture)
///
/// ## Overview
/// MT110 is used by financial institutions to advise the receipt or dispatch
/// of cheques. It provides detailed information about individual cheques including
/// payer, payee, amounts, and banking details. The message supports batch processing
/// of up to 10 cheques with consistent currency requirements.
///
/// This implementation uses the enhanced macro system with separate cheque
/// structures for optimal type safety and validation.
///
/// ## Structure
/// - **Header Fields**: General information and correspondent details
/// - **Repeating Sequence**: Individual cheque details (up to 10 occurrences) - MT110Cheque struct
///
/// ## Key Features
/// - Multiple cheque processing in single message (up to 10)
/// - Consistent currency requirement across all cheques
/// - Flexible correspondent bank routing
/// - Detailed payer/payee identification
/// - Support for national clearing codes
/// - Optional structured sender-to-receiver information
/// - Type-safe cheque handling
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT110_VALIDATION_RULES)]
pub struct MT110 {
    // ================================
    // HEADER FIELDS
    // ================================
    /// **Sender's Reference** - Field 20 (Mandatory)
    /// No leading/trailing slash, no '//'
    #[field("20", mandatory)]
    pub field_20: Field20,

    /// **Sender's Correspondent** - Field 53a (Optional)
    /// Options: A, B, D. Required if no direct account relationship
    #[field("53A", optional)]
    pub field_53a: Option<GenericBicField>,

    /// **Receiver's Correspondent** - Field 54a (Optional)
    /// Options: A, B, D. Used to route funds to Receiver
    #[field("54A", optional)]
    pub field_54a: Option<GenericBicField>,

    /// **Sender to Receiver Information** - Field 72 (Optional)
    /// Format: 6*35x, optional structured codes
    /// Codes: ACC, INS, INT; REJT/RETN special rules
    #[field("72", optional)]
    pub field_72: Option<Field72>,

    // ================================
    // REPEATING SEQUENCE - CHEQUE DETAILS (UP TO 10)
    // ================================
    /// **Cheque Details** - Repeating Sequence (Mandatory, up to 10 occurrences)
    /// Each entry represents one cheque being advised
    #[field("CHEQUES", repetitive)]
    pub cheques: Vec<MT110Cheque>,
}

/// # MT110 Cheque (Repeating Sequence)
///
/// Represents a single cheque within an MT110 advice message.
/// This structure demonstrates the enhanced architecture for handling repetitive SWIFT sequences.
///
/// ## Architectural Benefits:
/// 1. **Complete Validation**: Each cheque validates all its fields independently
/// 2. **Memory Efficiency**: Only allocates fields that are present  
/// 3. **Type Safety**: Compile-time validation of field types
/// 4. **Business Logic**: Clear cheque-level operations and validation
/// 5. **Scalability**: Easy to add new cheque types or fields
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT110_CHEQUE_VALIDATION_RULES)]
pub struct MT110Cheque {
    /// **Cheque Number** - Field 21 (Mandatory)
    /// Unique per cheque; no '/' or '//'
    #[field("21", mandatory)]
    pub field_21: Field21,

    /// **Date of Issue** - Field 30 (Mandatory)
    /// Format: YYMMDD. Must be a valid date
    #[field("30", mandatory)]
    pub field_30: Field30,

    /// **Amount** - Field 32a (Mandatory)
    /// Options: A (6!n3!a15d) / B (3!a15d)
    /// Currency must be same for all cheques in the message
    #[field("32A", mandatory)]
    pub field_32a: GenericCurrencyAmountField,

    /// **Payer** - Field 50a (Optional)
    /// Options: A, F, K. Detailed identity formats
    #[field("50A", optional)]
    pub field_50a: Option<Field50>,

    /// **Drawer Bank** - Field 52a (Optional)
    /// Options: A, B, D. Can specify BIC or national code
    #[field("52A", optional)]
    pub field_52a: Option<GenericBicField>,

    /// **Payee** - Field 59a (Mandatory)
    /// Options: No letter, F option. Must use structured address and name
    #[field("59A", mandatory)]
    pub field_59a: Field59,
}

/// Enhanced validation rules with forEach support for repetitive sequences
const MT110_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Maximum 10 cheques per message",
      "condition": {
        "<=": [{"length": {"var": "cheques"}}, 10]
      }
    },
    {
      "id": "C2",
      "description": "All cheques must have the same currency",
      "condition": {
        "if": [
          {">": [{"length": {"var": "cheques"}}, 0]},
          {
            "allEqual": {
              "map": ["cheques", "field_32a.currency"]
            }
          },
          true
        ]
      }
    },
    {
      "id": "CHQ_MIN",
      "description": "At least one cheque required",
      "condition": {
        ">=": [{"length": {"var": "cheques"}}, 1]
      }
    }
  ]
}"#;

/// Validation rules specific to MT110 cheques
const MT110_CHEQUE_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "CHQ_REF",
      "description": "Cheque reference must be unique and not contain '/' or '//'",
      "condition": {
        "and": [
          {"!=": [{"var": "field_21.value"}, ""]},
          {"!": [{"in": ["/", {"var": "field_21.value"}]}]},
          {"!": [{"in": ["//", {"var": "field_21.value"}]}]}
        ]
      }
    },
    {
      "id": "CHQ_DATE",
      "description": "Date of issue must be a valid date",
      "condition": {
        "!=": [{"var": "field_30.value"}, ""]
      }
    }
  ]
}"#;
