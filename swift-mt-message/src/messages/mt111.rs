use crate::fields::*;
use crate::{SwiftMessage, swift_serde};
use serde::{Deserialize, Serialize};

/// # MT111: Request for Stop Payment of a Cheque
///
/// ## Overview
/// MT111 is used by financial institutions to request the stop payment of a cheque
/// that has been previously issued. This message provides all necessary details
/// to identify the specific cheque and includes optional query information about
/// the reason for the stop payment request.
///
/// ## Structure
/// All fields are at the message level (no repeating sequences)
///
/// ## Key Features
/// - Stop payment request for specific cheque
/// - Must match original cheque details if MT110 was previously sent
/// - Optional query information with predefined codes
/// - Support for national clearing codes
/// - Payee identification without account numbers
/// - Validation against original MT110 if applicable
#[swift_serde]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[swift_message(mt = "111")]
pub struct MT111 {
    /// **Sender's Reference** - Field 20 (Mandatory)
    /// No '/' start/end, no '//'
    #[field("20")]
    pub field_20: Field20,

    /// **Cheque Number** - Field 21 (Mandatory)
    /// Must match original cheque if MT110 was sent
    #[field("21")]
    pub field_21: Field21,

    /// **Date of Issue** - Field 30 (Mandatory)
    /// Valid date format (YYMMDD)
    #[field("30")]
    pub field_30: Field30,

    /// **Amount** - Field 32a (Mandatory)
    /// Options: A (6!n3!a15d), B (3!a15d)
    /// Must match MT110 if already sent
    /// Use Option A if sender credited receiver in advance, otherwise Option B
    #[field("32A")]
    pub field_32a: GenericCurrencyAmountField,

    /// **Drawer Bank** - Field 52a (Optional)
    /// Options: A, B, D. Use national clearing codes if no BIC
    #[field("52A")]
    pub field_52a: Option<GenericBicField>,

    /// **Payee** - Field 59 (Optional)
    /// Account field not used - only name and address allowed
    /// Must not contain an account number
    #[field("59")]
    pub field_59: Option<Field59>,

    /// **Queries** - Field 75 (Optional)
    /// Format: 6*35x, optional format with codes
    /// Predefined codes: 3, 18, 19, 20, 21
    #[field("75")]
    pub field_75: Option<Field75>,
}

impl MT111 {
    /// Create a new MT111 with minimal required fields
    pub fn new(
        field_20: Field20,
        field_21: Field21,
        field_30: Field30,
        field_32a: GenericCurrencyAmountField,
    ) -> Self {
        Self {
            field_20,
            field_21,
            field_30,
            field_32a,
            field_52a: None,
            field_59: None,
            field_75: None,
        }
    }

    /// Get the sender's reference
    pub fn senders_reference(&self) -> &str {
        self.field_20.transaction_reference()
    }

    /// Get the cheque number
    pub fn cheque_number(&self) -> &str {
        self.field_21.related_reference()
    }

    /// Get the cheque amount
    pub fn amount(&self) -> f64 {
        self.field_32a.amount()
    }

    /// Get the cheque currency
    pub fn currency(&self) -> &str {
        self.field_32a.currency()
    }

    /// Check if drawer bank information is provided
    pub fn has_drawer_bank(&self) -> bool {
        self.field_52a.is_some()
    }

    /// Check if payee information is provided
    pub fn has_payee_info(&self) -> bool {
        self.field_59.is_some()
    }

    /// Check if query information is provided
    pub fn has_queries(&self) -> bool {
        self.field_75.is_some()
    }

    /// Get query information if available
    pub fn queries(&self) -> Option<&Field75> {
        self.field_75.as_ref()
    }

    /// Check if queries contain predefined codes
    pub fn has_predefined_query_codes(&self) -> bool {
        self.field_75
            .as_ref()
            .is_some_and(|f| f.has_predefined_codes())
    }

