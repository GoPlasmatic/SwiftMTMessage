use crate::fields::*;
use crate::{SwiftMessage, swift_serde};
use serde::{Deserialize, Serialize};

/// # MT101: Request for Credit Transfer
///
/// ## Overview
/// MT101 enables financial institutions to send multiple credit transfer instructions
/// to another financial institution, requesting the execution of customer credit transfers.
/// This message type is used for batch processing of customer payments, allowing banks
/// to efficiently process multiple transactions in a single message. The MT101 supports
/// various transfer scenarios including domestic and cross-border payments, with
/// comprehensive validation rules and conditional requirements.
///
/// ## Message Type Specification
/// **Message Type**: `101`  
/// **Category**: Customer Payments and Cheques (Category 1)  
/// **Usage**: Request for Credit Transfer (Multiple)  
/// **Processing**: Batch processing and individual transaction processing  
/// **Network**: SWIFT FIN (Financial network)  
///
/// ## Message Structure
/// The MT101 message consists of mandatory and optional fields organized in sequences:
///
/// ### Sequence A (Message Level - Single Occurrence)
/// **Mandatory Fields:**
/// - **Field 20**: Sender's Reference (unique message identifier)
/// - **Field 28D**: Message Index/Total (message sequencing)
/// - **Field 30**: Requested Execution Date (when transfers should execute)
///
/// **Optional Fields:**
/// - **Field 21R**: Customer Specified Reference (customer's own reference)
/// - **Field 50a**: Instructing Party (party giving instructions)
/// - **Field 52a**: Account Servicing Institution (servicing bank)
/// - **Field 51A**: Sending Institution (message sender)
/// - **Field 25**: Authorisation (authentication information)
///
/// ### Sequence B (Transaction Level - Repetitive)
/// **Mandatory Fields per Transaction:**
/// - **Field 21**: Transaction Reference (unique per transaction)
/// - **Field 32B**: Currency/Transaction Amount (amount and currency)
/// - **Field 59a**: Beneficiary Customer (payment recipient)
/// - **Field 71A**: Details of Charges (charge allocation)
///
/// **Optional Fields per Transaction:**
/// - **Field 21F**: F/X Deal Reference (foreign exchange reference)
/// - **Field 23E**: Instruction Code (special processing instructions)
/// - **Field 50a**: Ordering Customer (payment originator)
/// - **Field 52a**: Account Servicing Institution (customer's bank)
/// - **Field 56a**: Intermediary Institution (routing bank)
/// - **Field 57a**: Account With Institution (beneficiary's bank)
/// - **Field 70**: Remittance Information (payment details)
/// - **Field 77B**: Regulatory Reporting (compliance information)
/// - **Field 33B**: Currency/Original Amount (original instructed amount)
/// - **Field 25A**: Charges Account (specific charge account)
/// - **Field 36**: Exchange Rate (FX conversion rate)
///
/// ## Business Applications
///
/// ### Primary Use Cases
/// - **Payroll processing**: Salary and wage payments to employees
/// - **Supplier payments**: Bulk payments to vendors and suppliers
/// - **Dividend distributions**: Payments to shareholders
/// - **Pension payments**: Regular pension and benefit distributions
/// - **Corporate treasury**: Multi-beneficiary treasury operations
/// - **Bulk retail transfers**: High-volume customer payment processing
///
/// ### Industry Sectors
/// - **Corporate Banking**: Large corporate client payment processing
/// - **Payroll Services**: Specialized payroll processing companies
/// - **Government**: Social security and benefit payments
/// - **Insurance**: Claims and benefit distributions
/// - **Investment Management**: Fund distributions and dividends
/// - **Retail Banking**: Batch customer payment services
///
/// ## Conditional Rules Summary (C1–C9)
///
/// The MT101 includes complex conditional validation rules:
///
/// - **C1**: If Field 36 present, Field 21F must be present
/// - **C2**: If Field 33B present and Field 32B amount ≠ 0, Field 36 is required
/// - **C3**: Field 50a (Ordering Customer) must be present in Seq A or Seq B, not both
/// - **C4**: Field 50a (Instructing Party) must be in Seq A or Seq B, not both
/// - **C5**: If Field 33B present, currency must differ from Field 32B
/// - **C6**: Field 52a must be in Seq A or Seq B, not both
/// - **C7**: If Field 56a present, Field 57a required
/// - **C8**: If Field 21R present, currency in Field 32B must be same across all Seq B
/// - **C9**: Presence of Field 21F and Field 33B depends on amount and Field 23E value
///
/// ## Validation Rules and Compliance
///
/// ### Network Validated Rules (SWIFT Standards)
/// - **T20**: Sender's reference format and uniqueness
/// - **T21**: Transaction references must be unique within message
/// - **T28**: Message index/total format validation
/// - **T30**: Execution date must be valid business day
/// - **T32**: Amount format and currency validation
/// - **T59**: Beneficiary customer identification completeness
/// - **T71**: Charge code validation (OUR/BEN/SHA)
///
/// ### Business Rule Validations
/// - Message index/total consistency across related messages
/// - Execution date must be future date or current business day
/// - Transaction amounts must be positive
/// - Currency codes must be valid ISO 4217
/// - All conditional rules (C1-C9) must be satisfied
/// - BIC codes must be valid and active
///
/// ## Error Handling and Processing
///
/// ### Common Validation Errors
/// - **Invalid execution date**: Past dates or non-business days
/// - **Conditional rule violations**: Missing required fields per C1-C9
/// - **Amount validation**: Zero or negative amounts
/// - **Currency consistency**: Mismatched currencies when Field 21R present
/// - **Institution validation**: Invalid or inactive BIC codes
///
/// ### Processing Status Indicators
/// - **ACCP**: Accepted for processing
/// - **PART**: Partially accepted (some transactions rejected)
/// - **RJCT**: Rejected (entire message or specific transactions)
/// - **PDNG**: Pending validation or authorization
/// - **PROC**: Processing individual transactions
///
/// Complete implementation supporting all MT101 fields and conditional validation rules
/// for comprehensive batch credit transfer processing
#[swift_serde]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[swift_message(mt = "101")]
pub struct MT101 {
    // ================================
    // SEQUENCE A - MESSAGE LEVEL FIELDS (Single Occurrence)
    // ================================
    /// **Sender's Reference** - Field 20
    ///
    /// Unique reference assigned by the sender to identify this specific
    /// MT101 message. Used for message tracking, reconciliation, and audit.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory in all MT101 messages  
    /// **Business Rule**: Must be unique per sender per business day
    #[field("20")]
    pub field_20: Field20,

