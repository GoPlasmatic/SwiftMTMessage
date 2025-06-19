use crate::fields::{
    Field13D, Field20, Field21, Field25, Field28, Field60F, Field62F, Field64, Field65, Field86,
    Field90C, Field90D,
};
use crate::{SwiftMessageBody, SwiftResult};
use serde::{Deserialize, Serialize};

/// # MT941: Balance Report
///
/// ## Overview
/// MT941 is used by financial institutions to send balance reports
/// containing summary balance information without detailed transaction lines.
/// This message provides a condensed view of account balances and totals.
///
/// ## Message Structure
/// - **Field 20**: Transaction Reference Number (Mandatory)
/// - **Field 21**: Related Reference (Optional)
/// - **Field 25**: Account Identification (Mandatory)
/// - **Field 28**: Statement Number/Sequence No. (Mandatory)
/// - **Field 13D**: Date/Time Indication (Optional)
/// - **Field 60F**: Opening Balance (Optional)
/// - **Field 90D**: Sum of Debit Entries (Optional)
/// - **Field 90C**: Sum of Credit Entries (Optional)
/// - **Field 62F**: Book Balance (Mandatory)
/// - **Field 64**: Closing Available Balance (Optional)
/// - **Field 65**: Forward Available Balance (Optional)
/// - **Field 86**: Info to Account Owner (Optional)
///
/// ## Business Rules
/// - Currencies across fields (60F, 62F, 64, 65) must be consistent
/// - Statement supports multi-sequence statements (e.g., 001/001)
/// - Balance fields use comma as decimal separator
/// - Field 86 must only appear if paired correctly with a logical transaction group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT941 {
    /// **Transaction Reference Number** - Field 20 (Mandatory)
    /// No '//' or slashes at ends
    pub field_20: Field20,

    /// **Related Reference** - Field 21 (Optional)
    /// Links to MT920 if present
    pub field_21: Option<Field21>,

    /// **Account Identification** - Field 25 (Mandatory)
    /// Includes BIC if needed
    pub field_25: Field25,

    /// **Statement Number/Sequence No.** - Field 28 (Mandatory)
    /// Sequence optional
    pub field_28: Field28,

    /// **Date/Time Indication** - Field 13D (Optional)
    /// Date + Time + UTC offset
    pub field_13d: Option<Field13D>,

    /// **Opening Balance** - Field 60F (Optional)
    /// Must match prior 62F
    pub field_60f: Option<Field60F>,

    /// **Sum of Debit Entries** - Field 90D (Optional)
    /// Currency validation
    pub field_90d: Option<Field90D>,

    /// **Sum of Credit Entries** - Field 90C (Optional)
    /// Currency validation
    pub field_90c: Option<Field90C>,

    /// **Book Balance** - Field 62F (Mandatory)
    /// Final balance
    pub field_62f: Field62F,

    /// **Closing Available Balance** - Field 64 (Optional)
    /// Same rules as 62F
    pub field_64: Option<Field64>,

    /// **Forward Available Balance** - Field 65 (Optional)
    /// Value-dated available balance
    pub field_65: Option<Field65>,

    /// **Info to Account Owner** - Field 86 (Optional)
    /// Narrative, ERI, EXCH codes allowed
    pub field_86: Option<Field86>,
}

impl MT941 {
    /// Create a new MT941 with required fields
    pub fn new(
        field_20: Field20,
        field_25: Field25,
        field_28: Field28,
        field_62f: Field62F,
    ) -> Self {
        Self {
            field_20,
            field_21: None,
            field_25,
            field_28,
            field_13d: None,
            field_60f: None,
            field_90d: None,
            field_90c: None,
            field_62f,
            field_64: None,
            field_65: None,
            field_86: None,
        }
    }

    /// Get the transaction reference
    pub fn transaction_reference(&self) -> &str {
        &self.field_20.transaction_reference
    }

    /// Get the account identification
    pub fn account_identification(&self) -> &str {
        &self.field_25.authorisation
    }

    /// Set related reference
    pub fn set_related_reference(&mut self, field_21: Field21) {
        self.field_21 = Some(field_21);
    }

    /// Set date/time indication
    pub fn set_date_time_indication(&mut self, field_13d: Field13D) {
        self.field_13d = Some(field_13d);
    }

    /// Set opening balance
    pub fn set_opening_balance(&mut self, field_60f: Field60F) {
        self.field_60f = Some(field_60f);
    }

    /// Set sum of debit entries
    pub fn set_sum_of_debits(&mut self, field_90d: Field90D) {
        self.field_90d = Some(field_90d);
    }

