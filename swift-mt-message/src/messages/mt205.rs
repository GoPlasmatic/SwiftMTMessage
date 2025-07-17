use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

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
    pub field_13c: Vec<Field13C>, // Time Indication (repetitive)

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
    #[field("50")]
    pub field_50_seq_b: Option<Field50OrderingCustomerAFK>, // Ordering Customer

    #[field("52")]
    pub field_52_seq_b: Option<Field52OrderingInstitution>, // Ordering Institution (Seq B)

    #[field("56")]
    pub field_56_seq_b: Option<Field56Intermediary>, // Intermediary Institution (Seq B)

    #[field("57")]
    pub field_57_seq_b: Option<Field57AccountWithInstitution>, // Account With Institution (Seq B)

    #[field("59")]
    pub field_59_seq_b: Option<Field59>, // Beneficiary Customer

    #[field("70")]
    pub field_70_seq_b: Option<Field70>, // Remittance Information

    #[field("72")]
    pub field_72_seq_b: Option<Field72>, // Sender to Receiver Info (Seq B)

    #[field("33B")]
    pub field_33b_seq_b: Option<Field33B>, // Currency/Instructed Amount
}

impl MT205 {
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
        self.field_50_seq_b.is_some() || self.field_59_seq_b.is_some()
    }
}

/// Validation rules for MT205 - General Financial Institution Transfer
const MT205_VALIDATION_RULES: &str = r#"{
	"rules": [
		{
			"id": "C1",
			"description": "Transaction Reference (20) must not start or end with '/' and must not contain '//'",
			"condition": {
				"and": [
					{"!": {"matches": [{"var": "field_20.value"}, "^/"]}},
					{"!": {"matches": [{"var": "field_20.value"}, "/$"]}},
					{"!": {"matches": [{"var": "field_20.value"}, "//"]}}
				]
			}
		},
		{
			"id": "C2",
			"description": "Related Reference (21) must not start or end with '/' and must not contain '//'",
			"condition": {
				"and": [
					{"!": {"matches": [{"var": "field_21.value"}, "^/"]}},
					{"!": {"matches": [{"var": "field_21.value"}, "/$"]}},
					{"!": {"matches": [{"var": "field_21.value"}, "//"]}}
				]
			}
		},
		{
			"id": "C3",
			"description": "Field 52a is mandatory in MT205 (no fallback to sender BIC)",
			"condition": {
				"!=": [{"var": "field_52a.bic"}, ""]
			}
		},
		{
			"id": "C4",
			"description": "Field 54a is not present in MT205 (structural difference from MT202)",
			"condition": true
		},
		{
			"id": "C5",
			"description": "Cover message detection based on Sequence B customer fields presence",
			"condition": {
				"if": [
					{"or": [
						{"var": "field_50a.is_some"},
						{"var": "field_59a.is_some"},
						{"var": "field_70.is_some"}
					]},
					{"var": "field_52a_seq_b.is_some"},
					true
				]
			}
		},
		{
			"id": "C6",
			"description": "Cross-currency validation: if 33B present, currency should differ from 32A",
			"condition": {
				"if": [
					{"var": "field_33b.is_some"},
					{"!=": [{"var": "field_33b.currency"}, {"var": "field_32a.currency"}]},
					true
				]
			}
		},
		{
			"id": "C7",
			"description": "REJT/RETN indicator validation in field 72",
			"condition": {
				"if": [
					{"var": "field_72.is_some"},
					{"or": [
						{"!": {"matches": [{"var": "field_72.lines"}, "/REJT/"]}},
						{"!": {"matches": [{"var": "field_72.lines"}, "/RETN/"]}},
						true
					]},
					true
				]
			}
		},
		{
			"id": "C8",
			"description": "Time indication validation for CLS/TARGET timing",
			"condition": {
				"if": [
					{"var": "field_13c.is_some"},
					{"allValid": [
						{"var": "field_13c"},
						{"matches": [{"var": "time_code"}, "^(SNDTIME|RNCTIME|CLSTIME|TILTIME|FROTIME|REJTIME)$"]}
					]},
					true
				]
			}
		},
		{
			"id": "C9",
			"description": "Settlement method determination (METAFCT003 - simplified scenarios)",
			"condition": {
				"if": [
					{"var": "field_53a.is_some"},
					{"!=": [{"var": "field_53a.bic"}, {"var": "field_52a.bic"}]},
					true
				]
			}
		}
	]
}"#;
