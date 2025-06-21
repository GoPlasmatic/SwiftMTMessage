use crate::fields::{
    Field20, Field21, Field23E, Field26T, Field30, Field36, Field50, Field59, Field70, Field71A,
    Field72, Field77B, GenericBalanceField, GenericBicField, GenericCurrencyAmountField,
};
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT107: Request for Cancellation/Amendment (Enhanced Architecture)
///
/// ## Overview
/// MT107 is used by financial institutions to request cancellation or amendment
/// of previously sent direct debit instructions. It supports batch processing
/// of multiple transaction modifications with detailed settlement information.
///
/// This implementation uses the enhanced macro system with separate transaction
/// structures for optimal type safety and validation.
///
/// ## Structure
/// - **Sequence A**: General Information (mandatory, single occurrence)
/// - **Sequence B**: Transaction Details (mandatory, repetitive) - MT107Transaction struct
/// - **Sequence C**: Settlement Details (optional, single occurrence) - Individual fields
///
/// ## Key Features
/// - Multiple transaction modification support in single message
/// - Flexible creditor/debtor identification
/// - Optional settlement consolidation
/// - Comprehensive regulatory reporting
/// - Charge allocation options
/// - Amendment and cancellation instructions
/// - Type-safe transaction handling
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT107_VALIDATION_RULES)]
pub struct MT107 {
    // ================================
    // SEQUENCE A - GENERAL INFORMATION
    // ================================
    /// **Sender's Reference** - Field 20 (Mandatory)
    /// Unique ID assigned by the sender to identify this MT107 message.
    #[field("20", mandatory)]
    pub field_20: Field20,

    /// **Instruction Code** - Field 23E Seq A (Conditional)
    /// Values: AUTH, NAUT, OTHR, RTND (C1)
    #[field("23E", optional)]
    pub field_23e: Option<Field23E>,

    /// **Registration Reference** - Field 21E (Conditional)
    /// Optional ID. Subject to C2/C3
    #[field("21E", optional)]
    pub field_21e: Option<Field21>,

    /// **Requested Execution Date** - Field 30 (Mandatory)
    /// Format: YYMMDD
    #[field("30", mandatory)]
    pub field_30: Field30,

    /// **Sending Institution** - Field 51A (Optional)
    /// FileAct only
    #[field("51A", optional)]
    pub field_51a: Option<GenericBicField>,

    /// **Instructing Party** - Field 50a Seq A (Conditional)
    /// Options: C, L. Who orders debit. Subject to C2
    #[field("50A_INSTRUCTING", optional)]
    pub field_50a_instructing: Option<Field50>,

    /// **Creditor** - Field 50a Seq A (Conditional)
    /// Options: A, K. Name & account details. Subject to C1/C3
    #[field("50A_CREDITOR", optional)]
    pub field_50a_creditor: Option<Field50>,

    /// **Creditor's Bank** - Field 52a Seq A (Conditional)
    /// Options: A, C, D. Clearing/routing. Subject to C2
    #[field("52A", optional)]
    pub field_52a: Option<GenericBicField>,

    /// **Transaction Type Code** - Field 26T Seq A (Conditional)
    /// Purpose code. Subject to C2
    #[field("26T", optional)]
    pub field_26t: Option<Field26T>,

    /// **Regulatory Reporting** - Field 77B Seq A (Conditional)
    /// Statutory codes. Subject to C2
    #[field("77B", optional)]
    pub field_77b: Option<Field77B>,

    /// **Details of Charges** - Field 71A Seq A (Conditional)
    /// Values: BEN, OUR, SHA. Subject to C2
    #[field("71A", optional)]
    pub field_71a: Option<Field71A>,

    /// **Sender to Receiver Information** - Field 72 (Conditional)
    /// RTND required. Subject to C4
    #[field("72", optional)]
    pub field_72: Option<Field72>,

