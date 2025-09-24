use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT205: General Financial Institution Transfer
///
/// ## Purpose
/// Used for financial institution transfers where the ordering institution is always identified.
/// This message type is specifically designed for transfers initiated by a financial institution
/// for its own account or on behalf of its customers, with mandatory ordering institution details.
///
/// ## Scope
/// This message is:
/// - Sent between financial institutions for institutional transfers
/// - Used when the ordering institution must be explicitly identified (unlike MT202)
/// - Applicable for both direct transfers and cover payments
/// - Compatible with correspondent banking arrangements
/// - Subject to enhanced validation rules for cross-border payments
/// - Includes contingency processing capabilities for qualifying transfers
///
/// ## Key Features
/// - **Mandatory Ordering Institution**: Field 52 is always required (key difference from MT202)
/// - **Dual Sequence Architecture**:
///   - Sequence A: Institution-to-institution transfer details
///   - Sequence B: Underlying customer payment details (for cover payments)
/// - **Enhanced Validation**: Sophisticated rules for institution identification and routing
/// - **Cover Payment Support**: Full support for cover payment scenarios with customer details
/// - **Settlement Flexibility**: Compatible with various settlement mechanisms
/// - **Cross-Currency Capability**: Support for foreign exchange operations
///
/// ## Common Use Cases
/// - Central bank operations requiring explicit ordering institution identification
/// - Correspondent banking transfers with mandatory institution details
/// - Cross-border institutional payments
/// - Treasury operations between financial institutions
/// - Cover payments for underlying customer transfers
/// - Settlement of securities transactions
/// - Liquidity management between institutions
///
/// ## Message Structure
/// ### Sequence A (Institution Transfer Details)
/// - **Field 20**: Transaction Reference (mandatory) - Unique transaction identifier
/// - **Field 21**: Related Reference (mandatory) - Reference to related transaction/message
/// - **Field 13C**: Time Indication (optional, repetitive) - Processing time constraints
/// - **Field 32A**: Value Date/Currency/Amount (mandatory) - Settlement amount and timing
/// - **Field 52**: Ordering Institution (mandatory) - Institution initiating the transfer
/// - **Field 53**: Sender's Correspondent (optional) - Sender's correspondent bank
/// - **Field 56**: Intermediary Institution (optional) - Intermediary in payment chain
/// - **Field 57**: Account With Institution (optional) - Crediting institution
/// - **Field 58**: Beneficiary Institution (mandatory) - Final beneficiary institution
/// - **Field 72**: Sender to Receiver Information (optional) - Additional instructions
///
/// ### Sequence B (Cover Payment Details - Optional)
/// - **Field 50**: Ordering Customer (optional) - Underlying ordering customer
/// - **Field 52**: Ordering Institution (optional) - Additional ordering institution details
/// - **Field 56**: Intermediary Institution (optional) - Cover payment intermediary
/// - **Field 57**: Account With Institution (optional) - Cover payment account details
/// - **Field 59**: Beneficiary Customer (optional) - Underlying beneficiary customer
/// - **Field 70**: Remittance Information (optional) - Payment purpose and details
/// - **Field 72**: Sender to Receiver Info (optional) - Cover-specific instructions
/// - **Field 33B**: Currency/Instructed Amount (optional) - Original currency/amount
///
/// ## Network Validation Rules
/// - **Field 52 Mandatory**: Ordering institution validation (field 52) - key difference from MT202
/// - **Cover Payment Structure**: Validation of Sequence B customer fields presence
/// - **Cross-Currency Validation**: Currency consistency between fields 32A and 33B
/// - **Correspondent Chain**: Banking chain validation for intermediary institutions
/// - **Settlement Method**: Determination based on correspondent relationships
/// - **Time Indication**: Compliance checking for CLS/TARGET timing constraints
/// - **REJT/RETN Indicators**: Structured validation of reject/return codes in field 72
///
/// ## SRG2025 Status
/// - **Structural Changes**: None - MT205 structure remains unchanged
/// - **Enhanced Validation**: Additional network rules for institutional transfers
/// - **Contingency Processing**: Enhanced processing rules for qualifying transfers
/// - **Cross-Border Compliance**: Strengthened validation for international payments
///
/// ## Integration Considerations
/// - **Banking Systems**: Compatible with real-time gross settlement (RTGS) systems
/// - **Central Bank Operations**: Direct integration with central bank settlement mechanisms
/// - **API Integration**: RESTful API support for modern banking infrastructure
/// - **Processing Requirements**: Support for time-critical payment processing
///
/// ## Relationship to Other Messages
/// - **Triggers**: Can be triggered by MT202COV or customer payment instructions
/// - **Responses**: May generate MT202, MT210 (notice to receive), or MT292 (reject)
/// - **Related**: Works with MT950/MT940 for account reporting and confirmation
/// - **Alternatives**: MT202 for transfers without mandatory ordering institution
/// - **Cover Payments**: Supports underlying MT103 customer credit transfers

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT205_VALIDATION_RULES)]
pub struct MT205 {
    // Sequence A: Mandatory Fields
    #[field("20")]
    pub field_20: Field20, // Transaction Reference Number

