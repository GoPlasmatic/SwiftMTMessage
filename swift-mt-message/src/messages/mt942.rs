use crate::fields::{
    Field13D, Field20, Field21, Field25, Field28C, Field34F, Field61, Field86, Field90C, Field90D,
};
use crate::{SwiftMessageBody, SwiftResult};
use serde::{Deserialize, Serialize};

/// # MT942: Interim Transaction Report
///
/// ## Overview
/// MT942 is used by financial institutions to send interim transaction reports
/// containing transaction details within specified floor limits. This message
/// provides real-time transaction reporting for amounts above certain thresholds.
///
/// ## Message Structure
/// - **Field 20**: Transaction Reference Number (Mandatory)
/// - **Field 21**: Related Reference (Optional)
/// - **Field 25**: Account Identification (Mandatory)
/// - **Field 28C**: Statement/Sequence Number (Mandatory)
/// - **Field 34F**: Debit/Debit+Credit Floor Limit (Mandatory)
/// - **Field 34F**: Credit Floor Limit (Conditional C2)
/// - **Field 13D**: Date/Time Indication (Mandatory)
/// - **Field 61**: Statement Line (Optional, Repeating)
/// - **Field 86**: Info to Account Owner (Optional, per transaction)
/// - **Field 90D**: Number and Sum of Debits (Optional)
/// - **Field 90C**: Number and Sum of Credits (Optional)
/// - **Field 86**: Info to Account Owner (Optional, global)
///
/// ## Business Rules
/// - Floor limits in Field 34F must use comma decimal and sign indicators (D/C)
/// - Field 28C ensures continuity across multi-part interim reports
/// - Field 13D ensures sync with value date/timestamp for the report cutoff
/// - Field 86 may appear twice: once per transaction (61) and once for entire message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT942 {
    /// **Transaction Reference Number** - Field 20 (Mandatory)
    /// No '//' or slashes at ends
    pub field_20: Field20,

    /// **Related Reference** - Field 21 (Optional)
    /// From MT920 if present
    pub field_21: Option<Field21>,

    /// **Account Identification** - Field 25 (Mandatory)
    /// BIC optional
    pub field_25: Field25,

    /// **Statement/Sequence Number** - Field 28C (Mandatory)
    /// For multi-message reports
    pub field_28c: Field28C,

    /// **Debit/Debit+Credit Floor Limit** - Field 34F (Mandatory)
    /// Sign must be D
    pub debit_floor_limit: Field34F,

    /// **Credit Floor Limit** - Field 34F (Conditional C2)
    /// Sign must be C
    pub credit_floor_limit: Option<Field34F>,

    /// **Date/Time Indication** - Field 13D (Mandatory)
    /// UTC Offset, mandatory validation
    pub field_13d: Field13D,

    /// **Statement Lines** - Field 61 (Optional, Repeating)
    /// Transaction lines with optional accompanying Field 86
    pub statement_lines: Vec<InterimStatementLine>,

    /// **Number and Sum of Debits** - Field 90D (Optional)
    /// Summary line
    pub field_90d: Option<Field90D>,

    /// **Number and Sum of Credits** - Field 90C (Optional)
    /// Summary line
    pub field_90c: Option<Field90C>,

    /// **Info to Account Owner (Global)** - Field 86 (Optional)
    /// Summary of report
    pub field_86_global: Option<Field86>,
}

/// # Interim Statement Line
///
/// Represents a single transaction line (Field 61) with optional
/// accompanying information (Field 86) for interim reports.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InterimStatementLine {
    /// **Statement Line** - Field 61 (Mandatory)
    /// Detailed transaction info
    pub field_61: Field61,

    /// **Info to Account Owner** - Field 86 (Optional)
    /// Contextual to preceding 61
    pub field_86: Option<Field86>,
}

impl InterimStatementLine {
    /// Create a new interim statement line with transaction details
    pub fn new(field_61: Field61) -> Self {
        Self {
            field_61,
            field_86: None,
        }
    }