    /// Get all predefined query codes
    pub fn predefined_query_codes(&self) -> Vec<String> {
        self.field_75
            .as_ref()
            .map_or(Vec::new(), |f| f.predefined_codes())
    }

    /// Validate against original MT110 cheque details
    /// This would typically be called with the original MT110 data
    pub fn validate_against_mt110(
        &self,
        mt110_cheque_number: &str,
        mt110_issue_date: chrono::NaiveDate,
        mt110_amount: f64,
        mt110_currency: &str,
    ) -> bool {
        self.cheque_number() == mt110_cheque_number
            && self.field_30.naive_date() == mt110_issue_date
            && (self.amount() - mt110_amount).abs() < 0.01
            && self.currency() == mt110_currency
    }

    /// Check if this is likely a pre-credited scenario (should use Option A)
    pub fn is_pre_credited_scenario(&self) -> bool {
        // This would typically be determined by business logic
        // For now, we assume it's based on the presence of certain query codes
        self.predefined_query_codes().contains(&"3".to_string())
    }

    /// Get a summary description
    pub fn summary(&self) -> String {
        format!(
            "Stop payment request for cheque {} ({}), {} {} issued on {}",
            self.cheque_number(),
            self.senders_reference(),
            self.currency(),
            self.amount(),
            self.field_30.format_readable()
        )
    }

    /// Set drawer bank
    pub fn set_drawer_bank(&mut self, drawer_bank: GenericBicField) {
        self.field_52a = Some(drawer_bank);
    }

    /// Set payee information
    pub fn set_payee(&mut self, payee: Field59) {
        self.field_59 = Some(payee);
    }

    /// Set query information
    pub fn set_queries(&mut self, queries: Field75) {
        self.field_75 = Some(queries);
    }

