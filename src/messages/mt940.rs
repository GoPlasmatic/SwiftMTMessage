//! MT940: Customer Statement Message

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::common::{Amount, Field, MessageBlock, SwiftDate, tags};
use crate::error::{MTError, Result};
use crate::messages::{extract_text_block, find_field, find_fields, get_required_field_value, get_optional_field_value, MTMessageType};

/// MT940: Customer Statement Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT940 {
    /// All fields from the text block
    fields: Vec<Field>,
}

impl MT940 {
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

    /// Get all statement lines (Field 61)
    pub fn statement_lines(&self) -> Vec<String> {
        find_fields(&self.fields, tags::STATEMENT_LINE)
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Get all information to account owner (Field 86)
    pub fn information_to_account_owner(&self) -> Vec<String> {
        find_fields(&self.fields, tags::INFORMATION_TO_ACCOUNT_OWNER)
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Get forward available balance (Field 65) - optional
    pub fn forward_available_balance(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "65")
    }

    /// Parse a balance field (60F, 62F, 64, etc.) into components
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

    /// Parse a statement line (Field 61) into components
    /// This is a complex field with value date, entry date, debit/credit, amount, transaction type, etc.
    pub fn parse_statement_line(&self, line: &str) -> Result<StatementLineInfo> {
        if line.len() < 10 {
            return Err(MTError::InvalidFieldFormat {
                field: "61".to_string(),
                message: "Statement line too short".to_string(),
            });
        }

        // Basic parsing - this is a simplified version
        // Real MT940 statement line parsing is quite complex
        let value_date_str = &line[0..6];
        let value_date = SwiftDate::parse_yymmdd(value_date_str)?.date;

        // Entry date (optional, positions 6-10)
        // In MT940, entry date can be MMDD format (4 digits) when it's the same year as value date
        let entry_date = if line.len() > 10 && line.chars().nth(6).unwrap().is_ascii_digit() {
            let entry_date_str = &line[6..10];
            if entry_date_str.len() == 4 {
                // MMDD format - use the same year as value date
                let year_prefix = &line[0..2]; // YY from value date
                let full_entry_date = format!("{}{}", year_prefix, entry_date_str);
                Some(SwiftDate::parse_yymmdd(&full_entry_date)?.date)
            } else {
                None
            }
        } else {
            None
        };

        Ok(StatementLineInfo {
            value_date,
            entry_date,
            raw_line: line.to_string(),
        })
    }

    /// Get statement lines with parsed information
    pub fn parsed_statement_lines(&self) -> Vec<Result<StatementLineInfo>> {
        self.statement_lines()
            .into_iter()
            .map(|line| self.parse_statement_line(&line))
            .collect()
    }
}

/// Parsed information from a statement line (Field 61)
#[derive(Debug, Clone)]
pub struct StatementLineInfo {
    pub value_date: NaiveDate,
    pub entry_date: Option<NaiveDate>,
    pub raw_line: String,
}

impl MTMessageType for MT940 {
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

        Ok(MT940 { fields })
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

    fn create_test_mt940() -> MT940 {
        let fields = vec![
            Field::new("20", "STMT123456789"),
            Field::new("25", "12345678901234567890"),
            Field::new("28C", "123/1"),
            Field::new("60F", "C210315EUR1000000,00"),
            Field::new("61", "2103150315DR500,00NTRFNONREF//PAYMENT"),
            Field::new("86", "PAYMENT RECEIVED FROM CUSTOMER ABC"),
            Field::new("61", "2103160316CR1500,00NTRFNONREF//DEPOSIT"),
            Field::new("86", "CASH DEPOSIT AT BRANCH"),
            Field::new("62F", "C210316EUR2000000,00"),
            Field::new("64", "C210316EUR1950000,00"),
        ];
        MT940 { fields }
    }

    #[test]
    fn test_transaction_reference() {
        let mt940 = create_test_mt940();
        assert_eq!(mt940.transaction_reference().unwrap(), "STMT123456789");
    }

    #[test]
    fn test_account_identification() {
        let mt940 = create_test_mt940();
        assert_eq!(mt940.account_identification().unwrap(), "12345678901234567890");
    }

    #[test]
    fn test_statement_number() {
        let mt940 = create_test_mt940();
        assert_eq!(mt940.statement_number().unwrap(), "123/1");
    }

    #[test]
    fn test_opening_balance() {
        let mt940 = create_test_mt940();
        assert_eq!(mt940.opening_balance().unwrap(), "C210315EUR1000000,00");
    }

    #[test]
    fn test_closing_balance() {
        let mt940 = create_test_mt940();
        assert_eq!(mt940.closing_balance().unwrap(), "C210316EUR2000000,00");
    }

    #[test]
    fn test_parse_opening_balance() {
        let mt940 = create_test_mt940();
        let (dc, date, currency, amount) = mt940.parse_opening_balance().unwrap();
        assert_eq!(dc, "C");
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 3);
        assert_eq!(date.day(), 15);
        assert_eq!(currency, "EUR");
        assert_eq!(amount, 1000000.00);
    }

    #[test]
    fn test_parse_closing_balance() {
        let mt940 = create_test_mt940();
        let (dc, date, currency, amount) = mt940.parse_closing_balance().unwrap();
        assert_eq!(dc, "C");
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 3);
        assert_eq!(date.day(), 16);
        assert_eq!(currency, "EUR");
        assert_eq!(amount, 2000000.00);
    }

    #[test]
    fn test_statement_lines() {
        let mt940 = create_test_mt940();
        let lines = mt940.statement_lines();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "2103150315DR500,00NTRFNONREF//PAYMENT");
        assert_eq!(lines[1], "2103160316CR1500,00NTRFNONREF//DEPOSIT");
    }

    #[test]
    fn test_information_to_account_owner() {
        let mt940 = create_test_mt940();
        let info = mt940.information_to_account_owner();
        assert_eq!(info.len(), 2);
        assert_eq!(info[0], "PAYMENT RECEIVED FROM CUSTOMER ABC");
        assert_eq!(info[1], "CASH DEPOSIT AT BRANCH");
    }

    #[test]
    fn test_closing_available_balance() {
        let mt940 = create_test_mt940();
        assert_eq!(mt940.closing_available_balance().unwrap(), "C210316EUR1950000,00");
    }

    #[test]
    fn test_parse_statement_line() {
        let mt940 = create_test_mt940();
        let line_info = mt940.parse_statement_line("2103150315DR500,00NTRFNONREF//PAYMENT").unwrap();
        assert_eq!(line_info.value_date.year(), 2021);
        assert_eq!(line_info.value_date.month(), 3);
        assert_eq!(line_info.value_date.day(), 15);
        assert!(line_info.entry_date.is_some());
    }

    #[test]
    fn test_get_field() {
        let mt940 = create_test_mt940();
        let field = mt940.get_field("20").unwrap();
        assert_eq!(field.value(), "STMT123456789");
    }

    #[test]
    fn test_get_all_fields() {
        let mt940 = create_test_mt940();
        let fields = mt940.get_all_fields();
        assert_eq!(fields.len(), 10);
    }
} 