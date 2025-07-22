use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT104: Direct Debit and Request for Debit Transfer Message
///
/// ## Purpose
/// Used for customer direct debit instructions between financial institutions, allowing creditors to request debiting of debtor accounts through the banking network.
/// This message enables efficient collection of payments through authorized direct debit arrangements.
///
/// ## Scope
/// This message is:
/// - Sent by corporate entities to their financial institutions for payment collection
/// - Used to request direct debit of debtor accounts with proper authorization
/// - Applicable for both domestic and international direct debit scenarios
/// - Compatible with bulk processing of multiple direct debit transactions
/// - Subject to strict authorization and regulatory compliance requirements
/// - Used in conjunction with direct debit mandates and agreements
///
/// ## Key Features
/// - **Multi-Transaction Support**: Single message can contain multiple direct debit requests
/// - **Three-Sequence Architecture**: General info, transaction details, and settlement information
/// - **Authorization Framework**: Built-in support for direct debit mandates and agreements
/// - **Currency Flexibility**: Supports different currencies and exchange rate conversions
/// - **Charge Allocation Options**: Configurable charge handling (OUR/SHA/BEN)
/// - **Regulatory Compliance**: Comprehensive fields for regulatory reporting requirements
///
/// ## Common Use Cases
/// - Utility companies collecting monthly bills from customer accounts
/// - Insurance companies collecting premium payments via direct debit
/// - Subscription service providers collecting recurring fees
/// - Loan servicing companies collecting installment payments
/// - Government agencies collecting taxes and fees
/// - Corporate collection of accounts receivable from customers
/// - Automated clearing house (ACH) equivalent processing
///
/// ## Message Structure
/// ### Sequence A (General Information - Mandatory, Single)
/// - **Field 20**: Sender's Reference (mandatory) - Unique message identifier
/// - **Field 21R**: Customer Specified Reference (optional) - Creditor's batch reference
/// - **Field 28D**: Message Index/Total (mandatory) - For chained messages
/// - **Field 30**: Requested Execution Date (mandatory) - When debits should be executed
/// - **Field 25**: Account Identification (optional) - Creditor's account for credits
/// - **Field 50**: Instructing Party (optional) - Party authorizing the direct debits
///
/// ### Sequence B (Transaction Details - Mandatory, Repetitive)
/// - **Field 21**: Transaction Reference (mandatory) - Unique transaction identifier
/// - **Field 32B**: Currency/Amount (mandatory) - Amount to be debited
/// - **Field 50**: Ordering Customer/Debtor (mandatory) - Account to be debited
/// - **Field 52**: Account Servicing Institution (optional) - Debtor's bank
/// - **Field 57**: Account With Institution (optional) - Intermediary institution
/// - **Field 59**: Beneficiary/Creditor (mandatory) - Account to be credited
/// - **Field 70**: Remittance Information (optional) - Payment purpose and details
/// - **Field 77B**: Regulatory Reporting (optional) - Compliance information
/// - **Field 71A**: Details of Charges (mandatory) - Charge allocation instructions
///
/// ### Sequence C (Settlement Information - Optional, Single)
/// - **Field 32A**: Value Date/Currency/Total Amount (optional) - Settlement summary
/// - **Field 19**: Sum of Amounts (optional) - Total amount of all transactions
/// - **Field 71F**: Sender's Charges (optional) - Total charges claimed
/// - **Field 71G**: Receiver's Charges (optional) - Total charges to be deducted
///
/// ## Network Validation Rules
/// - **Authorization Validation**: Proper direct debit mandate verification required
/// - **Transaction Limits**: Individual and batch transaction limit enforcement
/// - **Currency Consistency**: Currency validation across sequences (Note: C11 requires custom implementation due to JSONLogic limitations)
/// - **Charge Allocation**: Consistent charge handling across all transactions
/// - **Settlement Totals**: Sequence C totals must match sum of Sequence B amounts
/// - **Reference Uniqueness**: Transaction references must be unique within batch
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT104 format remains unchanged
/// - **Enhanced Validation**: Strengthened authorization and mandate validation
/// - **Regulatory Compliance**: Enhanced field 77B validation for reporting
/// - **Processing Improvements**: Better support for modern direct debit frameworks
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with core banking and payment processing systems
/// - **Authorization Systems**: Integration with direct debit mandate management systems
/// - **API Integration**: RESTful API support for modern payment platforms
/// - **Processing Requirements**: Supports batch processing with settlement coordination
///
/// ## Relationship to Other Messages
/// - **Triggers**: Often triggered by recurring payment schedules or collection processes
/// - **Responses**: May generate MT103 (credit transfers) for creditor account credits
/// - **Related**: Works with MT202 for settlement and MT940/MT950 for account reporting
/// - **Alternatives**: MT101 for credit transfers, MT103 for individual payments
/// - **Authorization**: Requires underlying direct debit mandates and agreements
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT104_VALIDATION_RULES)]
pub struct MT104 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21R")]
    pub field_21r: Option<Field21R>,

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
    pub transactions: Vec<MT104Transaction>,

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

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT104Transaction {
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
    pub field_59: Field59Debtor,

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

/// Comprehensive MT104 validation rules based on SRG2025 specification
const MT104_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "If field 23E is present in sequence A and contains RFDD then field 23E must be present in all occurrences of sequence B. If field 23E is present in sequence A and does not contain RFDD then field 23E must not be present in any occurrence of sequence B. If field 23E is not present in sequence A then field 23E must be present in all occurrences of sequence B",
      "condition": {
        "if": [
          {"!!": {"var": "fields.23E"}},
          {
            "if": [
              {"==": [{"var": "fields.23E.instruction_code"}, "RFDD"]},
              {
                "all": [
                  {"var": "fields.#"},
                  {"!!": {"var": "23E"}}
                ]
              },
              {
                "all": [
                  {"var": "fields.#"},
                  {"!": {"var": "23E"}}
                ]
              }
            ]
          },
          {
            "all": [
              {"var": "fields.#"},
              {"!!": {"var": "23E"}}
            ]
          }
        ]
      }
    },
    {
      "id": "C2",
      "description": "Field 50a (option A or K) must be present in either sequence A or in each occurrence of sequence B, but must never be present in both sequences",
      "condition": {
        "or": [
          {
            "and": [
              {"or": [
                {"!!": {"var": "fields.50#2.A"}},
                {"!!": {"var": "fields.50#2.K"}}
              ]},
              {
                "all": [
                  {"var": "fields.#"},
                  {
                    "and": [
                      {"!": {"var": "50#2.A"}},
                      {"!": {"var": "50#2.K"}}
                    ]
                  }
                ]
              }
            ]
          },
          {
            "and": [
              {
                "and": [
                  {"!": {"var": "fields.50#2.A"}},
                  {"!": {"var": "fields.50#2.K"}}
                ]
              },
              {
                "all": [
                  {"var": "fields.#"},
                  {
                    "or": [
                      {"!!": {"var": "50#2.A"}},
                      {"!!": {"var": "50#2.K"}}
                    ]
                  }
                ]
              }
            ]
          }
        ]
      }
    },
    {
      "id": "C3",
      "description": "When present in sequence A, fields 21E, 26T, 52a, 71A, 77B and 50a (option C or L) must not be present in any occurrence of sequence B",
      "condition": {
        "and": [
          {
            "if": [
              {"!!": {"var": "fields.21E"}},
              {
                "all": [
                  {"var": "fields.#"},
                  {"!": {"var": "21E"}}
                ]
              },
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.26T"}},
              {
                "all": [
                  {"var": "fields.#"},
                  {"!": {"var": "26T"}}
                ]
              },
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.52"}},
              {
                "all": [
                  {"var": "fields.#"},
                  {"!": {"var": "52"}}
                ]
              },
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.71A"}},
              {
                "all": [
                  {"var": "fields.#"},
                  {"!": {"var": "71A"}}
                ]
              },
              true
            ]
          },
          {
            "if": [
              {"!!": {"var": "fields.77B"}},
              {
                "all": [
                  {"var": "fields.#"},
                  {"!": {"var": "77B"}}
                ]
              },
              true
            ]
          },
          {
            "if": [
              {"or": [
                {"!!": {"var": "fields.50#1.C"}},
                {"!!": {"var": "fields.50#1.L"}}
              ]},
              {
                "all": [
                  {"var": "fields.#"},
                  {
                    "and": [
                      {"!": {"var": "50#1.C"}},
                      {"!": {"var": "50#1.L"}}
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
      "id": "C4",
      "description": "If field 21E is present in sequence A, field 50a (option A or K) must also be present in sequence A. In each occurrence of sequence B, if field 21E is present, then field 50a (option A or K) must also be present",
      "condition": {
        "and": [
          {
            "if": [
              {"!!": {"var": "fields.21E"}},
              {
                "or": [
                  {"!!": {"var": "fields.50#2.A"}},
                  {"!!": {"var": "fields.50#2.K"}}
                ]
              },
              true
            ]
          },
          {
            "all": [
              {"var": "fields.#"},
              {
                "if": [
                  {"!!": {"var": "21E"}},
                  {
                    "or": [
                      {"!!": {"var": "50#2.A"}},
                      {"!!": {"var": "50#2.K"}}
                    ]
                  },
                  true
                ]
              }
            ]
          }
        ]
      }
    },
    {
      "id": "C5",
      "description": "In sequence A, if field 23E is present and contains RTND then field 72 must be present",
      "condition": {
        "if": [
          {"and": [
            {"!!": {"var": "fields.23E"}},
            {"==": [{"var": "fields.23E.instruction_code"}, "RTND"]}
          ]},
          {"!!": {"var": "fields.72"}},
          {
            "if": [
              {"!!": {"var": "fields.23E"}},
              {"!": {"var": "fields.72"}},
              {"!": {"var": "fields.72"}}
            ]
          }
        ]
      }
    },
    {
      "id": "C6",
      "description": "If field 71F is present in one or more occurrence of sequence B, then it must also be present in sequence C, and vice-versa. Same for field 71G",
      "condition": {
        "and": [
          {
            "if": [
              {
                "some": [
                  {"var": "fields.#"},
                  {"!!": {"var": "71F"}}
                ]
              },
              {"!!": {"var": "fields.71F"}},
              {
                "if": [
                  {"!!": {"var": "fields.71F"}},
                  {
                    "some": [
                      {"var": "fields.#"},
                      {"!!": {"var": "71F"}}
                    ]
                  },
                  true
                ]
              }
            ]
          },
          {
            "if": [
              {
                "some": [
                  {"var": "fields.#"},
                  {"!!": {"var": "71G"}}
                ]
              },
              {"!!": {"var": "fields.71G"}},
              {
                "if": [
                  {"!!": {"var": "fields.71G"}},
                  {
                    "some": [
                      {"var": "fields.#"},
                      {"!!": {"var": "71G"}}
                    ]
                  },
                  true
                ]
              }
            ]
          }
        ]
      }
    },
    {
      "id": "C7",
      "description": "In each occurrence of sequence B, if field 33B is present then the currency code or the amount, or both, must be different between fields 33B and 32B",
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
      "id": "C8",
      "description": "In any occurrence of sequence B, if field 33B is present and the currency codes in fields 32B and 33B are different, then field 36 must be present",
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
                  {
                    "if": [
                      {"==": [{"var": "33B.currency"}, {"var": "32B.currency"}]},
                      {"!": {"var": "36"}},
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
      "id": "C9",
      "description": "If sequence C is present and if the amount in field 32B of sequence C is equal to the sum of the amounts of the fields 32B of sequence B, then field 19 must not be present",
      "condition": {
        "if": [
          {"!!": {"var": "fields.32B"}},
          {
            "if": [
              {"==": [
                {"var": "fields.32B.amount"}, 
                {
                  "reduce": [
                    {"var": "fields.#"},
                    {"+": [{"var": "accumulator"}, {"var": "current.32B.amount"}]},
                    0
                  ]
                }
              ]},
              {"!": {"var": "fields.19"}},
              {"!!": {"var": "fields.19"}}
            ]
          },
          true
        ]
      }
    },
    {
      "id": "C10",
      "description": "If field 19 is present in sequence C then it must be equal to the sum of the amounts in all occurrences of field 32B in sequence B",
      "condition": {
        "if": [
          {"!!": {"var": "fields.19"}},
          {"==": [
            {"var": "fields.19.amount"}, 
            {
              "reduce": [
                {"var": "fields.#"},
                {"+": [{"var": "accumulator"}, {"var": "current.32B.amount"}]},
                0
              ]
            }
          ]},
          true
        ]
      }
    },
    {
      "id": "C11",
      "description": "The currency code in fields 32B and 71G in sequences B and C must be the same for all occurrences. The currency code in fields 71F must be the same for all occurrences",
      "condition": true
    },
    {
      "id": "C12",
      "description": "If field 23E in sequence A contains RFDD, then field 21R is optional, fields in sequence B are restricted, and sequence C must not be present",
      "condition": {
        "if": [
          {"and": [
            {"!!": {"var": "fields.23E"}},
            {"==": [{"var": "fields.23E.instruction_code"}, "RFDD"]}
          ]},
          {
            "and": [
              {
                "all": [
                  {"var": "fields.#"},
                  {
                    "and": [
                      {"!": {"var": "21E"}},
                      {"!": {"var": "50#2.A"}},
                      {"!": {"var": "50#2.K"}},
                      {"!": {"var": "52"}},
                      {"!": {"var": "71F"}},
                      {"!": {"var": "71G"}}
                    ]
                  }
                ]
              },
              {"!": {"var": "fields.32B"}},
              {"!": {"var": "fields.19"}},
              {"!": {"var": "fields.71F"}},
              {"!": {"var": "fields.71G"}},
              {"!": {"var": "fields.53"}}
            ]
          },
          {
            "and": [
              {"!": {"var": "fields.21R"}},
              {"or": [
                {"!!": {"var": "fields.32B"}},
                {"!!": {"var": "fields.19"}},
                {"!!": {"var": "fields.71F"}},
                {"!!": {"var": "fields.71G"}},
                {"!!": {"var": "fields.53"}}
              ]}
            ]
          }
        ]
      }
    },
    {
      "id": "C13",
      "description": "If field 23E in sequence A is present and contains RFDD, then field 119 of User Header must be present and contain RFDD",
      "condition": {
        "if": [
          {"and": [
            {"!!": {"var": "fields.23E"}},
            {"==": [{"var": "fields.23E.instruction_code"}, "RFDD"]}
          ]},
          {"and": [
            {"!!": {"var": "user_header.validation_flag"}},
            {"==": [{"var": "user_header.validation_flag"}, "RFDD"]}
          ]},
          {"!": {"var": "user_header.validation_flag"}}
        ]
      }
    },
    {
      "id": "TRANSACTION_MINIMUM",
      "description": "At least one transaction must be present in sequence B",
      "condition": true
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
    "VALID_INSTRUCTION_CODES_MT104": ["RFDD", "RTND", "SDVA", "INTC", "CORT"]
  }
}"#;