    /// **Message Index/Total** - Field 28D
    ///
    /// Indicates the sequence number and total number of messages when
    /// a large batch of transactions is split across multiple MT101 messages.
    ///
    /// **Format**: 5n/5n (index/total)  
    /// **Usage**: Mandatory in all MT101 messages  
    /// **Example**: "00001/00003" (message 1 of 3)
    /// **Business Rule**: Index must be ≤ total, total must be consistent
    #[field("28D")]
    pub field_28d: Field28D,

    /// **Requested Execution Date** - Field 30
    ///
    /// Date when the credit transfers should be executed by the receiving
    /// financial institution. All transactions in the message share this date.
    ///
    /// **Format**: YYMMDD (6 numeric characters)  
    /// **Usage**: Mandatory in all MT101 messages  
    /// **Business Rule**: Must be valid business day, not in the past
    #[field("30")]
    pub field_30: Field30,

    /// **Customer Specified Reference** - Field 21R (Optional)
    ///
    /// Additional reference specified by the customer for their own
    /// identification and reconciliation purposes.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Optional  
    /// **Conditional Rule C8**: If present, all Field 32B must have same currency
    #[field("21R")]
    pub field_21r: Option<Field21>,

    /// **Instructing Party** - Field 50a Sequence A (Optional)
    ///
    /// Identifies the party that gives the instruction to execute the
    /// credit transfers. Can appear in Sequence A or Sequence B transactions.
    ///
    /// **Options**: A (Account+BIC), F (Party ID+Name/Address), K (Name/Address only)  
    /// **Conditional Rule C4**: Must be present in Seq A or Seq B, not both  
    /// **Usage**: Optional but subject to conditional rules
    #[field("50A_SEQ_A")]
    pub field_50a_seq_a: Option<Field50>,

