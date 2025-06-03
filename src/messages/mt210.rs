//! MT210: Notice to Receive

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::common::{Amount, Field, MessageBlock, SwiftDate, tags};
use crate::error::{MTError, Result};
use crate::messages::{
    MTMessageType, extract_text_block, find_field, find_fields, get_optional_field_value,
    get_required_field_value,
};

/// MT210: Notice to Receive
/// This message is used as a pre-notification of an incoming funds transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT210 {
    /// All fields from the text block
    fields: Vec<Field>,
}

impl MT210 {
    /// Get transaction reference number (Field 20)
    pub fn transaction_reference(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::SENDER_REFERENCE)
    }

    /// Get related reference (Field 21) - Related reference to original message
    pub fn related_reference(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "21")
    }

    /// Get value date, currency and amount (Field 32A)
    pub fn value_date_currency_amount(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::VALUE_DATE_CURRENCY_AMOUNT)
    }

    /// Get parsed amount from field 32A
    pub fn amount(&self) -> Result<Amount> {
        let field_32a = get_required_field_value(&self.fields, tags::VALUE_DATE_CURRENCY_AMOUNT)?;

        // Format: YYMMDDCCCNNNNN,NN (date + currency + amount)
        if field_32a.len() < 9 {
            return Err(MTError::InvalidFieldFormat {
                field: "32A".to_string(),
                message: "Field 32A too short".to_string(),
            });
        }

        // Skip the date part (first 6 characters) and parse the currency+amount
        let currency_amount = &field_32a[6..];
        Amount::parse(currency_amount)
    }

    /// Get currency from field 32A
    pub fn currency(&self) -> Result<String> {
        let amount = self.amount()?;
        Ok(amount.currency)
    }

    /// Get value date from field 32A
    pub fn value_date(&self) -> Result<NaiveDate> {
        let field_32a = get_required_field_value(&self.fields, tags::VALUE_DATE_CURRENCY_AMOUNT)?;

        if field_32a.len() < 6 {
            return Err(MTError::InvalidFieldFormat {
                field: "32A".to_string(),
                message: "Field 32A too short for date".to_string(),
            });
        }

        let date_str = &field_32a[0..6];
        let swift_date = SwiftDate::parse_yymmdd(date_str)?;
        Ok(swift_date.date)
    }

    /// Get ordering customer (Field 50A/50F/50K) - Customer ordering the transfer
    pub fn ordering_customer(&self) -> Option<String> {
        // Try different variants in order of preference
        if let Some(customer) = get_optional_field_value(&self.fields, tags::ORDERING_CUSTOMER) {
            Some(customer)
        } else if let Some(customer) = get_optional_field_value(&self.fields, "50A") {
            Some(customer)
        } else {
            get_optional_field_value(&self.fields, "50F")
        }
    }

    /// Get ordering institution (Field 52A/52D) - Institution placing the order
    pub fn ordering_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::ORDERING_INSTITUTION)
    }

    /// Get ordering institution (Field 52D) - Alternative format
    pub fn ordering_institution_d(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "52D")
    }

    /// Get sender's correspondent (Field 53A/53B/53D) - Sender's correspondent
    pub fn senders_correspondent(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::SENDERS_CORRESPONDENT)
    }

    /// Get sender's correspondent (Field 53B) - Alternative format
    pub fn senders_correspondent_b(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "53B")
    }

    /// Get sender's correspondent (Field 53D) - Alternative format
    pub fn senders_correspondent_d(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "53D")
    }

    /// Get receiver's correspondent (Field 54A/54B/54D) - Receiver's correspondent
    pub fn receivers_correspondent(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::RECEIVERS_CORRESPONDENT)
    }

    /// Get receiver's correspondent (Field 54B) - Alternative format
    pub fn receivers_correspondent_b(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "54B")
    }

    /// Get receiver's correspondent (Field 54D) - Alternative format
    pub fn receivers_correspondent_d(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "54D")
    }

    /// Get intermediary institution (Field 56A/56C/56D) - Intermediary institution
    pub fn intermediary_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::INTERMEDIARY_INSTITUTION)
    }

    /// Get intermediary institution (Field 56C) - Alternative format
    pub fn intermediary_institution_c(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "56C")
    }

    /// Get intermediary institution (Field 56D) - Alternative format
    pub fn intermediary_institution_d(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "56D")
    }

    /// Get account with institution (Field 57A/57B/57C/57D) - Account with institution
    pub fn account_with_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::ACCOUNT_WITH_INSTITUTION)
    }

    /// Get account with institution (Field 57B) - Alternative format
    pub fn account_with_institution_b(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "57B")
    }

    /// Get account with institution (Field 57C) - Alternative format
    pub fn account_with_institution_c(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "57C")
    }

    /// Get account with institution (Field 57D) - Alternative format
    pub fn account_with_institution_d(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "57D")
    }

    /// Get beneficiary institution (Field 58A/58D) - Beneficiary institution
    pub fn beneficiary_institution(&self) -> Result<String> {
        if let Some(beneficiary) = get_optional_field_value(&self.fields, "58A") {
            Ok(beneficiary)
        } else if let Some(beneficiary) = get_optional_field_value(&self.fields, "58D") {
            Ok(beneficiary)
        } else {
            Err(MTError::missing_required_field("58A or 58D"))
        }
    }

    /// Get beneficiary institution (Field 58D) - Alternative format
    pub fn beneficiary_institution_d(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "58D")
    }

    /// Get beneficiary customer (Field 59/59A/59F) - Customer receiving the transfer
    pub fn beneficiary_customer(&self) -> Option<String> {
        // Try different variants in order of preference
        if let Some(customer) = get_optional_field_value(&self.fields, tags::BENEFICIARY_CUSTOMER) {
            Some(customer)
        } else if let Some(customer) = get_optional_field_value(&self.fields, "59A") {
            Some(customer)
        } else {
            get_optional_field_value(&self.fields, "59F")
        }
    }

    /// Get remittance information (Field 70) - Details of payment
    pub fn remittance_information(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::REMITTANCE_INFORMATION)
    }

    /// Get details of charges (Field 71A) - Details of charges
    pub fn details_of_charges(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::DETAILS_OF_CHARGES)
    }

    /// Get sender's charges (Field 71F) - Sender's charges
    pub fn senders_charges(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::SENDERS_CHARGES)
    }

    /// Get receiver's charges (Field 71G) - Receiver's charges
    pub fn receivers_charges(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::RECEIVERS_CHARGES)
    }

    /// Get regulatory reporting (Field 77B) - Regulatory reporting
    pub fn regulatory_reporting(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "77B")
    }

    /// Get instructions (Field 72) - Instructions to correspondent
    pub fn instructions(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "72")
    }

    /// Get all instructions (Field 72) - can have multiple
    pub fn all_instructions(&self) -> Vec<String> {
        find_fields(&self.fields, "72")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Get notice to receive details (Field 25) - Account identification at beneficiary institution
    pub fn account_identification(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::ACCOUNT_IDENTIFICATION)
    }

    /// Get notice reference (Field 21) - Notice reference number
    pub fn notice_reference(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "21")
    }

    /// Get notification details (Field 77A) - Notification details
    pub fn notification_details(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "77A")
    }

    /// Get all notification details (Field 77A) - can have multiple
    pub fn all_notification_details(&self) -> Vec<String> {
        find_fields(&self.fields, "77A")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Check if this is a pre-notification for an incoming transfer
    pub fn is_pre_notification(&self) -> bool {
        // MT210 is always a pre-notification message
        true
    }

    /// Get expected incoming amount and currency
    pub fn expected_incoming_amount(&self) -> Result<Amount> {
        self.amount()
    }

    /// Get expected value date for incoming transfer
    pub fn expected_value_date(&self) -> Result<NaiveDate> {
        self.value_date()
    }
}

