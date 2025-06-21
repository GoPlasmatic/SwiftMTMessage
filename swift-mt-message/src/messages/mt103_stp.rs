use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT103 STP: Customer Credit Transfer (Straight Through Processing)
///
/// STP variant of MT103 with enhanced automation and validation requirements.
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
          {"!!": {"var": "fields.33B"}},
          {
            "if": [
              {"!=": [{"var": "fields.33B.currency"}, {"var": "fields.32A.currency"}]},
              {"!!": {"var": "fields.36"}},
              {"!": {"var": "fields.36"}}
            ]
          },
          {"!": {"var": "fields.36"}}
        ]
      }
    },
    {
      "id": "C2", 
      "description": "33B is mandatory if both Sender and Receiver BICs are in EU/EEA country codes list",
      "condition": {
        "if": [
          {"and": [
            {"in": [{"var": "basic_header.sender_bic.country_code"}, {"var": "EU_EEA_COUNTRIES"}]},
            {"in": [{"var": "application_header.receiver_bic.country_code"}, {"var": "EU_EEA_COUNTRIES"}]}
          ]},
          {"!!": {"var": "fields.33B"}},
          true
        ]
      }
    },
    {
      "id": "C3_STP",
      "description": "STP: 23E instruction codes limited to CORT, INTC, SDVA, REPA",
      "condition": {
        "if": [
          {"!!": {"var": "fields.23E"}},
          {"in": [{"var": "fields.23E.instruction_code"}, {"var": "VALID_INSTRUCTION_CODES_STP"}]},
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
            {"!!": {"var": "fields.55A"}},
            {"!!": {"var": "fields.55B"}},
            {"!!": {"var": "fields.55D"}}
          ]},
          {"and": [
            {"or": [
              {"!!": {"var": "fields.53A"}},
              {"!!": {"var": "fields.53B"}}
            ]},
            {"!!": {"var": "fields.54A"}}
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
            {"!!": {"var": "fields.56A"}},
            {"!!": {"var": "fields.56C"}},
            {"!!": {"var": "fields.56D"}}
          ]},
          {"or": [
            {"!!": {"var": "fields.57A"}},
            {"!!": {"var": "fields.57B"}},
            {"!!": {"var": "fields.57C"}},
            {"!!": {"var": "fields.57D"}}
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
          {"==": [{"var": "fields.23B.value"}, "SPRI"]},
          {"!": {"or": [
            {"!!": {"var": "fields.56A"}},
            {"!!": {"var": "fields.56C"}},
            {"!!": {"var": "fields.56D"}}
          ]}},
          true
        ]
      }
    },
    {
      "id": "C7",
      "description": "Charge allocation rules: If 71A = OUR → 71F not allowed, 71G optional; If 71A = SHA → 71F optional, 71G not allowed; If 71A = BEN → 71F mandatory, 71G not allowed",
      "condition": {
        "and": [
          {
            "if": [
              {"==": [{"var": "fields.71A.value"}, "OUR"]},
              {"!": {"var": "fields.71F"}},
              true
            ]
          },
          {
            "if": [
              {"==": [{"var": "fields.71A.value"}, "SHA"]},
              {"!": {"var": "fields.71G"}},
              true
            ]
          },
          {
            "if": [
              {"==": [{"var": "fields.71A.value"}, "BEN"]},
              {"and": [
                {"!!": {"var": "fields.71F"}},
                {"!": {"var": "fields.71G"}}
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
            {"!!": {"var": "fields.71F"}},
            {"!!": {"var": "fields.71G"}}
          ]},
          {"!!": {"var": "fields.33B"}},
          true
        ]
      }
    },
    {
      "id": "STP_FIELD_RESTRICTIONS",
      "description": "STP: Verify restricted fields are not present (51A not allowed, only option A for certain fields)",
      "condition": {
        "and": [
          {"!": {"var": "fields.51A"}},
          {
            "if": [
              {"!!": {"var": "fields.52A"}},
              true,
              {"!": {"or": [
                {"!!": {"var": "fields.52D"}}
              ]}}
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.54A"}},
              true,
              {"!": {"or": [
                {"!!": {"var": "fields.54B"}},
                {"!!": {"var": "fields.54D"}}
              ]}}
            ]
          }
        ]
      }
    },
    {
      "id": "MANDATORY_FIELDS",
      "description": "All mandatory fields must be present and valid",
      "condition": {
        "and": [
          {"!!": {"var": "fields.20"}},
          {"!=": [{"var": "fields.20.value"}, ""]},
          {"!!": {"var": "fields.23B"}},
          {"in": [{"var": "fields.23B.value"}, {"var": "VALID_BANK_OPERATION_CODES"}]},
          {"!!": {"var": "fields.32A"}},
          {">": [{"var": "fields.32A.amount"}, 0]},
          {"!!": {"var": "fields.50"}},
          {"!!": {"var": "fields.59"}},
          {"!!": {"var": "fields.71A"}},
          {"in": [{"var": "fields.71A.value"}, {"var": "VALID_CHARGE_CODES"}]}
        ]
      }
    },
    {
      "id": "STP_ACCOUNT_VALIDATION",
      "description": "STP: Account information must be present in 59a field for beneficiary",
      "condition": {
        "if": [
          {"!!": {"var": "fields.59"}},
          {
            "or": [
              {"!!": {"var": "fields.59.A"}},
              {"!!": {"var": "fields.59.NoOption"}}
            ]
          },
          true
        ]
      }
    },
    {
      "id": "AMOUNT_CONSISTENCY",
      "description": "All amounts must be positive and properly formatted",
      "condition": {
        "and": [
          {">": [{"var": "fields.32A.amount"}, 0]},
          {
            "if": [
              {"!!": {"var": "fields.33B"}},
              {">": [{"var": "fields.33B.amount"}, 0]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.71F"}},
              {">": [{"var": "fields.71F.amount"}, 0]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.71G"}},
              {">": [{"var": "fields.71G.amount"}, 0]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "CURRENCY_CODE_VALIDATION",
      "description": "All currency codes must be valid ISO 4217 3-letter codes",
      "condition": {
        "and": [
          {"!=": [{"var": "fields.32A.currency"}, ""]},
          {
            "if": [
              {"!!": {"var": "fields.33B"}},
              {"!=": [{"var": "fields.33B.currency"}, ""]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.71F"}},
              {"!=": [{"var": "fields.71F.currency"}, ""]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.71G"}},
              {"!=": [{"var": "fields.71G.currency"}, ""]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "REFERENCE_FORMAT",
      "description": "Reference fields must not contain invalid patterns",
      "condition": {
        "and": [
          {"!=": [{"var": "fields.20.value"}, ""]},
          {"!": {"in": ["//", {"var": "fields.20.value"}]}}
        ]
      }
    },
    {
      "id": "BIC_VALIDATION",
      "description": "All BIC codes must be properly formatted (non-empty)",
      "condition": {
        "and": [
          {"!=": [{"var": "basic_header.sender_bic.raw"}, ""]},
          {"!=": [{"var": "application_header.receiver_bic.raw"}, ""]},
          {
            "if": [
              {"!!": {"var": "fields.52A"}},
              {"!=": [{"var": "fields.52A.bic.raw"}, ""]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.53A"}},
              {"!=": [{"var": "fields.53A.bic.raw"}, ""]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.57A"}},
              {"!=": [{"var": "fields.57A.bic.raw"}, ""]},
              true
            ]
          }
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