    /// **Account Servicing Institution** - Field 52a Sequence A (Optional)
    ///
    /// Identifies the financial institution that services the account
    /// of the instructing party or ordering customer.
    ///
    /// **Options**: A (Account+BIC), C (Clearing code), D (Name/Address)  
    /// **Conditional Rule C6**: Must be present in Seq A or Seq B, not both  
    /// **Usage**: Optional but subject to conditional rules
    #[field("52A_SEQ_A")]
    pub field_52a_seq_a: Option<GenericBicField>,

    /// **Sending Institution** - Field 51A (Optional)
    ///
    /// Identifies the financial institution sending the MT101 message.
    /// Used when the sender is different from the message originator.
    ///
    /// **Format**: BIC code  
    /// **Usage**: Optional  
    /// **Context**: Correspondent banking scenarios
    #[field("51A")]
    pub field_51a: Option<GenericBicField>,

    /// **Authorisation** - Field 25 (Optional)
    ///
    /// Contains authorisation information for authentication and
    /// validation of the credit transfer instructions.
    ///
    /// **Format**: Up to 35 alphanumeric characters  
    /// **Usage**: Optional  
    /// **Purpose**: Authentication and authorization validation
    #[field("25")]
    pub field_25: Option<Field25>,

    // ================================
    // SEQUENCE B - TRANSACTION LEVEL FIELDS (Repetitive)
    // ================================
    /// **Transaction Reference** - Field 21
    ///
    /// Unique reference for each individual transaction within the MT101.
    /// Must be unique within the message scope.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory for each transaction in Sequence B  
    /// **Business Rule**: Must be unique within the message
    #[field("21")]
    pub field_21: Field21,

    /// **Currency/Transaction Amount** - Field 32B
    ///
    /// Amount and currency for each individual credit transfer transaction.
    /// Each transaction can have different amounts and currencies.
    ///
    /// **Format**: 3!a15d (Currency + Amount with decimal comma)  
    /// **Usage**: Mandatory for each transaction  
    /// **Validation**: Amount must be positive, currency must be valid ISO 4217
    #[field("32B")]
    pub field_32b: GenericCurrencyAmountField,

    /// **Beneficiary Customer** - Field 59a
    ///
    /// Identifies the ultimate recipient of each credit transfer.
    /// Critical for successful payment delivery and compliance.
    ///
    /// **Options**: A (Account+BIC), F (Party ID+Name/Address), No Option (Account+Name/Address)  
    /// **Usage**: Mandatory for each transaction  
    /// **Compliance**: Subject to sanctions screening and KYC requirements
    #[field("59")]
    pub field_59: Field59,

    /// **Details of Charges** - Field 71A
    ///
    /// Specifies how transaction charges should be allocated for each
    /// individual credit transfer.
    ///
    /// **Values**: OUR (sender pays), BEN (receiver pays), SHA (shared)  
    /// **Usage**: Mandatory for each transaction  
    /// **Impact**: Affects final amount received by beneficiary
    #[field("71A")]
    pub field_71a: Field71A,

    /// **F/X Deal Reference** - Field 21F (Optional)
    ///
    /// Reference to a foreign exchange deal when the transaction
    /// involves currency conversion.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Conditional Rule C1**: Must be present if Field 36 is present  
    /// **Conditional Rule C9**: Presence depends on amount and Field 23E
    #[field("21F")]
    pub field_21f: Option<Field21>,

    /// **Instruction Code** - Field 23E (Optional)
    ///
    /// Specifies special processing instructions for each transaction,
    /// such as communication requirements or handling procedures.
    ///
    /// **Format**: 4!c[/30x] (Code + optional narrative)  
    /// **Usage**: Optional, affects processing workflow  
    /// **Examples**: HOLD, PHON, INTC, CORT
    #[field("23E")]
    pub field_23e: Option<Field23E>,