impl MTMessageType for MT210 {
    fn from_blocks(blocks: Vec<MessageBlock>) -> Result<Self> {
        let fields = extract_text_block(&blocks)?;

        // Validate required fields are present
        let required_fields = [
            tags::SENDER_REFERENCE,           // Field 20
            tags::VALUE_DATE_CURRENCY_AMOUNT, // Field 32A
        ];

        // Check for required fields
        for &field_tag in &required_fields {
            if !fields.iter().any(|f| f.tag.as_str() == field_tag) {
                return Err(MTError::missing_required_field(field_tag));
            }
        }

        // Check for either 58A or 58D (beneficiary institution)
        if !fields
            .iter()
            .any(|f| f.tag.as_str() == "58A" || f.tag.as_str() == "58D")
        {
            return Err(MTError::missing_required_field("58A or 58D"));
        }

        Ok(MT210 { fields })
    }

    fn get_field(&self, tag: &str) -> Option<&Field> {
        find_field(&self.fields, tag)
    }

    fn get_fields(&self, tag: &str) -> Vec<&Field> {
        find_fields(&self.fields, tag)
    }

    fn get_all_fields(&self) -> Vec<&Field> {
        self.fields.iter().collect()
    }

    fn text_fields(&self) -> &[Field] {
        &self.fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Field;
    use chrono::Datelike;

    fn create_test_mt210() -> MT210 {
        let fields = vec![
            Field::new("20", "NTR123456789"),
            Field::new("21", "INCOMING12345"),
            Field::new("25", "BENACCT123456789"),
            Field::new("32A", "210315EUR5000000,00"),
            Field::new("50K", "ORDERING CUSTOMER\nCOMPANY ABC\nPARIS FR"),
            Field::new("52A", "ORDBANK55XXX"),
            Field::new("53A", "SNDCOR55XXX"),
            Field::new("54A", "RCVCOR55XXX"),
            Field::new("57A", "ACWITH55XXX"),
            Field::new("58A", "BENBANK66XXX"),
            Field::new("59", "BENEFICIARY CUSTOMER\nCOMPANY XYZ\nLONDON GB"),
            Field::new("70", "TRADE FINANCE PAYMENT"),
            Field::new("71A", "SHA"),
            Field::new("72", "INCOMING WIRE TRANSFER NOTIFICATION"),
            Field::new("77A", "PLEASE EXPECT INCOMING FUNDS"),
        ];
        MT210 { fields }
    }

    #[test]
    fn test_transaction_reference() {
        let mt210 = create_test_mt210();
        assert_eq!(mt210.transaction_reference().unwrap(), "NTR123456789");
    }

    #[test]
    fn test_related_reference() {
        let mt210 = create_test_mt210();
        assert_eq!(mt210.related_reference().unwrap(), "INCOMING12345");
    }

    #[test]
    fn test_account_identification() {
        let mt210 = create_test_mt210();
        assert_eq!(mt210.account_identification().unwrap(), "BENACCT123456789");
    }

    #[test]
    fn test_amount_parsing() {
        let mt210 = create_test_mt210();
        let amount = mt210.amount().unwrap();
        assert_eq!(amount.value, 5000000.0);
        assert_eq!(amount.currency, "EUR");
    }

    #[test]
    fn test_currency() {
        let mt210 = create_test_mt210();
        assert_eq!(mt210.currency().unwrap(), "EUR");
    }

    #[test]
    fn test_value_date() {
        let mt210 = create_test_mt210();
        let date = mt210.value_date().unwrap();
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 3);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_ordering_customer() {
        let mt210 = create_test_mt210();
        assert_eq!(
            mt210.ordering_customer().unwrap(),
            "ORDERING CUSTOMER\nCOMPANY ABC\nPARIS FR"
        );
    }

    #[test]
    fn test_beneficiary_customer() {
        let mt210 = create_test_mt210();
        assert_eq!(
            mt210.beneficiary_customer().unwrap(),
            "BENEFICIARY CUSTOMER\nCOMPANY XYZ\nLONDON GB"
        );
    }

    #[test]
    fn test_beneficiary_institution() {
        let mt210 = create_test_mt210();
        assert_eq!(mt210.beneficiary_institution().unwrap(), "BENBANK66XXX");
    }

    #[test]
    fn test_ordering_institution() {
        let mt210 = create_test_mt210();
        assert_eq!(mt210.ordering_institution().unwrap(), "ORDBANK55XXX");
    }

    #[test]
    fn test_remittance_information() {
        let mt210 = create_test_mt210();
        assert_eq!(
            mt210.remittance_information().unwrap(),
            "TRADE FINANCE PAYMENT"
        );
    }

    #[test]
    fn test_details_of_charges() {
        let mt210 = create_test_mt210();
        assert_eq!(mt210.details_of_charges().unwrap(), "SHA");
    }

    #[test]
    fn test_instructions() {
        let mt210 = create_test_mt210();
        assert_eq!(
            mt210.instructions().unwrap(),
            "INCOMING WIRE TRANSFER NOTIFICATION"
        );
    }

    #[test]
    fn test_notification_details() {
        let mt210 = create_test_mt210();
        assert_eq!(
            mt210.notification_details().unwrap(),
            "PLEASE EXPECT INCOMING FUNDS"
        );
    }

    #[test]
    fn test_is_pre_notification() {
        let mt210 = create_test_mt210();
        assert!(mt210.is_pre_notification());
    }

    #[test]
    fn test_expected_incoming_amount() {
        let mt210 = create_test_mt210();
        let amount = mt210.expected_incoming_amount().unwrap();
        assert_eq!(amount.value, 5000000.0);
        assert_eq!(amount.currency, "EUR");
    }

    #[test]
    fn test_expected_value_date() {
        let mt210 = create_test_mt210();
        let date = mt210.expected_value_date().unwrap();
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 3);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_get_field() {
        let mt210 = create_test_mt210();
        let field_20 = mt210.get_field("20").unwrap();
        assert_eq!(field_20.value(), "NTR123456789");
    }

    #[test]
    fn test_get_all_fields() {
        let mt210 = create_test_mt210();
        let all_fields = mt210.get_all_fields();
        assert_eq!(all_fields.len(), 15);
    }

    #[test]
    fn test_all_notification_details() {
        let mt210 = create_test_mt210();
        let details = mt210.all_notification_details();
        assert_eq!(details.len(), 1);
        assert_eq!(details[0], "PLEASE EXPECT INCOMING FUNDS");
    }

    #[test]
    fn test_notice_reference() {
        let mt210 = create_test_mt210();
        assert_eq!(mt210.notice_reference().unwrap(), "INCOMING12345");
    }
}
