use crate::fields::{
    Field20, Field25, Field28C, Field60A, Field60F, Field61, Field62A, Field62F, Field64,
};
use crate::{SwiftMessageBody, SwiftResult};
use serde::{Deserialize, Serialize};

/// # MT950: Customer Statement Message
///
/// ## Overview
/// MT950 is used by financial institutions to send customer account statements
/// containing detailed transaction information. This message provides comprehensive
/// account activity reporting with structured transaction details.
///
/// ## Message Structure
/// - **Field 20**: Transaction Reference Number (Mandatory)
/// - **Field 25**: Account Identification (Mandatory)
/// - **Field 28C**: Statement/Sequence Number (Mandatory)
/// - **Field 60a**: Opening Balance (Mandatory, Option F or M)
/// - **Field 61**: Statement Line (Optional, Repeating)
/// - **Field 62a**: Closing Balance (Mandatory, Option F or M)
/// - **Field 64**: Closing Available Balance (Optional)
///
/// ## Business Rules
/// - D/C mark, currency, and amount in 60a must match 62a of prior message
/// - Field 62a must be Option F for final closing balance or M for intermediate
/// - When multiple MT950s are sent per day, use 28C sequencing and 60M/62M appropriately
/// - Field 64 provides available balance (not booked); optional
/// - Field 61 must follow defined structure and may repeat
/// - Comma is mandatory for decimal in amount fields
/// - Field 61 Transaction Types use structured codes like CHG, CHK, TRF, FEX, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT950 {
    /// **Transaction Reference Number** - Field 20 (Mandatory)
    /// Must not start/end with '/', or contain '//'
    pub field_20: Field20,

    /// **Account Identification** - Field 25 (Mandatory)
    /// Account number, may include BIC
    pub field_25: Field25,

    /// **Statement/Sequence Number** - Field 28C (Mandatory)
    /// Allows multiple parts per statement
    pub field_28c: Field28C,

    /// **Opening Balance** - Field 60a (Mandatory)
    /// D/C mark + date + currency + amount (Option F or M)
    pub opening_balance: OpeningBalance,

    /// **Statement Lines** - Field 61 (Optional, Repeating)
    /// Detailed transaction lines
    pub statement_lines: Vec<Field61>,

    /// **Closing Balance** - Field 62a (Mandatory)
    /// D/C mark + date + currency + amount (Option F or M)
    pub closing_balance: ClosingBalance,

    /// **Closing Available Balance** - Field 64 (Optional)
    /// Indicates available funds
    pub field_64: Option<Field64>,
}

/// # Opening Balance
///
/// Represents the opening balance which can be either booked (F) or intermediate (M).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OpeningBalance {
    /// **Field 60F**: Booked opening balance (final)
    Booked(Field60F),
    /// **Field 60A**: Intermediate opening balance (for multi-part statements)
    Intermediate(Field60A),
}

/// # Closing Balance
///
/// Represents the closing balance which can be either booked (F) or intermediate (M).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClosingBalance {
    /// **Field 62F**: Booked closing balance (final)
    Booked(Field62F),
    /// **Field 62A**: Intermediate closing balance (for multi-part statements)
    Intermediate(Field62A),
}

impl OpeningBalance {
    /// Create a booked opening balance
    pub fn booked(field_60f: Field60F) -> Self {
        Self::Booked(field_60f)
    }

    /// Create an intermediate opening balance
    pub fn intermediate(field_60a: Field60A) -> Self {
        Self::Intermediate(field_60a)
    }

    /// Check if this is a booked balance
    pub fn is_booked(&self) -> bool {
        matches!(self, Self::Booked(_))
    }

    /// Check if this is an intermediate balance
    pub fn is_intermediate(&self) -> bool {
        matches!(self, Self::Intermediate(_))
    }

    /// Get the balance type as string
    pub fn balance_type(&self) -> &'static str {
        match self {
            Self::Booked(_) => "Booked",
            Self::Intermediate(_) => "Intermediate",
        }
    }
}