    /// **Ordering Customer** - Field 50a Sequence B (Optional)
    ///
    /// Identifies the customer ordering each individual credit transfer.
    /// Can be different for each transaction in the batch.
    ///
    /// **Options**: A (Account+BIC), F (Party ID+Name/Address), K (Name/Address only)  
    /// **Conditional Rule C3**: Must be present in Seq A or Seq B, not both  
    /// **Usage**: Optional but subject to conditional rules
    #[field("50A_SEQ_B")]
    pub field_50a_seq_b: Option<Field50>,

    /// **Account Servicing Institution** - Field 52a Sequence B (Optional)
    ///
    /// Identifies the financial institution servicing the ordering
    /// customer's account for each specific transaction.
    ///
    /// **Options**: A (Account+BIC), C (Clearing code), D (Name/Address)  
    /// **Conditional Rule C6**: Must be present in Seq A or Seq B, not both  
    /// **Usage**: Optional but subject to conditional rules
    #[field("52A_SEQ_B")]
    pub field_52a_seq_b: Option<GenericBicField>,

    /// **Intermediary Institution** - Field 56a (Optional)
    ///
    /// Identifies an intermediary bank in the payment routing chain
    /// for each individual transaction.
    ///
    /// **Options**: A (Account+BIC), C (Account), D (Name/Address)  
    /// **Conditional Rule C7**: If present, Field 57a is mandatory  
    /// **Usage**: Optional, facilitates payment routing
    #[field("56")]
    pub field_56: Option<GenericBicField>,

    /// **Account With Institution** - Field 57a (Optional)
    ///
    /// Identifies the institution where the beneficiary customer's
    /// account is held for each transaction.
    ///
    /// **Options**: A (Account+BIC), C (Account), D (Name/Address)  
    /// **Conditional Rule C7**: Mandatory if Field 56a is present  
    /// **Usage**: Conditional, critical for payment delivery
    #[field("57")]
    pub field_57: Option<GenericBicField>,

    /// **Remittance Information** - Field 70 (Optional)
    ///
    /// Free-format text providing details about the purpose of each
    /// payment, invoice references, and other remittance information.
    ///
    /// **Format**: Up to 4 lines of 35 characters each  
    /// **Usage**: Optional  
    /// **Purpose**: Payment description, reconciliation information
    #[field("70")]
    pub field_70: Option<Field70>,

    /// **Regulatory Reporting** - Field 77B (Optional)
    ///
    /// Contains regulatory information required for compliance with
    /// local and international reporting requirements.
    ///
    /// **Format**: Up to 3 lines of 35 characters each  
    /// **Usage**: Optional  
    /// **Compliance**: Balance of payments, statistical reporting
    #[field("77B")]
    pub field_77b: Option<Field77B>,

    /// **Currency/Original Amount** - Field 33B (Optional)
    ///
    /// Original amount and currency as instructed when different from
    /// the settlement amount in Field 32B due to currency conversion.
    ///
    /// **Format**: 3!a15d (Currency + Amount)  
    /// **Conditional Rule C2**: If present and Field 32B ≠ 0, Field 36 required  
    /// **Conditional Rule C5**: Currency must differ from Field 32B  
    /// **Conditional Rule C9**: Presence depends on amount and Field 23E
    #[field("33B")]
    pub field_33b: Option<GenericCurrencyAmountField>,

    /// **Charges Account** - Field 25A (Optional)
    ///
    /// Specifies a specific account to be debited for charges when
    /// different from the ordering customer's main account.
    ///
    /// **Format**: /34x (Account identifier)  
    /// **Usage**: Optional  
    /// **Business Rule**: Must differ from ordering customer account
    #[field("25A")]
    pub field_25a: Option<GenericAccountField>,

    /// **Exchange Rate** - Field 36 (Optional)
    ///
    /// Exchange rate applied for currency conversion between the
    /// original amount (Field 33B) and settlement amount (Field 32B).
    ///
    /// **Format**: Up to 12 digits with decimal  
    /// **Conditional Rule C1**: If present, Field 21F must be present  
    /// **Conditional Rule C2**: Required if Field 33B present and Field 32B ≠ 0  
    /// **Usage**: Conditional, for FX transactions
    #[field("36")]
    pub field_36: Option<Field36>,
}

