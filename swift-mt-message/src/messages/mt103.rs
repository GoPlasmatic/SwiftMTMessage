use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT103: Single Customer Credit Transfer
///
/// ## Purpose
/// Used to convey funds transfer instructions between financial institutions where the ordering or beneficiary customer (or both) are non-financial institutions.
/// This is the most common payment message in the SWIFT network for retail and commercial payments worldwide.
///
/// ## Scope
/// This message is:
/// - Used for clean payment instructions without additional documents
/// - Applicable for cross-border payments between different countries/currencies
/// - Used for high-value domestic transfers via SWIFT network
/// - Not applicable for collection advices, documentary credits, or cover transactions
/// - Compatible with STP (Straight Through Processing) requirements
/// - Subject to comprehensive network validation rules for payment integrity
///
/// ## Key Features
/// - **Universal Payment Format**: Most widely used SWIFT payment message globally
/// - **STP Compliance**: Built-in support for straight-through processing requirements
/// - **REMIT Support**: Enhanced remittance information using field 77T for regulatory compliance
/// - **Service Level Options**: Multiple processing speeds and cost tiers available
/// - **Charge Allocation Flexibility**: OUR/SHA/BEN charge allocation options
/// - **Comprehensive Validation**: 18 network validation rules ensure message integrity
///
/// ## Common Use Cases
/// - International wire transfers for trade settlements and remittances
/// - Corporate payments for supplier settlements and salary transfers
/// - High-value domestic payments requiring SWIFT network routing
/// - Cross-border e-commerce and marketplace settlements
/// - Foreign exchange spot and forward settlement payments
/// - Investment and securities transaction settlements
/// - Emergency and expedited payment transfers
///
/// ## Message Structure
/// - **Field 20**: Sender's Reference (mandatory) - Unique transaction identifier
/// - **Field 13C**: Time Indication (optional, repetitive) - Processing time constraints
/// - **Field 23B**: Bank Operation Code (mandatory) - Service level (CRED/SPRI/SSTD/SPAY)
/// - **Field 23E**: Instruction Code (optional, repetitive) - Special processing instructions
/// - **Field 26T**: Transaction Type Code (optional) - Payment category classification
/// - **Field 32A**: Value Date/Currency/Amount (mandatory) - Settlement details
/// - **Field 33B**: Currency/Instructed Amount (optional) - Original currency before conversion
/// - **Field 36**: Exchange Rate (optional) - Rate for currency conversion
/// - **Field 50**: Ordering Customer (mandatory) - Customer initiating payment
/// - **Field 51A**: Sending Institution (optional) - Institution sending the payment
/// - **Field 52**: Ordering Institution (optional) - Institution of ordering customer
/// - **Field 53**: Sender's Correspondent (optional) - Sender's correspondent bank
/// - **Field 54**: Receiver's Correspondent (optional) - Receiver's correspondent bank  
/// - **Field 56**: Intermediary Institution (optional) - Intermediary in payment chain
/// - **Field 57**: Account With Institution (optional) - Institution crediting beneficiary
/// - **Field 59**: Beneficiary Customer (mandatory) - Final recipient of funds
/// - **Field 70**: Remittance Information (optional) - Payment purpose and details
/// - **Field 71A**: Details of Charges (mandatory) - Charge allocation (OUR/SHA/BEN)
/// - **Field 71F**: Sender's Charges (optional) - Charges claimed by sender
/// - **Field 71G**: Receiver's Charges (optional) - Charges claimed by receiver
/// - **Field 72**: Sender to Receiver Information (optional) - Additional instructions
/// - **Field 77B**: Regulatory Reporting (optional) - Compliance reporting information
/// - **Field 77T**: Envelope for Remittance Info (optional) - Structured remittance data
///
/// ## Network Validation Rules
/// - **Exchange Rate Logic**: Field 36 requirements when currencies differ (C1)
/// - **EU/EEA Requirements**: Field 33B validation for specific country combinations (C2)
/// - **Service Level Compatibility**: Instruction code restrictions by service level (C3-C6)
/// - **Correspondent Dependencies**: Field dependencies for correspondent institutions (C7)
/// - **Charge Allocation**: Currency and charge consistency rules (C8-C9)
/// - **STP Restrictions**: Service level limitations on intermediary fields (C10-C12)
/// - **Enhanced Validation**: Detailed charge handling and instruction code rules (C14-C18)
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT103 format remains stable
/// - **Enhanced STP**: Strengthened straight-through processing requirements
/// - **REMIT Enhancement**: Improved structured remittance information support
/// - **Regulatory Compliance**: Enhanced field 77B validation for reporting requirements
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with all major core banking and payment processing systems
/// - **API Integration**: Full RESTful API support for modern payment platforms
/// - **Processing Requirements**: Supports real-time, same-day, and next-day processing
/// - **Compliance Integration**: Built-in AML, sanctions screening, and regulatory reporting hooks
///
/// ## Relationship to Other Messages
/// - **Triggers**: Often triggered by customer payment instructions or corporate ERP systems
/// - **Responses**: May generate MT910 (confirmation) or MT900 (debit confirmation)
/// - **Related**: Works with MT202 (cover payment) and MT940/MT950 (account reporting)
/// - **Alternatives**: MT101 (bulk transfers), MT200 (financial institution transfers)
/// - **Status Updates**: May receive MT192/MT196/MT199 for status notifications
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT103_VALIDATION_RULES)]
pub struct MT103 {
    // Mandatory Fields
    #[field("20")]
    pub field_20: Field20,

