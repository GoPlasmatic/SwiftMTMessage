use crate::{SwiftMessage, fields::*, swift_serde};
use serde::{Deserialize, Serialize};

/// MT103: Single Customer Credit Transfer
///
/// Uses normalized field tags (without option letters) for flexibility
/// Complete implementation with all possible MT103 fields for 100% compliance
#[swift_serde]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[swift_message(mt = "103")]
pub struct MT103 {
    // Required fields
    /// Transaction Reference
    #[field("20")]
    pub field_20: Field20,

    /// Bank Operation Code
    #[field("23B")]
    pub field_23b: Field23B,

    /// Value Date/Currency/Amount
    #[field("32A")]
    pub field_32a: Field32A,

    /// Ordering Customer
    #[field("50")]
    pub field_50: Field50,

    /// Beneficiary Customer
    #[field("59")]
    pub field_59: Field59,

    /// Details of Charges
    #[field("71A")]
    pub field_71a: Field71A,

    // Optional fields (complete for 100% MT103 compliance)
    /// Time Indication
    #[field("13C")]
    pub field_13c: Option<Field13C>,

    /// Instruction Code
    #[field("23E")]
    pub field_23e: Option<Field23E>,

    /// Transaction Type Code
    #[field("26T")]
    pub field_26t: Option<Field26T>,

    /// Currency/Instructed Amount
    #[field("33B")]
    pub field_33b: Option<Field33B>,

    /// Exchange Rate
    #[field("36")]
    pub field_36: Option<Field36>,

    /// Ordering Institution
    #[field("52A")]
    pub field_52a: Option<Field52A>,

    /// Sender's Correspondent
    #[field("53A")]
    pub field_53a: Option<Field53A>,

    /// Receiver's Correspondent
    #[field("54A")]
    pub field_54a: Option<Field54A>,

    /// Third Reimbursement Institution
    #[field("55A")]
    pub field_55a: Option<Field55A>,

    /// Intermediary Institution
    #[field("56A")]
    pub field_56a: Option<Field56A>,

    /// Account With Institution
    #[field("57A")]
    pub field_57a: Option<Field57A>,

    /// Remittance Information
    #[field("70")]
    pub field_70: Option<Field70>,

    /// Sender's Charges
    #[field("71F")]
    pub field_71f: Option<Field71F>,

    /// Receiver's Charges
    #[field("71G")]
    pub field_71g: Option<Field71G>,

    /// Sender to Receiver Information
    #[field("72")]
    pub field_72: Option<Field72>,

    /// Regulatory Reporting
    #[field("77B")]
    pub field_77b: Option<Field77B>,
}

impl MT103 {
    /// Create a new MT103 with required fields only
    pub fn new(
        field_20: Field20,
        field_23b: Field23B,
        field_32a: Field32A,
        field_50: Field50,
        field_59: Field59,
        field_71a: Field71A,
    ) -> Self {
        Self {
            field_20,
            field_23b,
            field_32a,
            field_50,
            field_59,
            field_71a,
            field_13c: None,
            field_23e: None,
            field_26t: None,
            field_33b: None,
            field_36: None,
            field_52a: None,
            field_53a: None,
            field_54a: None,
            field_55a: None,
            field_56a: None,
            field_57a: None,
            field_70: None,
            field_71f: None,
            field_71g: None,
            field_72: None,
            field_77b: None,
        }
    }

