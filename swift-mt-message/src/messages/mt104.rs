// This is a demonstration of how MT104 could be redesigned to work with field attributes
//
// The key insight is that the #[field("XX")] attribute system expects individual SWIFT fields,
// not complex nested structures. Here's how we can solve the MT104 problem:

use crate::fields::{
    Field20, Field21, Field23B, Field23E, Field26T, Field30, Field36, Field50, Field59, Field70,
    Field71A, Field72, Field77B, GenericBicField, GenericCurrencyAmountField,
};
use crate::{SwiftMessage, swift_serde};
use serde::{Deserialize, Serialize};

/// # MT104: Customer Direct Debit (Redesigned with Field Attributes)
///
/// ## Overview
/// MT104 is used by financial institutions to send direct debit instructions.
/// It supports batch processing of multiple direct debit transactions with
/// detailed settlement information.
///
/// This implementation uses a flattened structure with individual Vec fields
/// instead of custom sub-structures, enabling full compatibility with the
/// `#[field("XX")]` attribute system for automatic parsing and serialization.
///
/// ## Structure
/// - **Sequence A**: General Information (mandatory, single occurrence)
/// - **Sequence B**: Transaction Details (mandatory, repetitive) - Flattened to Vec fields
/// - **Sequence C**: Settlement Details (optional, single occurrence) - Individual fields
///
/// ## Key Features
/// - Multiple transaction support in single message
/// - Flexible creditor/debtor identification
/// - Optional settlement consolidation
/// - Comprehensive regulatory reporting
/// - Charge allocation options
/// - Full field attribute compatibility
/// - Automatic SwiftMessageBody generation
#[swift_serde]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[swift_message(mt = "104")]
pub struct MT104 {
    // ================================
    // SEQUENCE A - GENERAL INFORMATION
    // ================================
    /// **Sender's Reference** - Field 20 (Mandatory)
    /// Unique reference assigned by the sender to identify this MT104 message.
    #[field("20")]
    pub field_20: Field20,

    /// **Customer Specified Reference** - Field 21R (Conditional)
    /// Required if Field 23E = HOLD (example condition)
    #[field("21R")]
    pub field_21r: Option<Field21>,

    /// **Instruction Code** - Field 23E (Optional)
    /// Values: CHQB, HOLD, INTC, PHOB, PHOI, PHON, REPA, SDVA, TELB, TELE, TELI
    #[field("23E")]
    pub field_23e: Option<Field23E>,

    /// **Registration Reference** - Field 21E (Conditional)
    /// Subject to C3/C12 conditions
    #[field("21E")]
    pub field_21e: Option<Field21>,

    /// **Requested Execution Date** - Field 30 (Mandatory)
    /// Format: YYMMDD
    #[field("30")]
    pub field_30: Field30,

    /// **Sending Institution** - Field 51A (Optional)
    /// Only for FileAct
    #[field("51A")]
    pub field_51a: Option<GenericBicField>,

    /// **Instructing Party** - Field 50a Seq A (Conditional)
    /// Options: C, L. Conditional C3 (if not present in any Seq B)
    #[field("50A_INSTRUCTING")]
    pub field_50a_instructing: Option<Field50>,

    /// **Creditor** - Field 50a Seq A (Conditional)
    /// Options: A, K. Subject to C2, C4, C12
    #[field("50A_CREDITOR")]
    pub field_50a_creditor: Option<Field50>,

    /// **Creditor's Bank** - Field 52a Seq A (Conditional)
    /// Options: A, C, D. Subject to C3, C12
    #[field("52A")]
    pub field_52a: Option<GenericBicField>,

    /// **Transaction Type Code** - Field 26T Seq A (Conditional)
    /// Subject to C3
    #[field("26T")]
    pub field_26t: Option<Field26T>,

    /// **Regulatory Reporting** - Field 77B Seq A (Conditional)
    /// Subject to C3
    #[field("77B")]
    pub field_77b: Option<Field77B>,

    /// **Details of Charges** - Field 71A Seq A (Conditional)
    /// Values: BEN, OUR, SHA
    #[field("71A")]
    pub field_71a: Option<Field71A>,

    /// **Sender to Receiver Information** - Field 72 (Conditional)
    /// Subject to C5
    #[field("72")]
    pub field_72: Option<Field72>,

    // ================================
    // SEQUENCE B - TRANSACTION DETAILS (REPEATING) - FLATTENED
    // ================================
    /// **Transaction Reference** - Field 21 Seq B (Mandatory, Repeating)
    /// Each element represents one transaction's unique reference
    #[field("21_TXN")]
    pub field_21_txn: Vec<Field21>,

