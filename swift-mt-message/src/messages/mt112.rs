use crate::fields::*;
use crate::{SwiftMessage, swift_serde};
use serde::{Deserialize, Serialize};

/// # MT112: Status of Request for Stop Payment of a Cheque
///
/// ## Overview
/// MT112 is used by financial institutions to communicate the status of a stop payment
/// request that was previously submitted via MT111. This message provides confirmation,
/// rejection, or status updates regarding the processing of the stop payment request.
///
/// ## Structure
/// All fields are at the message level (no repeating sequences)
///
/// ## Key Features
/// - Status response to MT111 stop payment requests
/// - References original stop payment request details
/// - Provides detailed status information and reasons
/// - Support for partial processing scenarios
/// - Optional additional correspondence information
/// - Maintains audit trail for stop payment lifecycle
#[swift_serde]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[swift_message(mt = "112")]
pub struct MT112 {
    /// **Transaction Reference Number** - Field 20 (Mandatory)
    /// Reference for this status message
    #[field("20")]
    pub field_20: Field20,

    /// **Cheque Number** - Field 21 (Mandatory)
    /// Must match cheque issued in MT110/111
    #[field("21")]
    pub field_21: Field21,

    /// **Date of Issue** - Field 30 (Mandatory)
    /// Must be a valid date format (YYMMDD)
    #[field("30")]
    pub field_30: Field30,

    /// **Amount** - Field 32a (Mandatory)
    /// Same currency across responses; Option A for credited cases
    #[field("32A")]
    pub field_32a: GenericCurrencyAmountField,

    /// **Drawer Bank** - Field 52a (Optional)
    /// National clearing codes supported
    #[field("52A")]
    pub field_52a: Option<GenericBicField>,

    /// **Payee** - Field 59 (Optional)
    /// Account field must not be used
    #[field("59")]
    pub field_59: Option<Field59>,

    /// **Answers** - Field 76 (Mandatory)
    /// Predefined codes with supplemental text allowed
    #[field("76")]
    pub field_76: Field76,
}

impl MT112 {
    /// Create a new MT112 with required fields
    pub fn new(
        field_20: Field20,
        field_21: Field21,
        field_30: Field30,
        field_32a: GenericCurrencyAmountField,
        field_76: Field76,
    ) -> Self {
        Self {
            field_20,
            field_21,
            field_30,
            field_32a,
            field_52a: None,
            field_59: None,
            field_76,
        }
    }

    /// Get the transaction reference
    pub fn transaction_reference(&self) -> &str {
        self.field_20.transaction_reference()
    }

    /// Get the cheque number
    pub fn cheque_number(&self) -> &str {
        self.field_21.related_reference()
    }

    /// Get the issue date
    pub fn issue_date(&self) -> chrono::NaiveDate {
        self.field_30.execution_date()
    }

    /// Get the amount
    pub fn amount(&self) -> f64 {
        self.field_32a.amount()
    }

    /// Get the currency
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

    /// Get status information
    pub fn status_info(&self) -> &Field76 {
        &self.field_76
    }

    /// Check if the stop payment request was successful
    pub fn is_successful(&self) -> bool {
        self.field_76.is_successful()
    }

    /// Check if the stop payment request was rejected
    pub fn is_rejected(&self) -> bool {
        self.field_76.is_rejected()
    }

    /// Check if the stop payment request is still pending
    pub fn is_pending(&self) -> bool {
        self.field_76.is_pending()
    }

    /// Get the overall status as a string
    pub fn status(&self) -> &str {
        if self.is_successful() {
            "Successful"
        } else if self.is_rejected() {
            "Rejected"
        } else if self.is_pending() {
            "Pending"
        } else {
            "Unknown"
        }
    }

    /// Get a summary description
    pub fn summary(&self) -> String {
        let mut summary = format!(
            "Stop payment status ({}) for reference {}",
            self.status(),
            self.cheque_number()
        );

        summary.push_str(&format!(" - {} {}", self.currency(), self.amount()));
        summary.push_str(&format!(
            " issued on {}",
            self.issue_date().format("%Y-%m-%d")
        ));

        summary
    }

    /// Set drawer bank
    pub fn set_drawer_bank(&mut self, drawer_bank: GenericBicField) {
        self.field_52a = Some(drawer_bank);
    }

    /// Set payee information
    pub fn set_payee(&mut self, payee: Field59) {
        self.field_59 = Some(payee);
    }

