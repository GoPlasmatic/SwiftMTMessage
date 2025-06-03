//! MT942: Interim Transaction Report

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::common::{Amount, Field, MessageBlock, SwiftDate, tags};
use crate::error::{MTError, Result};
use crate::messages::{
    MTMessageType, extract_text_block, find_field, find_fields, get_optional_field_value,
    get_required_field_value,
};

/// MT942: Interim Transaction Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT942 {
    /// All fields from the text block
    fields: Vec<Field>,
}

impl MT942 {
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

    /// Get floor limit indicator (Field 34F) - MT942 specific
    pub fn floor_limit_indicator(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "34F")
    }

    /// Get date/time indication (Field 13D) - MT942 specific
    pub fn date_time_indication(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "13D")
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
    pub fn parse_closing_available_balance(
        &self,
    ) -> Option<Result<(String, NaiveDate, String, f64)>> {
        self.closing_available_balance()
            .map(|balance_str| self.parse_balance_field(&balance_str))
    }

    /// Get all statement lines (Field 61) - transactions above floor limit
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

    /// Get all forward available balances (Field 65)
    pub fn forward_available_balances(&self) -> Vec<String> {
        find_fields(&self.fields, "65")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
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

        Ok((
            dc_indicator.to_string(),
            swift_date.date,
            amount.currency,
            amount.value,
        ))
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
        // Real MT942 statement line parsing is quite complex
        let value_date_str = &line[0..6];
        let value_date = SwiftDate::parse_yymmdd(value_date_str)?.date;

        // Entry date (optional, positions 6-10)
        // In MT942, entry date can be MMDD format (4 digits) when it's the same year as value date
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

    /// Parse floor limit indicator into currency and amount
    pub fn parse_floor_limit(&self) -> Option<Result<(String, f64)>> {
        self.floor_limit_indicator().map(|limit_str| {
            if limit_str.len() < 3 {
                return Err(MTError::InvalidFieldFormat {
                    field: "34F".to_string(),
                    message: "Floor limit field too short".to_string(),
                });
            }

            // Format: CCCNNNNN,NN (currency + amount)
            let amount = Amount::parse(&limit_str)?;
            Ok((amount.currency, amount.value))
        })
    }

    /// Get interim report summary
    pub fn interim_summary(&self) -> Result<InterimSummary> {
        let opening = self.parse_opening_balance()?;
        let closing = self.parse_closing_balance()?;
        let available = self.parse_closing_available_balance().transpose()?;
        let floor_limit = self.parse_floor_limit().transpose()?;
        let transaction_count = self.statement_lines().len();

        Ok(InterimSummary {
            opening_balance: opening,
            closing_balance: closing,
            closing_available_balance: available,
            floor_limit,
            transaction_count,
            date_time_indication: self.date_time_indication(),
        })
    }
}

/// Parsed information from a statement line (Field 61)
#[derive(Debug, Clone)]
pub struct StatementLineInfo {
    pub value_date: NaiveDate,
    pub entry_date: Option<NaiveDate>,
    pub raw_line: String,
}

/// Summary of an MT942 interim transaction report
#[derive(Debug, Clone)]
pub struct InterimSummary {
    pub opening_balance: (String, NaiveDate, String, f64), // (D/C, Date, Currency, Amount)
    pub closing_balance: (String, NaiveDate, String, f64),
    pub closing_available_balance: Option<(String, NaiveDate, String, f64)>,
    pub floor_limit: Option<(String, f64)>, // (Currency, Amount)
    pub transaction_count: usize,
    pub date_time_indication: Option<String>,
}

