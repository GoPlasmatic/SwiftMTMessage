use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// # MT103 REMIT: Customer Credit Transfer (Enhanced Remittance Information)
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT103_REMIT_VALIDATION_RULES)]
pub struct MT103REMIT {
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

    #[field("77T", mandatory)]
    pub field_77t: Field77T, // Mandatory in REMIT for structured remittance

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

    #[field("51A", optional)]
    pub field_51a: Option<GenericBicField>,

    #[field("52A", optional)]
    pub field_52a: Option<GenericBicField>,

    #[field("52D", optional)]
    pub field_52d: Option<GenericNameAddressField>,

    #[field("53A", optional)]
    pub field_53a: Option<GenericBicField>,

    #[field("53B", optional)]
    pub field_53b: Option<GenericPartyField>,

    #[field("53D", optional)]
    pub field_53d: Option<GenericNameAddressField>,

    #[field("54A", optional)]
    pub field_54a: Option<GenericBicField>,

    #[field("54B", optional)]
    pub field_54b: Option<GenericPartyField>,

    #[field("54D", optional)]
    pub field_54d: Option<GenericNameAddressField>,

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

    // Note: Field 70 not applicable in REMIT (replaced by 77T)
    #[field("71F", optional)]
    pub field_71f: Option<GenericCurrencyAmountField>,

    #[field("71G", optional)]
    pub field_71g: Option<GenericCurrencyAmountField>,

    #[field("72", optional)]
    pub field_72: Option<GenericMultiLine6x35>,

    #[field("77B", optional)]
    pub field_77b: Option<GenericMultiLine3x35>,
}

/// MT103 REMIT validation rules with REMIT-specific requirements
const MT103_REMIT_VALIDATION_RULES: &str = r#"{
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
    },
    {
      "id": "REMIT_77T",
      "description": "REMIT: 77T is mandatory and must contain structured remittance information",
      "condition": {
        "and": [
          {"!=": [{"var": "field_77t"}, null]},
          {"!=": [{"var": "field_77t.envelope_contents"}, ""]}
        ]
      }
    }
  ],
  "constants": {
    "EU_EEA_COUNTRIES": ["AT", "BE", "BG", "HR", "CY", "CZ", "DK", "EE", "FI", "FR", "DE", "GR", "HU", "IE", "IT", "LV", "LT", "LU", "MT", "NL", "PL", "PT", "RO", "SK", "SI", "ES", "SE", "IS", "LI", "NO"],
    "VALID_BANK_OPERATION_CODES": ["CRED", "CRTS", "SPAY", "SPRI", "SSTD"],
    "VALID_CHARGE_CODES": ["OUR", "SHA", "BEN"],
    "VALID_INSTRUCTION_CODES": ["CORT", "INTC", "REPA", "SDVA", "CHQB", "PHOB", "PHOI", "PHON", "TELE", "TELI", "TELB"]
  }
}"#;