    /// Get formatted description of the status response
    pub fn description(&self) -> String {
        let mut desc = format!(
            "Stop payment status response: {} for reference {}",
            self.status(),
            self.cheque_number()
        );

        desc.push_str(&format!(
            " (cheque dated {})",
            self.issue_date().format("%Y-%m-%d")
        ));

        desc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftMessageBody;
    use crate::fields::MultiLineField;

    #[test]
    fn test_field76_creation() {
        let status_info = vec![
            "STOP PAYMENT ACCEPTED".to_string(),
            "PROCESSED SUCCESSFULLY".to_string(),
        ];
        let field = Field76::new(status_info.clone()).unwrap();
        assert_eq!(field.status_info(), &status_info);
        assert_eq!(field.line_count(), 2);
    }

    #[test]
    fn test_field76_status_detection() {
        // Test successful status
        let success_field = Field76::new(vec!["STOP PAYMENT ACCEPTED".to_string()]).unwrap();
        assert!(success_field.is_successful());
        assert!(!success_field.is_rejected());
        assert!(!success_field.is_pending());

        // Test rejected status
        let reject_field = Field76::new(vec!["REQUEST REJECTED".to_string()]).unwrap();
        assert!(!reject_field.is_successful());
        assert!(reject_field.is_rejected());
        assert!(!reject_field.is_pending());

        // Test pending status
        let pending_field = Field76::new(vec!["PROCESSING PENDING".to_string()]).unwrap();
        assert!(!pending_field.is_successful());
        assert!(!pending_field.is_rejected());
        assert!(pending_field.is_pending());
    }

    #[test]
    fn test_field76_validation() {
        // Test too many lines
        let too_many = vec!["1".to_string(); 7];
        assert!(Field76::new(too_many).is_err());

        // Test line too long
        let too_long = vec!["A".repeat(36)];
        assert!(Field76::new(too_long).is_err());

        // Test empty
        assert!(Field76::new(vec![]).is_err());
    }

    #[test]
    fn test_mt112_message_type() {
        assert_eq!(MT112::message_type(), "112");
    }

    #[test]
    fn test_mt112_creation() {
        let field_20 = Field20::new("STATUS001".to_string());
        let field_21 = Field21::new("CHQ001".to_string());
        let field_30 = Field30::new("240315");
        let field_32a = GenericCurrencyAmountField::new("USD", 1500.00).unwrap();
        let field_76 = Field76::new(vec!["STOP PAYMENT ACCEPTED".to_string()]).unwrap();

        let mt112 = MT112::new(field_20, field_21, field_30, field_32a, field_76);

        assert_eq!(mt112.transaction_reference(), "STATUS001");
        assert_eq!(mt112.cheque_number(), "CHQ001");
        assert_eq!(mt112.amount(), 1500.00);
        assert_eq!(mt112.currency(), "USD");
        assert!(mt112.is_successful());
    }

    #[test]
    fn test_mt112_summary() {
        let field_20 = Field20::new("STATUS001".to_string());
        let field_21 = Field21::new("CHQ001".to_string());
        let field_76 = Field76::new(vec!["STOP PAYMENT ACCEPTED".to_string()]).unwrap();

        let mt112 = MT112::new(
            field_20,
            field_21,
            Field30::new("240315"),
            GenericCurrencyAmountField::new("USD", 1500.00).unwrap(),
            field_76,
        );

        let summary = mt112.summary();
        assert!(summary.contains("CHQ001"));
        assert!(summary.contains("USD"));
        assert!(summary.contains("1500"));
        assert!(summary.contains("2024-03-15"));
    }

    #[test]
    fn test_mt112_status_variations() {
        let field_20 = Field20::new("STATUS001".to_string());
        let field_21 = Field21::new("CHQ001".to_string());

        // Test rejected status
        let rejected_field = Field76::new(vec!["REQUEST REJECTED".to_string()]).unwrap();
        let rejected_mt112 = MT112::new(
            field_20.clone(),
            field_21.clone(),
            Field30::new("240315"),
            GenericCurrencyAmountField::new("USD", 1500.00).unwrap(),
            rejected_field,
        );
        assert!(rejected_mt112.is_rejected());
        assert_eq!(rejected_mt112.status(), "Rejected");

        // Test pending status
        let pending_field = Field76::new(vec!["PROCESSING PENDING".to_string()]).unwrap();
        let pending_mt112 = MT112::new(
            field_20.clone(),
            field_21.clone(),
            Field30::new("240315"),
            GenericCurrencyAmountField::new("USD", 1500.00).unwrap(),
            pending_field,
        );
        assert!(pending_mt112.is_pending());
        assert_eq!(pending_mt112.status(), "Pending");

        // Test unknown status
        let unknown_field = Field76::new(vec!["SOME OTHER STATUS".to_string()]).unwrap();
        let unknown_mt112 = MT112::new(
            field_20,
            field_21,
            Field30::new("240315"),
            GenericCurrencyAmountField::new("USD", 1500.00).unwrap(),
            unknown_field,
        );
        assert_eq!(unknown_mt112.status(), "Unknown");
    }

    #[test]
    fn test_mt112_with_payee() {
        let field_20 = Field20::new("STATUS001".to_string());
        let field_21 = Field21::new("CHQ001".to_string());
        let field_76 = Field76::new(vec!["STOP PAYMENT ACCEPTED".to_string()]).unwrap();

        let mut mt112 = MT112::new(
            field_20,
            field_21,
            Field30::new("240315"),
            GenericCurrencyAmountField::new("USD", 1500.00).unwrap(),
            field_76,
        );

        // Add payee information
        let payee = Field59::NoOption(
            crate::fields::field59::Field59Basic::new(vec![
                "JOHN DOE".to_string(),
                "123 MAIN STREET".to_string(),
            ])
            .unwrap(),
        );
        mt112.set_payee(payee);

        assert!(mt112.has_payee_info());

        let description = mt112.description();
        assert!(description.contains("Stop payment status response"));
        assert!(description.contains("Successful"));
        assert!(description.contains("CHQ001"));
    }
}
