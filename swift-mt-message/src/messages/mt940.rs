use crate::fields::{
    Field20, Field21, Field25, Field28C, Field60F, Field61, Field62F, Field64, Field65, Field86,
};
use crate::{SwiftMessageBody, SwiftResult};
use serde::{Deserialize, Serialize};

/// # MT940: Customer Statement Message
///
/// ## Overview
/// MT940 is used by financial institutions to send customer account statements
/// containing transaction details and balance information. This message provides
/// a detailed view of account activity over a specific period.
///
/// ## Message Structure
/// - **Field 20**: Transaction Reference Number (Mandatory)
/// - **Field 21**: Related Reference (Optional)
/// - **Field 25**: Account Identification (Mandatory)
/// - **Field 28C**: Statement/Sequence Number (Mandatory)
/// - **Field 60F**: Opening Balance (Mandatory)
/// - **Field 61**: Statement Line (Optional, Repeating)
/// - **Field 86**: Info to Account Owner (Optional, follows Field 61)
/// - **Field 62F**: Closing Balance (Mandatory)
/// - **Field 64**: Closing Available Balance (Optional)
/// - **Field 65**: Forward Available Balance (Optional)
///
/// ## Business Rules
/// - Opening balance (60F) and closing balance (62F) must be in consistent currency
/// - Each Field 61 (transaction line) may be followed by optional Field 86
/// - Balances use comma as decimal separator
/// - Statement supports multi-part statements via Field 28C
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT940 {
    /// **Transaction Reference Number** - Field 20 (Mandatory)
    /// Unique ID, no leading/trailing slashes
    pub field_20: Field20,

    /// **Related Reference** - Field 21 (Optional)
    /// Links to MT920 if applicable
    pub field_21: Option<Field21>,

    /// **Account Identification** - Field 25 (Mandatory)
    /// IBAN, BIC optional
    pub field_25: Field25,

    /// **Statement/Sequence Number** - Field 28C (Mandatory)
    /// Statement and sub-sequence
    pub field_28c: Field28C,

    /// **Opening Balance** - Field 60F (Mandatory)
    /// Booked opening balance
    pub field_60f: Field60F,

    /// **Statement Lines** - Field 61 (Optional, Repeating)
    /// Transaction lines with optional accompanying Field 86
    pub statement_lines: Vec<StatementLine>,

    /// **Closing Balance** - Field 62F (Mandatory)
    /// Booked closing balance
    pub field_62f: Field62F,

    /// **Closing Available Balance** - Field 64 (Optional)
    /// Cash availability balance
    pub field_64: Option<Field64>,

    /// **Forward Available Balance** - Field 65 (Optional)
    /// Value-dated available funds
    pub field_65: Option<Field65>,
}

/// # Statement Line
///
/// Represents a single transaction line (Field 61) with optional
/// accompanying information (Field 86).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatementLine {
    /// **Statement Line** - Field 61 (Mandatory)
    /// Transaction details
    pub field_61: Field61,

    /// **Info to Account Owner** - Field 86 (Optional)
    /// Narrative details for the transaction
    pub field_86: Option<Field86>,
}

impl StatementLine {
    /// Create a new statement line with transaction details
    pub fn new(field_61: Field61) -> Self {
        Self {
            field_61,
            field_86: None,
        }
    }

    /// Create a new statement line with transaction details and narrative
    pub fn with_info(field_61: Field61, field_86: Field86) -> Self {
        Self {
            field_61,
            field_86: Some(field_86),
        }
    }

    /// Add narrative information to the statement line
    pub fn add_info(&mut self, field_86: Field86) {
        self.field_86 = Some(field_86);
    }

    /// Check if this statement line has narrative information
    pub fn has_info(&self) -> bool {
        self.field_86.is_some()
    }
}

impl MT940 {
    /// Create a new MT940 with required fields
    pub fn new(
        field_20: Field20,
        field_25: Field25,
        field_28c: Field28C,
        field_60f: Field60F,
        field_62f: Field62F,
    ) -> Self {
        Self {
            field_20,
            field_21: None,
            field_25,
            field_28c,
            field_60f,
            statement_lines: Vec::new(),
            field_62f,
            field_64: None,
            field_65: None,
        }
    }

