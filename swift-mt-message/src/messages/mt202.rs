use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

/// MT202: General Financial Institution Transfer
///
/// ## Purpose
/// Used for financial institution-to-financial institution payments where both the ordering
/// and beneficiary customers are financial institutions. This message facilitates transfers
/// of funds between institutions for their own account or on behalf of third parties.
///
/// ## Scope
/// This message is:
/// - Sent between financial institutions for interbank transfers
/// - Used for settlement of obligations between financial institutions
/// - Applicable for both cover payments (with underlying customer details) and direct institution transfers
/// - Compatible with real-time gross settlement (RTGS) systems
/// - Subject to SRG2025 contingency processing for FI-to-FI transfers
///
/// ## Key Features
/// - **Dual Sequence Structure**:
///   - Sequence A: Basic interbank transfer details
///   - Sequence B: Cover payment details (when applicable)
/// - **Flexible Routing**: Support for correspondent banking chains through fields 52-57
/// - **Cover Payment Detection**: Automatic identification of cover vs. direct transfers
/// - **Reject/Return Handling**: Built-in support for payment exception processing
/// - **Settlement Integration**: Compatible with various settlement methods and systems
/// - **SRG2025 Compliance**: Enhanced network validation rules for contingency processing
///
/// ## Common Use Cases
/// - Interbank settlement transactions
/// - Cover payments for underlying customer transfers
/// - Foreign exchange settlement
/// - Correspondent banking transfers
/// - Central bank operations
/// - Cross-border payment settlement
/// - SWIFT gpi (global payments innovation) transactions
///
/// ## Message Structure
/// ### Sequence A (Mandatory)
/// - **Field 20**: Transaction Reference (mandatory) - Unique sender reference
/// - **Field 21**: Related Reference (mandatory) - Reference to related message/transaction
/// - **Field 13C**: Time Indication (optional, repetitive) - Processing time constraints
/// - **Field 32A**: Value Date/Currency/Amount (mandatory) - Settlement details
/// - **Field 52**: Ordering Institution (optional) - Institution initiating the transfer
/// - **Field 53**: Sender's Correspondent (optional) - Sender's correspondent bank
/// - **Field 54**: Receiver's Correspondent (optional) - Receiver's correspondent bank
/// - **Field 56**: Intermediary Institution (optional) - Intermediary in the payment chain
/// - **Field 57**: Account With Institution (optional) - Final crediting institution
/// - **Field 58**: Beneficiary Institution (mandatory) - Final beneficiary institution
/// - **Field 72**: Sender to Receiver Information (optional) - Additional instructions
///
/// ### Sequence B (Optional - Cover Payment Details)
/// - **Field 50**: Ordering Customer (optional) - Underlying ordering customer
/// - **Field 52**: Ordering Institution (optional) - Ordering institution details for cover
/// - **Field 56**: Intermediary Institution (optional) - Intermediary for cover payment
/// - **Field 57**: Account With Institution (optional) - Account details for cover
/// - **Field 59**: Beneficiary Customer (optional) - Underlying beneficiary customer
/// - **Field 70**: Remittance Information (optional) - Payment purpose/details
/// - **Field 72**: Sender to Receiver Info (optional) - Cover-specific instructions
/// - **Field 33B**: Currency/Instructed Amount (optional) - Original instructed amount
///
/// ## Network Validation Rules
/// - **Intermediary Chain Validation**: If field 56 is present, field 57 becomes mandatory
/// - **Cover Payment Structure**: Validation of Sequence B customer fields for cover detection
/// - **Cross-border Compliance**: Enhanced validation for contingency processing (SRG2025)
/// - **Settlement Method Validation**: Proper correspondent banking chain validation
/// - **Time Indication Compliance**: CLS/TARGET timing constraint validation
/// - **Reference Format Validation**: Proper format validation for all reference fields
/// - **REJT/RETN Indicators**: Structured validation of reject/return codes in field 72
///
/// ## SRG2025 Status
/// - **Structural Changes**: Enhanced - Additional network validated rules for contingency processing
/// - **Validation Updates**: Contingency processing applicable to FI-to-FI transfers
/// - **Processing Improvements**: ISO 20022 automatic conversion for compliant messages
/// - **Compliance Notes**: Scope includes FI-to-FI including MA-CUGs (excludes SCORE, MI-CUGs)
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with real-time gross settlement (RTGS) systems and net settlement systems
/// - **API Integration**: RESTful API support for modern interbank payment platforms
/// - **Processing Requirements**: Supports correspondent banking arrangements and central bank settlement
/// - **Compliance Integration**: Built-in support for cross-currency settlement and regulatory reporting
///
/// ## Relationship to Other Messages
/// - **Triggers**: Often triggered by MT103 customer payments requiring cover or institutional settlement
/// - **Responses**: May generate MT900/MT910 (confirmations) or MT292/MT296 (reject notifications)
/// - **Related**: Works with MT205 (with mandatory ordering institution) and account reporting messages
/// - **Alternatives**: MT205 for transfers requiring explicit ordering institution identification
/// - **Status Updates**: May receive MT192/MT196/MT199 for status notifications and inquiry responses

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT202_VALIDATION_RULES)]
pub struct MT202 {
    #[field("20")]
    pub field_20: Field20,

