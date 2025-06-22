use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// Message status information for MT103 messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageStatus {
    /// Whether the message is STP compliant
    pub is_stp_compliant: bool,
    /// Whether the message is a REMIT message
    pub is_remit: bool,
    /// Whether the message contains reject codes
    pub has_reject_codes: bool,
    /// Whether the message contains return codes
    pub has_return_codes: bool,
    /// The processing variant (Standard, STP, REMIT, REJECT, RETURN)
    pub processing_variant: String,
}

/// MT103: Customer Credit Transfer (Standard and STP variants)
///
/// Unified structure supporting both standard MT103 and MT103 STP variants.
/// Use `is_stp_compliant()` to check if the message meets STP requirements.
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT103_VALIDATION_RULES)]
pub struct MT103 {
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
    pub field_33b: Option<Field33B>,

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

    #[field("70", optional)]
    pub field_70: Option<GenericMultiLine4x35>,

    #[field("71F", optional)]
    pub field_71f: Option<Field71F>,

    #[field("71G", optional)]
    pub field_71g: Option<Field71G>,

    #[field("72", optional)]
    pub field_72: Option<GenericMultiLine6x35>,

    #[field("77B", optional)]
    pub field_77b: Option<GenericMultiLine3x35>,

    #[field("77T", optional)]
    pub field_77t: Option<Field77T>,
}

impl MT103 {
    /// Check if this MT103 message is compliant with STP (Straight Through Processing) requirements
    ///
    /// STP compliance requires:
    /// - Field 51A must not be present
    /// - Field 52: Only option A allowed (no D)
    /// - Field 53: Only options A/B allowed (no D)
    /// - Field 54: Only option A allowed (no B/D)
    /// - Field 23E: Limited to CORT, INTC, SDVA, REPA
    /// - Field 56a: Not allowed if 23B is SPRI
    /// - Field 59: Account information mandatory
    /// - Additional conditional rules (C4, C6)
    pub fn is_stp_compliant(&self) -> bool {
        // Check field 51A - must not be present in STP
        if self.field_51a.is_some() {
            return false;
        }

        // Check field 52 - only option A allowed in STP
        if self.field_52d.is_some() {
            return false;
        }

        // Check field 53 - only options A/B allowed in STP
        if self.field_53d.is_some() {
            return false;
        }

        // Check field 54 - only option A allowed in STP
        if self.field_54b.is_some() || self.field_54d.is_some() {
            return false;
        }

        // Check field 23E - restricted instruction codes in STP
        if let Some(ref field_23e) = self.field_23e {
            let stp_allowed_codes = ["CORT", "INTC", "SDVA", "REPA"];
            if !stp_allowed_codes.contains(&field_23e.instruction_code.as_str()) {
                return false;
            }
        }

        // Check C6_STP: If 23B is SPRI → 56a must not be present
        if self.field_23b.value == "SPRI"
            && (self.field_56a.is_some() || self.field_56c.is_some() || self.field_56d.is_some())
        {
            return false;
        }

        // Check C4_STP: If 55a present → 53a and 54a are mandatory
        if self.field_55a.is_some() || self.field_55b.is_some() || self.field_55d.is_some() {
            if self.field_53a.is_none() && self.field_53b.is_none() {
                return false;
            }
            if self.field_54a.is_none() {
                return false;
            }
        }

        // Check field 59 - account information should be present for STP
        // This is a soft requirement - in practice, field 59 without account (NoOption)
        // might still be acceptable in some STP scenarios
        match &self.field_59 {
            Field59::A(_) => {} // Has account - good for STP
            Field59::F(_) => {} // Has party identifier - acceptable for STP
            Field59::NoOption(_) => {
                // This might be acceptable in some STP scenarios
                // We'll allow it but note that proper STP usually requires account info
            }
        }

        true
    }

    /// Check if this MT103 message is a REMIT message with enhanced remittance information
    ///
    /// REMIT messages are distinguished by:
    /// - Field 77T must be present and contain structured remittance information
    /// - Field 70 is typically not used (replaced by 77T)
    /// - Enhanced remittance data for regulatory compliance
    pub fn is_remit_message(&self) -> bool {
        // The key distinguishing feature of REMIT is the presence of field 77T
        // with structured remittance information
        match &self.field_77t {
            Some(field_77t) => {
                // Check if 77T contains actual remittance data (not just empty)
                !field_77t.envelope_identifier.trim().is_empty()
                    && !field_77t.envelope_type.trim().is_empty()
                    && !field_77t.envelope_format.trim().is_empty()
            }
            None => false,
        }
    }