    // ================================
    // SEQUENCE B - TRANSACTION DETAILS (REPEATING)
    // ================================
    /// **Transaction Details** - Sequence B (Mandatory, Repetitive)
    /// Each entry represents one transaction to be cancelled/amended
    #[field("TRANSACTIONS", repetitive)]
    pub transactions: Vec<MT107Transaction>,

    // ================================
    // SEQUENCE C - SETTLEMENT DETAILS (OPTIONAL)
    // ================================
    /// **Settlement Amount** - Field 32B Seq C (Optional)
    /// Final amount including charges
    #[field("32B", optional)]
    pub field_32b: Option<GenericCurrencyAmountField>,

    /// **Sum of Amounts** - Field 19 (Conditional)
    /// If 32B not used. Subject to C8
    #[field("19", optional)]
    pub field_19: Option<GenericBalanceField>,

    /// **Sum of Sender's Charges** - Field 71F Seq C (Conditional)
    /// Totals from B blocks. Subject to C5
    #[field("71F", optional)]
    pub field_71f: Option<GenericCurrencyAmountField>,

    /// **Sum of Receiver's Charges** - Field 71G Seq C (Conditional)
    /// Totals from B blocks. Subject to C5
    #[field("71G", optional)]
    pub field_71g: Option<GenericCurrencyAmountField>,

    /// **Sender's Correspondent** - Field 53a (Optional)
    /// Options: A, B. Reimbursement branch
    #[field("53A", optional)]
    pub field_53a: Option<GenericBicField>,
}

/// # MT107 Transaction (Sequence B)
///
/// Represents a single transaction within an MT107 cancellation/amendment message.
/// This structure demonstrates the enhanced architecture for handling repetitive SWIFT sequences.
///
/// ## Architectural Benefits:
/// 1. **Complete Validation**: Each transaction validates all its fields independently
/// 2. **Memory Efficiency**: Only allocates fields that are present  
/// 3. **Type Safety**: Compile-time validation of field types
/// 4. **Business Logic**: Clear transaction-level operations and validation
/// 5. **Scalability**: Easy to add new transaction types or fields
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT107_TRANSACTION_VALIDATION_RULES)]
pub struct MT107Transaction {
    /// **Transaction Reference** - Field 21 (Mandatory)
    /// Unique reference for each transaction
    #[field("21", mandatory)]
    pub field_21: Field21,

    /// **Instruction Code** - Field 23E Seq B (Conditional)
    /// Values: AUTH, NAUT, OTHR (C1)
    #[field("23E", optional)]
    pub field_23e: Option<Field23E>,

    /// **Mandate Reference** - Field 21C (Optional)
    /// Used for mandates
    #[field("21C", optional)]
    pub field_21c: Option<Field21>,

    /// **Direct Debit Reference** - Field 21D (Optional)
    /// Used for returns
    #[field("21D", optional)]
    pub field_21d: Option<Field21>,

    /// **Registration Reference** - Field 21E Seq B (Conditional)
    /// Subject to C2/C3
    #[field("21E", optional)]
    pub field_21e: Option<Field21>,

    /// **Currency/Transaction Amount** - Field 32B (Mandatory)
    /// Amount to debit
    #[field("32B", mandatory)]
    pub field_32b: GenericCurrencyAmountField,

    /// **Instructing Party** - Field 50a Seq B (Conditional)
    /// Options: C, L. Who orders debit. Subject to C2
    #[field("50A_INSTRUCTING", optional)]
    pub field_50a_instructing: Option<Field50>,

    /// **Creditor** - Field 50a Seq B (Conditional)
    /// Options: A, K. Name & account details. Subject to C1/C3
    #[field("50A_CREDITOR", optional)]
    pub field_50a_creditor: Option<Field50>,

    /// **Creditor's Bank** - Field 52a Seq B (Conditional)
    /// Options: A, C, D. Routing bank. Subject to C2
    #[field("52A", optional)]
    pub field_52a: Option<GenericBicField>,