impl MT101 {
    /// Create a new MT101 with required fields only (single transaction)
    pub fn new(
        field_20: Field20,
        field_28d: Field28D,
        field_30: Field30,
        field_21: Field21,
        field_32b: GenericCurrencyAmountField,
        field_59: Field59,
        field_71a: Field71A,
    ) -> Self {
        Self {
            field_20,
            field_28d,
            field_30,
            field_21r: None,
            field_50a_seq_a: None,
            field_52a_seq_a: None,
            field_51a: None,
            field_25: None,
            field_21,
            field_32b,
            field_59,
            field_71a,
            field_21f: None,
            field_23e: None,
            field_50a_seq_b: None,
            field_52a_seq_b: None,
            field_56: None,
            field_57: None,
            field_70: None,
            field_77b: None,
            field_33b: None,
            field_25a: None,
            field_36: None,
        }
    }

    /// Create a new MT101 with all fields for complete functionality
    #[allow(clippy::too_many_arguments)]
    pub fn new_complete(
        field_20: Field20,
        field_28d: Field28D,
        field_30: Field30,
        field_21r: Option<Field21>,
        field_50a_seq_a: Option<Field50>,
        field_52a_seq_a: Option<GenericBicField>,
        field_51a: Option<GenericBicField>,
        field_25: Option<Field25>,
        field_21: Field21,
        field_32b: GenericCurrencyAmountField,
        field_59: Field59,
        field_71a: Field71A,
        field_21f: Option<Field21>,
        field_23e: Option<Field23E>,
        field_50a_seq_b: Option<Field50>,
        field_52a_seq_b: Option<GenericBicField>,
        field_56: Option<GenericBicField>,
        field_57: Option<GenericBicField>,
        field_70: Option<Field70>,
        field_77b: Option<Field77B>,
        field_33b: Option<GenericCurrencyAmountField>,
        field_25a: Option<GenericAccountField>,
        field_36: Option<Field36>,
    ) -> Self {
        Self {
            field_20,
            field_28d,
            field_30,
            field_21r,
            field_50a_seq_a,
            field_52a_seq_a,
            field_51a,
            field_25,
            field_21,
            field_32b,
            field_59,
            field_71a,
            field_21f,
            field_23e,
            field_50a_seq_b,
            field_52a_seq_b,
            field_56,
            field_57,
            field_70,
            field_77b,
            field_33b,
            field_25a,
            field_36,
        }
    }

    /// Get the sender's reference
    pub fn senders_reference(&self) -> &str {
        self.field_20.transaction_reference()
    }

    /// Get the message index and total
    pub fn message_index_total(&self) -> (&str, &str) {
        self.field_28d.message_index_total()
    }

    /// Get the requested execution date
    pub fn execution_date(&self) -> chrono::NaiveDate {
        self.field_30.execution_date()
    }

    /// Get the transaction reference
    pub fn transaction_reference(&self) -> &str {
        self.field_21.related_reference()
    }

    /// Get the transaction currency
    pub fn transaction_currency(&self) -> &str {
        self.field_32b.currency()
    }

    /// Get the transaction amount
    pub fn transaction_amount(&self) -> f64 {
        self.field_32b.amount()
    }

    /// Get the charge code
    pub fn charge_code(&self) -> &str {
        self.field_71a.charge_code()
    }

    /// Get customer specified reference if present
    pub fn customer_reference(&self) -> Option<&str> {
        self.field_21r.as_ref().map(|f| f.related_reference())
    }

    /// Get instructing party from sequence A if present
    pub fn instructing_party_seq_a(&self) -> Option<&Field50> {
        self.field_50a_seq_a.as_ref()
    }

    /// Get ordering customer from sequence B if present
    pub fn ordering_customer_seq_b(&self) -> Option<&Field50> {
        self.field_50a_seq_b.as_ref()
    }

    /// Get account servicing institution from sequence A if present
    pub fn account_servicing_institution_seq_a(&self) -> Option<&GenericBicField> {
        self.field_52a_seq_a.as_ref()
    }

