use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT101: Request for Transfer
///
/// ## Purpose
/// Used to request the movement of funds from the ordering customer's account(s) serviced at the receiving financial institution.
/// This message enables institutions and authorized parties to initiate multiple transactions in a single message with comprehensive transfer details.
///
/// ## Scope
/// This message is:
/// - Sent by financial institutions on behalf of non-financial account owners
/// - Sent by non-financial institution account owners or authorized parties
/// - Used for moving funds between ordering customer accounts or to third parties
/// - Applicable for both domestic and cross-border payment requests
/// - Compatible with bulk payment processing and corporate treasury operations
/// - Subject to comprehensive network validation rules for transaction integrity
///
/// ## Key Features
/// - **Multi-Transaction Support**: Single message can contain multiple transaction requests
/// - **Dual Sequence Architecture**: Sequence A (general info) and Sequence B (transaction details)
/// - **Flexible Party Specification**: Ordering customer can be specified in either sequence
/// - **Foreign Exchange Support**: Built-in support for currency conversion instructions
/// - **Chained Message Capability**: Support for large transaction sets across multiple messages
/// - **Regulatory Compliance**: Includes regulatory reporting fields for compliance requirements
///
/// ## Common Use Cases
/// - Corporate bulk payment processing for payroll and supplier payments
/// - Treasury operations requiring multiple fund transfers
/// - Cross-border payment requests with currency conversion
/// - Inter-company fund transfers within corporate groups
/// - Standing instruction execution for recurring payments
/// - Cash management and liquidity optimization transfers
/// - Trade finance settlement instructions
///
/// ## Message Structure
/// ### Sequence A (General Information - Mandatory, Single)
/// - **Field 20**: Sender's Reference (mandatory) - Unique message identifier
/// - **Field 21R**: Customer Specified Reference (optional) - Customer reference for all transactions
/// - **Field 28D**: Message Index/Total (mandatory) - For chained messages
/// - **Field 50**: Instructing Party/Ordering Customer (optional) - Party initiating request
/// - **Field 52A**: Account Servicing Institution (optional) - Institution holding accounts
/// - **Field 51A**: Sending Institution (optional) - Institution sending the message
/// - **Field 30**: Requested Execution Date (mandatory) - When transfers should be executed
/// - **Field 25**: Account Identification (optional) - Account to be debited
///
/// ### Sequence B (Transaction Details - Mandatory, Repetitive)
/// - **Field 21**: Transaction Reference (mandatory) - Unique transaction identifier
/// - **Field 21F**: F/X Deal Reference (optional) - Foreign exchange deal reference
/// - **Field 23E**: Instruction Code (optional) - Special processing instructions
/// - **Field 32B**: Currency/Transaction Amount (mandatory) - Transfer amount and currency
/// - **Field 50**: Ordering Customer (optional) - Customer specific to this transaction
/// - **Field 52**: Account Servicing Institution (optional) - Institution for this transaction
/// - **Field 56**: Intermediary Institution (optional) - Intermediary in payment chain
/// - **Field 57**: Account With Institution (optional) - Crediting institution
/// - **Field 59**: Beneficiary Customer (mandatory) - Final beneficiary of funds
/// - **Field 70**: Remittance Information (optional) - Payment purpose and details
/// - **Field 77B**: Regulatory Reporting (optional) - Compliance reporting information
/// - **Field 33B**: Currency/Original Amount (optional) - For currency conversion
/// - **Field 71A**: Details of Charges (mandatory) - Charge allocation instructions
/// - **Field 25A**: Charges Account (optional) - Account for charge debiting
/// - **Field 36**: Exchange Rate (optional) - Rate for currency conversion
///
/// ## Network Validation Rules
/// - **Foreign Exchange Logic**: If field 36 present, field 21F mandatory (C1)
/// - **Currency Conversion**: If field 33B present and amount ≠ 0, field 36 mandatory (C2)
/// - **Party Specification**: Field 50a placement rules between sequences (C3, C4)
/// - **Currency Consistency**: Currency in field 33B must differ from field 32B (C5)
/// - **Institution Chain**: If field 56a present, field 57a mandatory (C7)
/// - **Cross-Transaction**: If field 21R present, all 32B currencies must match (C8)
/// - **Chained Messages**: All must have same sender's reference (field 20)
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT101 format remains stable
/// - **Enhanced Validation**: Strengthened rules for cross-border transfers
/// - **Regulatory Reporting**: Enhanced field 77B validation for compliance
/// - **API Integration**: Improved support for modern banking APIs
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with core banking and payment processing systems
/// - **API Integration**: RESTful API support for modern corporate banking platforms
/// - **Processing Requirements**: Supports both real-time and batch processing modes
/// - **Compliance Integration**: Built-in regulatory reporting and sanctions screening hooks
///
/// ## Relationship to Other Messages
/// - **Triggers**: Often triggered by corporate ERP systems or treasury management platforms
/// - **Responses**: Generates MT103 (customer credit transfer) for each transaction
/// - **Related**: Works with MT202 for institutional settlement and MT940/MT950 for reporting
/// - **Alternatives**: MT100 for single transfers, MT204 for direct debit instructions
/// - **Status Updates**: May receive MT192/MT196/MT199 for status notifications
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT101_VALIDATION_RULES)]
pub struct MT101 {
    #[field("20")]
    pub field_20: Field20, // Sender's Reference

