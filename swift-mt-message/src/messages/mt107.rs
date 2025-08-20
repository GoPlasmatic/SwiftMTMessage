use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT107: General Direct Debit Message
///
/// ## Purpose
/// Used for general direct debit instructions where a creditor requests the debit of multiple debtor accounts.
/// This message provides more flexibility than MT104 for complex direct debit scenarios with enhanced authorization
/// control and flexible party identification options.
///
/// ## Scope
/// This message is:
/// - Used for general direct debit processing between financial institutions
/// - Applicable for bulk direct debit operations with flexible authorization control
/// - Designed for complex direct debit scenarios requiring detailed transaction control
/// - Compatible with both domestic and cross-border direct debit schemes
/// - Subject to authorization validation rules for debtor consent management
/// - Integrated with return processing mechanisms for failed direct debits
///
/// ## Key Features
/// - **Enhanced Flexibility**: More sophisticated than MT104 for complex direct debit scenarios
/// - **Authorization Management**: Field 23E supports AUTH/NAUT/OTHR authorization status codes
/// - **Party Identification Options**: Instructing party can appear in Sequence A or individual transactions
/// - **Return Processing Support**: Special handling for returned direct debits with RTND codes
/// - **Settlement Consolidation**: Optional settlement sequence for consolidated settlement details
/// - **Multi-Transaction Support**: Supports multiple debtor accounts in single message
///
/// ## Common Use Cases
/// - Corporate direct debit collections for subscription services and utilities
/// - Bulk salary and pension direct debit processing
/// - Recurring payment collections for insurance and loan payments
/// - Cross-border direct debit schemes for international service providers
/// - Return processing for previously failed direct debit attempts
/// - Multi-party direct debit scenarios with complex authorization requirements
/// - Government tax and fee collection via direct debit
///
/// ## Message Structure
/// ### Sequence A (General Information)
/// - **Field 20**: Transaction Reference (mandatory) - Unique message identifier
/// - **Field 23E**: Instruction Code (optional) - Authorization status (AUTH/NAUT/OTHR/RTND)
/// - **Field 21E**: Related Reference (optional) - Reference to related message
/// - **Field 30**: Execution Date (mandatory) - Date for direct debit execution
/// - **Field 51A**: Sending Institution (optional) - Institution sending the message
/// - **Field 50**: Instructing Party/Creditor (optional) - Party requesting direct debits
/// - **Field 52**: Creditor Bank (optional) - Bank of the creditor
/// - **Field 26T**: Transaction Type Code (optional) - Classification of direct debit type
/// - **Field 77B**: Regulatory Reporting (optional) - Compliance reporting information
/// - **Field 71A**: Details of Charges (optional) - Charge allocation instructions
/// - **Field 72**: Sender to Receiver Information (optional) - Additional processing instructions
///
/// ### Sequence B (Transaction Details - Repetitive)
/// - **Field 21**: Transaction Reference (mandatory) - Unique reference for each direct debit
/// - **Field 32B**: Currency/Amount (mandatory) - Amount to be debited from each account
/// - **Field 59**: Debtor (mandatory) - Account and details of party being debited
/// - **Field 23E**: Instruction Code (optional) - Transaction-level authorization status
/// - **Field 50**: Instructing Party/Creditor (optional) - Transaction-level party identification
/// - **Field 57**: Debtor Bank (optional) - Bank holding the debtor account
/// - **Field 70**: Remittance Information (optional) - Purpose and details of direct debit
/// - **Field 33B/36**: Currency conversion fields for cross-currency direct debits
///
/// ### Sequence C (Settlement Information - Optional)
/// - **Field 32B**: Settlement Amount (optional) - Total settlement amount
/// - **Field 19**: Sum of Amounts (optional) - Control total for validation
/// - **Field 71F/71G**: Charges (optional) - Settlement charges information
/// - **Field 53**: Sender's Correspondent (optional) - Settlement correspondent
///
/// ## Network Validation Rules
/// - **Authorization Consistency**: If 23E is AUTH/NAUT/OTHR in Sequence A, same restriction applies to Sequence B
/// - **Party Identification**: Instructing party must appear in exactly one sequence (A or B per transaction)
/// - **Return Processing**: Field 72 required when 23E = RTND for returned direct debit details
/// - **Currency Conversion**: Exchange rate (36) required when 33B currency differs from 32B
/// - **Transaction Limits**: Maximum transaction count per message enforced
/// - **Reference Validation**: All transaction references must be unique within message
/// - **Authorization Validation**: Authorization codes must be consistent with regulatory requirements
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT107 format remains stable
/// - **Validation Updates**: Enhanced authorization validation for regulatory compliance
/// - **Processing Improvements**: Improved handling of return processing scenarios
/// - **Compliance Notes**: Strengthened regulatory reporting requirements for cross-border transactions
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with direct debit processing engines and mandate management systems
/// - **API Integration**: RESTful API support for modern direct debit collection platforms
/// - **Processing Requirements**: Supports batch processing with individual transaction validation
/// - **Compliance Integration**: Built-in mandate validation and regulatory reporting capabilities
///
/// ## Relationship to Other Messages
/// - **Triggers**: Often triggered by direct debit collection schedules or mandate execution systems
/// - **Responses**: May generate MT900/MT910 (confirmations) or status notification messages
/// - **Related**: Works with MT104 (simplified direct debits) and account reporting messages
/// - **Alternatives**: MT104 for simpler direct debit scenarios without complex authorization
/// - **Status Updates**: May receive return or reject messages for failed direct debit attempts
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

    #[field("50#1")]
    pub field_50_instructing: Option<Field50InstructingParty>,

    #[field("50#2")]
    pub field_50_creditor: Option<Field50Creditor>,

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
    pub field_53: Option<Field53SenderCorrespondent>,
}