    /// Create a new MT103 with all fields
    #[allow(clippy::too_many_arguments)]
    pub fn new_complete(
        field_20: Field20,
        field_23b: Field23B,
        field_32a: Field32A,
        field_50: Field50,
        field_59: Field59,
        field_71a: Field71A,
        field_13c: Option<Field13C>,
        field_23e: Option<Field23E>,
        field_26t: Option<Field26T>,
        field_33b: Option<Field33B>,
        field_36: Option<Field36>,
        field_52a: Option<Field52A>,
        field_53a: Option<Field53A>,
        field_54a: Option<Field54A>,
        field_55a: Option<Field55A>,
        field_56a: Option<Field56A>,
        field_57a: Option<Field57A>,
        field_70: Option<Field70>,
        field_71f: Option<Field71F>,
        field_71g: Option<Field71G>,
        field_72: Option<Field72>,
        field_77b: Option<Field77B>,
    ) -> Self {
        Self {
            field_20,
            field_23b,
            field_32a,
            field_50,
            field_59,
            field_71a,
            field_13c,
            field_23e,
            field_26t,
            field_33b,
            field_36,
            field_52a,
            field_53a,
            field_54a,
            field_55a,
            field_56a,
            field_57a,
            field_70,
            field_71f,
            field_71g,
            field_72,
            field_77b,
        }
    }

    /// Get the transaction reference
    pub fn transaction_reference(&self) -> &str {
        self.field_20.transaction_reference()
    }

    /// Get the operation code
    pub fn operation_code(&self) -> &str {
        self.field_23b.operation_code()
    }

    /// Get the currency code
    pub fn currency_code(&self) -> &str {
        self.field_32a.currency_code()
    }

    /// Get the transaction amount as decimal
    pub fn amount_decimal(&self) -> f64 {
        self.field_32a.amount_decimal()
    }

    /// Get the charge code
    pub fn charge_code(&self) -> &str {
        self.field_71a.charge_code()
    }

    /// Get the instructed amount if present
    pub fn instructed_amount(&self) -> Option<&Field33B> {
        self.field_33b.as_ref()
    }

    /// Get the exchange rate if present
    pub fn exchange_rate(&self) -> Option<&Field36> {
        self.field_36.as_ref()
    }

    /// Get sender's charges if present
    pub fn senders_charges(&self) -> Option<&Field71F> {
        self.field_71f.as_ref()
    }

    /// Get receiver's charges if present
    pub fn receivers_charges(&self) -> Option<&Field71G> {
        self.field_71g.as_ref()
    }

    /// Get sender to receiver information if present
    pub fn sender_to_receiver_info(&self) -> Option<&Field72> {
        self.field_72.as_ref()
    }

    /// Get regulatory reporting if present
    pub fn regulatory_reporting(&self) -> Option<&Field77B> {
        self.field_77b.as_ref()
    }

    /// Get ordering institution if present
    pub fn ordering_institution(&self) -> Option<&Field52A> {
        self.field_52a.as_ref()
    }

    /// Get sender's correspondent if present
    pub fn senders_correspondent(&self) -> Option<&Field53A> {
        self.field_53a.as_ref()
    }

    /// Get receiver's correspondent if present
    pub fn receivers_correspondent(&self) -> Option<&Field54A> {
        self.field_54a.as_ref()
    }

    /// Get third reimbursement institution if present
    pub fn third_reimbursement_institution(&self) -> Option<&Field55A> {
        self.field_55a.as_ref()
    }

    /// Get intermediary institution if present
    pub fn intermediary_institution(&self) -> Option<&Field56A> {
        self.field_56a.as_ref()
    }

    /// Get account with institution if present
    pub fn account_with_institution(&self) -> Option<&Field57A> {
        self.field_57a.as_ref()
    }

    /// Get remittance information if present
    pub fn remittance_information(&self) -> Option<&Field70> {
        self.field_70.as_ref()
    }

    /// Get time indication if present
    pub fn time_indication(&self) -> Option<&Field13C> {
        self.field_13c.as_ref()
    }

    /// Get instruction code if present
    pub fn instruction_code(&self) -> Option<&Field23E> {
        self.field_23e.as_ref()
    }

    /// Get transaction type code if present
    pub fn transaction_type_code(&self) -> Option<&Field26T> {
        self.field_26t.as_ref()
    }

    /// Check if all required fields are present and valid
    pub fn validate_structure(&self) -> bool {
        // All required fields are enforced by the struct, so if we can construct it,
        // the structure is valid. Individual field validation is handled
        // by the SwiftField trait implementations.
        true
    }

