use crate::{GenericBicField, GenericCurrencyAmountField, SwiftMessage, fields::*, swift_serde};
use serde::{Deserialize, Serialize};

/// # MT202: General Financial Institution Transfer
///
/// ## Overview
/// MT202 enables financial institutions to transfer funds between themselves for their own
/// account or for the account of their customers. It serves as the backbone for correspondent
/// banking relationships, facilitating both institutional transfers and cover payments for
/// customer transactions. The MT202 supports various transfer scenarios including nostro/vostro
/// account movements, liquidity management, and settlement processing.
///
/// ## Message Type Specification
/// **Message Type**: `202`  
/// **Category**: Financial Institution Transfers (Category 2)  
/// **Usage**: General Financial Institution Transfer  
/// **Processing**: Real-time gross settlement (RTGS) and net settlement  
/// **Network**: SWIFT FIN (Financial network)  
///
/// ### Message Variants
/// ```text
/// MT202        - Standard financial institution transfer
/// MT202.COV    - Cover message for customer credit transfers
/// ```
///
/// ## Message Structure
/// The MT202 message consists of mandatory and optional fields organized in specific sequences:
///
/// ### Mandatory Fields (Core Requirements)
/// - **Field 20**: Transaction Reference Number (sender's unique reference)
/// - **Field 21**: Related Reference (link to previous message or transaction)
/// - **Field 32A**: Value Date/Currency/Amount (settlement details)
/// - **Field 58A**: Beneficiary Institution (final receiving institution)
///
/// ### Optional Fields (Enhanced Processing)
/// ```text
/// Field 13C   - Time Indication (processing timing)
/// Field 52A   - Ordering Institution (sender's bank)
/// Field 53A   - Sender's Correspondent (intermediate bank)
/// Field 54A   - Receiver's Correspondent (intermediate bank)
/// Field 56A   - Intermediary Institution (routing bank)
/// Field 57A   - Account With Institution (beneficiary's correspondent)
/// Field 72    - Sender to Receiver Information (processing instructions)
/// ```
///
/// ### MT202.COV Specific Fields (Cover Message)
/// When used as a cover message for customer transfers:
/// ```text
/// Field 50A   - Ordering Customer (ultimate originator)
/// Field 59A   - Beneficiary Customer (ultimate recipient)
/// Field 70    - Remittance Information (payment details)
/// Field 33B   - Currency/Instructed Amount (original amount)
/// ```
///
/// ## Business Applications
///
/// ### Primary Use Cases
/// - **Nostro/Vostro movements**: Account movements between correspondent banks
/// - **Liquidity management**: Funding and cash management between institutions
/// - **Settlement processing**: Final settlement of payment obligations
/// - **Cover payments**: Cover for underlying customer credit transfers
/// - **Reimbursement**: Institution-to-institution reimbursements
/// - **Treasury operations**: Bank treasury funding and investment settlements
///
/// ### Industry Sectors
/// - **Correspondent Banking**: Managing correspondent account relationships
/// - **Central Banking**: Central bank operations and monetary policy implementation
/// - **Commercial Banking**: Inter-bank funding and settlement
/// - **Investment Banking**: Securities settlement and margin funding
/// - **Corporate Banking**: Large corporate cash management
/// - **Trade Finance**: Trade settlement and financing
///
/// ## Routing and Settlement Patterns
///
/// ### Direct Settlement
/// ```text
/// Ordering Institution → Beneficiary Institution
/// (Field 52A → Field 58A)
/// ```
///
/// ### Correspondent Banking Chain
/// ```text
/// Ordering Institution → Sender's Correspondent → Receiver's Correspondent → Beneficiary Institution
/// (Field 52A → Field 53A → Field 54A → Field 58A)
/// ```
///
/// ### Complex Multi-Institution Routing
/// ```text
/// Ordering Institution → Sender's Correspondent → Intermediary →
/// Account With Institution → Beneficiary Institution
/// (Field 52A → Field 53A → Field 56A → Field 57A → Field 58A)
/// ```
///
/// ## Field Relationships and Dependencies
///
/// ### Time Indication (Field 13C)
/// - **Multiple occurrences**: Field 13C can appear multiple times for different timing requirements
/// - **CLS timing**: Cut-off times for Continuous Linked Settlement
/// - **TARGET timing**: European Central Bank TARGET system timing
/// - **Local timing**: Domestic settlement system timing requirements
///
/// ### Institution Chain Validation
/// - All institutions must have valid correspondent relationships
/// - BIC codes must be active and reachable via SWIFT network
/// - Account numbers must be valid for the specified institutions
/// - Routing must comply with sanctions and regulatory requirements
///
/// ### Cover Message Requirements (MT202.COV)
/// - Field 50A (Ordering Customer) becomes mandatory for cover messages
/// - Field 59A (Beneficiary Customer) identifies ultimate beneficiary
/// - Field 70 (Remittance Information) provides payment purpose details
/// - Field 33B used for currency conversion scenarios
///
/// ## Validation Rules and Compliance
///
/// ### Network Validated Rules (SWIFT Standards)
/// - **T20**: Related reference format validation
/// - **T21**: Transaction reference uniqueness
/// - **T32**: Date and amount format validation
/// - **T58**: Beneficiary institution BIC validation
/// - **C20**: Reference number consistency
/// - **C21**: Related reference requirement
/// - **C58**: Institution identification completeness
///
/// ### Business Rule Validations
/// - Transaction and related references should be unique per sender per day
/// - Value date should be valid business day for settlement currency
/// - Institution chain should form valid correspondent relationships
/// - Time indications should be reasonable for processing requirements
/// - Cover message fields should be consistent with underlying transaction
///
/// ### Regulatory Compliance
/// - **Sanctions Screening**: All institutions subject to sanctions checks
/// - **Regulatory Reporting**: Large value transfer reporting requirements
/// - **AML/KYC**: Know Your Customer requirements for cover messages
/// - **Correspondent Banking**: Due diligence and compliance monitoring
/// - **Capital Requirements**: Basel III liquidity and capital impact
///
/// ## Error Handling and Processing
///
/// ### Field 72 Processing Instructions
/// ```text
/// /INT/Internal transfer between own accounts
/// /COV/Cover payment for customer transfer  
/// /REIMBURSEMENT/Reimbursement payment
/// /SETTLEMENT/Settlement of obligations
/// ```
///
/// ### Common Processing Scenarios
/// - **Same-day settlement**: Immediate settlement requirement
/// - **Future-dated**: Settlement on specific future date
/// - **Recurring**: Standing instruction processing
/// - **Amendment**: Modification of previous instruction
/// - **Cancellation**: Reversal of previous transfer
#[swift_serde]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[swift_message(mt = "202")]
pub struct MT202 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique sender's reference identifying this specific financial institution transfer.
    /// Used throughout the transfer lifecycle for tracking, reconciliation, and audit.
    /// Must be unique within the sender's system per business day.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory in all MT202 variants  
    /// **Business Rule**: Should follow sender's reference numbering scheme
    #[field("20")]
    pub field_20: Field20,

    /// **Related Reference** - Field 21
    ///
    /// Reference to a related message or transaction that this MT202 is associated with.
    /// Critical for linking cover payments to underlying customer transfers and for
    /// maintaining audit trails across related transactions.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory in all MT202 messages  
    /// **Relationship**: Links to previous messages or transactions
    #[field("21")]
    pub field_21: Field21,

    /// **Value Date/Currency/Amount** - Field 32A
    ///
    /// Core settlement details specifying when, in what currency, and for what amount
    /// the institutional transfer should be processed. The value date determines when
    /// the settlement occurs between the institutions.
    ///
    /// **Components**: Date (YYMMDD) + Currency (3 chars) + Amount (decimal)  
    /// **Business Rule**: Value date should be valid business day for currency  
    /// **Settlement**: Determines actual settlement timing between institutions
    #[field("32A")]
    pub field_32a: Field32A,

    /// **Beneficiary Institution** - Field 58A
    ///
    /// Identifies the financial institution that will receive the funds being transferred.
    /// This is the final destination institution in the transfer chain and must be
    /// clearly identified for successful settlement.
    ///
    /// **Format**: [/account]BIC code  
    /// **Usage**: Mandatory in all MT202 messages  
    /// **Settlement**: Final destination for fund settlement
    #[field("58A")]
    pub field_58a: GenericBicField,

    /// **Time Indication** - Field 13C (Optional, Repetitive)
    ///
    /// Provides specific timing instructions for transfer processing, including
    /// cut-off times for various settlement systems and coordination requirements.
    /// Can appear multiple times for different timing requirements.
    ///
    /// **Usage**: Optional, used for time-sensitive transfers  
    /// **Multiple**: Can appear multiple times for different systems  
    /// **Format**: Time + UTC offsets for coordination  
    /// **Business Value**: Enables precise timing control across settlement systems
    #[field("13C")]
    pub field_13c: Option<Vec<Field13C>>,

    /// **Ordering Institution** - Field 52A (Optional)
    ///
    /// Identifies the financial institution that is ordering the transfer.
    /// This is typically the sender's own institution or the institution
    /// acting on behalf of the ordering party.
    ///
    /// **Format**: [/account]BIC code  
    /// **Usage**: Optional, identifies ordering institution  
    /// **Routing Role**: First institution in the transfer chain
    #[field("52A")]
    pub field_52a: Option<GenericBicField>,

    /// **Sender's Correspondent** - Field 53A (Optional)
    ///
    /// Identifies the correspondent bank of the sending institution.
    /// Used in correspondent banking arrangements where direct settlement
    /// relationships may not exist between sender and receiver.
    ///
    /// **Format**: [/account]BIC code  
    /// **Usage**: Optional, facilitates correspondent banking  
    /// **Routing Role**: Intermediate institution in transfer chain
    #[field("53A")]
    pub field_53a: Option<GenericBicField>,

    /// **Receiver's Correspondent** - Field 54A (Optional)
    ///
    /// Identifies the correspondent bank of the receiving institution.
    /// Critical for cross-border transfers where direct correspondent
    /// relationships may not exist between institutions.
    ///
    /// **Format**: [/account]BIC code  
    /// **Usage**: Optional, enables correspondent banking  
    /// **Routing Role**: Receiving-side correspondent institution
    #[field("54A")]
    pub field_54a: Option<GenericBicField>,

    /// **Intermediary Institution** - Field 56A (Optional)
    ///
    /// Identifies an intermediary bank in the transfer routing chain.
    /// Acts as a pass-through institution between correspondents or
    /// provides additional routing capabilities.
    ///
    /// **Format**: [/account]BIC code  
    /// **Usage**: Optional, facilitates complex routing  
    /// **Routing Role**: Intermediate routing institution
    #[field("56A")]
    pub field_56a: Option<GenericBicField>,

    /// **Account With Institution** - Field 57A (Optional)
    ///
    /// Identifies the institution where the beneficiary institution
    /// maintains its account. Used for indirect settlement arrangements
    /// where the beneficiary institution settles through another bank.
    ///
    /// **Format**: [/account]BIC code  
    /// **Usage**: Optional, enables indirect settlement  
    /// **Settlement Role**: Settlement institution for beneficiary
    #[field("57A")]
    pub field_57a: Option<GenericBicField>,

    /// **Sender to Receiver Information** - Field 72 (Optional)
    ///
    /// Free-format field for processing instructions, operational information,
    /// and special handling requirements. Critical for conveying processing
    /// context and operational requirements.
    ///
    /// **Format**: Up to 6 lines of 35 characters each  
    /// **Usage**: Optional, provides processing context  
    /// **Content**: Instructions, references, operational information
    #[field("72")]
    pub field_72: Option<Field72>,

    /// **Ordering Customer** - Field 50A (Optional, Cover Messages)
    ///
    /// Identifies the ultimate ordering customer when MT202 is used as a cover
    /// message for customer credit transfers. This field links the institutional
    /// transfer to the underlying customer transaction.
    ///
    /// **Usage**: Optional, mandatory for MT202.COV  
    /// **Cover Role**: Ultimate originator of underlying transaction  
    /// **Format**: [/account]BIC or name/address
    #[field("50A")]
    pub field_50a: Option<Field50>,

    /// **Beneficiary Customer** - Field 59A (Optional, Cover Messages)
    ///
    /// Identifies the ultimate beneficiary customer when MT202 is used as a cover
    /// message. This field identifies the final recipient of the underlying
    /// customer transfer being covered.
    ///
    /// **Usage**: Optional, used in cover messages  
    /// **Cover Role**: Ultimate beneficiary of underlying transaction  
    /// **Format**: [/account]BIC or name/address
    #[field("59A")]
    pub field_59a: Option<GenericBicField>,

    /// **Remittance Information** - Field 70 (Optional, Cover Messages)
    ///
    /// Provides details about the purpose and context of the underlying
    /// customer transfer when MT202 is used as a cover message. Contains
    /// payment references and remittance details.
    ///
    /// **Usage**: Optional, used in cover messages  
    /// **Format**: Up to 4 lines of 35 characters each  
    /// **Purpose**: Payment description, invoice numbers, contract references
    #[field("70")]
    pub field_70: Option<Field70>,

    /// **Currency/Instructed Amount** - Field 33B (Optional, Cover Messages)
    ///
    /// Original amount and currency as instructed by the ordering customer
    /// when different from the settlement amount in Field 32A. Used in
    /// cross-currency cover scenarios.
    ///
    /// **Usage**: Optional, for cross-currency covers  
    /// **Format**: Currency code + Amount  
    /// **Relationship**: May differ from Field 32A for FX transactions
    #[field("33B")]
    pub field_33b: Option<GenericCurrencyAmountField>,

    /// **Ordering Institution** - Field 52A Sequence B (Optional, Cover Messages)
    ///
    /// Identifies the ordering institution in the underlying customer transaction
    /// when MT202 is used as a cover message. This can be different from the
    /// institutional ordering institution in Sequence A.
    ///
    /// **Usage**: Optional, MT202.COV Sequence B only  
    /// **Cover Role**: Ordering institution for underlying customer transaction  
    /// **Difference**: Distinct from Sequence A Field 52A (institutional context)
    #[field("52A_SEQ_B")]
    pub field_52a_seq_b: Option<GenericBicField>,

    /// **Intermediary Institution** - Field 56A Sequence B (Optional, Cover Messages)
    ///
    /// Identifies an intermediary institution in the underlying customer transaction
    /// routing chain when MT202 is used as a cover message. This provides the
    /// customer transaction routing context separate from institutional routing.
    ///
    /// **Usage**: Optional, MT202.COV Sequence B only  
    /// **Cover Role**: Intermediary for underlying customer transaction  
    /// **Routing**: Customer transaction routing, not institutional routing
    #[field("56A_SEQ_B")]
    pub field_56a_seq_b: Option<GenericBicField>,

    /// **Account With Institution** - Field 57A Sequence B (Optional, Cover Messages)
    ///
    /// Identifies the account with institution in the underlying customer transaction
    /// when MT202 is used as a cover message. This specifies where the beneficiary
    /// customer's account is held in the underlying transaction.
    ///
    /// **Usage**: Optional, MT202.COV Sequence B only  
    /// **Cover Role**: Beneficiary's bank for underlying customer transaction  
    /// **Context**: Customer transaction settlement, not institutional settlement
    #[field("57A_SEQ_B")]
    pub field_57a_seq_b: Option<GenericBicField>,

    /// **Sender to Receiver Information** - Field 72 Sequence B (Optional, Cover Messages)
    ///
    /// Provides additional processing instructions and information specific to
    /// the underlying customer transaction when MT202 is used as a cover message.
    /// This is separate from institutional processing instructions in Sequence A.
    ///
    /// **Usage**: Optional, MT202.COV Sequence B only  
    /// **Cover Role**: Customer transaction processing instructions  
    /// **Content**: Customer-specific instructions, references, operational information
    #[field("72_SEQ_B")]
    pub field_72_seq_b: Option<Field72>,
}