impl MTMessageType for MT942 {
    fn from_blocks(blocks: Vec<MessageBlock>) -> Result<Self> {
        let fields = extract_text_block(&blocks)?;

        // Validate required fields are present
        let required_fields = [
            tags::TRANSACTION_REFERENCE,  // Field 20
            tags::ACCOUNT_IDENTIFICATION, // Field 25
            tags::STATEMENT_NUMBER,       // Field 28C
        ];

        for &field_tag in &required_fields {
            if !fields.iter().any(|f| f.tag.as_str() == field_tag) {
                return Err(MTError::missing_required_field(field_tag));
            }
        }

        // Check for opening balance (60F or 60M)
        if !fields
            .iter()
            .any(|f| f.tag.as_str() == "60F" || f.tag.as_str() == "60M")
        {
            return Err(MTError::missing_required_field("60F or 60M"));
        }

        // Check for closing balance (62F or 62M)
        if !fields
            .iter()
            .any(|f| f.tag.as_str() == "62F" || f.tag.as_str() == "62M")
        {
            return Err(MTError::missing_required_field("62F or 62M"));
        }

        Ok(MT942 { fields })
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

    fn create_test_mt942() -> MT942 {
        let fields = vec![
            Field::new("20", "INTERIM123456789"),
            Field::new("25", "12345678901234567890"),
            Field::new("28C", "123/1"),
            Field::new("13D", "2103151200+0100"),
            Field::new("34F", "EUR1000,00"),
            Field::new("60F", "C210315EUR1000000,00"),
            Field::new("61", "2103150315DR2500,00NTRFNONREF//PAYMENT"),
            Field::new("86", "LARGE PAYMENT ABOVE FLOOR LIMIT"),
            Field::new("61", "2103160316CR5000,00NTRFNONREF//DEPOSIT"),
            Field::new("86", "LARGE DEPOSIT ABOVE FLOOR LIMIT"),
            Field::new("62F", "C210316EUR1002500,00"),
            Field::new("64", "C210316EUR1000000,00"),
        ];
        MT942 { fields }
    }

    #[test]
    fn test_transaction_reference() {
        let mt942 = create_test_mt942();
        assert_eq!(mt942.transaction_reference().unwrap(), "INTERIM123456789");
    }

    #[test]
    fn test_account_identification() {
        let mt942 = create_test_mt942();
        assert_eq!(
            mt942.account_identification().unwrap(),
            "12345678901234567890"
        );
    }

    #[test]
    fn test_statement_number() {
        let mt942 = create_test_mt942();
        assert_eq!(mt942.statement_number().unwrap(), "123/1");
    }

    #[test]
    fn test_date_time_indication() {
        let mt942 = create_test_mt942();
        assert_eq!(mt942.date_time_indication().unwrap(), "2103151200+0100");
    }

    #[test]
    fn test_floor_limit_indicator() {
        let mt942 = create_test_mt942();
        assert_eq!(mt942.floor_limit_indicator().unwrap(), "EUR1000,00");
    }

    #[test]
    fn test_parse_floor_limit() {
        let mt942 = create_test_mt942();
        let (currency, amount) = mt942.parse_floor_limit().unwrap().unwrap();
        assert_eq!(currency, "EUR");
        assert_eq!(amount, 1000.00);
    }

    #[test]
    fn test_opening_balance() {
        let mt942 = create_test_mt942();
        assert_eq!(mt942.opening_balance().unwrap(), "C210315EUR1000000,00");
    }

    #[test]
    fn test_closing_balance() {
        let mt942 = create_test_mt942();
        assert_eq!(mt942.closing_balance().unwrap(), "C210316EUR1002500,00");
    }

    #[test]
    fn test_parse_opening_balance() {
        let mt942 = create_test_mt942();
        let (dc, date, currency, amount) = mt942.parse_opening_balance().unwrap();
        assert_eq!(dc, "C");
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 3);
        assert_eq!(date.day(), 15);
        assert_eq!(currency, "EUR");
        assert_eq!(amount, 1000000.00);
    }

    #[test]
    fn test_statement_lines() {
        let mt942 = create_test_mt942();
        let lines = mt942.statement_lines();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "2103150315DR2500,00NTRFNONREF//PAYMENT");
        assert_eq!(lines[1], "2103160316CR5000,00NTRFNONREF//DEPOSIT");
    }

    #[test]
    fn test_information_to_account_owner() {
        let mt942 = create_test_mt942();
        let info = mt942.information_to_account_owner();
        assert_eq!(info.len(), 2);
        assert_eq!(info[0], "LARGE PAYMENT ABOVE FLOOR LIMIT");
        assert_eq!(info[1], "LARGE DEPOSIT ABOVE FLOOR LIMIT");
    }

    #[test]
    fn test_closing_available_balance() {
        let mt942 = create_test_mt942();
        assert_eq!(
            mt942.closing_available_balance().unwrap(),
            "C210316EUR1000000,00"
        );
    }

    #[test]
    fn test_parse_statement_line() {
        let mt942 = create_test_mt942();
        let line_info = mt942
            .parse_statement_line("2103150315DR2500,00NTRFNONREF//PAYMENT")
            .unwrap();
        assert_eq!(line_info.value_date.year(), 2021);
        assert_eq!(line_info.value_date.month(), 3);
        assert_eq!(line_info.value_date.day(), 15);
        assert!(line_info.entry_date.is_some());
    }

    #[test]
    fn test_interim_summary() {
        let mt942 = create_test_mt942();
        let summary = mt942.interim_summary().unwrap();

        assert_eq!(summary.opening_balance.0, "C");
        assert_eq!(summary.opening_balance.3, 1000000.00);

        assert_eq!(summary.closing_balance.0, "C");
        assert_eq!(summary.closing_balance.3, 1002500.00);

        assert!(summary.closing_available_balance.is_some());
        assert!(summary.floor_limit.is_some());
        assert_eq!(summary.transaction_count, 2);
        assert!(summary.date_time_indication.is_some());
    }

    #[test]
    fn test_get_field() {
        let mt942 = create_test_mt942();
        let field = mt942.get_field("20").unwrap();
        assert_eq!(field.value(), "INTERIM123456789");
    }

    #[test]
    fn test_get_all_fields() {
        let mt942 = create_test_mt942();
        let fields = mt942.get_all_fields();
        assert_eq!(fields.len(), 12);
    }
}