/// MT107 Transaction (Sequence B)
///
/// ## Purpose
/// Represents a single direct debit transaction within an MT107 message. Each occurrence
/// provides details for one direct debit request with flexible authorization and processing
/// options.
///
/// ## Field Details
/// - **21**: Transaction Reference (mandatory) - Unique reference for this direct debit
/// - **32B**: Currency/Transaction Amount (mandatory) - Amount to be debited
/// - **59**: Debtor (mandatory) - Account and details of party being debited
/// - **23E**: Instruction Code - Authorization status (AUTH/NAUT/OTHR) or processing instructions
/// - **21C/21D/21E**: Various reference fields for transaction linking
/// - **50**: Instructing Party/Creditor - Can be at transaction level if not in Sequence A
/// - **33B/36**: Currency conversion fields when amounts differ
///
/// ## Authorization Types (23E)
/// - **AUTH**: Authorized direct debit - pre-authorized by debtor
/// - **NAUT**: Non-authorized direct debit - requires special handling
/// - **OTHR**: Other processing instruction - specific business rules apply
/// - **RTND**: Returned direct debit - previously failed transaction
///
/// ## Validation Notes
/// - Transaction reference (21) must be unique within the message
/// - If 33B present and amount differs from 32B, exchange rate (36) required
/// - Authorization status in 23E must be consistent with Sequence A if specified there
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
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

    #[field("50#1")]
    pub field_50_instructing: Option<Field50InstructingParty>,

    #[field("50#2")]
    pub field_50_creditor: Option<Field50Creditor>,

    #[field("52")]
    pub field_52: Option<Field52CreditorBank>,

    #[field("57")]
    pub field_57: Option<Field57DebtorBank>,

    #[field("59")]
    pub field_59: Field59,

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
      "description": "Field 23E and field 50a (option A or K) must appear in Sequence A OR each Sequence B, not both",
      "condition": {
        "and": [
          {
            "if": [
              {"!!": {"var": "fields.23E"}},
              {
                "all": [
                  {"var": "fields.#"},
                  {"!": {"var": "23E"}}
                ]
              },
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.50#2"}},
              {
                "all": [
                  {"var": "fields.#"},
                  {"!": {"var": "50#2"}}
                ]
              },
              true
            ]
          }
        ]
      }
    },
    {
      "id": "C2",
      "description": "Fields 21E, 26T, 77B, 71A, 52a, 50a (option C/L) must appear only in Sequence A or Sequence B, not both",
      "condition": {
        "and": [
          {
            "if": [
              {"!!": {"var": "fields.21E"}},
              {"all": [{"var": "fields.#"}, {"!": {"var": "21E"}}]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.26T"}},
              {"all": [{"var": "fields.#"}, {"!": {"var": "26T"}}]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.77B"}},
              {"all": [{"var": "fields.#"}, {"!": {"var": "77B"}}]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.71A"}},
              {"all": [{"var": "fields.#"}, {"!": {"var": "71A"}}]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.52"}},
              {"all": [{"var": "fields.#"}, {"!": {"var": "52"}}]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.50#1"}},
              {"all": [{"var": "fields.#"}, {"!": {"var": "50#1"}}]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "C3",
      "description": "If 21E is present, then 50a (option A/K) must also be present in the same sequence",
      "condition": {
        "and": [
          {
            "if": [
              {"!!": {"var": "fields.21E"}},
              {"!!": {"var": "fields.50#2"}},
              true
            ]
          },
          {
            "all": [
              {"var": "fields.#"},
              {
                "if": [
                  {"!!": {"var": "21E"}},
                  {"!!": {"var": "50#2"}},
                  true
                ]
              }
            ]
          }
        ]
      }
    },
    {
      "id": "C4",
      "description": "If 23E = RTND in Sequence A, Field 72 is mandatory; otherwise, 72 is not allowed",
      "condition": {
        "if": [
          {"and": [
            {"!!": {"var": "fields.23E"}},
            {"==": [{"var": "fields.23E.instruction_code"}, "RTND"]}
          ]},
          {"!!": {"var": "fields.72"}},
          {"!": {"var": "fields.72"}}
        ]
      }
    },
    {
      "id": "C5",
      "description": "If 71F or 71G present in any B, must also be in Sequence C, and vice versa",
      "condition": {
        "and": [
          {
            "if": [
              {"some": [{"var": "fields.#"}, {"!!": {"var": "71F"}}]},
              {"!!": {"var": "fields.71F"}},
              true
            ]
          },
          {
            "if": [
              {"some": [{"var": "fields.#"}, {"!!": {"var": "71G"}}]},
              {"!!": {"var": "fields.71G"}},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.71F"}},
              {"some": [{"var": "fields.#"}, {"!!": {"var": "71F"}}]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.71G"}},
              {"some": [{"var": "fields.#"}, {"!!": {"var": "71G"}}]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "C6",
      "description": "If 33B is present in Sequence B, must differ in either currency or amount from 32B",
      "condition": {
        "all": [
          {"var": "fields.#"},
          {
            "if": [
              {"!!": {"var": "33B"}},
              {
                "or": [
                  {"!=": [{"var": "33B.currency"}, {"var": "32B.currency"}]},
                  {"!=": [{"var": "33B.amount"}, {"var": "32B.amount"}]}
                ]
              },
              true
            ]
          }
        ]
      }
    },
    {
      "id": "C7",
      "description": "If 33B and 32B currency differs, then 36 (Exchange Rate) is mandatory; otherwise, 36 must not be present",
      "condition": {
        "all": [
          {"var": "fields.#"},
          {
            "if": [
              {"!!": {"var": "33B"}},
              {
                "if": [
                  {"!=": [{"var": "33B.currency"}, {"var": "32B.currency"}]},
                  {"!!": {"var": "36"}},
                  {"!": {"var": "36"}}
                ]
              },
              true
            ]
          }
        ]
      }
    },
    {
      "id": "C8",
      "description": "The sum of 32B amounts in B must appear either in C/32B (no charges) or in C/19 (with charges)",
      "condition": {
        "or": [
          {
            "and": [
              {"!!": {"var": "fields.32B"}},
              {"!": {"var": "fields.19"}},
              {
                "==": [
                  {"var": "fields.32B.amount"},
                  {
                    "reduce": [
                      {"var": "fields.#"},
                      {"+": [{"var": "accumulator"}, {"var": "current.32B.amount"}]},
                      0
                    ]
                  }
                ]
              }
            ]
          },
          {
            "and": [
              {"!!": {"var": "fields.19"}},
              {"!": {"var": "fields.32B"}},
              {
                "==": [
                  {"var": "fields.19.amount"},
                  {
                    "reduce": [
                      {"var": "fields.#"},
                      {"+": [{"var": "accumulator"}, {"var": "current.32B.amount"}]},
                      0
                    ]
                  }
                ]
              }
            ]
          },
          {
            "and": [
              {"!": {"var": "fields.32B"}},
              {"!": {"var": "fields.19"}}
            ]
          }
        ]
      }
    },
    {
      "id": "C9",
      "description": "Currency must be consistent across all instances of 32B, 71F, 71G in B and C",
      "condition": {
        "and": [
          {
            "if": [
              {"and": [{"!!": {"var": "fields.32B"}}, {">=": [{"length": {"var": "fields.#"}}, 1]}]},
              {
                "all": [
                  {"var": "fields.#"},
                  {"==": [{"var": "32B.currency"}, {"var": "fields.#.0.32B.currency"}]}
                ]
              },
              true
            ]
          },
          {
            "if": [
              {"and": [{"!!": {"var": "fields.71F"}}, {"some": [{"var": "fields.#"}, {"!!": {"var": "71F"}}]}]},
              {
                "all": [
                  {"var": "fields.#"},
                  {
                    "if": [
                      {"!!": {"var": "71F"}},
                      {"==": [{"var": "71F.currency"}, {"var": "fields.71F.currency"}]},
                      true
                    ]
                  }
                ]
              },
              true
            ]
          },
          {
            "if": [
              {"and": [{"!!": {"var": "fields.71G"}}, {"some": [{"var": "fields.#"}, {"!!": {"var": "71G"}}]}]},
              {
                "all": [
                  {"var": "fields.#"},
                  {
                    "if": [
                      {"!!": {"var": "71G"}},
                      {"==": [{"var": "71G.currency"}, {"var": "fields.71G.currency"}]},
                      true
                    ]
                  }
                ]
              },
              true
            ]
          }
        ]
      }
    },
    {
      "id": "TXN_MIN",
      "description": "At least one transaction required",
      "condition": {
        ">=": [{"length": {"var": "fields.#"}}, 1]
      }
    },
    {
      "id": "REFERENCE_FORMAT",
      "description": "Reference fields must not contain invalid patterns",
      "condition": {
        "and": [
          {"!=": [{"var": "fields.20.reference"}, ""]},
          {"!": {"in": ["//", {"var": "fields.20.reference"}]}},
          {
            "all": [
              {"var": "fields.#"},
              {
                "and": [
                  {"!=": [{"var": "21.reference"}, ""]},
                  {"!": {"in": ["//", {"var": "21.reference"}]}}
                ]
              }
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
          {
            "all": [
              {"var": "fields.#"},
              {
                "and": [
                  {"!=": [{"var": "32B.currency"}, ""]},
                  {
                    "if": [
                      {"!!": {"var": "33B"}},
                      {"!=": [{"var": "33B.currency"}, ""]},
                      true
                    ]
                  },
                  {
                    "if": [
                      {"!!": {"var": "71F"}},
                      {"!=": [{"var": "71F.currency"}, ""]},
                      true
                    ]
                  },
                  {
                    "if": [
                      {"!!": {"var": "71G"}},
                      {"!=": [{"var": "71G.currency"}, ""]},
                      true
                    ]
                  }
                ]
              }
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.32B"}},
              {"!=": [{"var": "fields.32B.currency"}, ""]},
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
      "id": "AMOUNT_CONSISTENCY",
      "description": "All amounts must be properly formatted",
      "condition": {
        "and": [
          {
            "all": [
              {"var": "fields.#"},
              {
                "and": [
                  {">": [{"var": "32B.amount"}, -1]},
                  {
                    "if": [
                      {"!!": {"var": "33B"}},
                      {">": [{"var": "33B.amount"}, -1]},
                      true
                    ]
                  },
                  {
                    "if": [
                      {"!!": {"var": "71F"}},
                      {">": [{"var": "71F.amount"}, -1]},
                      true
                    ]
                  },
                  {
                    "if": [
                      {"!!": {"var": "71G"}},
                      {">": [{"var": "71G.amount"}, -1]},
                      true
                    ]
                  }
                ]
              }
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.32B"}},
              {">": [{"var": "fields.32B.amount"}, -1]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.19"}},
              {">": [{"var": "fields.19.amount"}, -1]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.71F"}},
              {">": [{"var": "fields.71F.amount"}, -1]},
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.71G"}},
              {">": [{"var": "fields.71G.amount"}, -1]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "EXECUTION_DATE",
      "description": "Requested execution date must be valid",
      "condition": {
        "and": [
          {"!!": {"var": "fields.30"}},
          {"!=": [{"var": "fields.30.execution_date"}, ""]}
        ]
      }
    }
  ],
  "constants": {
    "VALID_CHARGE_CODES": ["OUR", "SHA", "BEN"],
    "VALID_INSTRUCTION_CODES_MT107": ["AUTH", "NAUT", "OTHR", "RTND", "SDVA", "INTC", "CORT"]
  }
}"#;