impl MT202 {
    /// Create a new MT202 with required fields only
    pub fn new(
        field_20: Field20,
        field_21: Field21,
        field_32a: Field32A,
        field_58a: GenericBicField,
    ) -> Self {
        Self {
            field_20,
            field_21,
            field_32a,
            field_58a,
            field_13c: None,
            field_52a: None,
            field_53a: None,
            field_54a: None,
            field_56a: None,
            field_57a: None,
            field_72: None,
            field_50a: None,
            field_59a: None,
            field_70: None,
            field_33b: None,
            field_52a_seq_b: None,
            field_56a_seq_b: None,
            field_57a_seq_b: None,
            field_72_seq_b: None,
        }
    }

    /// Create a new MT202 with all fields for complete functionality
    #[allow(clippy::too_many_arguments)]
    pub fn new_complete(
        field_20: Field20,
        field_21: Field21,
        field_32a: Field32A,
        field_58a: GenericBicField,
        field_13c: Option<Vec<Field13C>>,
        field_52a: Option<GenericBicField>,
        field_53a: Option<GenericBicField>,
        field_54a: Option<GenericBicField>,
        field_56a: Option<GenericBicField>,
        field_57a: Option<GenericBicField>,
        field_72: Option<Field72>,
        field_50a: Option<Field50>,
        field_59a: Option<GenericBicField>,
        field_70: Option<Field70>,
        field_33b: Option<GenericCurrencyAmountField>,
        field_52a_seq_b: Option<GenericBicField>,
        field_56a_seq_b: Option<GenericBicField>,
        field_57a_seq_b: Option<GenericBicField>,
        field_72_seq_b: Option<Field72>,
    ) -> Self {
        Self {
            field_20,
            field_21,
            field_32a,
            field_58a,
            field_13c,
            field_52a,
            field_53a,
            field_54a,
            field_56a,
            field_57a,
            field_72,
            field_50a,
            field_59a,
            field_70,
            field_33b,
            field_52a_seq_b,
            field_56a_seq_b,
            field_57a_seq_b,
            field_72_seq_b,
        }
    }

