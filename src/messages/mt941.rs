//! MT941: Balance Report Message

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::common::{Amount, Field, MessageBlock, SwiftDate, tags};
use crate::error::{MTError, Result};
use crate::messages::{extract_text_block, find_field, find_fields, get_required_field_value, get_optional_field_value, MTMessageType};

/// MT941: Balance Report Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT941 {
    /// All fields from the text block
    fields: Vec<Field>,
}

impl MT941 {
    /// Get transaction reference number (Field 20)
    pub fn transaction_reference(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::TRANSACTION_REFERENCE)
    }

    /// Get account identification (Field 25)
    pub fn account_identification(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::ACCOUNT_IDENTIFICATION)
    }

    /// Get statement number/sequence number (Field 28C)
    pub fn statement_number(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::STATEMENT_NUMBER)
    }

    /// Get opening balance (Field 60F or 60M)
    pub fn opening_balance(&self) -> Result<String> {
        // Try 60F first (first opening balance), then 60M (intermediate opening balance)
        if let Some(balance) = get_optional_field_value(&self.fields, tags::OPENING_BALANCE) {
            Ok(balance)
        } else if let Some(balance) = get_optional_field_value(&self.fields, "60M") {
            Ok(balance)
        } else {
            Err(MTError::missing_required_field("60F or 60M"))
        }
    }

    /// Parse opening balance into components
    pub fn parse_opening_balance(&self) -> Result<(String, NaiveDate, String, f64)> {
        let balance_str = self.opening_balance()?;
        self.parse_balance_field(&balance_str)
    }

    /// Get closing balance (Field 62F or 62M)
    pub fn closing_balance(&self) -> Result<String> {
        // Try 62F first (final closing balance), then 62M (intermediate closing balance)
        if let Some(balance) = get_optional_field_value(&self.fields, tags::CLOSING_BALANCE) {
            Ok(balance)
        } else if let Some(balance) = get_optional_field_value(&self.fields, "62M") {
            Ok(balance)
        } else {
            Err(MTError::missing_required_field("62F or 62M"))
        }
    }

    /// Parse closing balance into components
    pub fn parse_closing_balance(&self) -> Result<(String, NaiveDate, String, f64)> {
        let balance_str = self.closing_balance()?;
        self.parse_balance_field(&balance_str)
    }

    /// Get closing available balance (Field 64)
    pub fn closing_available_balance(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::CLOSING_AVAILABLE_BALANCE)
    }

    /// Parse closing available balance into components
    pub fn parse_closing_available_balance(&self) -> Option<Result<(String, NaiveDate, String, f64)>> {
        self.closing_available_balance().map(|balance_str| {
            self.parse_balance_field(&balance_str)
        })
    }

    /// Get all forward available balances (Field 65)
    pub fn forward_available_balances(&self) -> Vec<String> {
        find_fields(&self.fields, "65")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Get information to account owner (Field 86) - optional in MT941
    pub fn information_to_account_owner(&self) -> Vec<String> {
        find_fields(&self.fields, tags::INFORMATION_TO_ACCOUNT_OWNER)
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Parse a balance field (60F, 62F, 64, 65, etc.) into components
    /// Format: D/C + YYMMDD + Currency + Amount
    /// Example: "C210315EUR1234567,89"
    fn parse_balance_field(&self, balance_str: &str) -> Result<(String, NaiveDate, String, f64)> {
        if balance_str.len() < 10 {
            return Err(MTError::InvalidFieldFormat {
                field: "Balance".to_string(),
                message: "Balance field too short".to_string(),
            });
        }

        // Extract debit/credit indicator
        let dc_indicator = &balance_str[0..1];
        
        // Extract date (positions 1-6)
        let date_str = &balance_str[1..7];
        let swift_date = SwiftDate::parse_yymmdd(date_str)?;
        
        // Extract currency and amount (from position 7 onwards)
        let currency_amount = &balance_str[7..];
        let amount = Amount::parse(currency_amount)?;

        Ok((dc_indicator.to_string(), swift_date.date, amount.currency, amount.value))
    }

    /// Parse all forward available balances
    pub fn parse_forward_available_balances(&self) -> Vec<Result<(String, NaiveDate, String, f64)>> {
        self.forward_available_balances()
            .into_iter()
            .map(|balance_str| self.parse_balance_field(&balance_str))
            .collect()
    }

    /// Get balance summary - opening, closing, and available balances
    pub fn balance_summary(&self) -> Result<BalanceSummary> {
        let opening = self.parse_opening_balance()?;
        let closing = self.parse_closing_balance()?;
        let available = self.parse_closing_available_balance().transpose()?;
        let mut forward_balances = Vec::new();
        for balance_result in self.parse_forward_available_balances() {
            forward_balances.push(balance_result?);
        }

        Ok(BalanceSummary {
            opening_balance: opening,
            closing_balance: closing,
            closing_available_balance: available,
            forward_available_balances: forward_balances,
        })
    }
}

/// Summary of all balances in an MT941 message
#[derive(Debug, Clone)]
pub struct BalanceSummary {
    pub opening_balance: (String, NaiveDate, String, f64), // (D/C, Date, Currency, Amount)
    pub closing_balance: (String, NaiveDate, String, f64),
    pub closing_available_balance: Option<(String, NaiveDate, String, f64)>,
    pub forward_available_balances: Vec<(String, NaiveDate, String, f64)>,
}

impl MTMessageType for MT941 {
    fn from_blocks(blocks: Vec<MessageBlock>) -> Result<Self> {
        let fields = extract_text_block(&blocks)?;
        
        // Validate required fields are present
        let required_fields = [
            tags::TRANSACTION_REFERENCE, // Field 20
            tags::ACCOUNT_IDENTIFICATION, // Field 25
            tags::STATEMENT_NUMBER, // Field 28C
        ];

        for &field_tag in &required_fields {
            if !fields.iter().any(|f| f.tag.as_str() == field_tag) {
                return Err(MTError::missing_required_field(field_tag));
            }
        }

        // Check for opening balance (60F or 60M)
        if !fields.iter().any(|f| f.tag.as_str() == "60F" || f.tag.as_str() == "60M") {
            return Err(MTError::missing_required_field("60F or 60M"));
        }

        // Check for closing balance (62F or 62M)
        if !fields.iter().any(|f| f.tag.as_str() == "62F" || f.tag.as_str() == "62M") {
            return Err(MTError::missing_required_field("62F or 62M"));
        }

        Ok(MT941 { fields })
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

    fn create_test_mt941() -> MT941 {
        let fields = vec![
            Field::new("20", "BAL123456789"),
            Field::new("25", "12345678901234567890"),
            Field::new("28C", "123/1"),
            Field::new("60F", "C210315EUR1000000,00"),
            Field::new("62F", "C210316EUR1050000,00"),
            Field::new("64", "C210316EUR1000000,00"),
            Field::new("65", "C210317EUR1000000,00"),
            Field::new("65", "C210318EUR1000000,00"),
            Field::new("86", "BALANCE REPORT FOR ACCOUNT"),
        ];
        MT941 { fields }
    }

    #[test]
    fn test_transaction_reference() {
        let mt941 = create_test_mt941();
        assert_eq!(mt941.transaction_reference().unwrap(), "BAL123456789");
    }

    #[test]
    fn test_account_identification() {
        let mt941 = create_test_mt941();
        assert_eq!(mt941.account_identification().unwrap(), "12345678901234567890");
    }

    #[test]
    fn test_statement_number() {
        let mt941 = create_test_mt941();
        assert_eq!(mt941.statement_number().unwrap(), "123/1");
    }

    #[test]
    fn test_opening_balance() {
        let mt941 = create_test_mt941();
        assert_eq!(mt941.opening_balance().unwrap(), "C210315EUR1000000,00");
    }

    #[test]
    fn test_closing_balance() {
        let mt941 = create_test_mt941();
        assert_eq!(mt941.closing_balance().unwrap(), "C210316EUR1050000,00");
    }

    #[test]
    fn test_parse_opening_balance() {
        let mt941 = create_test_mt941();
        let (dc, date, currency, amount) = mt941.parse_opening_balance().unwrap();
        assert_eq!(dc, "C");
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 3);
        assert_eq!(date.day(), 15);
        assert_eq!(currency, "EUR");
        assert_eq!(amount, 1000000.00);
    }

    #[test]
    fn test_parse_closing_balance() {
        let mt941 = create_test_mt941();
        let (dc, date, currency, amount) = mt941.parse_closing_balance().unwrap();
        assert_eq!(dc, "C");
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 3);
        assert_eq!(date.day(), 16);
        assert_eq!(currency, "EUR");
        assert_eq!(amount, 1050000.00);
    }

    #[test]
    fn test_closing_available_balance() {
        let mt941 = create_test_mt941();
        assert_eq!(mt941.closing_available_balance().unwrap(), "C210316EUR1000000,00");
    }

    #[test]
    fn test_forward_available_balances() {
        let mt941 = create_test_mt941();
        let balances = mt941.forward_available_balances();
        assert_eq!(balances.len(), 2);
        assert_eq!(balances[0], "C210317EUR1000000,00");
        assert_eq!(balances[1], "C210318EUR1000000,00");
    }

    #[test]
    fn test_parse_forward_available_balances() {
        let mt941 = create_test_mt941();
        let parsed_balances = mt941.parse_forward_available_balances();
        assert_eq!(parsed_balances.len(), 2);
        
        let (dc1, date1, currency1, amount1) = parsed_balances[0].as_ref().unwrap();
        assert_eq!(dc1, "C");
        assert_eq!(date1.day(), 17);
        assert_eq!(currency1, "EUR");
        assert_eq!(*amount1, 1000000.00);

        let (dc2, date2, currency2, amount2) = parsed_balances[1].as_ref().unwrap();
        assert_eq!(dc2, "C");
        assert_eq!(date2.day(), 18);
        assert_eq!(currency2, "EUR");
        assert_eq!(*amount2, 1000000.00);
    }

    #[test]
    fn test_balance_summary() {
        let mt941 = create_test_mt941();
        let summary = mt941.balance_summary().unwrap();
        
        assert_eq!(summary.opening_balance.0, "C");
        assert_eq!(summary.opening_balance.3, 1000000.00);
        
        assert_eq!(summary.closing_balance.0, "C");
        assert_eq!(summary.closing_balance.3, 1050000.00);
        
        assert!(summary.closing_available_balance.is_some());
        assert_eq!(summary.forward_available_balances.len(), 2);
    }

    #[test]
    fn test_information_to_account_owner() {
        let mt941 = create_test_mt941();
        let info = mt941.information_to_account_owner();
        assert_eq!(info.len(), 1);
        assert_eq!(info[0], "BALANCE REPORT FOR ACCOUNT");
    }

    #[test]
    fn test_get_field() {
        let mt941 = create_test_mt941();
        let field = mt941.get_field("20").unwrap();
        assert_eq!(field.value(), "BAL123456789");
    }

    #[test]
    fn test_get_all_fields() {
        let mt941 = create_test_mt941();
        let fields = mt941.get_all_fields();
        assert_eq!(fields.len(), 9);
    }
} 