    #[field("21")]
    pub field_21: Field21NoOption, // Related Reference

    #[field("13C")]
    pub field_13c: Option<Vec<Field13C>>, // Time Indication (repetitive)

    #[field("32A")]
    pub field_32a: Field32A, // Value Date/Currency/Amount

    #[field("52")]
    pub field_52: Field52OrderingInstitution, // Ordering Institution (MANDATORY in MT205)

    #[field("53")]
    pub field_53: Option<Field53SenderCorrespondent>, // Sender's Correspondent

    #[field("56")]
    pub field_56: Option<Field56Intermediary>, // Intermediary Institution

    #[field("57")]
    pub field_57: Option<Field57AccountWithInstitution>, // Account With Institution

    #[field("58")]
    pub field_58: Field58, // Beneficiary Institution

    #[field("72")]
    pub field_72: Option<Field72>, // Sender to Receiver Information

    // Sequence B: COV Cover Message Fields (Optional)
    #[field("50", name = "ordering_customer_b")]
    pub ordering_customer_b: Option<Field50OrderingCustomerAFK>, // Ordering Customer

    #[field("52", name = "ordering_institution_b")]
    pub ordering_institution_b: Option<Field52OrderingInstitution>, // Ordering Institution (Seq B)

    #[field("56", name = "intermediary_b")]
    pub intermediary_b: Option<Field56Intermediary>, // Intermediary Institution (Seq B)

    #[field("57", name = "account_with_institution_b")]
    pub account_with_institution_b: Option<Field57AccountWithInstitution>, // Account With Institution (Seq B)

    #[field("59", name = "beneficiary_customer_b")]
    pub beneficiary_customer_b: Option<Field59>, // Beneficiary Customer

    #[field("70", name = "remittance_information_b")]
    pub remittance_information_b: Option<Field70>, // Remittance Information

    #[field("72", name = "sender_to_receiver_information_b")]
    pub sender_to_receiver_information_b: Option<Field72>, // Sender to Receiver Info (Seq B)

    #[field("33B", name = "currency_amount_b")]
    pub currency_amount_b: Option<Field33B>, // Currency/Instructed Amount
}

impl MT205 {
    /// Check if this MT205 message contains reject codes
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

    /// Check if this MT205 message contains return codes
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

    /// Check if this MT205 message is a Cover (COV) message
    ///
    /// COV messages are distinguished by:
    /// - Presence of Sequence B customer fields (50a, 59a)
    /// - Additional underlying customer credit transfer details
    ///
    /// Based on the MT205 specification: "Cover Detection: Based on presence of Sequence B customer fields (50a, 59a)"
    pub fn is_cover_message(&self) -> bool {
        // The key distinguishing feature of COV is the presence of Sequence B customer fields
        // According to spec: field 50a (Ordering Customer) or field 59a (Beneficiary Customer)
        self.ordering_customer_b.is_some() || self.beneficiary_customer_b.is_some()
    }
}

/// Validation rules for MT205 - General Financial Institution Transfer
const MT205_VALIDATION_RULES: &str = r#"{
	"rules": [
		{
			"id": "C1",
			"description": "If field 56a is present, then field 57a must also be present",
			"condition": {
				"if": [
					{"exists": ["fields", "56"]},
					{"exists": ["fields", "57"]},
					true
				]
			}
		},
		{
			"id": "C2",
			"description": "If field 56a is present in Sequence B, then field 57a must also be present",
			"condition": {
				"if": [
					{"exists": ["fields", "intermediary_b"]},
					{"exists": ["fields", "account_with_institution_b"]},
					true
				]
			}
		},
		{
			"id": "COV_FIELDS",
			"description": "MT205 COV must include both field 50a (Ordering Customer) and 59a (Beneficiary)",
			"condition": {
				"if": [
					{"or": [
						{"exists": ["fields", "ordering_customer_b"]},
						{"exists": ["fields", "beneficiary_customer_b"]}
					]},
					{"and": [
						{"exists": ["fields", "ordering_customer_b"]},
						{"exists": ["fields", "beneficiary_customer_b"]}
					]},
					true
				]
			}
		}
	]
}"#;