    /// Get the transaction reference
    pub fn transaction_reference(&self) -> &str {
        self.field_20.transaction_reference()
    }

    /// Get the related reference
    pub fn related_reference(&self) -> &str {
        self.field_21.related_reference()
    }

    /// Get the currency code
    pub fn currency_code(&self) -> &str {
        self.field_32a.currency_code()
    }

    /// Get the transaction amount as decimal
    pub fn amount_decimal(&self) -> f64 {
        self.field_32a.amount_decimal()
    }

    /// Get the beneficiary institution BIC
    pub fn beneficiary_institution_bic(&self) -> &str {
        self.field_58a.bic()
    }

    /// Get time indications if present
    pub fn time_indications(&self) -> Option<&Vec<Field13C>> {
        self.field_13c.as_ref()
    }

    /// Get ordering institution if present
    pub fn ordering_institution(&self) -> Option<&GenericBicField> {
        self.field_52a.as_ref()
    }

    /// Get sender's correspondent if present
    pub fn senders_correspondent(&self) -> Option<&GenericBicField> {
        self.field_53a.as_ref()
    }

    /// Get receiver's correspondent if present
    pub fn receivers_correspondent(&self) -> Option<&GenericBicField> {
        self.field_54a.as_ref()
    }

    /// Get intermediary institution if present
    pub fn intermediary_institution(&self) -> Option<&GenericBicField> {
        self.field_56a.as_ref()
    }