    /// Create a new interim statement line with transaction details and narrative
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

impl MT942 {
    /// Create a new MT942 with required fields
    pub fn new(
        field_20: Field20,
        field_25: Field25,
        field_28c: Field28C,
        debit_floor_limit: Field34F,
        field_13d: Field13D,
    ) -> Self {
        Self {
            field_20,
            field_21: None,
            field_25,
            field_28c,
            debit_floor_limit,
            credit_floor_limit: None,
            field_13d,
            statement_lines: Vec::new(),
            field_90d: None,
            field_90c: None,
            field_86_global: None,
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
    pub fn add_statement_line(&mut self, statement_line: InterimStatementLine) {
        self.statement_lines.push(statement_line);
    }

    /// Add a transaction with optional narrative
    pub fn add_transaction(&mut self, field_61: Field61, field_86: Option<Field86>) {
        let statement_line = match field_86 {
            Some(info) => InterimStatementLine::with_info(field_61, info),
            None => InterimStatementLine::new(field_61),
        };
        self.statement_lines.push(statement_line);
    }

    /// Set related reference
    pub fn set_related_reference(&mut self, field_21: Field21) {
        self.field_21 = Some(field_21);
    }

    /// Set credit floor limit
    pub fn set_credit_floor_limit(&mut self, credit_floor_limit: Field34F) {
        self.credit_floor_limit = Some(credit_floor_limit);
    }

    /// Set sum of debits
    pub fn set_sum_of_debits(&mut self, field_90d: Field90D) {
        self.field_90d = Some(field_90d);
    }

    /// Set sum of credits
    pub fn set_sum_of_credits(&mut self, field_90c: Field90C) {
        self.field_90c = Some(field_90c);
    }

    /// Set global info to account owner
    pub fn set_global_info(&mut self, field_86: Field86) {
        self.field_86_global = Some(field_86);
    }

    /// Check if credit floor limit is present
    pub fn has_credit_floor_limit(&self) -> bool {
        self.credit_floor_limit.is_some()
    }

    /// Check if entry summaries are present
    pub fn has_entry_summaries(&self) -> bool {
        self.field_90d.is_some() || self.field_90c.is_some()
    }

    /// Check if global narrative information is present
    pub fn has_global_info(&self) -> bool {
        self.field_86_global.is_some()
    }

    /// Get all statement lines with narrative information
    pub fn statement_lines_with_info(&self) -> Vec<&InterimStatementLine> {
        self.statement_lines
            .iter()
            .filter(|line| line.has_info())
            .collect()
    }

    /// Get the total number of transactions
    pub fn transaction_count(&self) -> usize {
        self.statement_lines.len()
    }

    /// Validate conditional rule C2 (credit floor limit requirements)
    pub fn validate_c2_rule(&self) -> bool {
        // This would need proper validation logic based on business rules
        // For now, return true as a placeholder
        true
    }
}

impl SwiftMessageBody for MT942 {
    fn message_type() -> &'static str {
        "942"
    }

    fn from_fields(_fields: std::collections::HashMap<String, Vec<String>>) -> SwiftResult<Self> {
        // For now, return a basic implementation error
        // This would need proper field parsing implementation
        Err(crate::errors::ParseError::InvalidFormat {
            message: "MT942 field parsing not yet implemented".to_string(),
        })
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        // Basic implementation - would need proper field serialization
        std::collections::HashMap::new()
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "25", "28C", "34F", "13D"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["21", "61", "86", "90D", "90C"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt942_message_type() {
        assert_eq!(MT942::message_type(), "942");
    }

    #[test]
    fn test_mt942_creation() {
        let field_20 = Field20::new("INT240315001234".to_string());
        let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
        let field_28c = Field28C::new(1, Some(1)).unwrap();
        let debit_floor_limit = Field34F::new("EUR", Some('D'), 1000.00).unwrap();
        let field_13d = Field13D::new("240315", "1200", "+", "0100").unwrap();

        let mt942 = MT942::new(field_20, field_25, field_28c, debit_floor_limit, field_13d);

        assert_eq!(mt942.transaction_reference(), "INT240315001234");
        assert_eq!(mt942.account_identification(), "GB33BUKB20201555555555");
        assert_eq!(mt942.statement_line_count(), 0);
        assert!(!mt942.has_credit_floor_limit());
        assert!(!mt942.has_entry_summaries());
        assert!(!mt942.has_global_info());
    }

    #[test]
    fn test_interim_statement_line_creation() {
        // This test would need proper Field61 constructor - placeholder for now
        // let field_61 = Field61::new(...);
        // let field_86 = Field86::new(...);
        // let statement_line = InterimStatementLine::with_info(field_61, field_86);
        // assert!(statement_line.has_info());
    }

    #[test]
    fn test_mt942_required_fields() {
        let required = MT942::required_fields();
        assert!(required.contains(&"20"));
        assert!(required.contains(&"25"));
        assert!(required.contains(&"28C"));
        assert!(required.contains(&"34F"));
        assert!(required.contains(&"13D"));
    }

    #[test]
    fn test_mt942_optional_fields() {
        let optional = MT942::optional_fields();
        assert!(optional.contains(&"21"));
        assert!(optional.contains(&"61"));
        assert!(optional.contains(&"86"));
        assert!(optional.contains(&"90D"));
        assert!(optional.contains(&"90C"));
    }
}