    #[field("21")]
    pub field_21: Field21NoOption,

    #[field("13C")]
    pub field_13c: Option<Vec<Field13C>>,

    #[field("32A")]
    pub field_32a: Field32A,

    #[field("52")]
    pub field_52: Option<Field52OrderingInstitution>,

    #[field("53")]
    pub field_53: Option<Field53SenderCorrespondent>,

    #[field("54")]
    pub field_54: Option<Field54ReceiverCorrespondent>,

    #[field("56")]
    pub field_56: Option<Field56Intermediary>,

    #[field("57")]
    pub field_57: Option<Field57AccountWithInstitution>,

    #[field("58")]
    pub field_58: Field58,

    #[field("72")]
    pub field_72: Option<Field72>,

    #[field("50")]
    pub field_50_seq_b: Option<Field50OrderingCustomerAFK>,

    #[field("52")]
    pub field_52_seq_b: Option<Field52OrderingInstitution>,

    #[field("56")]
    pub field_56_seq_b: Option<Field56Intermediary>,

    #[field("57")]
    pub field_57_seq_b: Option<Field57AccountWithInstitution>,

    #[field("59")]
    pub field_59_seq_b: Option<Field59>,

    #[field("70")]
    pub field_70_seq_b: Option<Field70>,

    #[field("72")]
    pub field_72_seq_b: Option<Field72>,

    #[field("33B")]
    pub field_33b_seq_b: Option<Field33B>,
}

impl MT202 {
    /// Check if this MT202 message contains reject codes
    ///
    /// Reject messages are identified by checking:
    /// 1. Field 20 (Transaction Reference) for "REJT" prefix or content
    /// 2. Field 72 (Sender to Receiver Information) containing `/REJT/` codes
    /// 3. Additional structured reject information in field 72
    pub fn has_reject_codes(&self) -> bool {
        // Check field 20 (transaction reference)
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

    /// Check if this MT202 message contains return codes
    ///
    /// Return messages are identified by checking:
    /// 1. Field 20 (Transaction Reference) for "RETN" prefix or content
    /// 2. Field 72 (Sender to Receiver Information) containing `/RETN/` codes
    /// 3. Additional structured return information in field 72
    pub fn has_return_codes(&self) -> bool {
        // Check field 20 (transaction reference)
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

    /// Check if this MT202 message is a Cover (COV) message
    ///
    /// COV messages are distinguished by:
    /// - Presence of customer fields (50A/50 and 59A/59) indicating underlying customer details
    /// - Field 121 (UETR) in Block 3 is typically mandatory for COV messages
    pub fn is_cover_message(&self) -> bool {
        // COV messages contain customer fields that indicate underlying customer credit transfer details
        self.field_50_seq_b.is_some() && (self.field_59_seq_b.is_some())
    }
}

const MT202_VALIDATION_RULES: &str = r#"{
  "rules": [
    {
      "id": "C1",
      "description": "If 56a is present, 57a becomes mandatory",
      "condition": {
        "if": [
          {"or": [
            {"var": "field_56a.is_some"},
            {"var": "field_56d.is_some"}
          ]},
          {"or": [
            {"var": "field_57a.is_some"},
            {"var": "field_57b.is_some"},
            {"var": "field_57d.is_some"}
          ]},
          true
        ]
      }
    }
  ],
  "constants": {
    "VALID_TIME_CODES": ["CLS", "RNC", "SND"],
    "VALID_INSTRUCTION_CODES": ["/INT/", "/COV/", "/REIMBURSEMENT/", "/SETTLEMENT/", "/SDVA/", "/RETN/", "/REJT/"]
  }
}"#;