    /// **Instruction Code** - Field 23E Seq B (Conditional, Repeating)
    /// Per-transaction instruction codes, depends on 23E in Seq A (C1)
    #[field("23E_TXN")]
    pub field_23e_txn: Option<Vec<Field23E>>,

    /// **Mandate Reference** - Field 21C (Optional, Repeating)
    /// Direct debit mandate reference for each transaction
    #[field("21C")]
    pub field_21c: Option<Vec<Field21>>,

    /// **Direct Debit Reference** - Field 21D (Optional, Repeating)
    /// Direct debit specific reference for each transaction
    #[field("21D")]
    pub field_21d: Option<Vec<Field21>>,

    /// **Registration Reference** - Field 21E Seq B (Conditional, Repeating)
    /// Subject to C3/C12 conditions, per transaction
    #[field("21E_TXN")]
    pub field_21e_txn: Option<Vec<Field21>>,

    /// **Currency and Amount** - Field 32B (Mandatory, Repeating)
    /// Each element represents one transaction's currency and amount
    #[field("32B")]
    pub field_32b: Vec<GenericCurrencyAmountField>,

    /// **Instructing Party** - Field 50a Seq B (Conditional, Repeating)
    /// Options: C, L. Must not appear if in Seq A (C3)
    #[field("50A_INSTRUCTING_TXN")]
    pub field_50a_instructing_txn: Option<Vec<Field50>>,

    /// **Creditor** - Field 50a Seq B (Conditional, Repeating)
    /// Options: A, K. Subject to C2, C4, C12, per transaction
    #[field("50A_CREDITOR_TXN")]
    pub field_50a_creditor_txn: Option<Vec<Field50>>,

    /// **Creditor's Bank** - Field 52a Seq B (Conditional, Repeating)
    /// Options: A, C, D. Subject to C3, C12, per transaction
    #[field("52A_TXN")]
    pub field_52a_txn: Option<Vec<GenericBicField>>,

    /// **Debtor's Bank** - Field 57a (Optional, Repeating)
    /// Options: A, C, D, per transaction
    #[field("57A")]
    pub field_57a: Option<Vec<GenericBicField>>,

    /// **Debtor** - Field 59a (Mandatory, Repeating)
    /// Must include account. Options: A/none, each element represents one transaction's debtor
    #[field("59A")]
    pub field_59a: Vec<Field59>,

    /// **Remittance Information** - Field 70 (Optional, Repeating)
    /// Codes: INV, IPI, RFB, ROC, per transaction
    #[field("70_TXN")]
    pub field_70_txn: Option<Vec<Field70>>,

    /// **Transaction Type Code** - Field 26T Seq B (Conditional, Repeating)
    /// Per transaction type codes
    #[field("26T_TXN")]
    pub field_26t_txn: Option<Vec<Field26T>>,

    /// **Regulatory Reporting** - Field 77B Seq B (Conditional, Repeating)
    /// Per transaction regulatory reporting
    #[field("77B_TXN")]
    pub field_77b_txn: Option<Vec<Field77B>>,

    /// **Original Ordered Amount** - Field 33B (Optional, Repeating)
    /// Must differ from 32B, per transaction
    #[field("33B")]
    pub field_33b: Option<Vec<GenericCurrencyAmountField>>,

    /// **Details of Charges** - Field 71A Seq B (Conditional, Repeating)
    /// Subject to C3, per transaction
    #[field("71A_TXN")]
    pub field_71a_txn: Option<Vec<Field71A>>,

    /// **Sender's Charges** - Field 71F (Conditional, Repeating)
    /// Subject to C6, C12, per transaction
    #[field("71F")]
    pub field_71f: Option<Vec<GenericCurrencyAmountField>>,

    /// **Receiver's Charges** - Field 71G (Conditional, Repeating)
    /// Subject to C6, C12, per transaction
    #[field("71G")]
    pub field_71g: Option<Vec<GenericCurrencyAmountField>>,

    /// **Exchange Rate** - Field 36 (Conditional, Repeating)
    /// Required if 33B present & different from 32B, per transaction
    #[field("36")]
    pub field_36: Option<Vec<Field36>>,

    // ================================
    // SEQUENCE C - SETTLEMENT DETAILS (OPTIONAL) - INDIVIDUAL FIELDS
    // ================================
    /// **Instruction Code** - Field 23E Seq C (Conditional)
    /// C2, C10, C11 settlement instruction codes
    #[field("23E_SETTLEMENT")]
    pub field_23e_settlement: Option<Field23E>,

