use crate::SwiftMessageBody;
use crate::fields::{
    Field20, Field21, Field30, Field50, Field59, Field72, GenericBicField,
    GenericCurrencyAmountField,
};
use serde::{Deserialize, Serialize};

/// # MT110 Cheque Details (Repeating Sequence)
///
/// Represents a single cheque within an MT110 batch.
/// This sequence can repeat up to 10 times per message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT110ChequeDetails {
    /// **Cheque Number** - Field 21 (Mandatory)
    /// Unique per cheque; no '/' or '//'
    pub field_21: Field21,

    /// **Date of Issue** - Field 30 (Mandatory)
    /// Format: YYMMDD. Must be a valid date
    pub field_30: Field30,

    /// **Amount** - Field 32a (Mandatory)
    /// Options: A (6!n3!a15d) / B (3!a15d)
    /// Currency must be same for all cheques in the message
    pub field_32a: GenericCurrencyAmountField,

    /// **Payer** - Field 50a (Optional)
    /// Options: A, F, K. Detailed identity formats
    pub field_50a: Option<Field50>,

    /// **Drawer Bank** - Field 52a (Optional)
    /// Options: A, B, D. Can specify BIC or national code
    pub field_52a: Option<GenericBicField>,

    /// **Payee** - Field 59a (Mandatory)
    /// Options: No letter, F option. Must use structured address and name
    pub field_59a: Field59,
}

/// # MT110: Advice of Cheque
///
/// ## Overview
/// MT110 is used by financial institutions to advise the receipt or dispatch
/// of cheques. It provides detailed information about individual cheques including
/// payer, payee, amounts, and banking details. The message supports batch processing
/// of up to 10 cheques with consistent currency requirements.
///
/// ## Structure
/// - **Header Fields**: General information and correspondent details
/// - **Repeating Sequence**: Individual cheque details (up to 10 occurrences)
///
/// ## Key Features
/// - Multiple cheque processing in single message (up to 10)
/// - Consistent currency requirement across all cheques
/// - Flexible correspondent bank routing
/// - Detailed payer/payee identification
/// - Support for national clearing codes
/// - Optional structured sender-to-receiver information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT110 {
    // ================================
    // HEADER FIELDS
    // ================================
    /// **Sender's Reference** - Field 20 (Mandatory)
    /// No leading/trailing slash, no '//'
    pub field_20: Field20,

    /// **Sender's Correspondent** - Field 53a (Optional)
    /// Options: A, B, D. Required if no direct account relationship
    pub field_53a: Option<GenericBicField>,

    /// **Receiver's Correspondent** - Field 54a (Optional)
    /// Options: A, B, D. Used to route funds to Receiver
    pub field_54a: Option<GenericBicField>,

    /// **Sender to Receiver Information** - Field 72 (Optional)
    /// Format: 6*35x, optional structured codes
    /// Codes: ACC, INS, INT; REJT/RETN special rules
    pub field_72: Option<Field72>,

    // ================================
    // REPEATING SEQUENCE - CHEQUE DETAILS (UP TO 10)
    // ================================
    /// **Cheque Details** - Repeating Sequence (Mandatory, up to 10 occurrences)
    /// Each entry represents one cheque being advised
    pub cheques: Vec<MT110ChequeDetails>,
}

impl MT110ChequeDetails {
    /// Create a new cheque details with required fields
    pub fn new(
        field_21: Field21,
        field_30: Field30,
        field_32a: GenericCurrencyAmountField,
        field_59a: Field59,
    ) -> Self {
        Self {
            field_21,
            field_30,
            field_32a,
            field_50a: None,
            field_52a: None,
            field_59a,
        }
    }

    /// Get the cheque number
    pub fn cheque_number(&self) -> &str {
        self.field_21.related_reference()
    }

    /// Get the issue date
    pub fn issue_date(&self) -> chrono::NaiveDate {
        self.field_30.execution_date()
    }

    /// Get the cheque amount
    pub fn amount(&self) -> f64 {
        self.field_32a.amount()
    }

    /// Get the cheque currency
    pub fn currency(&self) -> &str {
        self.field_32a.currency()
    }

    /// Check if cheque has payer information
    pub fn has_payer_details(&self) -> bool {
        self.field_50a.is_some()
    }