    #[field("13C")]
    pub field_13c: Option<Vec<Field13C>>,

    #[field("23B")]
    pub field_23b: Field23B,

    #[field("23E")]
    pub field_23e: Option<Vec<Field23E>>,

    #[field("26T")]
    pub field_26t: Option<Field26T>,

    #[field("32A")]
    pub field_32a: Field32A,

    #[field("33B")]
    pub field_33b: Option<Field33B>,

    #[field("36")]
    pub field_36: Option<Field36>,

    #[field("50")]
    pub field_50: Field50OrderingCustomerAFK,

    #[field("51A")]
    pub field_51a: Option<Field51A>,

    #[field("52")]
    pub field_52: Option<Field52OrderingInstitution>,

    #[field("53")]
    pub field_53: Option<Field53SenderCorrespondent>,

    #[field("54")]
    pub field_54: Option<Field54ReceiverCorrespondent>,

    #[field("55")]
    pub field_55: Option<Field55ThirdReimbursementInstitution>,

    #[field("56")]
    pub field_56: Option<Field56Intermediary>,

    #[field("57")]
    pub field_57: Option<Field57AccountWithInstitution>,

    #[field("59")]
    pub field_59: Field59,

    #[field("70")]
    pub field_70: Option<Field70>,

    #[field("71A")]
    pub field_71a: Field71A,

    #[field("71F")]
    pub field_71f: Option<Vec<Field71F>>,

    #[field("71G")]
    pub field_71g: Option<Field71G>,

    #[field("72")]
    pub field_72: Option<Field72>,

    #[field("77B")]
    pub field_77b: Option<Field77B>,

    #[field("77T")]
    pub field_77t: Option<Field77T>,
}

