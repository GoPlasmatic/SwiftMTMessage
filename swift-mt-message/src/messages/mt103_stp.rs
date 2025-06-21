use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT103 STP: Customer Credit Transfer (Straight Through Processing)
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT103_STP_VALIDATION_RULES)]
pub struct MT103STP {
    // Mandatory Fields
    #[field("20", mandatory)]
    pub field_20: GenericReferenceField,

    #[field("23B", mandatory)]
    pub field_23b: GenericTextField,

    #[field("32A", mandatory)]
    pub field_32a: Field32A,

    #[field("50", mandatory)]
    pub field_50: Field50,

    #[field("59", mandatory)]
    pub field_59: Field59,

    #[field("71A", mandatory)]
    pub field_71a: GenericTextField,

    // Optional Fields
    #[field("13C", optional)]
    pub field_13c: Option<Field13C>,

    #[field("23E", optional)]
    pub field_23e: Option<Field23E>,

    #[field("26T", optional)]
    pub field_26t: Option<GenericTextField>,

    #[field("33B", optional)]
    pub field_33b: Option<GenericCurrencyAmountField>,

    #[field("36", optional)]
    pub field_36: Option<Field36>,

    // Note: Field 51A not applicable in STP
    #[field("52A", optional)]
    pub field_52a: Option<GenericBicField>, // Only option A allowed in STP

    // Note: Field 52D not allowed in STP (only option A)
    #[field("53A", optional)]
    pub field_53a: Option<GenericBicField>,

    #[field("53B", optional)]
    pub field_53b: Option<GenericPartyField>, // Options A and B only in STP

    // Note: Field 53D not allowed in STP
    #[field("54A", optional)]
    pub field_54a: Option<GenericBicField>, // Only option A allowed in STP

    // Note: Fields 54B, 54D not allowed in STP
    #[field("55A", optional)]
    pub field_55a: Option<GenericBicField>,

    #[field("55B", optional)]
    pub field_55b: Option<GenericPartyField>,

    #[field("55D", optional)]
    pub field_55d: Option<GenericNameAddressField>,

    #[field("56A", optional)]
    pub field_56a: Option<GenericBicField>,

    #[field("56C", optional)]
    pub field_56c: Option<GenericAccountField>,

    #[field("56D", optional)]
    pub field_56d: Option<GenericNameAddressField>,

    #[field("57A", optional)]
    pub field_57a: Option<GenericBicField>,

    #[field("57B", optional)]
    pub field_57b: Option<GenericPartyField>,

    #[field("57C", optional)]
    pub field_57c: Option<GenericAccountField>,

    #[field("57D", optional)]
    pub field_57d: Option<GenericNameAddressField>,

    #[field("70", optional)]
    pub field_70: Option<GenericMultiLine4x35>,

    #[field("71F", optional)]
    pub field_71f: Option<GenericCurrencyAmountField>,

    #[field("71G", optional)]
    pub field_71g: Option<GenericCurrencyAmountField>,

    #[field("72", optional)]
    pub field_72: Option<GenericMultiLine6x35>,

    #[field("77B", optional)]
    pub field_77b: Option<GenericMultiLine3x35>,

    #[field("77T", optional)]
    pub field_77t: Option<Field77T>,
}