    /// Check if cheque has drawer bank information
    pub fn has_drawer_bank(&self) -> bool {
        self.field_52a.is_some()
    }

    /// Get a description of the cheque
    pub fn description(&self) -> String {
        format!(
            "Cheque {} for {} {} issued on {}",
            self.cheque_number(),
            self.currency(),
            self.amount(),
            self.issue_date().format("%Y-%m-%d")
        )
    }
}

impl MT110 {
    /// Create a new MT110 with minimal required fields
    pub fn new(field_20: Field20, cheques: Vec<MT110ChequeDetails>) -> Self {
        Self {
            field_20,
            field_53a: None,
            field_54a: None,
            field_72: None,
            cheques,
        }
    }

    /// Create a new MT110 with a single cheque
    pub fn new_single_cheque(
        field_20: Field20,
        field_21: Field21,
        field_30: Field30,
        field_32a: GenericCurrencyAmountField,
        field_59a: Field59,
    ) -> Self {
        let cheque = MT110ChequeDetails::new(field_21, field_30, field_32a, field_59a);
        Self::new(field_20, vec![cheque])
    }

    /// Get the sender's reference
    pub fn senders_reference(&self) -> &str {
        self.field_20.transaction_reference()
    }

    /// Get the number of cheques in this advice
    pub fn cheque_count(&self) -> usize {
        self.cheques.len()
    }

    /// Get the total amount across all cheques
    pub fn total_amount(&self) -> f64 {
        self.cheques.iter().map(|c| c.amount()).sum()
    }

    /// Get the currency (should be consistent across all cheques)
    pub fn currency(&self) -> Option<&str> {
        self.cheques.first().map(|c| c.currency())
    }

    /// Get all unique issue dates
    pub fn issue_dates(&self) -> Vec<chrono::NaiveDate> {
        let mut dates: Vec<chrono::NaiveDate> =
            self.cheques.iter().map(|c| c.issue_date()).collect();
        dates.sort_unstable();
        dates.dedup();
        dates
    }

    /// Get the earliest issue date
    pub fn earliest_issue_date(&self) -> Option<chrono::NaiveDate> {
        self.cheques.iter().map(|c| c.issue_date()).min()
    }

    /// Get the latest issue date
    pub fn latest_issue_date(&self) -> Option<chrono::NaiveDate> {
        self.cheques.iter().map(|c| c.issue_date()).max()
    }

    /// Check how many cheques have payer details
    pub fn cheques_with_payer_count(&self) -> usize {
        self.cheques
            .iter()
            .filter(|c| c.has_payer_details())
            .count()
    }

    /// Check how many cheques have drawer bank details
    pub fn cheques_with_drawer_bank_count(&self) -> usize {
        self.cheques.iter().filter(|c| c.has_drawer_bank()).count()
    }

    /// Validate C1: Maximum 10 cheques per message
    pub fn validate_c1(&self) -> bool {
        self.cheques.len() <= 10
    }

    /// Validate C2: All cheques must have the same currency
    pub fn validate_c2(&self) -> bool {
        if self.cheques.is_empty() {
            return true;
        }

        let first_currency = self.cheques[0].currency();
        self.cheques.iter().all(|c| c.currency() == first_currency)
    }

    /// Validate all conditional rules
    pub fn validate_conditional_rules(&self) -> bool {
        self.validate_c1() && self.validate_c2()
    }

    /// Check if message has correspondent banking information
    pub fn has_correspondent_info(&self) -> bool {
        self.field_53a.is_some() || self.field_54a.is_some()
    }

    /// Check if message has sender-to-receiver information
    pub fn has_sender_info(&self) -> bool {
        self.field_72.is_some()
    }

    /// Add a cheque to the advice
    pub fn add_cheque(&mut self, cheque: MT110ChequeDetails) -> Result<(), String> {
        if self.cheques.len() >= 10 {
            return Err("Cannot add more than 10 cheques per MT110 message".to_string());
        }

        // Validate currency consistency
        if let Some(existing_currency) = self.currency() {
            if cheque.currency() != existing_currency {
                return Err(format!(
                    "Currency mismatch: expected {}, got {}",
                    existing_currency,
                    cheque.currency()
                ));
            }
        }

        self.cheques.push(cheque);
        Ok(())
    }