impl MT103 {
    /// Check if this MT103 message is compliant with STP (Straight Through Processing) requirements
    ///
    /// STP compliance requires specific options and restrictions when field 23B contains SPRI, SSTD, or SPAY:
    /// - Field 23B contains SPRI/SSTD/SPAY: Enforces strict STP rules
    /// - Field 53a: Must not be option D (C4)
    /// - Field 53B: Party Identifier mandatory if option B used (C5)
    /// - Field 54a: Only option A allowed (C6)
    /// - Field 55a: Only option A allowed (C8)
    /// - Field 56a: Not allowed if SPRI; only A or C if SSTD/SPAY (C10)
    /// - Field 57a: Only options A, C, or D allowed; D requires Party Identifier (C11)
    /// - Field 59a: Account mandatory (C12)
    /// - Field 23E: Restricted codes for SPRI (C3)
    pub fn is_stp_compliant(&self) -> bool {
        // Check if this is an STP message (SPRI, SSTD, or SPAY)
        let bank_op_code = &self.field_23b.instruction_code;
        if !["SPRI", "SSTD", "SPAY"].contains(&bank_op_code.as_str()) {
            // Not an STP message type
            return true;
        }

        // C3: If 23B is SPRI, field 23E may contain only SDVA, TELB, PHOB, INTC
        // If 23B is SSTD or SPAY, field 23E must not be used
        if bank_op_code == "SPRI" {
            if let Some(ref field_23e_vec) = self.field_23e {
                let allowed_codes = ["SDVA", "TELB", "PHOB", "INTC"];
                for field_23e in field_23e_vec {
                    if !allowed_codes.contains(&field_23e.instruction_code.as_str()) {
                        return false;
                    }
                }
            }
        } else if ["SSTD", "SPAY"].contains(&bank_op_code.as_str())
            && self.field_23e.is_some()
            && !self.field_23e.as_ref().unwrap().is_empty()
        {
            return false;
        }

        // C4: Field 53a must not be used with option D
        if let Some(ref field_53) = self.field_53 {
            if let Field53SenderCorrespondent::D(_) = field_53 {
                return false;
            }

            // C5: If field 53a is option B, Party Identifier must be present
            if let Field53SenderCorrespondent::B(field_53b) = field_53 {
                if field_53b.party_identifier.is_none() {
                    return false;
                }
            }
        }

        // C6: Field 54a may be used with option A only
        if let Some(ref field_54) = self.field_54 {
            match field_54 {
                Field54ReceiverCorrespondent::A(_) => {}
                _ => return false,
            }
        }

        // C8: Field 55a may be used with option A only
        if let Some(ref field_55) = self.field_55 {
            match field_55 {
                Field55ThirdReimbursementInstitution::A(_) => {}
                _ => return false,
            }
        }

        // C10: If 23B is SPRI, field 56a must not be present
        // If 23B is SSTD or SPAY, field 56a may be used with option A or C only
        if bank_op_code == "SPRI" {
            if self.field_56.is_some() {
                return false;
            }
        } else if ["SSTD", "SPAY"].contains(&bank_op_code.as_str()) {
            if let Some(ref field_56) = self.field_56 {
                match field_56 {
                    Field56Intermediary::A(_) | Field56Intermediary::C(_) => {}
                    _ => return false,
                }
            }
        }

        // C11: Field 57a may be used with option A, C or D
        // In option D, Party Identifier is mandatory
        if let Some(ref field_57) = self.field_57 {
            match field_57 {
                Field57AccountWithInstitution::A(_) | Field57AccountWithInstitution::C(_) => {}
                Field57AccountWithInstitution::D(field_57d) => {
                    if field_57d.party_identifier.is_none() {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        // C12: Account in field 59a is mandatory
        match &self.field_59 {
            Field59::NoOption(field_59) => {
                if field_59.account.is_none() {
                    return false;
                }
            }
            Field59::A(field_59a) => {
                if field_59a.account.is_none() {
                    return false;
                }
            }
            Field59::F(field_59f) => {
                if field_59f.party_identifier.is_none() {
                    return false;
                }
            }
        }

        // Additional STP-specific validation rules
        // C7: If field 55a is present, then both fields 53a and 54a must also be present
        if self.field_55.is_some() && (self.field_53.is_none() || self.field_54.is_none()) {
            return false;
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
                !field_77t.envelope_content.trim().is_empty()
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
        if self.field_20.reference.to_uppercase().contains("REJT") {
            return true;
        }

        // Check field 72 for structured reject codes
        if let Some(field_72) = &self.field_72 {
            let content = field_72.information.join(" ").to_uppercase();
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
        if self.field_20.reference.to_uppercase().contains("RETN") {
            return true;
        }

        // Check field 72 for structured return codes
        if let Some(field_72) = &self.field_72 {
            let content = field_72.information.join(" ").to_uppercase();
            if content.contains("/RETN/") || content.contains("RETN") {
                return true;
            }
        }

        false
    }
}

/// Comprehensive MT103 validation rules based on SRG2025 specification
const MT103_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "If field 33B is present and the currency code is different from the currency code in field 32A, field 36 must be present, otherwise field 36 is not allowed",
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
      "description": "If the country codes of the Sender's and the Receiver's BICs are within EU/EEA list, then field 33B is mandatory",
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
      "description": "If field 23B contains SPRI, field 23E may contain only SDVA, TELB, PHOB, INTC. If field 23B contains SSTD or SPAY, field 23E must not be used",
      "condition": {
        "and": [
          {
            "if": [
              {"==": [{"var": "fields.23B.instruction_code"}, "SPRI"]},
              {
                "if": [
                  {"!!": {"var": "fields.23E"}},
                  {
                    "all": [
                      {"var": "fields.23E"},
                      {
                        "in": [{"var": "instruction_code"}, ["SDVA", "TELB", "PHOB", "INTC"]]
                      }
                    ]
                  },
                  true
                ]
              },
              true
            ]
          },
          {
            "if": [
              {"in": [{"var": "fields.23B.instruction_code"}, ["SSTD", "SPAY"]]},
              {"!": {"var": "fields.23E"}},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "C4",
      "description": "If field 23B contains SPRI, SSTD or SPAY, field 53a must not be used with option D",
      "condition": {
        "if": [
          {"in": [{"var": "fields.23B.instruction_code"}, ["SPRI", "SSTD", "SPAY"]]},
          {
            "if": [
              {"!!": {"var": "fields.53"}},
              {"!=": [{"var": "fields.53.option"}, "D"]},
              true
            ]
          },
          true
        ]
      }
    },
    {
      "id": "C5",
      "description": "If field 23B contains SPRI, SSTD or SPAY and field 53a is present with option B, Party Identifier must be present",
      "condition": {
        "if": [
          {"and": [
            {"in": [{"var": "fields.23B.instruction_code"}, ["SPRI", "SSTD", "SPAY"]]},
            {"!!": {"var": "fields.53"}},
            {"==": [{"var": "fields.53.option"}, "B"]}
          ]},
          {"!!": {"var": "fields.53.party_identifier"}},
          true
        ]
      }
    },
    {
      "id": "C6",
      "description": "If field 23B contains SPRI, SSTD or SPAY, field 54a may be used with option A only",
      "condition": {
        "if": [
          {"in": [{"var": "fields.23B.instruction_code"}, ["SPRI", "SSTD", "SPAY"]]},
          {
            "if": [
              {"!!": {"var": "fields.54"}},
              {"==": [{"var": "fields.54.option"}, "A"]},
              true
            ]
          },
          true
        ]
      }
    },
    {
      "id": "C7",
      "description": "If field 55a is present, then both fields 53a and 54a must also be present",
      "condition": {
        "if": [
          {"!!": {"var": "fields.55"}},
          {"and": [
            {"!!": {"var": "fields.53"}},
            {"!!": {"var": "fields.54"}}
          ]},
          true
        ]
      }
    },
    {
      "id": "C8",
      "description": "If field 23B contains SPRI, SSTD or SPAY, field 55a may be used with option A only",
      "condition": {
        "if": [
          {"in": [{"var": "fields.23B.instruction_code"}, ["SPRI", "SSTD", "SPAY"]]},
          {
            "if": [
              {"!!": {"var": "fields.55"}},
              {"==": [{"var": "fields.55.option"}, "A"]},
              true
            ]
          },
          true
        ]
      }
    },
    {
      "id": "C9",
      "description": "If field 56a is present, field 57a must also be present",
      "condition": {
        "if": [
          {"!!": {"var": "fields.56"}},
          {"!!": {"var": "fields.57"}},
          true
        ]
      }
    },
    {
      "id": "C10",
      "description": "If field 23B contains SPRI, field 56a must not be present. If field 23B contains SSTD or SPAY, field 56a may be used with option A or C only",
      "condition": {
        "and": [
          {
            "if": [
              {"==": [{"var": "fields.23B.instruction_code"}, "SPRI"]},
              {"!": {"var": "fields.56"}},
              true
            ]
          },
          {
            "if": [
              {"and": [
                {"in": [{"var": "fields.23B.instruction_code"}, ["SSTD", "SPAY"]]},
                {"!!": {"var": "fields.56"}}
              ]},
              {"in": [{"var": "fields.56.option"}, ["A", "C"]]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "C11",
      "description": "If field 23B contains SPRI, SSTD or SPAY, field 57a may be used with option A, C or D. In option D, Party Identifier is mandatory",
      "condition": {
        "if": [
          {"and": [
            {"in": [{"var": "fields.23B.instruction_code"}, ["SPRI", "SSTD", "SPAY"]]},
            {"!!": {"var": "fields.57"}}
          ]},
          {"and": [
            {"in": [{"var": "fields.57.option"}, ["A", "C", "D"]]},
            {
              "if": [
                {"==": [{"var": "fields.57.option"}, "D"]},
                {"!!": {"var": "fields.57.party_identifier"}},
                true
              ]
            }
          ]},
          true
        ]
      }
    },
    {
      "id": "C12",
      "description": "If field 23B contains SPRI, SSTD or SPAY, Account in field 59a is mandatory",
      "condition": {
        "if": [
          {"in": [{"var": "fields.23B.instruction_code"}, ["SPRI", "SSTD", "SPAY"]]},
          {"!!": {"var": "fields.59.account"}},
          true
        ]
      }
    },
    {
      "id": "C13",
      "description": "If any field 23E contains CHQB, Account in field 59a is not allowed",
      "condition": {
        "if": [
          {"and": [
            {"!!": {"var": "fields.23E"}},
            {
              "some": [
                {"var": "fields.23E"},
                {"==": [{"var": "instruction_code"}, "CHQB"]}
              ]
            }
          ]},
          {"!": {"var": "fields.59.account"}},
          true
        ]
      }
    },
    {
      "id": "C14",
      "description": "If field 71A contains OUR, then field 71F is not allowed and field 71G is optional. If field 71A contains SHA, then field 71F is optional and field 71G is not allowed. If field 71A contains BEN, then at least one occurrence of field 71F is mandatory and field 71G is not allowed",
      "condition": {
        "and": [
          {
            "if": [
              {"==": [{"var": "fields.71A.code"}, "OUR"]},
              {"!": {"var": "fields.71F"}},
              true
            ]
          },
          {
            "if": [
              {"==": [{"var": "fields.71A.code"}, "SHA"]},
              {"!": {"var": "fields.71G"}},
              true
            ]
          },
          {
            "if": [
              {"==": [{"var": "fields.71A.code"}, "BEN"]},
              {"and": [
                {"!!": {"var": "fields.71F"}},
                {">": [{"var": "fields.71F.length"}, 0]},
                {"!": {"var": "fields.71G"}}
              ]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "C15",
      "description": "If either field 71F (at least one occurrence) or field 71G is present, then field 33B is mandatory",
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
      "id": "C16",
      "description": "If field 56a is not present, no field 23E may contain TELI or PHOI",
      "condition": {
        "if": [
          {"!": {"var": "fields.56"}},
          {
            "if": [
              {"!!": {"var": "fields.23E"}},
              {
                "none": [
                  {"var": "fields.23E"},
                  {"in": [{"var": "instruction_code"}, ["TELI", "PHOI"]]}
                ]
              },
              true
            ]
          },
          true
        ]
      }
    },
    {
      "id": "C17",
      "description": "If field 57a is not present, no field 23E may contain TELE or PHON",
      "condition": {
        "if": [
          {"!": {"var": "fields.57"}},
          {
            "if": [
              {"!!": {"var": "fields.23E"}},
              {
                "none": [
                  {"var": "fields.23E"},
                  {"in": [{"var": "instruction_code"}, ["TELE", "PHON"]]}
                ]
              },
              true
            ]
          },
          true
        ]
      }
    },
    {
      "id": "C18",
      "description": "The currency code in the fields 71G and 32A must be the same",
      "condition": {
        "if": [
          {"!!": {"var": "fields.71G"}},
          {"==": [{"var": "fields.71G.currency"}, {"var": "fields.32A.currency"}]},
          true
        ]
      }
    },
    {
      "id": "INSTRUCTION_CODE_VALIDATION",
      "description": "23E instruction codes must be valid when present",
      "condition": {
        "if": [
          {"!!": {"var": "fields.23E"}},
          {
            "all": [
              {"var": "fields.23E"},
              {"in": [{"var": "instruction_code"}, ["CORT", "INTC", "REPA", "SDVA", "CHQB", "PHOB", "PHOI", "PHON", "TELE", "TELI", "TELB"]]}
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
              {
                "all": [
                  {"var": "fields.71F"},
                  {">": [{"var": "amount"}, 0]}
                ]
              },
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
              {
                "all": [
                  {"var": "fields.71F"},
                  {"!=": [{"var": "currency"}, ""]}
                ]
              },
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
          {"!=": [{"var": "fields.20.reference"}, ""]},
          {"!": {"in": ["//", {"var": "fields.20.reference"}]}}
        ]
      }
    },
    {
      "id": "BIC_VALIDATION",
      "description": "All BIC codes must be properly formatted (non-empty)",
      "condition": {
        "and": [
          {"!=": [{"var": "basic_header.sender_bic"}, ""]},
          {"!=": [{"var": "application_header.receiver_bic"}, ""]},
          {
            "if": [
              {"!!": {"var": "fields.52.A"}},
              {"!=": [{"var": "fields.52.A.bic"}, ""]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.53.A"}},
              {"!=": [{"var": "fields.53.A.bic"}, ""]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.57.A"}},
              {"!=": [{"var": "fields.57.A.bic"}, ""]},
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
            {"!=": [{"var": "fields.77T.envelope_content"}, ""]}
          ]},
          true
        ]
      }
    }
  ],
  "constants": {
    "EU_EEA_COUNTRIES": ["AD", "AT", "BE", "BG", "BV", "CH", "CY", "CZ", "DE", "DK", "ES", "EE", "FI", "FR", "GB", "GF", "GI", "GP", "GR", "HU", "IE", "IS", "IT", "LI", "LT", "LU", "LV", "MC", "MQ", "MT", "NL", "NO", "PL", "PM", "PT", "RE", "RO", "SE", "SI", "SJ", "SK", "SM", "TF", "VA"],
    "VALID_BANK_OPERATION_CODES": ["CRED", "CRTS", "SPAY", "SPRI", "SSTD"],
    "VALID_CHARGE_CODES": ["OUR", "SHA", "BEN"],
    "VALID_INSTRUCTION_CODES": ["CORT", "INTC", "REPA", "SDVA", "CHQB", "PHOB", "PHOI", "PHON", "TELE", "TELI", "TELB"]
  }
}"#;