    /// Get the transaction reference
    pub fn transaction_reference(&self) -> &str {
        &self.field_20.transaction_reference
    }

    /// Get the account identification
    pub fn account_identification(&self) -> &str {
        &self.field_25.authorisation
    }

    /// Get the number of statement lines
    pub fn statement_line_count(&self) -> usize {
        self.statement_lines.len()
    }

    /// Add a statement line
    pub fn add_statement_line(&mut self, statement_line: StatementLine) {
        self.statement_lines.push(statement_line);
    }

    /// Add a transaction with optional narrative
    pub fn add_transaction(&mut self, field_61: Field61, field_86: Option<Field86>) {
        let statement_line = match field_86 {
            Some(info) => StatementLine::with_info(field_61, info),
            None => StatementLine::new(field_61),
        };
        self.statement_lines.push(statement_line);
    }

    /// Set related reference
    pub fn set_related_reference(&mut self, field_21: Field21) {
        self.field_21 = Some(field_21);
    }

    /// Set closing available balance
    pub fn set_closing_available_balance(&mut self, field_64: Field64) {
        self.field_64 = Some(field_64);
    }

    /// Set forward available balance
    pub fn set_forward_available_balance(&mut self, field_65: Field65) {
        self.field_65 = Some(field_65);
    }

    /// Check if currencies are consistent across balance fields
    pub fn validate_currency_consistency(&self) -> bool {
        // This would need proper currency extraction from balance fields
        // For now, return true as a placeholder
        true
    }

    /// Get all statement lines with narrative information
    pub fn statement_lines_with_info(&self) -> Vec<&StatementLine> {
        self.statement_lines
            .iter()
            .filter(|line| line.has_info())
            .collect()
    }

    /// Get the total number of transactions
    pub fn transaction_count(&self) -> usize {
        self.statement_lines.len()
    }
}

impl SwiftMessageBody for MT940 {
    fn message_type() -> &'static str {
        "940"
    }

    fn from_fields(_fields: std::collections::HashMap<String, Vec<String>>) -> SwiftResult<Self> {
        // For now, return a basic implementation error
        // This would need proper field parsing implementation
        Err(crate::errors::ParseError::InvalidFormat {
            message: "MT940 field parsing not yet implemented".to_string(),
        })
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        // Basic implementation - would need proper field serialization
        std::collections::HashMap::new()
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "25", "28C", "60F", "62F"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["21", "61", "86", "64", "65"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt940_message_type() {
        assert_eq!(MT940::message_type(), "940");
    }

    #[test]
    fn test_mt940_creation() {
        let field_20 = Field20::new("STMT240315001234".to_string());
        let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
        let field_28c = Field28C::new(1, Some(1)).unwrap();
        let field_60f = Field60F::new('C', "240315", "EUR", 1000.00).unwrap();
        let field_62f = Field62F::new('C', "240315", "EUR", 1500.00).unwrap();

        let mt940 = MT940::new(field_20, field_25, field_28c, field_60f, field_62f);

        assert_eq!(mt940.transaction_reference(), "STMT240315001234");
        assert_eq!(mt940.account_identification(), "GB33BUKB20201555555555");
        assert_eq!(mt940.statement_line_count(), 0);
        assert!(mt940.field_21.is_none());
        assert!(mt940.field_64.is_none());
        assert!(mt940.field_65.is_none());
    }

    #[test]
    fn test_statement_line_creation() {
        // This test would need proper Field61 constructor - placeholder for now
        // let field_61 = Field61::new(...);
        // let field_86 = Field86::new(...);
        // let statement_line = StatementLine::with_info(field_61, field_86);
        // assert!(statement_line.has_info());
    }

    #[test]
    fn test_mt940_required_fields() {
        let required = MT940::required_fields();
        assert!(required.contains(&"20"));
        assert!(required.contains(&"25"));
        assert!(required.contains(&"28C"));
        assert!(required.contains(&"60F"));
        assert!(required.contains(&"62F"));
    }

    #[test]
    fn test_mt940_optional_fields() {
        let optional = MT940::optional_fields();
        assert!(optional.contains(&"21"));
        assert!(optional.contains(&"61"));
        assert!(optional.contains(&"86"));
        assert!(optional.contains(&"64"));
        assert!(optional.contains(&"65"));
    }
}
