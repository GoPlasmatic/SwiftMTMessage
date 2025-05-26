//! MT102: Multiple Customer Credit Transfer

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::common::{Amount, Field, MessageBlock, SwiftDate, tags};
use crate::error::{MTError, Result};
use crate::messages::{extract_text_block, find_field, find_fields, get_required_field_value, get_optional_field_value, MTMessageType};

/// MT102: Multiple Customer Credit Transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT102 {
    /// All fields from the text block
    fields: Vec<Field>,
}

impl MT102 {
    /// Get sender's reference (Field 20)
    pub fn sender_reference(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::SENDER_REFERENCE)
    }

    /// Get bank operation code (Field 23B)
    pub fn bank_operation_code(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::BANK_OPERATION_CODE)
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

    /// Get ordering customer (Field 50K)
    pub fn ordering_customer(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::ORDERING_CUSTOMER)
    }

    /// Get ordering institution (Field 52A) - optional
    pub fn ordering_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::ORDERING_INSTITUTION)
    }

    /// Get sender's correspondent (Field 53A) - optional
    pub fn senders_correspondent(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::SENDERS_CORRESPONDENT)
    }

    /// Get receiver's correspondent (Field 54A) - optional
    pub fn receivers_correspondent(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::RECEIVERS_CORRESPONDENT)
    }

    /// Get third reimbursement institution (Field 55A) - optional
    pub fn third_reimbursement_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::THIRD_REIMBURSEMENT_INSTITUTION)
    }

    /// Get intermediary institution (Field 56A) - optional
    pub fn intermediary_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::INTERMEDIARY_INSTITUTION)
    }

    /// Get account with institution (Field 57A) - optional
    pub fn account_with_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::ACCOUNT_WITH_INSTITUTION)
    }

    /// Get all beneficiary customers (Field 59) - MT102 can have multiple
    pub fn beneficiaries(&self) -> Vec<String> {
        find_fields(&self.fields, tags::BENEFICIARY_CUSTOMER)
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Get first beneficiary customer (Field 59)
    pub fn beneficiary(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::BENEFICIARY_CUSTOMER)
    }

    /// Get remittance information (Field 70) - optional
    pub fn remittance_information(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::REMITTANCE_INFORMATION)
    }

    /// Get details of charges (Field 71A) - optional
    pub fn details_of_charges(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::DETAILS_OF_CHARGES)
    }

    /// Get sender's charges (Field 71F) - optional
    pub fn senders_charges(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::SENDERS_CHARGES)
    }

    /// Get receiver's charges (Field 71G) - optional
    pub fn receivers_charges(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::RECEIVERS_CHARGES)
    }

    /// Get transaction type code (Field 26T) - MT102 specific
    pub fn transaction_type_code(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "26T")
    }

    /// Get number of transactions (Field 77B) - MT102 specific
    pub fn number_of_transactions(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "77B")
    }

    /// Get sum of amounts (Field 19) - MT102 specific
    pub fn sum_of_amounts(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "19")
    }

    /// Get all transaction references (Field 21) - MT102 can have multiple
    pub fn transaction_references(&self) -> Vec<String> {
        find_fields(&self.fields, "21")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Get all amounts (Field 32B) - MT102 can have multiple transaction amounts
    pub fn transaction_amounts(&self) -> Vec<String> {
        find_fields(&self.fields, "32B")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }
}

impl MTMessageType for MT102 {
    fn from_blocks(blocks: Vec<MessageBlock>) -> Result<Self> {
        let fields = extract_text_block(&blocks)?;
        
        // Validate required fields are present
        let required_fields = [
            tags::SENDER_REFERENCE,
            tags::BANK_OPERATION_CODE,
            tags::VALUE_DATE_CURRENCY_AMOUNT,
            tags::ORDERING_CUSTOMER,
            tags::BENEFICIARY_CUSTOMER,
        ];

        for &field_tag in &required_fields {
            if !fields.iter().any(|f| f.tag.as_str() == field_tag) {
                return Err(MTError::missing_required_field(field_tag));
            }
        }

        Ok(MT102 { fields })
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

    fn create_test_mt102() -> MT102 {
        let fields = vec![
            Field::new("20", "MT102REF123456"),
            Field::new("23B", "CRED"),
            Field::new("26T", "001"),
            Field::new("32A", "210315EUR5000000,00"),
            Field::new("19", "5000000,00"),
            Field::new("77B", "2"),
            Field::new("50K", "ORDERING BANK\nMAIN STREET 123\nCITY"),
            Field::new("21", "TXN001"),
            Field::new("32B", "EUR2500000,00"),
            Field::new("59", "BENEFICIARY 1\nADDRESS 1"),
            Field::new("21", "TXN002"),
            Field::new("32B", "EUR2500000,00"),
            Field::new("59", "BENEFICIARY 2\nADDRESS 2"),
            Field::new("70", "Multiple payments"),
        ];
        MT102 { fields }
    }

    #[test]
    fn test_sender_reference() {
        let mt102 = create_test_mt102();
        assert_eq!(mt102.sender_reference().unwrap(), "MT102REF123456");
    }

    #[test]
    fn test_bank_operation_code() {
        let mt102 = create_test_mt102();
        assert_eq!(mt102.bank_operation_code().unwrap(), "CRED");
    }

    #[test]
    fn test_amount_parsing() {
        let mt102 = create_test_mt102();
        let amount = mt102.amount().unwrap();
        assert_eq!(amount.currency, "EUR");
        assert_eq!(amount.value, 5000000.00);
    }

    #[test]
    fn test_currency() {
        let mt102 = create_test_mt102();
        assert_eq!(mt102.currency().unwrap(), "EUR");
    }

    #[test]
    fn test_value_date() {
        let mt102 = create_test_mt102();
        let date = mt102.value_date().unwrap();
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 3);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_ordering_customer() {
        let mt102 = create_test_mt102();
        assert_eq!(mt102.ordering_customer().unwrap(), "ORDERING BANK\nMAIN STREET 123\nCITY");
    }

    #[test]
    fn test_beneficiaries() {
        let mt102 = create_test_mt102();
        let beneficiaries = mt102.beneficiaries();
        assert_eq!(beneficiaries.len(), 2);
        assert_eq!(beneficiaries[0], "BENEFICIARY 1\nADDRESS 1");
        assert_eq!(beneficiaries[1], "BENEFICIARY 2\nADDRESS 2");
    }

    #[test]
    fn test_transaction_references() {
        let mt102 = create_test_mt102();
        let refs = mt102.transaction_references();
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0], "TXN001");
        assert_eq!(refs[1], "TXN002");
    }

    #[test]
    fn test_transaction_amounts() {
        let mt102 = create_test_mt102();
        let amounts = mt102.transaction_amounts();
        assert_eq!(amounts.len(), 2);
        assert_eq!(amounts[0], "EUR2500000,00");
        assert_eq!(amounts[1], "EUR2500000,00");
    }

    #[test]
    fn test_mt102_specific_fields() {
        let mt102 = create_test_mt102();
        assert_eq!(mt102.transaction_type_code().unwrap(), "001");
        assert_eq!(mt102.number_of_transactions().unwrap(), "2");
        assert_eq!(mt102.sum_of_amounts().unwrap(), "5000000,00");
    }

    #[test]
    fn test_get_field() {
        let mt102 = create_test_mt102();
        let field = mt102.get_field("20").unwrap();
        assert_eq!(field.value(), "MT102REF123456");
    }

    #[test]
    fn test_get_all_fields() {
        let mt102 = create_test_mt102();
        let fields = mt102.get_all_fields();
        assert_eq!(fields.len(), 14);
    }
} 