use crate::SwiftMessageBody;
use crate::fields::{
    Field20, Field21, Field23E, Field26T, Field30, Field36, Field50, Field59, Field70, Field71A,
    Field72, Field77B, GenericBicField, GenericCurrencyAmountField,
};
use serde::{Deserialize, Serialize};

/// # MT104 Transaction Details (Sequence B)
///
/// Represents a single transaction within an MT104 batch.
/// This sequence is repeating, allowing multiple transactions per message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT104TransactionDetails {
    /// **Transaction Reference** - Field 21 (Mandatory)
    /// Unique reference for each transaction
    pub field_21: Field21,

    /// **Instruction Code** - Field 23E Seq B (Conditional)
    /// Depends on 23E in Seq A (C1)
    pub field_23e: Option<Field23E>,

    /// **Mandate Reference** - Field 21C (Optional)
    pub field_21c: Option<Field21>,

    /// **Direct Debit Reference** - Field 21D (Optional)
    pub field_21d: Option<Field21>,

    /// **Registration Reference** - Field 21E Seq B (Conditional)
    /// Subject to C3/C12
    pub field_21e: Option<Field21>,

    /// **Currency and Amount** - Field 32B (Mandatory)
    pub field_32b: GenericCurrencyAmountField,

    /// **Instructing Party** - Field 50a Seq B (Conditional)
    /// Options: C, L. Must not appear if in Seq A (C3)
    pub field_50a_instructing: Option<Field50>,

    /// **Creditor** - Field 50a Seq B (Conditional)
    /// Options: A, K. Subject to C2, C4, C12
    pub field_50a_creditor: Option<Field50>,

    /// **Creditor's Bank** - Field 52a Seq B (Conditional)
    /// Options: A, C, D. Subject to C3, C12
    pub field_52a: Option<GenericBicField>,

    /// **Debtor's Bank** - Field 57a (Optional)
    /// Options: A, C, D
    pub field_57: Option<GenericBicField>,

    /// **Debtor** - Field 59a (Mandatory)
    /// Must include account. Options: A/none
    pub field_59: Field59,

    /// **Remittance Information** - Field 70 (Optional)
    /// Codes: INV, IPI, RFB, ROC
    pub field_70: Option<Field70>,

    /// **Transaction Type Code** - Field 26T Seq B (Conditional)
    pub field_26t: Option<Field26T>,

    /// **Regulatory Reporting** - Field 77B Seq B (Conditional)
    pub field_77b: Option<Field77B>,

    /// **Original Ordered Amount** - Field 33B (Optional)
    /// Must differ from 32B
    pub field_33b: Option<GenericCurrencyAmountField>,

    /// **Details of Charges** - Field 71A Seq B (Conditional)
    /// Subject to C3
    pub field_71a: Option<Field71A>,

    /// **Sender's Charges** - Field 71F (Conditional)
    /// Subject to C6, C12
    pub field_71f: Option<GenericCurrencyAmountField>,

    /// **Receiver's Charges** - Field 71G (Conditional)
    /// Subject to C6, C12
    pub field_71g: Option<GenericCurrencyAmountField>,

    /// **Exchange Rate** - Field 36 (Conditional)
    /// Required if 33B present & different from 32B
    pub field_36: Option<Field36>,
}

/// # MT104 Settlement Details (Sequence C)
///
/// Optional settlement consolidation information for the entire batch.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT104SettlementDetails {
    /// **Currency & Settlement Amount** - Field 32B Seq C (Conditional)
    /// Sum or explicit amount
    pub field_32b: Option<GenericCurrencyAmountField>,

    /// **Sum of Amounts** - Field 19 (Conditional)
    /// Required if 32B not total of B-32Bs
    pub field_19: Option<GenericCurrencyAmountField>,

    /// **Sum of Sender's Charges** - Field 71F Seq C (Conditional)
    /// If 71F in B
    pub field_71f: Option<GenericCurrencyAmountField>,

    /// **Sum of Receiver's Charges** - Field 71G Seq C (Conditional)
    /// If 71G in B
    pub field_71g: Option<GenericCurrencyAmountField>,

    /// **Sender's Correspondent** - Field 53a (Optional)
    /// Options: A, B. Reimbursement instruction
    pub field_53: Option<GenericBicField>,
}

