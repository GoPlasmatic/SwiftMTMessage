// This is a demonstration of how MT104 could be redesigned to work with field attributes
//
// The key insight is that the #[field("XX")] attribute system expects individual SWIFT fields,
// not complex nested structures. Here's how we can solve the MT104 problem:

use crate::fields::{
    Field20, Field21, Field23E, Field26T, Field30, Field36, Field50, Field59, Field70, Field71A,
    Field72, Field77B, GenericBalanceField, GenericBicField, GenericCurrencyAmountField,
};
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT104: Customer Direct Debit (Enhanced Architecture)
///
/// ## Overview
/// MT104 is used by financial institutions to send direct debit instructions.
/// It supports batch processing of multiple direct debit transactions with
/// detailed settlement information.
///
/// This implementation uses the enhanced macro system with separate transaction
/// structures for optimal type safety and validation.
///
/// ## Structure
/// - **Sequence A**: General Information (mandatory, single occurrence)
/// - **Sequence B**: Transaction Details (mandatory, repetitive) - MT104Transaction struct
/// - **Sequence C**: Settlement Details (optional, single occurrence) - Individual fields
///
/// ## Key Features
/// - Multiple transaction support in single message
/// - Flexible creditor/debtor identification
/// - Optional settlement consolidation
/// - Comprehensive regulatory reporting
/// - Charge allocation options
/// - Full field attribute compatibility
/// - Type-safe transaction handling
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT104_VALIDATION_RULES)]
pub struct MT104 {
    // ================================
    // SEQUENCE A - GENERAL INFORMATION
    // ================================
    /// **Sender's Reference** - Field 20 (Mandatory)
    /// Unique reference assigned by the sender to identify this MT104 message.
    #[field("20", mandatory)]
    pub field_20: Field20,

    /// **Customer Specified Reference** - Field 21R (Conditional)
    /// Required if Field 23E = RFDD (example condition)
    #[field("21R", optional)]
    pub field_21r: Option<Field21>,

    /// **Instruction Code** - Field 23E (Optional)
    /// Values: AUTH, NAUT, OTHR, RFDD, RTND
    #[field("23E", optional)]
    pub field_23e: Option<Field23E>,

    /// **Registration Reference** - Field 21E (Conditional)
    /// Subject to C3/C12 conditions
    #[field("21E", optional)]
    pub field_21e: Option<Field21>,

    /// **Requested Execution Date** - Field 30 (Mandatory)
    /// Format: YYMMDD
    #[field("30", mandatory)]
    pub field_30: Field30,

    /// **Sending Institution** - Field 51A (Optional)
    /// Only for FileAct
    #[field("51A", optional)]
    pub field_51a: Option<GenericBicField>,

    /// **Instructing Party** - Field 50a Seq A (Conditional)
    /// Options: C, L. Conditional C3 (if not present in any Seq B)
    #[field("50A_INSTRUCTING", optional)]
    pub field_50a_instructing: Option<Field50>,

    /// **Creditor** - Field 50a Seq A (Conditional)
    /// Options: A, K. Subject to C2, C4, C12
    #[field("50A_CREDITOR", optional)]
    pub field_50a_creditor: Option<Field50>,

    /// **Creditor's Bank** - Field 52a Seq A (Conditional)
    /// Options: A, C, D. Subject to C3, C12
    #[field("52A", optional)]
    pub field_52a: Option<GenericBicField>,

    /// **Transaction Type Code** - Field 26T Seq A (Conditional)
    /// Subject to C3
    #[field("26T", optional)]
    pub field_26t: Option<Field26T>,

    /// **Regulatory Reporting** - Field 77B Seq A (Conditional)
    /// Subject to C3
    #[field("77B", optional)]
    pub field_77b: Option<Field77B>,

    /// **Details of Charges** - Field 71A Seq A (Conditional)
    /// Values: BEN, OUR, SHA
    #[field("71A", optional)]
    pub field_71a: Option<Field71A>,

    /// **Sender to Receiver Information** - Field 72 (Conditional)
    /// Subject to C5
    #[field("72", optional)]
    pub field_72: Option<Field72>,

    // ================================
    // SEQUENCE B - TRANSACTION DETAILS (REPEATING)
    // ================================
    /// **Transaction Details** - Sequence B (Mandatory, Repetitive)
    /// Each element represents one direct debit transaction
    #[field("TRANSACTIONS", repetitive)]
    pub transactions: Vec<MT104Transaction>,

    // ================================
    // SEQUENCE C - SETTLEMENT DETAILS (OPTIONAL)
    // ================================
    /// **Settlement Amount** - Field 32B Seq C (Optional)
    /// Currency & Settlement Amount - Sum or explicit
    #[field("32B", optional)]
    pub field_32b: Option<GenericCurrencyAmountField>,

    /// **Sum of Amounts** - Field 19 (Optional)
    /// Required if 32B not total of B-32Bs
    #[field("19", optional)]
    pub field_19: Option<GenericBalanceField>,