    /// Get formatted description of the stop payment request
    pub fn description(&self) -> String {
        let mut desc = format!(
            "Stop payment request for cheque {} issued on {} for {} {}",
            self.cheque_number(),
            self.field_30.format_readable(),
            self.currency(),
            self.amount()
        );

        if self.has_payee_info() {
            desc.push_str(" to specified payee");
        }

        if self.has_queries() {
            desc.push_str(" with additional queries");
        }

        desc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftMessageBody;
    use crate::fields::MultiLineField;

    #[test]
    fn test_field75_creation() {
        let queries = vec![
            "3 STOP PAYMENT REQUEST".to_string(),
            "INSUFFICIENT FUNDS".to_string(),
        ];
        let field = Field75::new(queries.clone()).unwrap();
        assert_eq!(field.queries(), &queries);
        assert_eq!(field.line_count(), 2);
    }

    #[test]
    fn test_field75_predefined_codes() {
        let queries = vec![
            "3 STOP PAYMENT REQUEST".to_string(),
            "18 ACCOUNT CLOSED".to_string(),
            "NARRATIVE TEXT".to_string(),
        ];
        let field = Field75::new(queries).unwrap();
        assert!(field.has_predefined_codes());
        let codes = field.predefined_codes();
        assert_eq!(codes, vec!["18", "3"]);
    }

    #[test]
    fn test_field75_validation() {
        // Test too many lines
        let too_many = vec!["1".to_string(); 7];
        assert!(Field75::new(too_many).is_err());

        // Test line too long
        let too_long = vec!["A".repeat(36)];
        assert!(Field75::new(too_long).is_err());

        // Test empty
        assert!(Field75::new(vec![]).is_err());
    }

    #[test]
    fn test_mt111_message_type() {
        assert_eq!(MT111::message_type(), "111");
    }

    #[test]
    fn test_mt111_creation() {
        let field_20 = Field20::new("STOP001".to_string());
        let field_21 = Field21::new("CHQ123456".to_string());
        let field_30 = Field30::new("240315");
        let field_32a = GenericCurrencyAmountField::new("USD", 1500.00).unwrap();

        let mt111 = MT111::new(field_20, field_21, field_30, field_32a);

        assert_eq!(mt111.senders_reference(), "STOP001");
        assert_eq!(mt111.cheque_number(), "CHQ123456");
        assert_eq!(mt111.currency(), "USD");
        assert_eq!(mt111.amount(), 1500.00);
        assert!(!mt111.has_drawer_bank());
        assert!(!mt111.has_payee_info());
        assert!(!mt111.has_queries());
    }

    #[test]
    fn test_mt111_with_queries() {
        let field_20 = Field20::new("STOP001".to_string());
        let field_21 = Field21::new("CHQ123456".to_string());
        let field_30 = Field30::new("240315");
        let field_32a = GenericCurrencyAmountField::new("USD", 1500.00).unwrap();

        let mut mt111 = MT111::new(field_20, field_21, field_30, field_32a);

        // Add queries
        let queries = Field75::new(vec![
            "3 STOP PAYMENT REQUEST".to_string(),
            "ACCOUNT HOLDER DECEASED".to_string(),
        ])
        .unwrap();
        mt111.set_queries(queries);

        assert!(mt111.has_queries());
        assert!(mt111.has_predefined_query_codes());
        assert_eq!(mt111.predefined_query_codes(), vec!["3"]);
        assert!(mt111.is_pre_credited_scenario());
    }

    #[test]
    fn test_mt111_validation_against_mt110() {
        let field_20 = Field20::new("STOP001".to_string());
        let field_21 = Field21::new("CHQ123456".to_string());
        let field_30 = Field30::new("240315");
        let field_32a = GenericCurrencyAmountField::new("USD", 1500.00).unwrap();

        let mt111 = MT111::new(field_20, field_21, field_30, field_32a);

        // Test matching MT110 data
        let mt110_date = chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        assert!(mt111.validate_against_mt110("CHQ123456", mt110_date, 1500.00, "USD"));

        // Test non-matching data
        assert!(!mt111.validate_against_mt110("CHQ999999", mt110_date, 1500.00, "USD"));
        assert!(!mt111.validate_against_mt110("CHQ123456", mt110_date, 2000.00, "USD"));
        assert!(!mt111.validate_against_mt110("CHQ123456", mt110_date, 1500.00, "EUR"));
    }

    #[test]
    fn test_mt111_with_payee_and_drawer() {
        let field_20 = Field20::new("STOP001".to_string());
        let field_21 = Field21::new("CHQ123456".to_string());
        let field_30 = Field30::new("240315");
        let field_32a = GenericCurrencyAmountField::new("USD", 1500.00).unwrap();

        let mut mt111 = MT111::new(field_20, field_21, field_30, field_32a);

        // Add drawer bank
        let drawer_bank = GenericBicField::new(None, None, "CHASUS33").unwrap();
        mt111.set_drawer_bank(drawer_bank);
        assert!(mt111.has_drawer_bank());

        // Add payee (without account number as per spec)
        let payee = Field59::A(GenericBicField::new(None, None, "DEUTDEFF").unwrap());
        mt111.set_payee(payee);
        assert!(mt111.has_payee_info());

        let summary = mt111.summary();
        assert!(summary.contains("CHQ123456"));
        assert!(summary.contains("USD"));
        assert!(summary.contains("1500"));
        assert!(summary.contains("2024-03-15"));
    }

    #[test]
    fn test_mt111_description() {
        let field_20 = Field20::new("STOP001".to_string());
        let field_21 = Field21::new("CHQ123456".to_string());
        let field_30 = Field30::new("240315");
        let field_32a = GenericCurrencyAmountField::new("GBP", 750.50).unwrap();

        let mt111 = MT111::new(field_20, field_21, field_30, field_32a);

        let description = mt111.description();
        assert!(description.contains("Stop payment request"));
        assert!(description.contains("CHQ123456"));
        assert!(description.contains("2024-03-15"));
        assert!(description.contains("GBP"));
        assert!(description.contains("750.5"));
    }
}