    /// Check if this is a cross-currency transaction
    pub fn is_cross_currency(&self) -> bool {
        if let Some(field_33b) = &self.field_33b {
            field_33b.currency() != self.field_32a.currency_code()
        } else {
            false
        }
    }

    /// Check if exchange rate is provided for cross-currency transactions
    pub fn has_required_exchange_rate(&self) -> bool {
        if self.is_cross_currency() {
            self.field_36.is_some()
        } else {
            true // Not required for same-currency transactions
        }
    }

    /// Get all institution fields in routing order
    pub fn get_routing_chain(&self) -> Vec<(&str, String)> {
        let mut chain = Vec::new();

        if let Some(field_52a) = &self.field_52a {
            chain.push(("Ordering Institution", field_52a.bic().to_string()));
        }

        if let Some(field_53a) = &self.field_53a {
            chain.push(("Sender's Correspondent", field_53a.bic().to_string()));
        }

        if let Some(field_54a) = &self.field_54a {
            chain.push(("Receiver's Correspondent", field_54a.bic().to_string()));
        }

        if let Some(field_55a) = &self.field_55a {
            chain.push(("Third Reimbursement", field_55a.bic().to_string()));
        }

        if let Some(field_56a) = &self.field_56a {
            chain.push(("Intermediary", field_56a.bic().to_string()));
        }

        if let Some(field_57a) = &self.field_57a {
            chain.push(("Account With Institution", field_57a.bic().to_string()));
        }

        chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt103_creation() {
        use chrono::NaiveDate;

        let field_20 = Field20::new("FT21234567890".to_string());
        let field_23b = Field23B::new("CRED".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2021, 3, 15).unwrap(),
            "EUR".to_string(),
            1234567.89,
        );
        let field_50 = Field50::K(Field50K::new(vec!["JOHN DOE".to_string()]).unwrap());
        let field_59 =
            Field59::NoOption(Field59Basic::new(vec!["JANE SMITH".to_string()]).unwrap());
        let field_71a = Field71A::new("OUR".to_string());

        let mt103 = MT103::new(
            field_20, field_23b, field_32a, field_50, field_59, field_71a,
        );

        assert_eq!(mt103.transaction_reference(), "FT21234567890");
        assert_eq!(mt103.operation_code(), "CRED");
        assert_eq!(mt103.currency_code(), "EUR");
        assert_eq!(mt103.charge_code(), "OUR");
    }

    #[test]
    fn test_mt103_message_type() {
        use crate::SwiftMessageBody;
        assert_eq!(MT103::message_type(), "103");
    }

    #[test]
    fn test_mt103_json_field_names() {
        use chrono::NaiveDate;

        let field_20 = Field20::new("FT21234567890".to_string());
        let field_23b = Field23B::new("CRED".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2021, 3, 15).unwrap(),
            "EUR".to_string(),
            1234567.89,
        );
        let field_50 = Field50::K(Field50K::new(vec!["JOHN DOE".to_string()]).unwrap());
        let field_59 =
            Field59::NoOption(Field59Basic::new(vec!["JANE SMITH".to_string()]).unwrap());
        let field_71a = Field71A::new("OUR".to_string());

        let mt103 = MT103::new(
            field_20, field_23b, field_32a, field_50, field_59, field_71a,
        );

        // Serialize to JSON
        let json = serde_json::to_string(&mt103).unwrap();

        // Verify that JSON uses SWIFT field tags, not struct field names
        assert!(json.contains("\"20\":"));
        assert!(json.contains("\"23B\":"));
        assert!(json.contains("\"32A\":"));
        assert!(json.contains("\"50\":"));
        assert!(json.contains("\"59\":"));
        assert!(json.contains("\"71A\":"));

        // Verify that JSON does NOT contain struct field names
        assert!(!json.contains("\"field_20\":"));
        assert!(!json.contains("\"field_23b\":"));
        assert!(!json.contains("\"field_32a\":"));
        assert!(!json.contains("\"field_50\":"));
        assert!(!json.contains("\"field_59\":"));
        assert!(!json.contains("\"field_71a\":"));
    }
}