    /// Set sum of credit entries
    pub fn set_sum_of_credits(&mut self, field_90c: Field90C) {
        self.field_90c = Some(field_90c);
    }

    /// Set closing available balance
    pub fn set_closing_available_balance(&mut self, field_64: Field64) {
        self.field_64 = Some(field_64);
    }

    /// Set forward available balance
    pub fn set_forward_available_balance(&mut self, field_65: Field65) {
        self.field_65 = Some(field_65);
    }

    /// Set info to account owner
    pub fn set_info_to_account_owner(&mut self, field_86: Field86) {
        self.field_86 = Some(field_86);
    }

    /// Check if currencies are consistent across balance fields
    pub fn validate_currency_consistency(&self) -> bool {
        // This would need proper currency extraction from balance fields
        // For now, return true as a placeholder
        true
    }

    /// Check if opening balance is present
    pub fn has_opening_balance(&self) -> bool {
        self.field_60f.is_some()
    }

    /// Check if debit/credit summaries are present
    pub fn has_entry_summaries(&self) -> bool {
        self.field_90d.is_some() || self.field_90c.is_some()
    }

    /// Check if additional balance information is present
    pub fn has_additional_balances(&self) -> bool {
        self.field_64.is_some() || self.field_65.is_some()
    }

    /// Check if narrative information is present
    pub fn has_narrative_info(&self) -> bool {
        self.field_86.is_some()
    }
}

impl SwiftMessageBody for MT941 {
    fn message_type() -> &'static str {
        "941"
    }

    fn from_fields(_fields: std::collections::HashMap<String, Vec<String>>) -> SwiftResult<Self> {
        // For now, return a basic implementation error
        // This would need proper field parsing implementation
        Err(crate::errors::ParseError::InvalidFormat {
            message: "MT941 field parsing not yet implemented".to_string(),
        })
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        // Basic implementation - would need proper field serialization
        std::collections::HashMap::new()
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "25", "28", "62F"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["21", "13D", "60F", "90D", "90C", "64", "65", "86"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftField;

    #[test]
    fn test_mt941_message_type() {
        assert_eq!(MT941::message_type(), "941");
    }

    #[test]
    fn test_mt941_creation() {
        let field_20 = Field20::new("BAL240315001234".to_string());
        let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
        let field_28 = Field28::new(1, Some(1)).unwrap();
        let field_62f = Field62F::new('C', "240315", "EUR", 1500.00).unwrap();

        let mt941 = MT941::new(field_20, field_25, field_28, field_62f);

        assert_eq!(mt941.transaction_reference(), "BAL240315001234");
        assert_eq!(mt941.account_identification(), "GB33BUKB20201555555555");
        assert!(!mt941.has_opening_balance());
        assert!(!mt941.has_entry_summaries());
        assert!(!mt941.has_additional_balances());
        assert!(!mt941.has_narrative_info());
    }

    #[test]
    fn test_mt941_optional_fields() {
        let field_20 = Field20::new("BAL240315001235".to_string());
        let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
        let field_28 = Field28::new(1, Some(1)).unwrap();
        let field_62f = Field62F::new('C', "240315", "EUR", 1500.00).unwrap();

        let mut mt941 = MT941::new(field_20, field_25, field_28, field_62f);

        // Add optional fields
        let field_60f = Field60F::new('C', "240314", "EUR", 1000.00).unwrap();
        mt941.set_opening_balance(field_60f);

        let field_90d = Field90D::parse("005EUR200,00").unwrap();
        mt941.set_sum_of_debits(field_90d);

        let field_90c = Field90C::parse("010EUR700,00").unwrap();
        mt941.set_sum_of_credits(field_90c);

        assert!(mt941.has_opening_balance());
        assert!(mt941.has_entry_summaries());
    }

    #[test]
    fn test_mt941_required_fields() {
        let required = MT941::required_fields();
        assert!(required.contains(&"20"));
        assert!(required.contains(&"25"));
        assert!(required.contains(&"28"));
        assert!(required.contains(&"62F"));
    }

    #[test]
    fn test_mt941_optional_fields_list() {
        let optional = MT941::optional_fields();
        assert!(optional.contains(&"21"));
        assert!(optional.contains(&"13D"));
        assert!(optional.contains(&"60F"));
        assert!(optional.contains(&"90D"));
        assert!(optional.contains(&"90C"));
        assert!(optional.contains(&"64"));
        assert!(optional.contains(&"65"));
        assert!(optional.contains(&"86"));
    }
}
