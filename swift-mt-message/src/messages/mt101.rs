use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT101: Request for Credit Transfer
///
/// Message for requesting multiple credit transfers with transaction details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT101_VALIDATION_RULES)]
#[serde_swift_fields]
pub struct MT101 {
    #[field("20")]
    pub field_20: Field20, // Sender's Reference

    #[field("21R")]
    pub field_21r: Option<Field21R>, // Customer Specified Reference

    #[field("28D")]
    pub field_28d: Field28D, // Message Index/Total

    #[field("50")]
    pub field_50a_instructing_party: Option<Field50InstructingParty>, // Instructing Party

    #[field("50")]
    pub field_50a_ordering_customer: Option<Field50OrderingCustomerFGH>, // Ordering Customer

    #[field("52")]
    pub field_52a: Option<Field52AccountServicingInstitution>, // Account Servicing Institution (Seq A)

    #[field("51A")]
    pub field_51a: Option<Field51A>, // Sending Institution

    #[field("30")]
    pub field_30: Field30, // Requested Execution Date

    #[field("25")]
    pub field_25: Option<Field25NoOption>,

    #[field("#")]
    pub transactions: Vec<MT101Transaction>,
}

/// MT101 Transaction (Sequence B)
///
/// Single transaction within an MT101 message.
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT101_TRANSACTION_VALIDATION_RULES)]
pub struct MT101Transaction {
    #[field("21")]
    pub field_21: Field21NoOption, // Transaction Reference

    #[field("21F")]
    pub field_21f: Option<Field21F>, // F/X Deal Reference

    #[field("23E")]
    pub field_23e: Option<Vec<Field23E>>, // Instruction Code

    #[field("32B")]
    pub field_32b: Field32B, // Currency/Amount

    #[field("50")]
    pub field_50_instructing_party: Option<Field50InstructingParty>, // Instructing Party

    #[field("50")]
    pub field_50_ordering_customer: Option<Field50OrderingCustomerFGH>, // Ordering Customer

    #[field("52")]
    pub field_52: Option<Field52AccountServicingInstitution>, // Account Servicing Institution

    #[field("56")]
    pub field_56: Option<Field56Intermediary>, // Intermediary

    #[field("57")]
    pub field_57: Option<Field57AccountWithInstitution>, // Account With Institution

    #[field("59")]
    pub field_59: Field59, // Beneficiary Customer

    #[field("70")]
    pub field_70: Option<Field70>, // Remittance Information

    #[field("77B")]
    pub field_77b: Option<Field77B>, // Regulatory Reporting

    #[field("33B")]
    pub field_33b: Option<Field33B>, // Currency/Original Amount

    #[field("71A")]
    pub field_71a: Field71A, // Details of Charges

    #[field("25A")]
    pub field_25a: Option<Field25A>, // Charges Account

    #[field("36")]
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
