use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};

/// MT200: Financial Institution Transfer for its Own Account
///
/// ## Purpose
/// Used for transfers between financial institutions where the institution is acting for its own account,
/// not on behalf of a customer. This message type facilitates proprietary fund movements between institutions
/// for purposes such as liquidity management, nostro account funding, or internal account adjustments.
///
/// ## Scope
/// This message is:
/// - Sent between financial institutions for their own account transfers
/// - Used for nostro/vostro account management and liquidity adjustments
/// - Applicable for same-currency transfers between correspondent accounts
/// - Not used for customer-initiated transfers or third-party payments
/// - Compatible with real-time gross settlement (RTGS) systems
/// - Suitable for both domestic and cross-border institutional transfers
///
/// ## Key Features
/// - **Simplified Structure**: Streamlined format for institution-to-institution transfers
/// - **Own Account Focus**: Specifically designed for proprietary institutional movements
/// - **Direct Routing**: Minimal intermediary support for direct institutional transfers
/// - **Settlement Efficiency**: Optimized for same-day value and immediate settlement
/// - **Correspondent Banking**: Built for nostro/vostro account management
/// - **Minimal Validation**: No complex customer validation rules required
///
/// ## Common Use Cases
/// - Nostro account funding and adjustments
/// - Vostro account management
/// - Liquidity transfers between branches
/// - Foreign exchange position management
/// - End-of-day settlement positions
/// - Cash concentration and disbursement
/// - Internal book transfers between institutions
///
/// ## Message Structure
/// - **Field 20**: Transaction Reference Number (mandatory) - Unique sender reference
/// - **Field 32A**: Value Date, Currency Code, Amount (mandatory) - Settlement details
/// - **Field 53B**: Sender's Correspondent (optional) - Account at correspondent bank
/// - **Field 56**: Intermediary (optional) - Intermediary institution if required
/// - **Field 57**: Account With Institution (mandatory) - Institution maintaining the account
/// - **Field 72**: Sender to Receiver Information (optional) - Additional instructions
///
/// ## Network Validation Rules
/// - No specific network validation rules apply to MT200
/// - Standard SWIFT field format validations apply
/// - BIC and account number format validations as per SWIFT standards
///
/// ## Integration Considerations
/// - **Banking Systems**: Direct integration with treasury and liquidity management systems
/// - **Settlement**: Compatible with major settlement systems and RTGS platforms
/// - **Processing**: Typically processed with high priority for same-day value
/// - **Reconciliation**: Simplified reconciliation due to institution-to-institution nature
///
/// ## Relationship to Other Messages
/// - **Related to MT202**: MT202 is used when acting on behalf of customers
/// - **Related to MT205**: MT205 includes mandatory ordering institution field
/// - **Confirmations**: May generate MT900 (debit) or MT910 (credit) confirmations
/// - **Account Reporting**: Reflected in MT940/MT950 account statements
/// - **Status Updates**: May receive MT192/MT196 for queries and responses
#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT200_VALIDATION_RULES)]
pub struct MT200 {
    #[field("20")]
    pub field_20: Field20,

    #[field("32A")]
    pub field_32a: Field32A,

    #[field("53B")]
    pub field_53b: Option<Field53B>,

    #[field("56")]
    pub field_56: Option<Field56IntermediaryAD>,

    #[field("57")]
    pub field_57: Field57DebtInstitution,

    #[field("72")]
    pub field_72: Option<Field72>,
}

impl MT200 {
    /// Creates a new MT200 message with mandatory fields
    pub fn new(
        transaction_reference: Field20,
        value_date_amount: Field32A,
        account_with_institution: Field57DebtInstitution,
    ) -> Self {
        Self {
            field_20: transaction_reference,
            field_32a: value_date_amount,
            field_53b: None,
            field_56: None,
            field_57: account_with_institution,
            field_72: None,
        }
    }

    /// Sets the sender's correspondent (Field 53B)
    pub fn with_senders_correspondent(mut self, correspondent: Field53B) -> Self {
        self.field_53b = Some(correspondent);
        self
    }

    /// Sets the intermediary institution (Field 56)
    pub fn with_intermediary(mut self, intermediary: Field56IntermediaryAD) -> Self {
        self.field_56 = Some(intermediary);
        self
    }

    /// Sets sender to receiver information (Field 72)
    pub fn with_sender_to_receiver_info(mut self, info: Field72) -> Self {
        self.field_72 = Some(info);
        self
    }
}

// Validation rules for MT200 using JSONLogic
// MT200 has no specific network validated rules according to SWIFT standards
pub const MT200_VALIDATION_RULES: &str = r#"{
    "rules": []
}"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt200_creation() {
        let reference = Field20 {
            reference: "REF123456".to_string(),
        };
        let amount = Field32A {
            value_date: Some(chrono::NaiveDate::from_ymd_opt(2024, 12, 25).unwrap()),
            currency: "USD".to_string(),
            amount: 1000000.00,
        };
        let account_with = Field57DebtInstitution::A(Field57A {
            party_identifier: None,
            bic: "BANKUSAA".to_string(),
        });

        let mt200 = MT200::new(reference, amount, account_with);

        assert_eq!(mt200.field_20.reference, "REF123456");
        assert_eq!(mt200.field_32a.currency, "USD");
        assert!(mt200.field_53b.is_none());
        assert!(mt200.field_56.is_none());
        assert!(mt200.field_72.is_none());
    }

    #[test]
    fn test_mt200_with_optional_fields() {
        let reference = Field20 {
            reference: "REF123456".to_string(),
        };
        let amount = Field32A {
            value_date: Some(chrono::NaiveDate::from_ymd_opt(2024, 12, 25).unwrap()),
            currency: "USD".to_string(),
            amount: 1000000.00,
        };
        let account_with = Field57DebtInstitution::A(Field57A {
            party_identifier: None,
            bic: "BANKUSAA".to_string(),
        });
        let intermediary = Field56IntermediaryAD::A(Field56A {
            party_identifier: None,
            bic: "INTMUSAA".to_string(),
        });

        let mt200 = MT200::new(reference, amount, account_with).with_intermediary(intermediary);

        assert!(mt200.field_56.is_some());
    }
}