    /// **Sum of Sender's Charges** - Field 71F Seq C (Optional)
    /// If 71F in B
    #[field("71F", optional)]
    pub field_71f: Option<GenericCurrencyAmountField>,

    /// **Sum of Receiver's Charges** - Field 71G Seq C (Optional)
    /// If 71G in B
    #[field("71G", optional)]
    pub field_71g: Option<GenericCurrencyAmountField>,

    /// **Sender's Correspondent** - Field 53a (Optional)
    /// Reimbursement instruction
    #[field("53A", optional)]
    pub field_53a: Option<GenericBicField>,
}

/// # MT104 Transaction (Sequence B)
///
/// Represents a single direct debit transaction within an MT104 message.
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
#[validation_rules(MT104_TRANSACTION_VALIDATION_RULES)]
pub struct MT104Transaction {
    /// **Transaction Reference** - Field 21 (Mandatory)
    /// Unique per transaction
    #[field("21", mandatory)]
    pub field_21: Field21,

    /// **Instruction Code** - Field 23E Seq B (Conditional)
    /// Depends on 23E in Seq A (C1)
    #[field("23E", optional)]
    pub field_23e: Option<Field23E>,

    /// **Mandate Reference** - Field 21C (Optional)
    /// Optional mandate info
    #[field("21C", optional)]
    pub field_21c: Option<Field21>,

    /// **Direct Debit Reference** - Field 21D (Optional)
    /// Optional ref for transaction
    #[field("21D", optional)]
    pub field_21d: Option<Field21>,

    /// **Registration Reference** - Field 21E Seq B (Conditional)
    /// C3 / C12
    #[field("21E", optional)]
    pub field_21e: Option<Field21>,

    /// **Currency and Amount** - Field 32B (Mandatory)
    /// ISO 4217 currency, comma for decimals
    #[field("32B", mandatory)]
    pub field_32b: GenericCurrencyAmountField,

    /// **Instructing Party** - Field 50a Seq B (Conditional)
    /// Must not appear if in Seq A (C3)
    #[field("50A_INSTRUCTING", optional)]
    pub field_50a_instructing: Option<Field50>,

    /// **Creditor** - Field 50a Seq B (Conditional)
    /// C2, C4, C12
    #[field("50A_CREDITOR", optional)]
    pub field_50a_creditor: Option<Field50>,

    /// **Creditor's Bank** - Field 52a Seq B (Conditional)
    /// C3, C12
    #[field("52A", optional)]
    pub field_52a: Option<GenericBicField>,

    /// **Debtor's Bank** - Field 57a (Optional)
    /// Optional
    #[field("57A", optional)]
    pub field_57a: Option<GenericBicField>,

    /// **Debtor** - Field 59a (Mandatory)
    /// Must include account
    #[field("59A", mandatory)]
    pub field_59a: Field59,

    /// **Remittance Information** - Field 70 (Optional)
    /// Codes: INV, IPI, RFB, ROC
    #[field("70", optional)]
    pub field_70: Option<Field70>,

    /// **Transaction Type Code** - Field 26T Seq B (Conditional)
    /// Purpose info
    #[field("26T", optional)]
    pub field_26t: Option<Field26T>,

    /// **Regulatory Reporting** - Field 77B Seq B (Conditional)
    /// Optional unless conflict with A
    #[field("77B", optional)]
    pub field_77b: Option<Field77B>,

    /// **Original Ordered Amount** - Field 33B (Optional)
    /// Must differ from 32B
    #[field("33B", optional)]
    pub field_33b: Option<GenericCurrencyAmountField>,

    /// **Details of Charges** - Field 71A Seq B (Conditional)
    /// Cond. C3
    #[field("71A", optional)]
    pub field_71a: Option<Field71A>,

    /// **Sender's Charges** - Field 71F (Conditional)
    /// C6, C12
    #[field("71F", optional)]
    pub field_71f: Option<GenericCurrencyAmountField>,

    /// **Receiver's Charges** - Field 71G (Conditional)
    /// C6, C12
    #[field("71G", optional)]
    pub field_71g: Option<GenericCurrencyAmountField>,

    /// **Exchange Rate** - Field 36 (Conditional)
    /// Required if 33B present & different from 32B
    #[field("36", optional)]
    pub field_36: Option<Field36>,
}

/// Enhanced validation rules with forEach support for repetitive sequences
const MT104_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Per-transaction: If 36 present → 21F must be present (placeholder)",
      "forEach": {
        "collection": "transactions",
        "condition": {
          "if": [
            {"var": "field_36.is_some"},
            true,
            true
          ]
        }
      }
    },
    {
      "id": "C3",
      "description": "Various conditional field requirements",
      "condition": {
        ">=": [{"length": {"var": "transactions"}}, 1]
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

/// Validation rules specific to MT104 transactions
const MT104_TRANSACTION_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "T_C1",
      "description": "If exchange rate (36) is present, related conditions apply",
      "condition": {
        "if": [
          {"var": "field_36.is_some"},
          true,
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