    /// **Instruction Code** - Field 23B (Conditional)
    /// Settlement instruction code
    #[field("23B")]
    pub field_23b: Option<Field23B>,

    /// **Recipient Bank** - Field 57a (Conditional)
    /// Options: A, C, D. Subject to C3
    #[field("57A_SETTLEMENT")]
    pub field_57a_settlement: Option<GenericBicField>,

    /// **Correspondent Bank** - Field 58a (Conditional)
    /// Options: A, D. Subject to C3
    #[field("58A")]
    pub field_58a: Option<GenericBicField>,

    /// **Receiver's Correspondent** - Field 53a (Conditional)
    /// Options: A, D. Subject to C3
    #[field("53A")]
    pub field_53a: Option<GenericBicField>,

    /// **Sender's Correspondent** - Field 54a (Conditional)
    /// Options: A, D. Subject to C3
    #[field("54A")]
    pub field_54a: Option<GenericBicField>,

    /// **Account with Institution** - Field 56a (Conditional)
    /// Options: A, C, D. Subject to C3
    #[field("56A")]
    pub field_56a: Option<GenericBicField>,

    /// **Ordering Institution** - Field 52a (Conditional)
    /// Options: A, D. Subject to C3
    #[field("52A_SETTLEMENT")]
    pub field_52a_settlement: Option<GenericBicField>,

    /// **Details of Charges** - Field 71A Seq C (Conditional)
    /// Subject to C3
    #[field("71A_SETTLEMENT")]
    pub field_71a_settlement: Option<Field71A>,

    /// **Sender's Charges** - Field 71F (Conditional)
    /// Subject to C5, C6
    #[field("71F_SETTLEMENT")]
    pub field_71f_settlement: Option<GenericCurrencyAmountField>,

    /// **Receiver's Charges** - Field 71G (Conditional)
    /// Subject to C5, C6
    #[field("71G_SETTLEMENT")]
    pub field_71g_settlement: Option<GenericCurrencyAmountField>,

    /// **Sender to Receiver Information** - Field 72 (Optional)
    /// Settlement-specific information
    #[field("72_SETTLEMENT")]
    pub field_72_settlement: Option<Field72>,
}

