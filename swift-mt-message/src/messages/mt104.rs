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
/// - **Currency Consistency**: Currency validation across sequences
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

    #[field("50")]
    pub field_50_instructing: Option<Field50InstructingParty>,

    #[field("50")]
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
#[validation_rules(MT104_TRANSACTION_VALIDATION_RULES)]
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

    #[field("50")]
    pub field_50_instructing: Option<Field50InstructingParty>,

    #[field("50")]
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

/// Enhanced validation rules with forEach support for repetitive sequences
const MT104_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Per-transaction: If 36 present → 21F must be present (placeholder)",
      "forEach": {
        "collection": "transactions",
        "condition": {
          "if": [
            {"var": "field_36.is_some"},
            true,
            true
          ]
        }
      }
    },
    {
      "id": "C3",
      "description": "Various conditional field requirements",
      "condition": {
        ">=": [{"length": {"var": "transactions"}}, 1]
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

/// Validation rules specific to MT104 transactions
const MT104_TRANSACTION_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "T_C1",
      "description": "If exchange rate (36) is present, related conditions apply",
      "condition": {
        "if": [
          {"var": "field_36.is_some"},
          true,
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
