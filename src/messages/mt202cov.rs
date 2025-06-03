//! MT202COV: General Financial Institution Transfer (Cover)

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::common::{Amount, Field, MessageBlock, SwiftDate, tags};
use crate::error::{MTError, Result};
use crate::messages::{
    MTMessageType, extract_text_block, find_field, find_fields, get_optional_field_value,
    get_required_field_value,
};

/// MT202COV: General Financial Institution Transfer (Cover)
/// This message is used in correspondent banking to provide cover for an underlying customer credit transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT202COV {
    /// All fields from the text block
    fields: Vec<Field>,
}

impl MT202COV {
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

    /// Get ordering customer (Field 50A/50F/50K) - Customer ordering the transfer
    pub fn ordering_customer(&self) -> Result<String> {
        // Try different variants in order of preference
        if let Some(customer) = get_optional_field_value(&self.fields, tags::ORDERING_CUSTOMER) {
            Ok(customer)
        } else if let Some(customer) = get_optional_field_value(&self.fields, "50A") {
            Ok(customer)
        } else if let Some(customer) = get_optional_field_value(&self.fields, "50F") {
            Ok(customer)
        } else {
            Err(MTError::missing_required_field("50K/50A/50F"))
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

    /// Get third reimbursement institution (Field 55A/55D) - Third reimbursement institution
    pub fn third_reimbursement_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::THIRD_REIMBURSEMENT_INSTITUTION)
    }

    /// Get third reimbursement institution (Field 55D) - Alternative format
    pub fn third_reimbursement_institution_d(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "55D")
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
    pub fn beneficiary_customer(&self) -> Result<String> {
        if let Some(customer) = get_optional_field_value(&self.fields, tags::BENEFICIARY_CUSTOMER) {
            Ok(customer)
        } else if let Some(customer) = get_optional_field_value(&self.fields, "59A") {
            Ok(customer)
        } else if let Some(customer) = get_optional_field_value(&self.fields, "59F") {
            Ok(customer)
        } else {
            Err(MTError::missing_required_field("59/59A/59F"))
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

    /// Get underlying customer credit transfer reference (Field 21) - Reference to underlying MT103
    pub fn underlying_customer_credit_transfer(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "21")
    }

    /// Check if this is a cover message for an underlying customer transfer
    pub fn is_cover_message(&self) -> bool {
        // MT202COV is always a cover message if it has customer fields (50K and 59)
        self.get_field("50K").is_some() || self.get_field("59").is_some()
    }
}

impl MTMessageType for MT202COV {
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

        // For COV messages, we also need ordering customer and beneficiary customer
        if !fields
            .iter()
            .any(|f| f.tag.as_str() == "50K" || f.tag.as_str() == "50A" || f.tag.as_str() == "50F")
        {
            return Err(MTError::missing_required_field("50K/50A/50F"));
        }

        if !fields
            .iter()
            .any(|f| f.tag.as_str() == "59" || f.tag.as_str() == "59A" || f.tag.as_str() == "59F")
        {
            return Err(MTError::missing_required_field("59/59A/59F"));
        }

        Ok(MT202COV { fields })
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

    fn create_test_mt202cov() -> MT202COV {
        let fields = vec![
            Field::new("20", "COV123456789"),
            Field::new("21", "NOTPROVIDED"),
            Field::new("32A", "210315USD10000000,00"),
            Field::new("50K", "ORDERING CUSTOMER\nCOMPANY ABC\nNEW YORK NY"),
            Field::new("52A", "ORDBANK33XXX"),
            Field::new("53A", "SNDCOR33XXX"),
            Field::new("54A", "RCVCOR33XXX"),
            Field::new("57A", "ACWITH33XXX"),
            Field::new("58A", "BENBANK44XXX"),
            Field::new("59", "BENEFICIARY CUSTOMER\nCOMPANY XYZ\nLONDON GB"),
            Field::new("70", "INVOICE PAYMENT INV-2021-001"),
            Field::new("71A", "OUR"),
            Field::new("72", "COVER FOR UNDERLYING MT103"),
        ];
        MT202COV { fields }
    }

    #[test]
    fn test_transaction_reference() {
        let mt202cov = create_test_mt202cov();
        assert_eq!(mt202cov.transaction_reference().unwrap(), "COV123456789");
    }

    #[test]
    fn test_related_reference() {
        let mt202cov = create_test_mt202cov();
        assert_eq!(mt202cov.related_reference().unwrap(), "NOTPROVIDED");
    }

    #[test]
    fn test_amount_parsing() {
        let mt202cov = create_test_mt202cov();
        let amount = mt202cov.amount().unwrap();
        assert_eq!(amount.value, 10000000.0);
        assert_eq!(amount.currency, "USD");
    }

    #[test]
    fn test_currency() {
        let mt202cov = create_test_mt202cov();
        assert_eq!(mt202cov.currency().unwrap(), "USD");
    }

    #[test]
    fn test_value_date() {
        let mt202cov = create_test_mt202cov();
        let date = mt202cov.value_date().unwrap();
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 3);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_ordering_customer() {
        let mt202cov = create_test_mt202cov();
        assert_eq!(
            mt202cov.ordering_customer().unwrap(),
            "ORDERING CUSTOMER\nCOMPANY ABC\nNEW YORK NY"
        );
    }

    #[test]
    fn test_beneficiary_customer() {
        let mt202cov = create_test_mt202cov();
        assert_eq!(
            mt202cov.beneficiary_customer().unwrap(),
            "BENEFICIARY CUSTOMER\nCOMPANY XYZ\nLONDON GB"
        );
    }

    #[test]
    fn test_beneficiary_institution() {
        let mt202cov = create_test_mt202cov();
        assert_eq!(mt202cov.beneficiary_institution().unwrap(), "BENBANK44XXX");
    }

    #[test]
    fn test_ordering_institution() {
        let mt202cov = create_test_mt202cov();
        assert_eq!(mt202cov.ordering_institution().unwrap(), "ORDBANK33XXX");
    }

    #[test]
    fn test_remittance_information() {
        let mt202cov = create_test_mt202cov();
        assert_eq!(
            mt202cov.remittance_information().unwrap(),
            "INVOICE PAYMENT INV-2021-001"
        );
    }

    #[test]
    fn test_details_of_charges() {
        let mt202cov = create_test_mt202cov();
        assert_eq!(mt202cov.details_of_charges().unwrap(), "OUR");
    }

    #[test]
    fn test_instructions() {
        let mt202cov = create_test_mt202cov();
        assert_eq!(
            mt202cov.instructions().unwrap(),
            "COVER FOR UNDERLYING MT103"
        );
    }

    #[test]
    fn test_is_cover_message() {
        let mt202cov = create_test_mt202cov();
        assert!(mt202cov.is_cover_message());
    }

    #[test]
    fn test_get_field() {
        let mt202cov = create_test_mt202cov();
        let field_20 = mt202cov.get_field("20").unwrap();
        assert_eq!(field_20.value(), "COV123456789");
    }

    #[test]
    fn test_get_all_fields() {
        let mt202cov = create_test_mt202cov();
        let all_fields = mt202cov.get_all_fields();
        assert_eq!(all_fields.len(), 13);
    }
}