/// # MT104: Customer Direct Debit
///
/// ## Overview
/// MT104 is used by financial institutions to send direct debit instructions.
/// It supports batch processing of multiple direct debit transactions with
/// detailed settlement information.
///
/// ## Structure
/// - **Sequence A**: General Information (mandatory, single occurrence)
/// - **Sequence B**: Transaction Details (mandatory, repetitive)  
/// - **Sequence C**: Settlement Details (optional, single occurrence)
///
/// ## Key Features
/// - Multiple transaction support in single message
/// - Flexible creditor/debtor identification
/// - Optional settlement consolidation
/// - Comprehensive regulatory reporting
/// - Charge allocation options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT104 {
    // ================================
    // SEQUENCE A - GENERAL INFORMATION
    // ================================
    /// **Sender's Reference** - Field 20 (Mandatory)
    /// Unique reference assigned by the sender to identify this MT104 message.
    pub field_20: Field20,

    /// **Customer Specified Reference** - Field 21R (Conditional)
    /// Required if Field 23E = HOLD (example condition)
    pub field_21r: Option<Field21>,

    /// **Instruction Code** - Field 23E (Optional)
    /// Values: CHQB, HOLD, INTC, PHOB, PHOI, PHON, REPA, SDVA, TELB, TELE, TELI
    pub field_23e: Option<Field23E>,

    /// **Registration Reference** - Field 21E (Conditional)
    /// Subject to C3/C12 conditions
    pub field_21e: Option<Field21>,

    /// **Requested Execution Date** - Field 30 (Mandatory)
    /// Format: YYMMDD
    pub field_30: Field30,

    /// **Sending Institution** - Field 51A (Optional)
    /// Only for FileAct
    pub field_51a: Option<GenericBicField>,

    /// **Instructing Party** - Field 50a Seq A (Conditional)
    /// Options: C, L. Conditional C3 (if not present in any Seq B)
    pub field_50a_instructing: Option<Field50>,

    /// **Creditor** - Field 50a Seq A (Conditional)
    /// Options: A, K. Subject to C2, C4, C12
    pub field_50a_creditor: Option<Field50>,

    /// **Creditor's Bank** - Field 52a Seq A (Conditional)
    /// Options: A, C, D. Subject to C3, C12
    pub field_52a: Option<GenericBicField>,

    /// **Transaction Type Code** - Field 26T Seq A (Conditional)
    /// Subject to C3
    pub field_26t: Option<Field26T>,

    /// **Regulatory Reporting** - Field 77B Seq A (Conditional)
    /// Subject to C3
    pub field_77b: Option<Field77B>,

    /// **Details of Charges** - Field 71A Seq A (Conditional)
    /// Values: BEN, OUR, SHA
    pub field_71a: Option<Field71A>,

    /// **Sender to Receiver Information** - Field 72 (Conditional)
    /// Subject to C5
    pub field_72: Option<Field72>,

    // ================================
    // SEQUENCE B - TRANSACTION DETAILS (REPEATING)
    // ================================
    /// **Transaction Details** - Sequence B (Mandatory, Repetitive)
    /// Each entry represents one direct debit transaction
    pub transactions: Vec<MT104TransactionDetails>,

    // ================================
    // SEQUENCE C - SETTLEMENT DETAILS (OPTIONAL)
    // ================================
    /// **Settlement Details** - Sequence C (Optional)
    /// Consolidated settlement information for the entire batch
    pub settlement_details: Option<MT104SettlementDetails>,
}

impl MT104TransactionDetails {
    /// Create a new transaction details with minimal required fields
    pub fn new(
        field_21: Field21,
        field_32b: GenericCurrencyAmountField,
        field_59: Field59,
    ) -> Self {
        Self {
            field_21,
            field_23e: None,
            field_21c: None,
            field_21d: None,
            field_21e: None,
            field_32b,
            field_50a_instructing: None,
            field_50a_creditor: None,
            field_52a: None,
            field_57: None,
            field_59,
            field_70: None,
            field_26t: None,
            field_77b: None,
            field_33b: None,
            field_71a: None,
            field_71f: None,
            field_71g: None,
            field_36: None,
        }
    }

    /// Get the transaction reference
    pub fn transaction_reference(&self) -> &str {
        self.field_21.related_reference()
    }

    /// Get the transaction currency
    pub fn currency(&self) -> &str {
        self.field_32b.currency()
    }

    /// Get the transaction amount
    pub fn amount(&self) -> f64 {
        self.field_32b.amount()
    }
}