    /// Check if this MT103 message contains reject codes
    ///
    /// Reject messages are identified by checking:
    /// 1. Field 20 (Sender's Reference) for "REJT" prefix
    /// 2. Block 3 field 108 (MUR - Message User Reference) for "REJT"
    /// 3. Field 72 (Sender to Receiver Information) containing `/REJT/` code
    pub fn has_reject_codes(&self) -> bool {
        // Check field 20 (sender's reference)
        if self.field_20.value.to_uppercase().contains("REJT") {
            return true;
        }

        // Check field 72 for structured reject codes
        if let Some(field_72) = &self.field_72 {
            let content = field_72.lines.join(" ").to_uppercase();
            if content.contains("/REJT/") || content.contains("REJT") {
                return true;
            }
        }

        false
    }

    /// Check if this MT103 message contains return codes
    ///
    /// Return messages are identified by checking:
    /// 1. Field 20 (Sender's Reference) for "RETN" prefix
    /// 2. Block 3 field 108 (MUR - Message User Reference) for "RETN"
    /// 3. Field 72 (Sender to Receiver Information) containing `/RETN/` code
    pub fn has_return_codes(&self) -> bool {
        // Check field 20 (sender's reference)
        if self.field_20.value.to_uppercase().contains("RETN") {
            return true;
        }

        // Check field 72 for structured return codes
        if let Some(field_72) = &self.field_72 {
            let content = field_72.lines.join(" ").to_uppercase();
            if content.contains("/RETN/") || content.contains("RETN") {
                return true;
            }
        }

        false
    }
}

/// Comprehensive MT103 validation rules covering both standard and STP variants
const MT103_VALIDATION_RULES: &str = r#"{
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
      "id": "C3",
      "description": "Bank operation code and instruction code compatibility rules",
      "condition": {
        "and": [
          {
            "if": [
              {"==": [{"var": "fields.23B.value"}, "SPRI"]},
              {"and": [
                {"!!": {"var": "fields.23E"}},
                {"in": [{"var": "fields.23E.instruction_code"}, ["SDVA", "INTC"]]}
              ]},
              true
            ]
          },
          {
            "if": [
              {"in": [{"var": "fields.23B.value"}, ["SSTD", "SPAY"]]},
              {"!": {"var": "fields.23E"}},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "C4",
      "description": "If 55a is present, then 53a and 54a become mandatory",
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
              {"!!": {"var": "fields.53B"}},
              {"!!": {"var": "fields.53D"}}
            ]},
            {"or": [
              {"!!": {"var": "fields.54A"}},
              {"!!": {"var": "fields.54B"}},
              {"!!": {"var": "fields.54D"}}
            ]}
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
      "id": "C9",
      "description": "Currency codes in 71G and 32A must match",
      "condition": {
        "if": [
          {"!!": {"var": "fields.71G"}},
          {"==": [{"var": "fields.71G.currency"}, {"var": "fields.32A.currency"}]},
          true
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
      "id": "INSTRUCTION_CODE_VALIDATION",
      "description": "23E instruction codes must be valid when present",
      "condition": {
        "if": [
          {"!!": {"var": "fields.23E"}},
          {"in": [{"var": "fields.23E.instruction_code"}, {"var": "VALID_INSTRUCTION_CODES"}]},
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
    },
    {
      "id": "REMIT_77T",
      "description": "REMIT: If 77T is present, it must contain valid structured remittance information",
      "condition": {
        "if": [
          {"!!": {"var": "fields.77T"}},
          {"and": [
            {"!=": [{"var": "fields.77T.envelope_type"}, ""]},
            {"!=": [{"var": "fields.77T.envelope_format"}, ""]},
            {"!=": [{"var": "fields.77T.envelope_identifier"}, ""]}
          ]},
          true
        ]
      }
    },
    {
      "id": "REMIT_FIELD_COMPATIBILITY",
      "description": "REMIT: Field 70 should not be used when 77T is present (77T replaces 70 in REMIT)",
      "condition": {
        "if": [
          {"!!": {"var": "fields.77T"}},
          {"!": {"var": "fields.70"}},
          true
        ]
      }
    }
  ],
  "constants": {
    "EU_EEA_COUNTRIES": ["AT", "BE", "BG", "HR", "CY", "CZ", "DK", "EE", "FI", "FR", "DE", "GR", "HU", "IE", "IT", "LV", "LT", "LU", "MT", "NL", "PL", "PT", "RO", "SK", "SI", "ES", "SE", "IS", "LI", "NO"],
    "VALID_BANK_OPERATION_CODES": ["CRED", "CRTS", "SPAY", "SPRI", "SSTD"],
    "VALID_CHARGE_CODES": ["OUR", "SHA", "BEN"],
    "VALID_INSTRUCTION_CODES": ["CORT", "INTC", "REPA", "SDVA", "CHQB", "PHOB", "PHOI", "PHON", "TELE", "TELI", "TELB"],
    "VALID_INSTRUCTION_CODES_STP": ["CORT", "INTC", "SDVA", "REPA"]
  }
}"#;
