use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

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