impl MT104SettlementDetails {
    /// Create new settlement details
    pub fn new() -> Self {
        Self {
            field_32b: None,
            field_19: None,
            field_71f: None,
            field_71g: None,
            field_53: None,
        }
    }
}

impl Default for MT104SettlementDetails {
    fn default() -> Self {
        Self::new()
    }
}

impl MT104 {
    /// Create a new MT104 with minimal required fields
    pub fn new(
        field_20: Field20,
        field_30: Field30,
        transactions: Vec<MT104TransactionDetails>,
    ) -> Self {
        Self {
            field_20,
            field_21r: None,
            field_23e: None,
            field_21e: None,
            field_30,
            field_51a: None,
            field_50a_instructing: None,
            field_50a_creditor: None,
            field_52a: None,
            field_26t: None,
            field_77b: None,
            field_71a: None,
            field_72: None,
            transactions,
            settlement_details: None,
        }
    }

    /// Create a new MT104 with a single transaction
    pub fn new_single_transaction(
        field_20: Field20,
        field_30: Field30,
        field_21: Field21,
        field_32b: GenericCurrencyAmountField,
        field_59: Field59,
    ) -> Self {
        let transaction = MT104TransactionDetails::new(field_21, field_32b, field_59);
        Self::new(field_20, field_30, vec![transaction])
    }

    /// Get the sender's reference
    pub fn senders_reference(&self) -> &str {
        self.field_20.transaction_reference()
    }

    /// Get the number of transactions in this batch
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Get the execution date
    pub fn execution_date(&self) -> chrono::NaiveDate {
        self.field_30.execution_date()
    }

    /// Get total amount across all transactions
    pub fn total_amount(&self) -> f64 {
        self.transactions.iter().map(|t| t.amount()).sum()
    }

    /// Get all unique currencies used in transactions
    pub fn currencies(&self) -> Vec<&str> {
        let mut currencies: Vec<&str> = self.transactions.iter().map(|t| t.currency()).collect();
        currencies.sort_unstable();
        currencies.dedup();
        currencies
    }

    /// Check if 21R is required (when 23E = HOLD - example condition)
    pub fn validate_c1(&self) -> bool {
        if let Some(field_23e) = &self.field_23e {
            if field_23e.code() == "HOLD" {
                self.field_21r.is_some()
            } else {
                true
            }
        } else {
            true
        }
    }

    /// Validate that instructing party appears in exactly one sequence
    pub fn validate_c3(&self) -> bool {
        let seq_a_present = self.field_50a_instructing.is_some();
        let any_seq_b_present = self
            .transactions
            .iter()
            .any(|t| t.field_50a_instructing.is_some());

        // Must be present in exactly one sequence (not both, not neither for this rule)
        seq_a_present ^ any_seq_b_present
    }

    /// Validate all conditional rules
    pub fn validate_conditional_rules(&self) -> bool {
        self.validate_c1() && self.validate_c3()
    }

    /// Check if message has settlement details (Sequence C)
    pub fn has_settlement_details(&self) -> bool {
        self.settlement_details.is_some()
    }

    /// Add a transaction to the batch
    pub fn add_transaction(&mut self, transaction: MT104TransactionDetails) {
        self.transactions.push(transaction);
    }

    /// Set settlement details
    pub fn set_settlement_details(&mut self, settlement_details: MT104SettlementDetails) {
        self.settlement_details = Some(settlement_details);
    }
}