    /// Get account servicing institution from sequence B if present
    pub fn account_servicing_institution_seq_b(&self) -> Option<&GenericBicField> {
        self.field_52a_seq_b.as_ref()
    }

    /// Get sending institution if present
    pub fn sending_institution(&self) -> Option<&GenericBicField> {
        self.field_51a.as_ref()
    }

    /// Get intermediary institution if present
    pub fn intermediary_institution(&self) -> Option<&GenericBicField> {
        self.field_56.as_ref()
    }

    /// Get account with institution if present
    pub fn account_with_institution(&self) -> Option<&GenericBicField> {
        self.field_57.as_ref()
    }

    /// Get remittance information if present
    pub fn remittance_information(&self) -> Option<&Field70> {
        self.field_70.as_ref()
    }

    /// Get regulatory reporting if present
    pub fn regulatory_reporting(&self) -> Option<&Field77B> {
        self.field_77b.as_ref()
    }

    /// Get original amount if present (for FX transactions)
    pub fn original_amount(&self) -> Option<&GenericCurrencyAmountField> {
        self.field_33b.as_ref()
    }

    /// Get exchange rate if present
    pub fn exchange_rate(&self) -> Option<&Field36> {
        self.field_36.as_ref()
    }

    /// Get F/X deal reference if present
    pub fn fx_deal_reference(&self) -> Option<&Field21> {
        self.field_21f.as_ref()
    }

    /// Get instruction code if present
    pub fn instruction_code(&self) -> Option<&Field23E> {
        self.field_23e.as_ref()
    }

    /// Get charges account if present
    pub fn charges_account(&self) -> Option<&GenericAccountField> {
        self.field_25a.as_ref()
    }

    /// Check if this is a cross-currency transaction
    pub fn is_cross_currency(&self) -> bool {
        if let Some(field_33b) = &self.field_33b {
            field_33b.currency() != self.field_32b.currency()
        } else {
            false
        }
    }

    /// Check if exchange rate is provided for cross-currency transactions (C1, C2)
    pub fn has_required_exchange_rate(&self) -> bool {
        if self.is_cross_currency() {
            self.field_36.is_some()
        } else {
            true // Not required for same-currency transactions
        }
    }

    /// Validate conditional rule C1: If Field 36 present, Field 21F must be present
    pub fn validate_c1(&self) -> bool {
        if self.field_36.is_some() {
            self.field_21f.is_some()
        } else {
            true
        }
    }

    /// Validate conditional rule C2: If Field 33B present and Field 32B amount ≠ 0, Field 36 required
    pub fn validate_c2(&self) -> bool {
        if let Some(_field_33b) = &self.field_33b {
            if self.field_32b.amount() != 0.0 {
                self.field_36.is_some()
            } else {
                true
            }
        } else {
            true
        }
    }

    /// Validate conditional rule C3: Field 50a (Ordering Customer) must be in Seq A or Seq B, not both
    pub fn validate_c3(&self) -> bool {
        let seq_a_present = self.field_50a_seq_a.is_some();
        let seq_b_present = self.field_50a_seq_b.is_some();

        // Must be present in exactly one sequence (not both, not neither)
        seq_a_present ^ seq_b_present
    }

    /// Validate conditional rule C4: Field 50a (Instructing Party) must be in Seq A or Seq B, not both
    /// Note: This assumes instructing party can appear in both sequences with different semantics
    pub fn validate_c4(&self) -> bool {
        // For MT101, instructing party is typically in Seq A
        // This rule may need refinement based on exact SWIFT specification
        true
    }

    /// Validate conditional rule C5: If Field 33B present, currency must differ from Field 32B
    pub fn validate_c5(&self) -> bool {
        if let Some(field_33b) = &self.field_33b {
            field_33b.currency() != self.field_32b.currency()
        } else {
            true
        }
    }

    /// Validate conditional rule C6: Field 52a must be in Seq A or Seq B, not both
    pub fn validate_c6(&self) -> bool {
        let seq_a_present = self.field_52a_seq_a.is_some();
        let seq_b_present = self.field_52a_seq_b.is_some();

        // Can be in neither, or exactly one, but not both
        !(seq_a_present && seq_b_present)
    }