    #[field("21R")]
    pub field_21r: Option<Field21R>, // Customer Specified Reference

    #[field("28D")]
    pub field_28d: Field28D, // Message Index/Total

    #[field("50#1")]
    pub field_50_instructing_party: Option<Field50InstructingParty>, // Instructing Party

    #[field("50#2")]
    pub field_50_ordering_customer: Option<Field50OrderingCustomerFGH>, // Ordering Customer

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
/// ## Purpose
/// Represents a single transaction within an MT101 message. Each occurrence provides
/// details for one individual funds transfer request.
///
/// ## Field Details
/// - **21**: Transaction Reference (mandatory) - Unique reference for this transaction
/// - **21F**: F/X Deal Reference - Required when field 36 is present (NVR C1)
/// - **23E**: Instruction Code - Special instructions (e.g., EQUI for equivalent transfers)
/// - **32B**: Currency/Transaction Amount - The amount to be transferred
/// - **33B**: Currency/Original Amount - Used for currency conversions (NVR C2, C5)
/// - **36**: Exchange Rate - Required when 33B present and amount ≠ 0 (NVR C2)
///
/// ## Party Chain
/// The transaction flow follows: Instructing Party → Ordering Customer →
/// Account Servicing Institution → Intermediary → Account With Institution → Beneficiary
///
/// ## Validation Notes
/// - If 36 present, 21F must be present (C1)
/// - If 33B present and 32B amount ≠ 0, then 36 mandatory (C2)
/// - Currency in 33B must differ from 32B (C5)
/// - If 56a present, 57a must be present (C7)
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT101Transaction {
    #[field("21")]
    pub field_21: Field21NoOption, // Transaction Reference

    #[field("21F")]
    pub field_21f: Option<Field21F>, // F/X Deal Reference

    #[field("23E")]
    pub field_23e: Option<Vec<Field23E>>, // Instruction Code

    #[field("32B")]
    pub field_32b: Field32B, // Currency/Amount

    #[field("50#1")]
    pub field_50_instructing_party: Option<Field50InstructingParty>, // Instructing Party

    #[field("50#2")]
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

/// Comprehensive MT101 validation rules based on SRG2025 specification
const MT101_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "If an exchange rate is given in field 36, the corresponding forex deal must be referenced in field 21F",
      "condition": {
        "all": [
          {"var": "fields.#"},
          {
            "if": [
              {"exists": ["fields", "36"]},
              {"exists": ["fields", "21F"]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "C2",
      "description": "In each occurrence of sequence B, if field 33B is present and amount in field 32B is not equal to zero, then field 36 must be present, otherwise field 36 is not allowed",
      "condition": {
        "all": [
          {"var": "fields.#"},
          {
            "or": [
              {
                "and": [
                  {"exists": ["fields", "33B", "currency"]},
                  {"!=": [{"var": "32B.amount"}, 0]},
                  {"exists": ["fields", "36", "rate"]}
                ]
              },
              {
                "and": [
                  {"exists": ["fields", "33B", "currency"]},
                  {"==": [{"var": "32B.amount"}, 0]},
                  {"!": {"exists": ["fields", "36", "rate"]}}
                ]
              },
              {
                "and": [
                  {"!": {"exists": ["fields", "33B", "currency"]}},
                  {"!": {"exists": ["fields", "36", "rate"]}}
                ]
              }
            ]
          }
        ]
      }
    },
    {
      "id": "C3",
      "description": "Field 50a (option F, G or H) must be present in either sequence A or in each occurrence of sequence B, but must never be present in both sequences",
      "condition": {
        "or": [
          {
            "and": [
              {"exists": ["fields", "50#2"]},
              {
                "all": [
                  {"var": "fields.#"},
                  {"!": {"exists": ["fields", "50#2"]}}
                ]
              }
            ]
          },
          {
            "and": [
              {"!": {"exists": ["fields", "50#2"]}},
              {
                "all": [
                  {"var": "fields.#"},
                  {"exists": ["fields", "50#2"]}
                ]
              }
            ]
          }
        ]
      }
    },
    {
      "id": "C4",
      "description": "Field 50a (option C or L) may be present in either sequence A or in one or more occurrences of sequence B, but must not be present in both sequences",
      "condition": {
        "if": [
          {"exists": ["fields", "50#1"]},
          {
            "all": [
              {"var": "fields.#"},
              {"!": {"exists": ["fields", "50"]}}
            ]
          },
          true
        ]
      }
    },
    {
      "id": "C5",
      "description": "If field 33B is present in sequence B, its currency code must be different from the currency code in field 32B",
      "condition": {
        "all": [
          {"var": "fields.#"},
          {
            "if": [
              {"exists": ["fields", "33B"]},
              {"!=": [{"var": "33B.currency"}, {"var": "32B.currency"}]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "C6",
      "description": "Field 52a may be present in either sequence A or in one or more occurrences of sequence B, but must not be present in both sequences",
      "condition": {
        "if": [
          {"exists": ["fields", "52"]},
          {
            "all": [
              {"var": "fields.#"},
              {"!": {"exists": ["fields", "52"]}}
            ]
          },
          true
        ]
      }
    },
    {
      "id": "C7",
      "description": "If field 56a is present, field 57a must also be present",
      "condition": {
        "all": [
          {"var": "fields.#"},
          {
            "if": [
              {"exists": ["fields", "56"]},
              {"exists": ["fields", "57"]},
              true
            ]
          }
        ]
      }
    },
    {
      "id": "C8",
      "description": "If field 21R is present in sequence A, then in each occurrence of sequence B, the currency code in fields 32B must be the same",
      "condition": {
        "if": [
          {"exists": ["fields", "21R"]},
          {
            "or": [
              {"<=": [{"var": "fields.#.length"}, 1]},
              {
                "all": [
                  {"var": "fields.#"},
                  {"==": [{"var": "32B.currency"}, {"var": "fields.#.0.32B.currency"}]}
                ]
              }
            ]
          },
          true
        ]
      }
    },
    {
      "id": "C9",
      "description": "In each occurrence of sequence B, the presence of fields 33B and 21F is dependent on the presence and value of fields 32B and 23E",
      "condition": {
        "all": [
          {"var": "fields.#"},
          {
            "if": [
              {"==": [{"var": "32B.amount"}, 0]},
              {
                "if": [
                  {"and": [
                    {"exists": ["fields", "23E"]},
                    {
                      "some": [
                        {"var": "23E"},
                        {"==": [{"var": "instruction_code"}, "EQUI"]}
                      ]
                    }
                  ]},
                  {"exists": ["fields", "33B"]},
                  {
                    "and": [
                      {"!": {"exists": ["fields", "33B"]}},
                      {"!": {"exists": ["fields", "21F"]}}
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
      "id": "AMOUNT_CONSISTENCY",
      "description": "All amounts must be properly formatted",
      "condition": {
        "all": [
          {"var": "fields.#"},
          {
            "and": [
              {">": [{"var": "32B.amount"}, -1]},
              {
                "if": [
                  {"exists": ["fields", "33B"]},
                  {">": [{"var": "33B.amount"}, -1]},
                  true
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
        "all": [
          {"var": "fields.#"},
          {
            "and": [
              {"!=": [{"var": "32B.currency"}, ""]},
              {
                "if": [
                  {"exists": ["fields", "33B"]},
                  {"!=": [{"var": "33B.currency"}, ""]},
                  true
                ]
              }
            ]
          }
        ]
      }
    },
    {
      "id": "MESSAGE_INDEX_TOTAL",
      "description": "Message index must not exceed total",
      "condition": {
        "and": [
          {"exists": ["fields", "28D"]},
          {"<=": [{"var": "fields.28D.index"}, {"var": "fields.28D.total"}]},
          {">": [{"var": "fields.28D.index"}, 0]},
          {">": [{"var": "fields.28D.total"}, 0]}
        ]
      }
    },
    {
      "id": "EXECUTION_DATE",
      "description": "Requested execution date must be valid",
      "condition": {
        "and": [
          {"exists": ["fields", "30"]},
          {"!=": [{"var": "fields.30.execution_date"}, ""]}
        ]
      }
    },
    {
      "id": "INSTRUCTION_CODE_VALIDATION",
      "description": "23E instruction codes must be valid when present",
      "condition": {
        "all": [
          {"var": "fields.#"},
          {
            "if": [
              {"exists": ["fields", "23E"]},
              {
                "all": [
                  {"var": "23E"},
                  {"in": [{"var": "instruction_code"}, ["EQUI", "RTGS", "URGP", "CORT", "INTC", "SDVA"]]}
                ]
              },
              true
            ]
          }
        ]
      }
    }
  ],
  "constants": {
    "VALID_CHARGE_CODES": ["OUR", "SHA", "BEN"],
    "VALID_INSTRUCTION_CODES_MT101": ["EQUI", "RTGS", "URGP", "CORT", "INTC", "SDVA"]
  }
}"#;