    /// Get account with institution if present
    pub fn account_with_institution(&self) -> Option<&GenericBicField> {
        self.field_57a.as_ref()
    }

    /// Get sender to receiver information if present
    pub fn sender_to_receiver_info(&self) -> Option<&Field72> {
        self.field_72.as_ref()
    }

    /// Get ordering customer if present (cover message)
    pub fn ordering_customer(&self) -> Option<&Field50> {
        self.field_50a.as_ref()
    }

    /// Get beneficiary customer if present (cover message)
    pub fn beneficiary_customer(&self) -> Option<&GenericBicField> {
        self.field_59a.as_ref()
    }

    /// Get remittance information if present (cover message)
    pub fn remittance_information(&self) -> Option<&Field70> {
        self.field_70.as_ref()
    }

    /// Get instructed amount if present (cover message)
    pub fn instructed_amount(&self) -> Option<&GenericCurrencyAmountField> {
        self.field_33b.as_ref()
    }

    /// Check if this is a cover message (MT202.COV)
    ///
    /// According to METAFCT003 specification, a message is COVER when:
    /// - Both fields 53A and 54A are present AND
    /// - Field 53A BIC ≠ Message Sender BIC OR Field 54A BIC ≠ Message Receiver BIC
    ///
    /// # Parameters
    /// - `sender_bic`: The BIC of the message sender (from SWIFT message header)
    /// - `receiver_bic`: The BIC of the message receiver (from SWIFT message header)
    pub fn is_cover_message(&self, sender_bic: &str, receiver_bic: &str) -> bool {
        if let (Some(field_53a), Some(field_54a)) = (&self.field_53a, &self.field_54a) {
            // Get first 6 characters for BIC comparison (institution identifier)
            let message_sender_prefix = if sender_bic.len() >= 6 {
                &sender_bic[0..6]
            } else {
                sender_bic
            };
            let message_receiver_prefix = if receiver_bic.len() >= 6 {
                &receiver_bic[0..6]
            } else {
                receiver_bic
            };

            // Get correspondent BICs from fields 53A and 54A
            let field_53a_bic = field_53a.bic();
            let field_54a_bic = field_54a.bic();

            let field_53a_prefix = if field_53a_bic.len() >= 6 {
                &field_53a_bic[0..6]
            } else {
                field_53a_bic
            };
            let field_54a_prefix = if field_54a_bic.len() >= 6 {
                &field_54a_bic[0..6]
            } else {
                field_54a_bic
            };

            // Cover logic: If 53A ≠ Sender OR 54A ≠ Receiver, then it's COVER
            field_53a_prefix != message_sender_prefix || field_54a_prefix != message_receiver_prefix
        } else {
            false // No correspondent banks = not a cover
        }
    }