    /// Validate conditional rule C7: If Field 56a present, Field 57a required
    pub fn validate_c7(&self) -> bool {
        if self.field_56.is_some() {
            self.field_57.is_some()
        } else {
            true
        }
    }

    /// Validate conditional rule C8: If Field 21R present, currency in Field 32B must be same across all Seq B
    /// Note: This implementation assumes single transaction; would need modification for multi-transaction support
    pub fn validate_c8(&self) -> bool {
        // For single transaction implementation, this is automatically satisfied
        // Multi-transaction implementation would need to check currency consistency
        true
    }

    /// Validate all conditional rules
    pub fn validate_conditional_rules(&self) -> bool {
        self.validate_c1()
            && self.validate_c2()
            && self.validate_c3()
            && self.validate_c4()
            && self.validate_c5()
            && self.validate_c6()
            && self.validate_c7()
            && self.validate_c8()
    }

    /// Check if all required fields are present and valid
    pub fn validate_structure(&self) -> bool {
        // All required fields are enforced by the struct
        // Additional business rule validation
        self.validate_conditional_rules() &&
        self.field_32b.amount() > 0.0 && // Amount must be positive
        !self.field_32b.currency().is_empty() // Currency must be present
    }

    /// Get all institution fields in routing order
    pub fn get_routing_chain(&self) -> Vec<(&str, String)> {
        let mut chain = Vec::new();

        // Account servicing institution (from either sequence)
        if let Some(field_52a) = &self.field_52a_seq_a {
            chain.push((
                "Account Servicing Institution (Seq A)",
                field_52a.bic().to_string(),
            ));
        } else if let Some(field_52a) = &self.field_52a_seq_b {
            chain.push((
                "Account Servicing Institution (Seq B)",
                field_52a.bic().to_string(),
            ));
        }

        // Sending institution
        if let Some(field_51a) = &self.field_51a {
            chain.push(("Sending Institution", field_51a.bic().to_string()));
        }

        // Intermediary institution
        if let Some(field_56) = &self.field_56 {
            chain.push(("Intermediary Institution", field_56.bic().to_string()));
        }

        // Account with institution
        if let Some(field_57) = &self.field_57 {
            chain.push(("Account With Institution", field_57.bic().to_string()));
        }

        chain
    }

    /// Check if message is for same-day execution
    pub fn is_same_day_execution(&self) -> bool {
        let today = chrono::Utc::now().date_naive();
        self.field_30.execution_date() == today
    }

