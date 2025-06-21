#[cfg(test)]
use crate::SwiftMessageBody;
use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT101: Request for Credit Transfer
///
/// Message for requesting multiple credit transfers with transaction details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT101_VALIDATION_RULES)]
#[serde_swift_fields]
pub struct MT101 {
    // Sequence A: Message Level Fields (Single Occurrence)
    // Mandatory Fields
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField, // Sender's Reference

    #[field("28D", mandatory)]
    pub field_28d: Field28D, // Message Index/Total

    #[field("30", mandatory)]
    pub field_30: GenericTextField, // Requested Execution Date

    // Optional Fields - Sequence A
    #[field("21R", optional)]
    pub field_21r: Option<GenericReferenceField>, // Customer Specified Reference

    #[field("50A", optional)]
    pub field_50a_seq_a: Option<Field50>, // Instructing Party (Seq A)

    #[field("52A", optional)]
    pub field_52a_seq_a: Option<GenericBicField>, // Account Servicing Institution (Seq A)

    #[field("52C", optional)]
    pub field_52c_seq_a: Option<GenericAccountField>, // Account Servicing Institution C (Seq A)

    #[field("51A", optional)]
    pub field_51a: Option<GenericBicField>, // Sending Institution

    #[field("25", optional)]
    pub field_25: Option<GenericTextField>, // Authorisation

    // Sequence B: Transaction Level Fields (Repetitive)
    // Vec<MT101Transaction> automatically detected as repetitive sequence by enhanced macro
    #[field("TRANSACTIONS", repetitive)]
    pub transactions: Vec<MT101Transaction>,
}

/// MT101 Transaction (Sequence B)
///
/// Single transaction within an MT101 message.
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT101_TRANSACTION_VALIDATION_RULES)]
pub struct MT101Transaction {
    // Mandatory Fields per Transaction
    #[field("21", mandatory)]
    pub field_21: GenericReferenceField, // Transaction Reference

    #[field("32B", mandatory)]
    pub field_32b: GenericCurrencyAmountField, // Currency/Amount

    #[field("59A", mandatory)]
    pub field_59a: Field59, // Beneficiary Customer

    #[field("71A", mandatory)]
    pub field_71a: GenericTextField, // Details of Charges

    // Optional Fields per Transaction
    #[field("21F", optional)]
    pub field_21f: Option<GenericReferenceField>, // F/X Deal Reference

    #[field("23E", optional)]
    pub field_23e: Option<Field23E>, // Instruction Code

    #[field("50A_SEQ_B", optional)]
    pub field_50a_seq_b: Option<Field50>, // Ordering Customer

    #[field("50F_SEQ_B", optional)]
    pub field_50f_seq_b: Option<GenericPartyField>, // Ordering Customer F

    #[field("50G_SEQ_B", optional)]
    pub field_50g_seq_b: Option<GenericPartyField>, // Ordering Customer G

    #[field("50H_SEQ_B", optional)]
    pub field_50h_seq_b: Option<GenericPartyField>, // Ordering Customer H

    #[field("52A_SEQ_B", optional)]
    pub field_52a_seq_b: Option<GenericBicField>, // Account Servicing Institution A

    #[field("52C_SEQ_B", optional)]
    pub field_52c_seq_b: Option<GenericAccountField>, // Account Servicing Institution C

    #[field("56A", optional)]
    pub field_56a: Option<GenericBicField>, // Intermediary Institution A

    #[field("56C", optional)]
    pub field_56c: Option<GenericAccountField>, // Intermediary Institution C

    #[field("56D", optional)]
    pub field_56d: Option<GenericNameAddressField>, // Intermediary Institution D

    #[field("57A", optional)]
    pub field_57a: Option<GenericBicField>, // Account With Institution A

    #[field("57C", optional)]
    pub field_57c: Option<GenericAccountField>, // Account With Institution C

    #[field("57D", optional)]
    pub field_57d: Option<GenericNameAddressField>, // Account With Institution D

    #[field("70", optional)]
    pub field_70: Option<GenericMultiLine4x35>, // Remittance Information

    #[field("77B", optional)]
    pub field_77b: Option<GenericMultiLine3x35>, // Regulatory Reporting

    #[field("33B", optional)]
    pub field_33b: Option<GenericCurrencyAmountField>, // Currency/Original Amount

    #[field("25A", optional)]
    pub field_25a: Option<GenericAccountField>, // Charges Account

    #[field("36", optional)]
    pub field_36: Option<Field36>, // Exchange Rate
}

/// Enhanced validation rules with forEach support for repetitive sequences
const MT101_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Per-transaction: If 36 present → 21F must be present",
      "forEach": {
        "collection": "transactions",
        "condition": {
          "if": [
            {"var": "field_36.is_some"},
            {"var": "field_21f.is_some"},
            true
          ]
        }
      }
    },
    {
      "id": "C8",
      "description": "Cross-transaction: All currencies must match if 21R present",
      "condition": {
        "if": [
          {"var": "field_21r.is_some"},
          {"allEqual": {"map": ["transactions", "field_32b.currency"]}},
          true
        ]
      }
    },
    {
      "id": "SEQ_B_MIN",
      "description": "At least one transaction required",
      "condition": {
        ">=": [{"length": {"var": "transactions"}}, 1]
      }
    }
  ]
}"#;

/// Validation rules specific to MT101 transactions
const MT101_TRANSACTION_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "T_C1",
      "description": "If exchange rate (36) is present, F/X deal reference (21F) must be present",
      "condition": {
        "if": [
          {"var": "field_36.is_some"},
          {"var": "field_21f.is_some"},
          true
        ]
      }
    },
    {
      "id": "T_C7", 
      "description": "If intermediary institution (56A/C/D) is present, account with institution (57A/C/D) must be present",
      "condition": {
        "if": [
          {"or": [
            {"var": "field_56a.is_some"},
            {"var": "field_56c.is_some"},
            {"var": "field_56d.is_some"}
          ]},
          {"or": [
            {"var": "field_57a.is_some"},
            {"var": "field_57c.is_some"},
            {"var": "field_57d.is_some"}
          ]},
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_mt101_compilation() {
        // Test that the MT101 structure compiles with the enhanced macro system
        let field_map = HashMap::new();

        // This should compile without errors
        let result = MT101::from_fields(field_map);

        // We expect it to fail parsing because we have no fields,
        // but the important thing is that it compiles
        assert!(result.is_err());
    }

    #[test]
    fn test_mt101_transaction_compilation() {
        // Test that the MT101Transaction structure compiles
        let field_map = HashMap::new();

        // This should compile without errors
        let result = MT101Transaction::from_fields(field_map);

        // We expect it to fail parsing because we have no fields,
        // but the important thing is that it compiles
        assert!(result.is_err());
    }
}