impl ClosingBalance {
    /// Create a booked closing balance
    pub fn booked(field_62f: Field62F) -> Self {
        Self::Booked(field_62f)
    }

    /// Create an intermediate closing balance
    pub fn intermediate(field_62a: Field62A) -> Self {
        Self::Intermediate(field_62a)
    }

    /// Check if this is a booked balance
    pub fn is_booked(&self) -> bool {
        matches!(self, Self::Booked(_))
    }

    /// Check if this is an intermediate balance
    pub fn is_intermediate(&self) -> bool {
        matches!(self, Self::Intermediate(_))
    }

    /// Get the balance type as string
    pub fn balance_type(&self) -> &'static str {
        match self {
            Self::Booked(_) => "Booked",
            Self::Intermediate(_) => "Intermediate",
        }
    }
}

impl MT950 {
    /// Create a new MT950 with booked balances
    pub fn new_with_booked_balances(
        field_20: Field20,
        field_25: Field25,
        field_28c: Field28C,
        field_60f: Field60F,
        field_62f: Field62F,
    ) -> Self {
        Self {
            field_20,
            field_25,
            field_28c,
            opening_balance: OpeningBalance::booked(field_60f),
            statement_lines: Vec::new(),
            closing_balance: ClosingBalance::booked(field_62f),
            field_64: None,
        }
    }

    /// Create a new MT950 with intermediate balances
    pub fn new_with_intermediate_balances(
        field_20: Field20,
        field_25: Field25,
        field_28c: Field28C,
        field_60a: Field60A,
        field_62a: Field62A,
    ) -> Self {
        Self {
            field_20,
            field_25,
            field_28c,
            opening_balance: OpeningBalance::intermediate(field_60a),
            statement_lines: Vec::new(),
            closing_balance: ClosingBalance::intermediate(field_62a),
            field_64: None,
        }
    }

    /// Create a new MT950 with mixed balance types
    pub fn new(
        field_20: Field20,
        field_25: Field25,
        field_28c: Field28C,
        opening_balance: OpeningBalance,
        closing_balance: ClosingBalance,
    ) -> Self {
        Self {
            field_20,
            field_25,
            field_28c,
            opening_balance,
            statement_lines: Vec::new(),
            closing_balance,
            field_64: None,
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
    pub fn add_statement_line(&mut self, field_61: Field61) {
        self.statement_lines.push(field_61);
    }

    /// Set closing available balance
    pub fn set_closing_available_balance(&mut self, field_64: Field64) {
        self.field_64 = Some(field_64);
    }

    /// Check if this is a final statement (both balances are booked)
    pub fn is_final_statement(&self) -> bool {
        self.opening_balance.is_booked() && self.closing_balance.is_booked()
    }

    /// Check if this is an intermediate statement
    pub fn is_intermediate_statement(&self) -> bool {
        self.opening_balance.is_intermediate() || self.closing_balance.is_intermediate()
    }

    /// Check if available balance information is present
    pub fn has_available_balance(&self) -> bool {
        self.field_64.is_some()
    }

    /// Get the total number of transactions
    pub fn transaction_count(&self) -> usize {
        self.statement_lines.len()
    }

    /// Validate balance consistency (currencies should match)
    pub fn validate_balance_consistency(&self) -> bool {
        // This would need proper currency extraction from balance fields
        // For now, return true as a placeholder
        true
    }

    /// Get opening balance type
    pub fn opening_balance_type(&self) -> &'static str {
        self.opening_balance.balance_type()
    }

    /// Get closing balance type
    pub fn closing_balance_type(&self) -> &'static str {
        self.closing_balance.balance_type()
    }
}