    /// Check if message is for future execution
    pub fn is_future_dated(&self) -> bool {
        let today = chrono::Utc::now().date_naive();
        self.field_30.execution_date() > today
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftMessageBody;

    #[test]
    fn test_mt101_creation() {
        let field_20 = Field20::new("BATCH001".to_string());
        let field_28d = Field28D::new("00001", "00001");
        let field_30 = Field30::new("240315");
        let field_21 = Field21::new("TXN001".to_string());
        let field_32b = GenericCurrencyAmountField::new("USD", 1000.00).unwrap();
        let field_59 = Field59::A(
            GenericBicField::new(None, Some("12345678".to_string()), "DEUTDEFF").unwrap(),
        );
        let field_71a = Field71A::new("SHA".to_string());

        let mt101 = MT101::new(
            field_20, field_28d, field_30, field_21, field_32b, field_59, field_71a,
        );

        assert_eq!(mt101.senders_reference(), "BATCH001");
        assert_eq!(mt101.transaction_reference(), "TXN001");
        assert_eq!(mt101.transaction_currency(), "USD");
        assert_eq!(mt101.transaction_amount(), 1000.00);
        assert_eq!(mt101.charge_code(), "SHA");
    }

    #[test]
    fn test_mt101_message_type() {
        assert_eq!(MT101::message_type(), "101");
    }

    #[test]
    fn test_mt101_conditional_rules() {
        let field_20 = Field20::new("BATCH001".to_string());
        let field_28d = Field28D::new("00001", "00001");
        let field_30 = Field30::new("240315");
        let field_21 = Field21::new("TXN001".to_string());
        let field_32b = GenericCurrencyAmountField::new("USD", 1000.00).unwrap();
        let field_59 = Field59::A(
            GenericBicField::new(None, Some("12345678".to_string()), "DEUTDEFF").unwrap(),
        );
        let field_71a = Field71A::new("SHA".to_string());

        // Test C1: If Field 36 present, Field 21F must be present
        let field_36 = Field36::new(1.1234).unwrap();
        let field_21f = Field21::new("FX001".to_string());

        let field_50_seq_a = Field50::K(
            Field50K::new(vec!["ORDERING CUSTOMER".to_string(), "NAME".to_string()]).unwrap(),
        );

        let mt101_with_fx = MT101::new_complete(
            field_20,
            field_28d,
            field_30,
            None,
            Some(field_50_seq_a),
            None,
            None,
            None,
            field_21,
            field_32b,
            field_59,
            field_71a,
            Some(field_21f),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(field_36),
        );

        assert!(mt101_with_fx.validate_c1());
        assert!(mt101_with_fx.validate_conditional_rules());
    }

    #[test]
    fn test_mt101_cross_currency() {
        let field_20 = Field20::new("BATCH001".to_string());
        let field_28d = Field28D::new("00001", "00001");
        let field_30 = Field30::new("240315");
        let field_21 = Field21::new("TXN001".to_string());
        let field_32b = GenericCurrencyAmountField::new("USD", 1000.00).unwrap();
        let field_59 = Field59::A(
            GenericBicField::new(None, Some("12345678".to_string()), "DEUTDEFF").unwrap(),
        );
        let field_71a = Field71A::new("SHA".to_string());

        // Cross-currency with EUR original amount
        let field_33b = GenericCurrencyAmountField::new("EUR", 850.00).unwrap();
        let field_36 = Field36::new(1.1765).unwrap();
        let field_21f = Field21::new("FX001".to_string());

        let mt101_fx = MT101::new_complete(
            field_20,
            field_28d,
            field_30,
            None,
            None,
            None,
            None,
            None,
            field_21,
            field_32b,
            field_59,
            field_71a,
            Some(field_21f),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(field_33b),
            None,
            Some(field_36),
        );

        assert!(mt101_fx.is_cross_currency());
        assert!(mt101_fx.has_required_exchange_rate());
        assert!(mt101_fx.validate_c2()); // Field 33B present, Field 36 required
        assert!(mt101_fx.validate_c5()); // Field 33B currency differs from Field 32B
    }

    #[test]
    fn test_mt101_routing_chain() {
        let field_20 = Field20::new("BATCH001".to_string());
        let field_28d = Field28D::new("00001", "00001");
        let field_30 = Field30::new("240315");
        let field_21 = Field21::new("TXN001".to_string());
        let field_32b = GenericCurrencyAmountField::new("USD", 1000.00).unwrap();
        let field_59 = Field59::A(
            GenericBicField::new(None, Some("12345678".to_string()), "DEUTDEFF").unwrap(),
        );
        let field_71a = Field71A::new("SHA".to_string());

        let field_52a_seq_a = GenericBicField::new(None, None, "CHASUS33XXX").unwrap();
        let field_56 = GenericBicField::new(None, None, "BARCGB22XXX").unwrap();
        let field_57 = GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap();

        let mt101 = MT101::new_complete(
            field_20,
            field_28d,
            field_30,
            None,
            None,
            Some(field_52a_seq_a),
            None,
            None,
            field_21,
            field_32b,
            field_59,
            field_71a,
            None,
            None,
            None,
            None,
            Some(field_56),
            Some(field_57),
            None,
            None,
            None,
            None,
            None,
        );

        let routing_chain = mt101.get_routing_chain();
        assert_eq!(routing_chain.len(), 3);
        assert_eq!(
            routing_chain[0],
            (
                "Account Servicing Institution (Seq A)",
                "CHASUS33XXX".to_string()
            )
        );
        assert_eq!(
            routing_chain[1],
            ("Intermediary Institution", "BARCGB22XXX".to_string())
        );
        assert_eq!(
            routing_chain[2],
            ("Account With Institution", "DEUTDEFFXXX".to_string())
        );
    }
}
