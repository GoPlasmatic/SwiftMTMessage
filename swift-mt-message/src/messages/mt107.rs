use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT107_VALIDATION_RULES)]
pub struct MT107 {
    #[field("20")]
    pub field_20: Field20,

    #[field("23E")]
    pub field_23e: Option<Field23E>,

    #[field("21E")]
    pub field_21e: Option<Field21E>,

    #[field("30")]
    pub field_30: Field30,

    #[field("51A")]
    pub field_51a: Option<Field51A>,

    #[field("50")]
    pub field_50a_instructing: Option<Field50InstructingParty>,

    #[field("50")]
    pub field_50a_creditor: Option<Field50Creditor>,

    #[field("52")]
    pub field_52: Option<Field52CreditorBank>,

    #[field("26T")]
    pub field_26t: Option<Field26T>,

    #[field("77B")]
    pub field_77b: Option<Field77B>,

    #[field("71A")]
    pub field_71a: Option<Field71A>,

    #[field("72")]
    pub field_72: Option<Field72>,

    #[field("#")]
    pub transactions: Vec<MT107Transaction>,

    #[field("32B")]
    pub field_32b: Option<Field32B>,

    #[field("19")]
    pub field_19: Option<Field19>,

    #[field("71F")]
    pub field_71f: Option<Field71F>,

    #[field("71G")]
    pub field_71g: Option<Field71G>,

    #[field("53")]
    pub field_53a: Option<Field53SenderCorrespondent>,
}

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT107_TRANSACTION_VALIDATION_RULES)]
pub struct MT107Transaction {
    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("23E")]
    pub field_23e: Option<Field23E>,

    #[field("21C")]
    pub field_21c: Option<Field21C>,

    #[field("21D")]
    pub field_21d: Option<Field21D>,

    #[field("21E")]
    pub field_21e: Option<Field21E>,

    #[field("32B")]
    pub field_32b: Field32B,

    #[field("50")]
    pub field_50a_instructing: Option<Field50InstructingParty>,

    #[field("50")]
    pub field_50a_creditor: Option<Field50Creditor>,

    #[field("52")]
    pub field_52a: Option<Field52CreditorBank>,

    #[field("57")]
    pub field_57a: Option<Field57DebtorBank>,

    #[field("59")]
    pub field_59a: Field59,

    #[field("70")]
    pub field_70: Option<Field70>,

    #[field("26T")]
    pub field_26t: Option<Field26T>,

    #[field("77B")]
    pub field_77b: Option<Field77B>,

    #[field("33B")]
    pub field_33b: Option<Field33B>,

    #[field("71A")]
    pub field_71a: Option<Field71A>,

    #[field("71F")]
    pub field_71f: Option<Field71F>,

    #[field("71G")]
    pub field_71g: Option<Field71G>,

    #[field("36")]
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