impl SwiftMessageBody for MT950 {
    fn message_type() -> &'static str {
        "950"
    }

    fn from_fields(_fields: std::collections::HashMap<String, Vec<String>>) -> SwiftResult<Self> {
        // For now, return a basic implementation error
        // This would need proper field parsing implementation
        Err(crate::errors::ParseError::InvalidFormat {
            message: "MT950 field parsing not yet implemented".to_string(),
        })
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        // Basic implementation - would need proper field serialization
        std::collections::HashMap::new()
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "25", "28C", "60a", "62a"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["61", "64"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt950_message_type() {
        assert_eq!(MT950::message_type(), "950");
    }

    #[test]
    fn test_mt950_creation_with_booked_balances() {
        let field_20 = Field20::new("STMT240315001234".to_string());
        let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
        let field_28c = Field28C::new(1, Some(1)).unwrap();
        let field_60f = Field60F::new('C', "240315", "EUR", 1000.00).unwrap();
        let field_62f = Field62F::new('C', "240315", "EUR", 1500.00).unwrap();

        let mt950 =
            MT950::new_with_booked_balances(field_20, field_25, field_28c, field_60f, field_62f);

        assert_eq!(mt950.transaction_reference(), "STMT240315001234");
        assert_eq!(mt950.account_identification(), "GB33BUKB20201555555555");
        assert_eq!(mt950.statement_line_count(), 0);
        assert!(mt950.is_final_statement());
        assert!(!mt950.is_intermediate_statement());
        assert!(!mt950.has_available_balance());
    }

    #[test]
    fn test_mt950_creation_with_intermediate_balances() {
        let field_20 = Field20::new("STMT240315001235".to_string());
        let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
        let field_28c = Field28C::new(1, Some(2)).unwrap();
        let field_60a = Field60A::new('C', "240315", "EUR", 1000.00).unwrap();
        let field_62a = Field62A::new('C', "240315", "EUR", 1200.00).unwrap();

        let mt950 = MT950::new_with_intermediate_balances(
            field_20, field_25, field_28c, field_60a, field_62a,
        );

        assert_eq!(mt950.transaction_reference(), "STMT240315001235");
        assert!(!mt950.is_final_statement());
        assert!(mt950.is_intermediate_statement());
        assert_eq!(mt950.opening_balance_type(), "Intermediate");
        assert_eq!(mt950.closing_balance_type(), "Intermediate");
    }

    #[test]
    fn test_opening_balance_types() {
        let field_60f = Field60F::new('C', "240315", "EUR", 1000.00).unwrap();
        let field_60a = Field60A::new('C', "240315", "EUR", 1000.00).unwrap();

        let booked = OpeningBalance::booked(field_60f);
        let intermediate = OpeningBalance::intermediate(field_60a);

        assert!(booked.is_booked());
        assert!(!booked.is_intermediate());
        assert_eq!(booked.balance_type(), "Booked");

        assert!(!intermediate.is_booked());
        assert!(intermediate.is_intermediate());
        assert_eq!(intermediate.balance_type(), "Intermediate");
    }

    #[test]
    fn test_closing_balance_types() {
        let field_62f = Field62F::new('C', "240315", "EUR", 1500.00).unwrap();
        let field_62a = Field62A::new('C', "240315", "EUR", 1500.00).unwrap();

        let booked = ClosingBalance::booked(field_62f);
        let intermediate = ClosingBalance::intermediate(field_62a);

        assert!(booked.is_booked());
        assert!(!booked.is_intermediate());
        assert_eq!(booked.balance_type(), "Booked");

        assert!(!intermediate.is_booked());
        assert!(intermediate.is_intermediate());
        assert_eq!(intermediate.balance_type(), "Intermediate");
    }

    #[test]
    fn test_mt950_required_fields() {
        let required = MT950::required_fields();
        assert!(required.contains(&"20"));
        assert!(required.contains(&"25"));
        assert!(required.contains(&"28C"));
        assert!(required.contains(&"60a"));
        assert!(required.contains(&"62a"));
    }

    #[test]
    fn test_mt950_optional_fields() {
        let optional = MT950::optional_fields();
        assert!(optional.contains(&"61"));
        assert!(optional.contains(&"64"));
    }
}
