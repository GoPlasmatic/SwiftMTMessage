//! MT202: General Financial Institution Transfer

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::common::{Amount, Field, MessageBlock, SwiftDate, tags};
use crate::error::{MTError, Result};
use crate::messages::{extract_text_block, find_field, find_fields, get_required_field_value, get_optional_field_value, MTMessageType};

/// MT202: General Financial Institution Transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT202 {
    /// All fields from the text block
    fields: Vec<Field>,
}

impl MT202 {
    /// Get transaction reference number (Field 20)
    pub fn transaction_reference(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::SENDER_REFERENCE)
    }

    /// Get related reference (Field 21) - Related reference
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

    /// Get ordering institution (Field 52A) - Institution placing the order
    pub fn ordering_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::ORDERING_INSTITUTION)
    }

    /// Get ordering institution (Field 52D) - Alternative format
    pub fn ordering_institution_d(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "52D")
    }

    /// Get sender's correspondent (Field 53A) - Sender's correspondent
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

    /// Get receiver's correspondent (Field 54A) - Receiver's correspondent
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

    /// Get third reimbursement institution (Field 55A) - Third reimbursement institution
    pub fn third_reimbursement_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::THIRD_REIMBURSEMENT_INSTITUTION)
    }

    /// Get third reimbursement institution (Field 55D) - Alternative format
    pub fn third_reimbursement_institution_d(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "55D")
    }

    /// Get intermediary institution (Field 56A) - Intermediary institution
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

    /// Get account with institution (Field 57A) - Account with institution
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

    /// Get beneficiary institution (Field 58A) - Beneficiary institution
    pub fn beneficiary_institution(&self) -> Result<String> {
        get_required_field_value(&self.fields, "58A")
    }

    /// Get beneficiary institution (Field 58D) - Alternative format
    pub fn beneficiary_institution_d(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "58D")
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

    /// Get instructions to paying/receiving/cover bank (Field 72) - Instructions
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
}

impl MTMessageType for MT202 {
    fn from_blocks(blocks: Vec<MessageBlock>) -> Result<Self> {
        let fields = extract_text_block(&blocks)?;
        
        // Validate required fields are present
        let required_fields = [
            tags::SENDER_REFERENCE, // Field 20
            tags::VALUE_DATE_CURRENCY_AMOUNT, // Field 32A
            "58A", // Beneficiary institution (either 58A or 58D required)
        ];

        // Check for field 20 and 32A
        for &field_tag in &required_fields[0..2] {
            if !fields.iter().any(|f| f.tag.as_str() == field_tag) {
                return Err(MTError::missing_required_field(field_tag));
            }
        }

        // Check for either 58A or 58D
        if !fields.iter().any(|f| f.tag.as_str() == "58A" || f.tag.as_str() == "58D") {
            return Err(MTError::missing_required_field("58A or 58D"));
        }

        Ok(MT202 { fields })
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

    fn create_test_mt202() -> MT202 {
        let fields = vec![
            Field::new("20", "FI202123456789"),
            Field::new("21", "REL987654321"),
            Field::new("32A", "210315USD10000000,00"),
            Field::new("52A", "ORDERING BANK\nNEW YORK"),
            Field::new("53A", "SENDERS CORRESPONDENT\nLONDON"),
            Field::new("54A", "RECEIVERS CORRESPONDENT\nTOKYO"),
            Field::new("56A", "INTERMEDIARY BANK\nFRANKFURT"),
            Field::new("57A", "ACCOUNT WITH BANK\nZURICH"),
            Field::new("58A", "BENEFICIARY BANK\nSINGAPORE"),
            Field::new("70", "INTERBANK TRANSFER"),
            Field::new("71A", "OUR"),
            Field::new("72", "URGENT PROCESSING REQUIRED"),
            Field::new("77B", "/ORDERRES/BE//MEILAAN 1, 1000 BRUSSELS"),
        ];
        MT202 { fields }
    }

    #[test]
    fn test_transaction_reference() {
        let mt202 = create_test_mt202();
        assert_eq!(mt202.transaction_reference().unwrap(), "FI202123456789");
    }

    #[test]
    fn test_related_reference() {
        let mt202 = create_test_mt202();
        assert_eq!(mt202.related_reference().unwrap(), "REL987654321");
    }

    #[test]
    fn test_amount_parsing() {
        let mt202 = create_test_mt202();
        let amount = mt202.amount().unwrap();
        assert_eq!(amount.currency, "USD");
        assert_eq!(amount.value, 10000000.00);
    }

    #[test]
    fn test_currency() {
        let mt202 = create_test_mt202();
        assert_eq!(mt202.currency().unwrap(), "USD");
    }

    #[test]
    fn test_value_date() {
        let mt202 = create_test_mt202();
        let date = mt202.value_date().unwrap();
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 3);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_ordering_institution() {
        let mt202 = create_test_mt202();
        assert_eq!(mt202.ordering_institution().unwrap(), "ORDERING BANK\nNEW YORK");
    }

    #[test]
    fn test_beneficiary_institution() {
        let mt202 = create_test_mt202();
        assert_eq!(mt202.beneficiary_institution().unwrap(), "BENEFICIARY BANK\nSINGAPORE");
    }

    #[test]
    fn test_intermediary_institution() {
        let mt202 = create_test_mt202();
        assert_eq!(mt202.intermediary_institution().unwrap(), "INTERMEDIARY BANK\nFRANKFURT");
    }

    #[test]
    fn test_remittance_information() {
        let mt202 = create_test_mt202();
        assert_eq!(mt202.remittance_information().unwrap(), "INTERBANK TRANSFER");
    }

    #[test]
    fn test_details_of_charges() {
        let mt202 = create_test_mt202();
        assert_eq!(mt202.details_of_charges().unwrap(), "OUR");
    }

    #[test]
    fn test_instructions() {
        let mt202 = create_test_mt202();
        assert_eq!(mt202.instructions().unwrap(), "URGENT PROCESSING REQUIRED");
    }

    #[test]
    fn test_regulatory_reporting() {
        let mt202 = create_test_mt202();
        assert_eq!(mt202.regulatory_reporting().unwrap(), "/ORDERRES/BE//MEILAAN 1, 1000 BRUSSELS");
    }

    #[test]
    fn test_get_field() {
        let mt202 = create_test_mt202();
        let field = mt202.get_field("20").unwrap();
        assert_eq!(field.value(), "FI202123456789");
    }

    #[test]
    fn test_get_all_fields() {
        let mt202 = create_test_mt202();
        let fields = mt202.get_all_fields();
        assert_eq!(fields.len(), 13);
    }
} 