/// MT103 STP validation rules with STP-specific restrictions
const MT103_STP_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "If 33B is present and its currency differs from 32A, then 36 must be present; otherwise, 36 must not be present",
      "condition": {
        "if": [
          {"var": "field_33b.is_some"},
          {
            "if": [
              {"!=": [{"var": "field_33b.currency"}, {"var": "field_32a.currency"}]},
              {"var": "field_36.is_some"},
              {"not": {"var": "field_36.is_some"}}
            ]
          },
          {"not": {"var": "field_36.is_some"}}
        ]
      }
    },
    {
      "id": "C2", 
      "description": "If both Sender and Receiver BICs are in EU/EEA country codes list, then 33B is mandatory",
      "condition": {
        "if": [
          {"and": [
            {"in": [{"var": "message_context.sender_country"}, {"var": "EU_EEA_COUNTRIES"}]},
            {"in": [{"var": "message_context.receiver_country"}, {"var": "EU_EEA_COUNTRIES"}]}
          ]},
          {"var": "field_33b.is_some"},
          true
        ]
      }
    },
    {
      "id": "C3_STP",
      "description": "STP: 23E instruction codes limited to CORT, INTC, SDVA, REPA",
      "condition": {
        "if": [
          {"var": "field_23e.is_some"},
          {"in": [{"var": "field_23e.instruction_code"}, ["CORT", "INTC", "SDVA", "REPA"]]},
          true
        ]
      }
    },
    {
      "id": "C4_STP",
      "description": "STP: If 55a present → 53a and 54a are mandatory",
      "condition": {
        "if": [
          {"or": [
            {"var": "field_55a.is_some"},
            {"var": "field_55b.is_some"},
            {"var": "field_55d.is_some"}
          ]},
          {"and": [
            {"or": [
              {"var": "field_53a.is_some"},
              {"var": "field_53b.is_some"}
            ]},
            {"var": "field_54a.is_some"}
          ]},
          true
        ]
      }
    },
    {
      "id": "C5",
      "description": "If 56a is present, 57a becomes mandatory",
      "condition": {
        "if": [
          {"or": [
            {"var": "field_56a.is_some"},
            {"var": "field_56c.is_some"},
            {"var": "field_56d.is_some"}
          ]},
          {"or": [
            {"var": "field_57a.is_some"},
            {"var": "field_57b.is_some"},
            {"var": "field_57c.is_some"},
            {"var": "field_57d.is_some"}
          ]},
          true
        ]
      }
    },
    {
      "id": "C6_STP",
      "description": "STP: If 23B is SPRI → 56a must not be present",
      "condition": {
        "if": [
          {"==": [{"var": "field_23b.value"}, "SPRI"]},
          {"not": {"or": [
            {"var": "field_56a.is_some"},
            {"var": "field_56c.is_some"},
            {"var": "field_56d.is_some"}
          ]}},
          true
        ]
      }
    },
    {
      "id": "C7",
      "description": "Charge allocation rules",
      "condition": {
        "and": [
          {
            "if": [
              {"==": [{"var": "field_71a.value"}, "OUR"]},
              {"not": {"var": "field_71f.is_some"}},
              true
            ]
          },
          {
            "if": [
              {"==": [{"var": "field_71a.value"}, "SHA"]},
              {"not": {"var": "field_71g.is_some"}},
              true
            ]
          },
          {
            "if": [
              {"==": [{"var": "field_71a.value"}, "BEN"]},
              {"and": [
                {"var": "field_71f.is_some"},
                {"not": {"var": "field_71g.is_some"}}
              ]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "C8",
      "description": "If either 71F or 71G is present, 33B becomes mandatory",
      "condition": {
        "if": [
          {"or": [
            {"var": "field_71f.is_some"},
            {"var": "field_71g.is_some"}
          ]},
          {"var": "field_33b.is_some"},
          true
        ]
      }
    }
  ],
  "constants": {
    "EU_EEA_COUNTRIES": ["AT", "BE", "BG", "HR", "CY", "CZ", "DK", "EE", "FI", "FR", "DE", "GR", "HU", "IE", "IT", "LV", "LT", "LU", "MT", "NL", "PL", "PT", "RO", "SK", "SI", "ES", "SE", "IS", "LI", "NO"],
    "VALID_BANK_OPERATION_CODES": ["CRED", "CRTS", "SPAY", "SPRI", "SSTD"],
    "VALID_CHARGE_CODES": ["OUR", "SHA", "BEN"],
    "VALID_INSTRUCTION_CODES_STP": ["CORT", "INTC", "SDVA", "REPA"]
  }
}"#;