    /// **Debtor's Bank** - Field 57a (Optional)
    /// Options: A, C, D. Account servicing bank
    #[field("57A", optional)]
    pub field_57a: Option<GenericBicField>,

    /// **Debtor** - Field 59a (Mandatory)
    /// Must include account. Options: A/none
    #[field("59A", mandatory)]
    pub field_59a: Field59,

    /// **Remittance Information** - Field 70 (Optional)
    /// Details to debtor
    #[field("70", optional)]
    pub field_70: Option<Field70>,

    /// **Transaction Type Code** - Field 26T Seq B (Conditional)
    /// Reason for payment. Subject to C2
    #[field("26T", optional)]
    pub field_26t: Option<Field26T>,

    /// **Regulatory Reporting** - Field 77B Seq B (Conditional)
    /// Residence, codes. Subject to C2
    #[field("77B", optional)]
    pub field_77b: Option<Field77B>,

    /// **Original Ordered Amount** - Field 33B (Optional)
    /// Must differ from 32B
    #[field("33B", optional)]
    pub field_33b: Option<GenericCurrencyAmountField>,

    /// **Details of Charges** - Field 71A Seq B (Conditional)
    /// Values: BEN, OUR, SHA. Subject to C2
    #[field("71A", optional)]
    pub field_71a: Option<Field71A>,

    /// **Sender's Charges** - Field 71F (Conditional)
    /// Total sender charges. Subject to C5
    #[field("71F", optional)]
    pub field_71f: Option<GenericCurrencyAmountField>,

    /// **Receiver's Charges** - Field 71G (Conditional)
    /// Total receiver charges. Subject to C5
    #[field("71G", optional)]
    pub field_71g: Option<GenericCurrencyAmountField>,

    /// **Exchange Rate** - Field 36 (Conditional)
    /// Required if 33B ≠ 32B. Subject to C7
    #[field("36", optional)]
    pub field_36: Option<Field36>,
}

/// Enhanced validation rules with forEach support for repetitive sequences
const MT107_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "If 23E is AUTH/NAUT/OTHR in Seq A, same restriction applies to Seq B",
      "condition": {
        "if": [
          {"var": "field_23e.is_some"},
          {
            "forEach": {
              "collection": "transactions",
              "condition": {
                "if": [
                  {"var": "field_23e.is_some"},
                  {"in": [{"var": "field_23e.code"}, ["AUTH", "NAUT", "OTHR"]]},
                  true
                ]
              }
            }
          },
          true
        ]
      }
    },
    {
      "id": "C2",
      "description": "Instructing party appears in exactly one sequence",
      "condition": {
        "xor": [
          {"var": "field_50a_instructing.is_some"},
          {
            "any": {
              "map": ["transactions", "field_50a_instructing.is_some"]
            }
          }
        ]
      }
    },
    {
      "id": "C4",
      "description": "Field 72 required when 23E = RTND",
      "condition": {
        "if": [
          {"and": [
            {"var": "field_23e.is_some"},
            {"==": [{"var": "field_23e.code"}, "RTND"]}
          ]},
          {"var": "field_72.is_some"},
          true
        ]
      }
    },
    {
      "id": "TXN_MIN",
      "description": "At least one transaction required",
      "condition": {
        ">=": [{"length": {"var": "transactions"}}, 1]
      }
    }
  ]
}"#;

/// Validation rules specific to MT107 transactions
const MT107_TRANSACTION_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "T_C7",
      "description": "Exchange rate required when 33B differs from 32B",
      "condition": {
        "if": [
          {"and": [
            {"var": "field_33b.is_some"},
            {"!=": [{"var": "field_33b.amount"}, {"var": "field_32b.amount"}]}
          ]},
          {"var": "field_36.is_some"},
          true
        ]
      }
    },
    {
      "id": "T_REF",
      "description": "Transaction reference must be unique within the message",
      "condition": {
        "!=": [{"var": "field_21.value"}, ""]
      }
    }
  ]
}"#;