impl SwiftMessageBody for MT104 {
    fn message_type() -> &'static str {
        "104"
    }

    fn from_fields(
        _fields: std::collections::HashMap<String, Vec<String>>,
    ) -> crate::SwiftResult<Self> {
        // Placeholder implementation
        todo!("MT104 field parsing not yet implemented")
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        // Placeholder implementation
        todo!("MT104 field serialization not yet implemented")
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "30", "21", "32B", "59"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec![
            "21R", "23E", "21E", "51A", "50A", "50K", "52A", "52C", "52D", "26T", "77B", "71A",
            "72", "23E", "21C", "21D", "57A", "57C", "57D", "70", "33B", "71F", "71G", "36", "32B",
            "19", "53A", "53B",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt104_creation() {
        let field_20 = Field20::new("DD001".to_string());
        let field_30 = Field30::new("240315");
        let field_21 = Field21::new("TXN001".to_string());
        let field_32b = GenericCurrencyAmountField::new("EUR", 500.00).unwrap();
        let field_59 = Field59::A(
            GenericBicField::new(None, Some("87654321".to_string()), "DEUTDEFF").unwrap(),
        );

        let mt104 =
            MT104::new_single_transaction(field_20, field_30, field_21, field_32b, field_59);

        assert_eq!(mt104.senders_reference(), "DD001");
        assert_eq!(mt104.transaction_count(), 1);
        assert_eq!(mt104.transactions[0].transaction_reference(), "TXN001");
        assert_eq!(mt104.transactions[0].currency(), "EUR");
        assert_eq!(mt104.transactions[0].amount(), 500.00);
        assert_eq!(mt104.total_amount(), 500.00);
    }

    #[test]
    fn test_mt104_message_type() {
        assert_eq!(MT104::message_type(), "104");
    }

    #[test]
    fn test_mt104_conditional_rules() {
        let field_20 = Field20::new("DD001".to_string());
        let field_30 = Field30::new("240315");
        let field_21 = Field21::new("TXN001".to_string());
        let field_32b = GenericCurrencyAmountField::new("EUR", 500.00).unwrap();
        let field_59 = Field59::A(
            GenericBicField::new(None, Some("87654321".to_string()), "DEUTDEFF").unwrap(),
        );

        let mut mt104 =
            MT104::new_single_transaction(field_20, field_30, field_21, field_32b, field_59);

        // Test C1: If 23E = HOLD, then 21R required (example condition)
        mt104.field_23e = Some(Field23E::new("HOLD".to_string(), None).unwrap());
        assert!(!mt104.validate_c1()); // Should fail without 21R

        mt104.field_21r = Some(Field21::new("REF123".to_string()));
        assert!(mt104.validate_c1()); // Should pass with 21R
    }

    #[test]
    fn test_mt104_settlement_details() {
        let field_20 = Field20::new("DD001".to_string());
        let field_30 = Field30::new("240315");
        let field_21 = Field21::new("TXN001".to_string());
        let field_32b = GenericCurrencyAmountField::new("EUR", 500.00).unwrap();
        let field_59 = Field59::A(
            GenericBicField::new(None, Some("87654321".to_string()), "DEUTDEFF").unwrap(),
        );

        let mut mt104 =
            MT104::new_single_transaction(field_20, field_30, field_21, field_32b, field_59);

        assert!(!mt104.has_settlement_details());

        let mut settlement_details = MT104SettlementDetails::new();
        settlement_details.field_32b =
            Some(GenericCurrencyAmountField::new("EUR", 500.00).unwrap());
        mt104.set_settlement_details(settlement_details);

        assert!(mt104.has_settlement_details());
    }

    #[test]
    fn test_mt104_multiple_transactions() {
        let field_20 = Field20::new("DD001".to_string());
        let field_30 = Field30::new("240315");

        // Create multiple transactions
        let transaction1 = MT104TransactionDetails::new(
            Field21::new("TXN001".to_string()),
            GenericCurrencyAmountField::new("EUR", 500.00).unwrap(),
            Field59::A(
                GenericBicField::new(None, Some("12345678".to_string()), "DEUTDEFF").unwrap(),
            ),
        );

        let transaction2 = MT104TransactionDetails::new(
            Field21::new("TXN002".to_string()),
            GenericCurrencyAmountField::new("EUR", 250.00).unwrap(),
            Field59::A(
                GenericBicField::new(None, Some("87654321".to_string()), "CHASUS33").unwrap(),
            ),
        );

        let transaction3 = MT104TransactionDetails::new(
            Field21::new("TXN003".to_string()),
            GenericCurrencyAmountField::new("USD", 1000.00).unwrap(),
            Field59::A(
                GenericBicField::new(None, Some("11111111".to_string()), "BARCGB22").unwrap(),
            ),
        );

        let transactions = vec![transaction1, transaction2, transaction3];
        let mt104 = MT104::new(field_20, field_30, transactions);

        assert_eq!(mt104.senders_reference(), "DD001");
        assert_eq!(mt104.transaction_count(), 3);
        assert_eq!(mt104.total_amount(), 1750.00); // 500 + 250 + 1000

        let currencies = mt104.currencies();
        assert_eq!(currencies, vec!["EUR", "USD"]);

        // Check individual transactions
        assert_eq!(mt104.transactions[0].transaction_reference(), "TXN001");
        assert_eq!(mt104.transactions[1].currency(), "EUR");
        assert_eq!(mt104.transactions[2].amount(), 1000.00);
    }
}