    /// Check if this is a cover message using ordering/beneficiary institutions as fallback
    ///
    /// This is a convenience method when message header BICs are not available.
    /// Uses field_52a (ordering institution) as sender and field_58a (beneficiary institution) as receiver.
    pub fn is_cover_message_from_fields(&self) -> bool {
        // Use ordering institution (52A) as sender, or empty string if not present
        let sender_bic = self.field_52a.as_ref().map(|f| f.bic()).unwrap_or("");

        // Use beneficiary institution (58A) as receiver
        let receiver_bic = self.field_58a.bic();

        self.is_cover_message(sender_bic, receiver_bic)
    }

    /// Check if this is a cross-currency transfer
    pub fn is_cross_currency(&self) -> bool {
        if let Some(field_33b) = &self.field_33b {
            field_33b.currency() != self.field_32a.currency_code()
        } else {
            false
        }
    }

    /// Get the message variant type
    ///
    /// Uses the fallback method since header BICs are not available at this level
    pub fn get_variant(&self) -> &'static str {
        if self.is_cover_message_from_fields() {
            "MT202.COV"
        } else {
            "MT202"
        }
    }

    /// Get all institution fields in routing order
    pub fn get_routing_chain(&self) -> Vec<(&str, String)> {
        let mut chain = Vec::new();

        // Ordering Institution
        if let Some(field_52a) = &self.field_52a {
            chain.push(("Ordering Institution", field_52a.bic().to_string()));
        }

        // Sender's Correspondent
        if let Some(field_53a) = &self.field_53a {
            chain.push(("Sender's Correspondent", field_53a.bic().to_string()));
        }

        // Receiver's Correspondent
        if let Some(field_54a) = &self.field_54a {
            chain.push(("Receiver's Correspondent", field_54a.bic().to_string()));
        }

        // Intermediary Institution
        if let Some(field_56a) = &self.field_56a {
            chain.push(("Intermediary", field_56a.bic().to_string()));
        }

        // Account With Institution
        if let Some(field_57a) = &self.field_57a {
            chain.push(("Account With Institution", field_57a.bic().to_string()));
        }

        // Beneficiary Institution
        chain.push(("Beneficiary Institution", self.field_58a.bic().to_string()));

        chain
    }

    /// Check if all required fields are present and valid
    pub fn validate_structure(&self) -> bool {
        // All required fields are enforced by the struct
        true
    }

    /// Validate cover message requirements
    pub fn validate_cover_message(&self) -> bool {
        if self.is_cover_message_from_fields() {
            // Cover messages should have meaningful cover information
            self.field_50a.is_some() || self.field_59a.is_some() || self.field_70.is_some()
        } else {
            true
        }
    }

    /// Get all time indications with descriptions
    pub fn get_time_indications_with_descriptions(&self) -> Vec<String> {
        if let Some(time_indications) = &self.field_13c {
            time_indications
                .iter()
                .map(|field| field.description())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Check if message has CLS timing requirements
    pub fn has_cls_timing(&self) -> bool {
        if let Some(time_indications) = &self.field_13c {
            time_indications.iter().any(|field| field.is_cls_time())
        } else {
            false
        }
    }

    /// Check if message has TARGET timing requirements
    pub fn has_target_timing(&self) -> bool {
        if let Some(time_indications) = &self.field_13c {
            time_indications.iter().any(|field| field.is_target_time())
        } else {
            false
        }
    }

    /// Get processing instructions from field 72
    pub fn get_processing_instructions(&self) -> Vec<String> {
        if let Some(field_72) = &self.field_72 {
            field_72.information.clone()
        } else {
            Vec::new()
        }
    }

    // ================================
    // SEQUENCE B FIELD ACCESSORS (MT202.COV Cover Message Support)
    // ================================

    /// Get ordering institution from sequence B if present (cover message)
    pub fn ordering_institution_seq_b(&self) -> Option<&GenericBicField> {
        self.field_52a_seq_b.as_ref()
    }

    /// Get intermediary institution from sequence B if present (cover message)
    pub fn intermediary_institution_seq_b(&self) -> Option<&GenericBicField> {
        self.field_56a_seq_b.as_ref()
    }

    /// Get account with institution from sequence B if present (cover message)
    pub fn account_with_institution_seq_b(&self) -> Option<&GenericBicField> {
        self.field_57a_seq_b.as_ref()
    }

    /// Get sender to receiver information from sequence B if present (cover message)
    pub fn sender_to_receiver_info_seq_b(&self) -> Option<&Field72> {
        self.field_72_seq_b.as_ref()
    }

    /// Get customer transaction routing chain from sequence B (cover message)
    pub fn get_customer_routing_chain(&self) -> Vec<(&str, String)> {
        let mut chain = Vec::new();

        // Sequence B Ordering Institution
        if let Some(field_52a) = &self.field_52a_seq_b {
            chain.push(("Customer Ordering Institution", field_52a.bic().to_string()));
        }

        // Sequence B Intermediary Institution
        if let Some(field_56a) = &self.field_56a_seq_b {
            chain.push(("Customer Intermediary", field_56a.bic().to_string()));
        }

        // Sequence B Account With Institution
        if let Some(field_57a) = &self.field_57a_seq_b {
            chain.push((
                "Customer Account With Institution",
                field_57a.bic().to_string(),
            ));
        }

        chain
    }

    /// Get processing instructions from sequence B field 72 (cover message)
    pub fn get_customer_processing_instructions(&self) -> Vec<String> {
        if let Some(field_72_seq_b) = &self.field_72_seq_b {
            field_72_seq_b.information.clone()
        } else {
            Vec::new()
        }
    }

    /// Check if this message has RETN (return) indicators in field 72
    pub fn has_return_codes(&self) -> bool {
        // Check both sequence A and sequence B field 72 for return codes
        let seq_a_has_return = if let Some(field_72) = &self.field_72 {
            field_72
                .information
                .iter()
                .any(|line| line.contains("/RETN/") || line.to_uppercase().contains("RETURN"))
        } else {
            false
        };

        let seq_b_has_return = if let Some(field_72_seq_b) = &self.field_72_seq_b {
            field_72_seq_b
                .information
                .iter()
                .any(|line| line.contains("/RETN/") || line.to_uppercase().contains("RETURN"))
        } else {
            false
        };

        seq_a_has_return || seq_b_has_return
    }

    /// Check if this message has REJT (reject) indicators in field 72
    pub fn has_reject_codes(&self) -> bool {
        // Check both sequence A and sequence B field 72 for reject codes
        let seq_a_has_reject = if let Some(field_72) = &self.field_72 {
            field_72
                .information
                .iter()
                .any(|line| line.contains("/REJT/") || line.to_uppercase().contains("REJECT"))
        } else {
            false
        };

        let seq_b_has_reject = if let Some(field_72_seq_b) = &self.field_72_seq_b {
            field_72_seq_b
                .information
                .iter()
                .any(|line| line.contains("/REJT/") || line.to_uppercase().contains("REJECT"))
        } else {
            false
        };

        seq_a_has_reject || seq_b_has_reject
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftMessageBody;
    use chrono::NaiveDate;

    #[test]
    fn test_mt202_creation() {
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_58a = GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap();

        let mt202 = MT202::new(field_20, field_21, field_32a, field_58a);

        assert_eq!(mt202.transaction_reference(), "FT21234567890");
        assert_eq!(mt202.related_reference(), "REL20241201001");
        assert_eq!(mt202.currency_code(), "USD");
        assert_eq!(mt202.amount_decimal(), 1000000.00);
        assert_eq!(mt202.beneficiary_institution_bic(), "DEUTDEFFXXX");
    }

    #[test]
    fn test_mt202_with_multiple_field13c() {
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_58a = GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap();

        // Create multiple Field13C instances with correct constructor signature
        let field_13c_1 = Field13C::new("090000+0", "+0100", "+0900").unwrap();
        let field_13c_2 = Field13C::new("150000+1", "+0100", "-0500").unwrap();
        let field_13c_vec = vec![field_13c_1, field_13c_2];

        let mt202 = MT202::new_complete(
            field_20,
            field_21,
            field_32a,
            field_58a,
            Some(field_13c_vec),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        // Verify the structure
        assert!(mt202.field_13c.is_some());
        let field_13c_instances = mt202.field_13c.as_ref().unwrap();
        assert_eq!(field_13c_instances.len(), 2);
        assert_eq!(field_13c_instances[0].time(), "090000+0");
        assert_eq!(field_13c_instances[1].time(), "150000+1");
    }

    #[test]
    fn test_mt202_message_type() {
        assert_eq!(MT202::message_type(), "202");
    }

    #[test]
    fn test_mt202_required_fields() {
        let required = MT202::required_fields();
        assert!(required.contains(&"20"));
        assert!(required.contains(&"21"));
        assert!(required.contains(&"32A"));
        assert!(required.contains(&"58A"));
    }

    #[test]
    fn test_mt202_optional_fields() {
        let optional = MT202::optional_fields();
        assert!(optional.contains(&"13C"));
        assert!(optional.contains(&"52A"));
        assert!(optional.contains(&"53A"));
        assert!(optional.contains(&"54A"));
        assert!(optional.contains(&"56A"));
        assert!(optional.contains(&"57A"));
        assert!(optional.contains(&"72"));
        assert!(optional.contains(&"50A"));
        assert!(optional.contains(&"59A"));
        assert!(optional.contains(&"70"));
        assert!(optional.contains(&"33B"));
    }

    #[test]
    fn test_mt202_cover_message_detection() {
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_58a = GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap();
        let field_50a = Field50::K(Field50K::new(vec!["JOHN DOE".to_string()]).unwrap());

        // Standard MT202
        let mt202_standard = MT202::new(
            field_20.clone(),
            field_21.clone(),
            field_32a.clone(),
            field_58a.clone(),
        );
        // Test with sample BICs - should not be cover since no 53A/54A fields
        assert!(!mt202_standard.is_cover_message("BANKUS33XXX", "BANKDE55XXX"));
        assert_eq!(mt202_standard.get_variant(), "MT202");

        // MT202.COV with ordering customer and correspondent banks
        let field_53a = GenericBicField::new(None, None, "CHASUS33XXX").unwrap();
        let field_54a = GenericBicField::new(None, None, "RBOSGGSGXXX").unwrap();

        let mt202_cover = MT202::new_complete(
            field_20,
            field_21,
            field_32a,
            field_58a,
            None,
            None,
            Some(field_53a),
            Some(field_54a),
            None,
            None,
            None,
            Some(field_50a),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        // Test with different sender/receiver BICs to trigger cover detection
        assert!(mt202_cover.is_cover_message("BANKUS33XXX", "BANKDE55XXX"));
        assert_eq!(mt202_cover.get_variant(), "MT202.COV");
    }

    #[test]
    fn test_mt202_routing_chain() {
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_58a = GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap();
        let field_52a = GenericBicField::new(None, None, "CHASUS33XXX").unwrap();
        let field_53a = GenericBicField::new(None, None, "BARCGB22XXX").unwrap();

        let mt202 = MT202::new_complete(
            field_20,
            field_21,
            field_32a,
            field_58a,
            None,
            Some(field_52a),
            Some(field_53a),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let routing_chain = mt202.get_routing_chain();
        assert_eq!(routing_chain.len(), 3);
        assert_eq!(
            routing_chain[0],
            ("Ordering Institution", "CHASUS33XXX".to_string())
        );
        assert_eq!(
            routing_chain[1],
            ("Sender's Correspondent", "BARCGB22XXX".to_string())
        );
        assert_eq!(
            routing_chain[2],
            ("Beneficiary Institution", "DEUTDEFFXXX".to_string())
        );
    }

    #[test]
    fn test_mt202_cross_currency() {
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_58a = GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap();
        let field_33b = GenericCurrencyAmountField::new("EUR", 850000.00).unwrap();

        let mt202 = MT202::new_complete(
            field_20,
            field_21,
            field_32a,
            field_58a,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(field_33b),
            None,
            None,
            None,
            None,
        );

        assert!(mt202.is_cross_currency());
        assert!(mt202.instructed_amount().is_some());
    }

    #[test]
    fn test_mt202_timing_detection() {
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_58a = GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap();

        // Create time indications with CLS and TARGET timing
        let cls_time = Field13C::new("153045+1", "+0100", "-0500").unwrap();
        let target_time = Field13C::new("090000+0", "+0100", "+0900").unwrap();
        let time_indications = vec![cls_time, target_time];

        let mt202 = MT202::new_complete(
            field_20,
            field_21,
            field_32a,
            field_58a,
            Some(time_indications),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert!(mt202.has_cls_timing());
        assert!(mt202.has_target_timing());

        let descriptions = mt202.get_time_indications_with_descriptions();
        assert_eq!(descriptions.len(), 2);
        assert!(descriptions[0].contains("CLS Bank cut-off time"));
        assert!(descriptions[1].contains("TARGET system time"));
    }

    #[test]
    fn test_mt202_return_reject_codes() {
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_58a = GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap();

        // Test with return codes in sequence A field 72
        let field_72_return = Field72::new(vec![
            "/RETN/AC01/Account id incorrect".to_string(),
            "Additional return information".to_string(),
        ])
        .unwrap();

        let mt202_return = MT202::new_complete(
            field_20.clone(),
            field_21.clone(),
            field_32a.clone(),
            field_58a.clone(),
            None,
            None,
            None,
            None,
            None,
            None,
            Some(field_72_return),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert!(mt202_return.has_return_codes());
        assert!(!mt202_return.has_reject_codes());

        // Test with reject codes in sequence B field 72
        let field_72_reject = Field72::new(vec![
            "/REJT/AC03/Account id invalid".to_string(),
            "Additional reject information".to_string(),
        ])
        .unwrap();

        let mt202_reject = MT202::new_complete(
            field_20.clone(),
            field_21.clone(),
            field_32a.clone(),
            field_58a.clone(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(field_72_reject),
        );

        assert!(!mt202_reject.has_return_codes());
        assert!(mt202_reject.has_reject_codes());

        // Test without return/reject codes
        let field_72_normal = Field72::new(vec![
            "/INT/Internal transfer own accts".to_string(),
            "Regular processing instructions".to_string(),
        ])
        .unwrap();

        let mt202_normal = MT202::new_complete(
            field_20,
            field_21,
            field_32a,
            field_58a,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(field_72_normal),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert!(!mt202_normal.has_return_codes());
        assert!(!mt202_normal.has_reject_codes());
    }

    #[test]
    fn test_mt202_cover_payment_alias() {
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_58a = GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap();
        let field_50a = Field50::K(Field50K::new(vec!["ORDERING CUSTOMER".to_string()]).unwrap());

        // Test cover payment fields presence
        let mt202_cover = MT202::new_complete(
            field_20,
            field_21,
            field_32a,
            field_58a,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(field_50a),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        // Test that cover payment fields are accessible
        assert!(mt202_cover.ordering_customer().is_some());
        assert!(mt202_cover.validate_cover_message());

        // Cover message detection requires correspondent banks or explicit BIC comparison
        // Having just customer fields doesn't make it a cover message by the SWIFT standard
        assert!(!mt202_cover.is_cover_message_from_fields());
    }
}