    /// Get cheques sorted by issue date
    pub fn cheques_by_date(&self) -> Vec<&MT110ChequeDetails> {
        let mut cheques: Vec<&MT110ChequeDetails> = self.cheques.iter().collect();
        cheques.sort_by_key(|c| c.issue_date());
        cheques
    }

    /// Get cheques sorted by amount (descending)
    pub fn cheques_by_amount(&self) -> Vec<&MT110ChequeDetails> {
        let mut cheques: Vec<&MT110ChequeDetails> = self.cheques.iter().collect();
        cheques.sort_by(|a, b| {
            b.amount()
                .partial_cmp(&a.amount())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        cheques
    }

    /// Get summary statistics
    pub fn summary(&self) -> String {
        format!(
            "MT110 Advice: {} cheques, {} {} total, dates {} to {}",
            self.cheque_count(),
            self.currency().unwrap_or("N/A"),
            self.total_amount(),
            self.earliest_issue_date()
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or("N/A".to_string()),
            self.latest_issue_date()
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or("N/A".to_string())
        )
    }

    /// Check if this is likely a reject/return advice
    pub fn is_reject_return_advice(&self) -> bool {
        if let Some(field_72) = &self.field_72 {
            let info_text = field_72.to_string().to_uppercase();
            info_text.contains("REJT") || info_text.contains("RETN")
        } else {
            false
        }
    }

    /// Set sender's correspondent
    pub fn set_senders_correspondent(&mut self, correspondent: GenericBicField) {
        self.field_53a = Some(correspondent);
    }

    /// Set receiver's correspondent
    pub fn set_receivers_correspondent(&mut self, correspondent: GenericBicField) {
        self.field_54a = Some(correspondent);
    }

    /// Set sender to receiver information
    pub fn set_sender_info(&mut self, info: Field72) {
        self.field_72 = Some(info);
    }
}

impl SwiftMessageBody for MT110 {
    fn message_type() -> &'static str {
        "110"
    }

    fn from_fields(
        _fields: std::collections::HashMap<String, Vec<String>>,
    ) -> crate::SwiftResult<Self> {
        // Placeholder implementation
        todo!("MT110 field parsing not yet implemented")
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        // Placeholder implementation
        todo!("MT110 field serialization not yet implemented")
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "21", "30", "32A", "59"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec![
            "53A", "53B", "53D", "54A", "54B", "54D", "72", "50A", "50F", "50K", "52A", "52B",
            "52D", "59F",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt110_message_type() {
        assert_eq!(MT110::message_type(), "110");
    }

    #[test]
    fn test_mt110_creation() {
        let field_20 = Field20::new("CHQ001".to_string());
        let field_21 = Field21::new("CHQ123456".to_string());
        let field_30 = Field30::new("240315");
        let field_32a = GenericCurrencyAmountField::new("USD", 1500.00).unwrap();
        let field_59a = Field59::A(
            GenericBicField::new(None, Some("12345678".to_string()), "CHASUS33").unwrap(),
        );

        let mt110 = MT110::new_single_cheque(field_20, field_21, field_30, field_32a, field_59a);

        assert_eq!(mt110.senders_reference(), "CHQ001");
        assert_eq!(mt110.cheque_count(), 1);
        assert_eq!(mt110.cheques[0].cheque_number(), "CHQ123456");
        assert_eq!(mt110.cheques[0].currency(), "USD");
        assert_eq!(mt110.cheques[0].amount(), 1500.00);
        assert_eq!(mt110.total_amount(), 1500.00);
        assert_eq!(mt110.currency(), Some("USD"));
    }

    #[test]
    fn test_mt110_multiple_cheques() {
        let field_20 = Field20::new("CHQ001".to_string());

        // Create multiple cheques
        let cheque1 = MT110ChequeDetails::new(
            Field21::new("CHQ001001".to_string()),
            Field30::new("240315"),
            GenericCurrencyAmountField::new("EUR", 750.00).unwrap(),
            Field59::A(
                GenericBicField::new(None, Some("11111111".to_string()), "DEUTDEFF").unwrap(),
            ),
        );

        let cheque2 = MT110ChequeDetails::new(
            Field21::new("CHQ001002".to_string()),
            Field30::new("240316"),
            GenericCurrencyAmountField::new("EUR", 1250.00).unwrap(),
            Field59::A(
                GenericBicField::new(None, Some("22222222".to_string()), "DEUTDEFF").unwrap(),
            ),
        );

        let cheque3 = MT110ChequeDetails::new(
            Field21::new("CHQ001003".to_string()),
            Field30::new("240314"),
            GenericCurrencyAmountField::new("EUR", 500.00).unwrap(),
            Field59::A(
                GenericBicField::new(None, Some("33333333".to_string()), "DEUTDEFF").unwrap(),
            ),
        );

        let cheques = vec![cheque1, cheque2, cheque3];
        let mt110 = MT110::new(field_20, cheques);

        assert_eq!(mt110.senders_reference(), "CHQ001");
        assert_eq!(mt110.cheque_count(), 3);
        assert_eq!(mt110.total_amount(), 2500.00); // 750 + 1250 + 500
        assert_eq!(mt110.currency(), Some("EUR"));

        // Check individual cheques
        assert_eq!(mt110.cheques[0].cheque_number(), "CHQ001001");
        assert_eq!(mt110.cheques[1].amount(), 1250.00);
        assert_eq!(mt110.cheques[2].currency(), "EUR");

        // Check date range
        assert_eq!(
            mt110
                .earliest_issue_date()
                .unwrap()
                .format("%y%m%d")
                .to_string(),
            "240314"
        );
        assert_eq!(
            mt110
                .latest_issue_date()
                .unwrap()
                .format("%y%m%d")
                .to_string(),
            "240316"
        );

        // Check sorting
        let by_date = mt110.cheques_by_date();
        assert_eq!(by_date[0].cheque_number(), "CHQ001003"); // 240314 - earliest
        assert_eq!(by_date[2].cheque_number(), "CHQ001002"); // 240316 - latest

        let by_amount = mt110.cheques_by_amount();
        assert_eq!(by_amount[0].amount(), 1250.00); // Highest amount first
        assert_eq!(by_amount[2].amount(), 500.00); // Lowest amount last
    }

    #[test]
    fn test_mt110_conditional_rules() {
        let field_20 = Field20::new("CHQ001".to_string());

        // Test C1: Maximum 10 cheques
        let mut mt110 = MT110::new(field_20.clone(), vec![]);
        assert!(mt110.validate_c1());

        // Add exactly 10 cheques
        for i in 1..=10 {
            let cheque = MT110ChequeDetails::new(
                Field21::new(format!("CHQ{:06}", i)),
                Field30::new("240315"),
                GenericCurrencyAmountField::new("USD", 100.00).unwrap(),
                Field59::A(
                    GenericBicField::new(None, Some("12345678".to_string()), "CHASUS33").unwrap(),
                ),
            );
            mt110.add_cheque(cheque).unwrap();
        }
        assert_eq!(mt110.cheque_count(), 10);
        assert!(mt110.validate_c1());

        // Try to add 11th cheque - should fail
        let extra_cheque = MT110ChequeDetails::new(
            Field21::new("CHQ000011".to_string()),
            Field30::new("240315"),
            GenericCurrencyAmountField::new("USD", 100.00).unwrap(),
            Field59::A(
                GenericBicField::new(None, Some("12345678".to_string()), "CHASUS33").unwrap(),
            ),
        );
        let result = mt110.add_cheque(extra_cheque);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Cannot add more than 10 cheques")
        );
    }

    #[test]
    fn test_mt110_currency_consistency() {
        let field_20 = Field20::new("CHQ001".to_string());
        let mut mt110 = MT110::new(field_20, vec![]);

        // Add first cheque in EUR
        let cheque1 = MT110ChequeDetails::new(
            Field21::new("CHQ001".to_string()),
            Field30::new("240315"),
            GenericCurrencyAmountField::new("EUR", 750.00).unwrap(),
            Field59::A(
                GenericBicField::new(None, Some("11111111".to_string()), "DEUTDEFF").unwrap(),
            ),
        );
        mt110.add_cheque(cheque1).unwrap();
        assert!(mt110.validate_c2());

        // Try to add cheque in different currency - should fail
        let cheque2 = MT110ChequeDetails::new(
            Field21::new("CHQ002".to_string()),
            Field30::new("240315"),
            GenericCurrencyAmountField::new("USD", 1000.00).unwrap(),
            Field59::A(
                GenericBicField::new(None, Some("22222222".to_string()), "CHASUS33").unwrap(),
            ),
        );
        let result = mt110.add_cheque(cheque2);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Currency mismatch"));

        // Add another EUR cheque - should succeed
        let cheque3 = MT110ChequeDetails::new(
            Field21::new("CHQ003".to_string()),
            Field30::new("240315"),
            GenericCurrencyAmountField::new("EUR", 500.00).unwrap(),
            Field59::A(
                GenericBicField::new(None, Some("33333333".to_string()), "DEUTDEFF").unwrap(),
            ),
        );
        mt110.add_cheque(cheque3).unwrap();
        assert!(mt110.validate_c2());
        assert_eq!(mt110.cheque_count(), 2);
    }

    #[test]
    fn test_mt110_correspondent_and_info() {
        let field_20 = Field20::new("CHQ001".to_string());
        let field_21 = Field21::new("CHQ123456".to_string());
        let field_30 = Field30::new("240315");
        let field_32a = GenericCurrencyAmountField::new("USD", 1500.00).unwrap();
        let field_59a = Field59::A(
            GenericBicField::new(None, Some("12345678".to_string()), "CHASUS33").unwrap(),
        );

        let mut mt110 =
            MT110::new_single_cheque(field_20, field_21, field_30, field_32a, field_59a);

        assert!(!mt110.has_correspondent_info());
        assert!(!mt110.has_sender_info());
        assert!(!mt110.is_reject_return_advice());

        // Add correspondent information
        let sender_correspondent = GenericBicField::new(None, None, "DEUTDEFF").unwrap();
        mt110.set_senders_correspondent(sender_correspondent);
        assert!(mt110.has_correspondent_info());

        // Add sender-to-receiver information with REJT
        let sender_info = Field72::new(vec![
            "/INS/REJT".to_string(),
            "/REASON/INSUFFICIENT FUNDS".to_string(),
        ])
        .unwrap();
        mt110.set_sender_info(sender_info);
        assert!(mt110.has_sender_info());
        assert!(mt110.is_reject_return_advice());
    }

    #[test]
    fn test_mt110_cheque_details() {
        let cheque = MT110ChequeDetails::new(
            Field21::new("CHQ987654".to_string()),
            Field30::new("240320"),
            GenericCurrencyAmountField::new("GBP", 2750.50).unwrap(),
            Field59::A(
                GenericBicField::new(None, Some("87654321".to_string()), "BARCGB22").unwrap(),
            ),
        );

        assert_eq!(cheque.cheque_number(), "CHQ987654");
        assert_eq!(cheque.currency(), "GBP");
        assert_eq!(cheque.amount(), 2750.50);
        assert_eq!(
            cheque.issue_date().format("%Y-%m-%d").to_string(),
            "2024-03-20"
        );
        assert!(!cheque.has_payer_details());
        assert!(!cheque.has_drawer_bank());

        let description = cheque.description();
        assert!(description.contains("CHQ987654"));
        assert!(description.contains("GBP"));
        assert!(description.contains("2750.5"));
        assert!(description.contains("2024-03-20"));
    }

    #[test]
    fn test_mt110_summary() {
        let field_20 = Field20::new("CHQ001".to_string());
        let cheque1 = MT110ChequeDetails::new(
            Field21::new("CHQ001".to_string()),
            Field30::new("240315"),
            GenericCurrencyAmountField::new("EUR", 1000.00).unwrap(),
            Field59::A(
                GenericBicField::new(None, Some("11111111".to_string()), "DEUTDEFF").unwrap(),
            ),
        );
        let cheque2 = MT110ChequeDetails::new(
            Field21::new("CHQ002".to_string()),
            Field30::new("240318"),
            GenericCurrencyAmountField::new("EUR", 500.00).unwrap(),
            Field59::A(
                GenericBicField::new(None, Some("22222222".to_string()), "DEUTDEFF").unwrap(),
            ),
        );

        let mt110 = MT110::new(field_20, vec![cheque1, cheque2]);
        let summary = mt110.summary();

        assert!(summary.contains("2 cheques"));
        assert!(summary.contains("EUR"));
        assert!(summary.contains("1500")); // Total amount
        assert!(summary.contains("2024-03-15")); // Earliest date
        assert!(summary.contains("2024-03-18")); // Latest date
    }
}