impl MT104 {
    /// Create a new MT104 with minimal required fields
    pub fn new(
        field_20: Field20,
        field_30: Field30,
        transaction_references: Vec<Field21>,
        transaction_amounts: Vec<GenericCurrencyAmountField>,
        transaction_debtors: Vec<Field59>,
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
            field_21_txn: transaction_references,
            field_23e_txn: None,
            field_21c: None,
            field_21d: None,
            field_21e_txn: None,
            field_32b: transaction_amounts,
            field_50a_instructing_txn: None,
            field_50a_creditor_txn: None,
            field_52a_txn: None,
            field_57a: None,
            field_59a: transaction_debtors,
            field_70_txn: None,
            field_26t_txn: None,
            field_77b_txn: None,
            field_33b: None,
            field_71a_txn: None,
            field_71f: None,
            field_71g: None,
            field_36: None,
            field_23e_settlement: None,
            field_23b: None,
            field_57a_settlement: None,
            field_58a: None,
            field_53a: None,
            field_54a: None,
            field_56a: None,
            field_52a_settlement: None,
            field_71a_settlement: None,
            field_71f_settlement: None,
            field_71g_settlement: None,
            field_72_settlement: None,
        }
    }

    /// Create a single transaction MT104
    pub fn new_single_transaction(
        field_20: Field20,
        field_30: Field30,
        field_21: Field21,
        field_32b: GenericCurrencyAmountField,
        field_59: Field59,
    ) -> Self {
        Self::new(
            field_20,
            field_30,
            vec![field_21],
            vec![field_32b],
            vec![field_59],
        )
    }

    /// Create new MT104 with required fields (backward compatibility)
    pub fn new_simple(
        sender_ref: String,
        execution_date: &str,
        transactions: Vec<(String, &str, f64, Field59)>, // (ref, currency, amount, debtor)
    ) -> Self {
        let field_20 = Field20::new(sender_ref);
        let field_30 = Field30::new(execution_date);

        let mut field_21_txn = Vec::new();
        let mut field_32b = Vec::new();
        let mut field_59a = Vec::new();

        for (ref_str, currency, amount, debtor) in transactions {
            field_21_txn.push(Field21::new(ref_str));
            field_32b.push(GenericCurrencyAmountField::new(currency, amount).unwrap());
            field_59a.push(debtor);
        }

        Self::new(field_20, field_30, field_21_txn, field_32b, field_59a)
    }

    /// Get the sender's reference
    pub fn senders_reference(&self) -> &str {
        self.field_20.transaction_reference()
    }

    /// Get the number of transactions in this batch
    pub fn transaction_count(&self) -> usize {
        self.field_21_txn.len()
    }

    /// Get total amount across all transactions
    pub fn total_amount(&self) -> f64 {
        self.field_32b.iter().map(|amount| amount.amount()).sum()
    }

    /// Get all unique currencies used in transactions
    pub fn currencies(&self) -> Vec<&str> {
        let mut currencies: Vec<&str> = self
            .field_32b
            .iter()
            .map(|amount| amount.currency())
            .collect();
        currencies.sort_unstable();
        currencies.dedup();
        currencies
    }

    /// Get transaction reference by index
    pub fn transaction_reference(&self, index: usize) -> Option<&str> {
        self.field_21_txn.get(index).map(|f| f.related_reference())
    }

    /// Get transaction amount by index
    pub fn transaction_amount(&self, index: usize) -> Option<f64> {
        self.field_32b.get(index).map(|f| f.amount())
    }

    /// Get transaction currency by index
    pub fn transaction_currency(&self, index: usize) -> Option<&str> {
        self.field_32b.get(index).map(|f| f.currency())
    }

    /// Check if message has settlement details
    pub fn has_settlement_details(&self) -> bool {
        self.field_23e_settlement.is_some()
            || self.field_23b.is_some()
            || self.field_57a_settlement.is_some()
            || self.field_58a.is_some()
            || self.field_53a.is_some()
            || self.field_54a.is_some()
            || self.field_56a.is_some()
            || self.field_52a_settlement.is_some()
            || self.field_71a_settlement.is_some()
            || self.field_71f_settlement.is_some()
            || self.field_71g_settlement.is_some()
            || self.field_72_settlement.is_some()
    }

    /// Add a transaction to the batch
    pub fn add_transaction(
        &mut self,
        reference: Field21,
        amount: GenericCurrencyAmountField,
        debtor: Field59,
    ) {
        self.field_21_txn.push(reference);
        self.field_32b.push(amount);
        self.field_59a.push(debtor);
    }

    /// Add a transaction (backward compatibility)
    pub fn add_transaction_simple(
        &mut self,
        reference: String,
        currency: &str,
        amount: f64,
        debtor: Field59,
    ) {
        self.field_21_txn.push(Field21::new(reference));
        self.field_32b
            .push(GenericCurrencyAmountField::new(currency, amount).unwrap());
        self.field_59a.push(debtor);
    }

    /// Validate that transaction arrays have consistent lengths
    pub fn validate_transaction_consistency(&self) -> bool {
        let base_len = self.field_21_txn.len();

        // Check mandatory fields have same length
        if self.field_32b.len() != base_len || self.field_59a.len() != base_len {
            return false;
        }

        // Check optional fields either None or same length
        if let Some(ref vec) = self.field_23e_txn {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_21c {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_21d {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_21e_txn {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_50a_instructing_txn {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_50a_creditor_txn {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_52a_txn {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_57a {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_70_txn {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_26t_txn {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_77b_txn {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_33b {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_71a_txn {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_71f {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_71g {
            if vec.len() != base_len {
                return false;
            }
        }
        if let Some(ref vec) = self.field_36 {
            if vec.len() != base_len {
                return false;
            }
        }

        true
    }

    /// Validate C1 condition: Field 23E Seq B depends on Field 23E Seq A
    pub fn validate_c1(&self) -> bool {
        match (&self.field_23e, &self.field_23e_txn) {
            (Some(_), Some(_)) => true,
            (Some(_), None) => true,
            (None, Some(_)) => false, // C1 violation
            (None, None) => true,
        }
    }

    /// Validate C3 condition: Various conditional field requirements
    pub fn validate_c3(&self) -> bool {
        // Simplified C3 validation - in practice this would be more complex
        // This is a placeholder for the actual business rule validation
        true
    }

    /// Validate all conditional rules
    pub fn validate_conditional_rules(&self) -> bool {
        self.validate_c1() && self.validate_c3() && self.validate_transaction_consistency()
    }

    /// Validate transaction consistency (backward compatibility)
    pub fn validate_consistency(&self) -> bool {
        self.validate_transaction_consistency()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftMessageBody;
    use crate::fields::GenericBicField;

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
        assert_eq!(mt104.transaction_reference(0).unwrap(), "TXN001");
        assert_eq!(mt104.transaction_currency(0).unwrap(), "EUR");
        assert_eq!(mt104.transaction_amount(0).unwrap(), 500.00);
        assert_eq!(mt104.total_amount(), 500.00);
    }

    #[test]
    fn test_mt104_demo_creation() {
        let debtor = Field59::A(
            GenericBicField::new(None, Some("12345678".to_string()), "DEUTDEFF").unwrap(),
        );

        let transactions = vec![
            ("TXN001".to_string(), "EUR", 500.00, debtor.clone()),
            ("TXN002".to_string(), "USD", 750.00, debtor),
        ];

        let mt104 = MT104::new_simple("BATCH001".to_string(), "240315", transactions);

        assert_eq!(mt104.transaction_count(), 2);
        assert!(mt104.validate_consistency());

        let txn1 = mt104.transaction_reference(0).unwrap();
        assert_eq!(txn1, "TXN001");
        let txn1_currency = mt104.transaction_currency(0).unwrap();
        assert_eq!(txn1_currency, "EUR");
        let txn1_amount = mt104.transaction_amount(0).unwrap();
        assert_eq!(txn1_amount, 500.00);
    }

    #[test]
    fn test_message_type() {
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

        let mt104 =
            MT104::new_single_transaction(field_20, field_30, field_21, field_32b, field_59);

        assert!(mt104.validate_c1());
        assert!(mt104.validate_c3());
        assert!(mt104.validate_conditional_rules());
        assert!(mt104.validate_transaction_consistency());
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

        mt104.field_72_settlement =
            Some(Field72::new(vec!["Settlement Info".to_string()]).unwrap());

        assert!(mt104.has_settlement_details());
    }

    #[test]
    fn test_mt104_multiple_transactions() {
        let field_20 = Field20::new("DD001".to_string());
        let field_30 = Field30::new("240315");

        let references = vec![
            Field21::new("TXN001".to_string()),
            Field21::new("TXN002".to_string()),
        ];
        let amounts = vec![
            GenericCurrencyAmountField::new("EUR", 500.00).unwrap(),
            GenericCurrencyAmountField::new("EUR", 250.00).unwrap(),
        ];
        let debtors = vec![
            Field59::A(
                GenericBicField::new(None, Some("12345678".to_string()), "DEUTDEFF").unwrap(),
            ),
            Field59::A(
                GenericBicField::new(None, Some("87654321".to_string()), "CHASUS33").unwrap(),
            ),
        ];

        let mt104 = MT104::new(field_20, field_30, references, amounts, debtors);

        assert_eq!(mt104.transaction_count(), 2);
        assert_eq!(mt104.total_amount(), 750.00);
        assert_eq!(mt104.transaction_reference(0).unwrap(), "TXN001");
        assert_eq!(mt104.transaction_reference(1).unwrap(), "TXN002");
        assert!(mt104.validate_transaction_consistency());

        let txn1 = mt104.transaction_reference(0).unwrap();
        assert_eq!(txn1, "TXN001");
        let txn1_currency = mt104.transaction_currency(0).unwrap();
        assert_eq!(txn1_currency, "EUR");
        let txn1_amount = mt104.transaction_amount(0).unwrap();
        assert_eq!(txn1_amount, 500.00);
    }

    #[test]
    fn test_add_transaction() {
        let field_20 = Field20::new("DD001".to_string());
        let field_30 = Field30::new("240315");
        let field_21 = Field21::new("TXN001".to_string());
        let field_32b = GenericCurrencyAmountField::new("EUR", 500.00).unwrap();
        let field_59 = Field59::A(
            GenericBicField::new(None, Some("87654321".to_string()), "DEUTDEFF").unwrap(),
        );

        let mut mt104 =
            MT104::new_single_transaction(field_20, field_30, field_21, field_32b, field_59);

        let field_21_2 = Field21::new("TXN002".to_string());
        let field_32b_2 = GenericCurrencyAmountField::new("USD", 750.00).unwrap();
        let field_59_2 = Field59::A(
            GenericBicField::new(None, Some("12345678".to_string()), "CHASUS33").unwrap(),
        );

        mt104.add_transaction(field_21_2, field_32b_2, field_59_2);

        assert_eq!(mt104.transaction_count(), 2);
        assert_eq!(mt104.total_amount(), 1250.00);
        assert_eq!(mt104.transaction_reference(1).unwrap(), "TXN002");
        assert_eq!(mt104.transaction_currency(1).unwrap(), "USD");
        assert!(mt104.validate_transaction_consistency());
    }
}
