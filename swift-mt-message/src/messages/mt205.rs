use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT205: General Financial Institution Transfer
///
/// This message enables financial institutions to transfer funds between themselves for their own
/// account or for the account of their customers. Similar to MT202 but with key structural
/// differences: field 54a is not present and field 52a is always mandatory.
///
/// ## Key Differences from MT202
/// - **Field 54a**: Not present in MT205 (completely absent)
/// - **Field 52a**: Always mandatory (no fallback to sender BIC)
/// - **Settlement Logic**: Uses METAFCT003 (simplified scenarios)
/// - **Cover Detection**: Based on Sequence B presence
///
/// ## Message Variants
/// - **MT205**: Standard financial institution transfer
/// - **MT205.COV**: Cover message for customer credit transfers
/// - **MT205.REJT**: Rejection message
/// - **MT205.RETN**: Return message
///
/// ## Structure
/// - **Sequence A**: Bank-to-bank financial institution details
/// - **Sequence B**: Customer details (COV variant only)
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT205_VALIDATION_RULES)]
pub struct MT205 {
    // Sequence A: Mandatory Fields
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField, // Transaction Reference Number

    #[field("21", mandatory)]
    pub field_21: GenericReferenceField, // Related Reference

    #[field("32A", mandatory)]
    pub field_32a: Field32A, // Value Date/Currency/Amount

    #[field("52A", mandatory)]
    pub field_52a: GenericBicField, // Ordering Institution (MANDATORY in MT205)

    #[field("58A", mandatory)]
    pub field_58a: GenericBicField, // Beneficiary Institution

    // Sequence A: Optional Fields
    #[field("13C", optional)]
    pub field_13c: Option<Vec<Field13C>>, // Time Indication (repetitive)

    #[field("53A", optional)]
    pub field_53a: Option<GenericBicField>, // Sender's Correspondent

    #[field("56A", optional)]
    pub field_56a: Option<GenericBicField>, // Intermediary Institution

    #[field("57A", optional)]
    pub field_57a: Option<GenericBicField>, // Account With Institution

    #[field("72", optional)]
    pub field_72: Option<GenericMultiLine6x35>, // Sender to Receiver Information

    // Sequence B: COV Cover Message Fields (Optional)
    #[field("50A", optional)]
    pub field_50a: Option<Field50>, // Ordering Customer

    #[field("52A_SEQ_B", optional)]
    pub field_52a_seq_b: Option<GenericBicField>, // Ordering Institution (Seq B)

    #[field("56A_SEQ_B", optional)]
    pub field_56a_seq_b: Option<GenericBicField>, // Intermediary Institution (Seq B)

    #[field("57A_SEQ_B", optional)]
    pub field_57a_seq_b: Option<GenericBicField>, // Account With Institution (Seq B)

    #[field("59A", optional)]
    pub field_59a: Option<GenericBicField>, // Beneficiary Customer

    #[field("70", optional)]
    pub field_70: Option<GenericMultiLine4x35>, // Remittance Information

    #[field("72_SEQ_B", optional)]
    pub field_72_seq_b: Option<GenericMultiLine6x35>, // Sender to Receiver Info (Seq B)

    #[field("33B", optional)]
    pub field_33b: Option<GenericCurrencyAmountField>, // Currency/Instructed Amount
}

/// Validation rules for MT205 - General Financial Institution Transfer
const MT205_VALIDATION_RULES: &str = r#"{
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
      "id": "C2",
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
      "id": "C3",
      "description": "Field 52a is mandatory in MT205 (no fallback to sender BIC)",
      "condition": {
        "!=": [{"var": "field_52a.bic"}, ""]
      }
    },
    {
      "id": "C4",
      "description": "Field 54a is not present in MT205 (structural difference from MT202)",
      "condition": true
    },
    {
      "id": "C5",
      "description": "Cover message detection based on Sequence B customer fields presence",
      "condition": {
        "if": [
          {"or": [
            {"var": "field_50a.is_some"},
            {"var": "field_59a.is_some"},
            {"var": "field_70.is_some"}
          ]},
          {"var": "field_52a_seq_b.is_some"},
          true
        ]
      }
    },
    {
      "id": "C6",
      "description": "Cross-currency validation: if 33B present, currency should differ from 32A",
      "condition": {
        "if": [
          {"var": "field_33b.is_some"},
          {"!=": [{"var": "field_33b.currency"}, {"var": "field_32a.currency"}]},
          true
        ]
      }
    },
    {
      "id": "C7",
      "description": "REJT/RETN indicator validation in field 72",
      "condition": {
        "if": [
          {"var": "field_72.is_some"},
          {"or": [
            {"!": {"matches": [{"var": "field_72.lines"}, "/REJT/"]}},
            {"!": {"matches": [{"var": "field_72.lines"}, "/RETN/"]}},
            true
          ]},
          true
        ]
      }
    },
    {
      "id": "C8",
      "description": "Time indication validation for CLS/TARGET timing",
      "condition": {
        "if": [
          {"var": "field_13c.is_some"},
          {"allValid": [
            {"var": "field_13c"},
            {"matches": [{"var": "time_code"}, "^(SNDTIME|RNCTIME|CLSTIME|TILTIME|FROTIME|REJTIME)$"]}
          ]},
          true
        ]
      }
    },
    {
      "id": "C9",
      "description": "Settlement method determination (METAFCT003 - simplified scenarios)",
      "condition": {
        "if": [
          {"var": "field_53a.is_some"},
          {"!=": [{"var": "field_53a.bic"}, {"var": "field_52a.bic"}]},
          true
        ]
      }
    }
  ]
}"#;
