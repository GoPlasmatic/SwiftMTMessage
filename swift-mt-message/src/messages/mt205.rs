use crate::{SwiftMessage, fields::*, swift_serde};
use serde::{Deserialize, Serialize};

/// # MT205: General Financial Institution Transfer
///
/// ## Overview
/// MT205 enables financial institutions to transfer funds between themselves for their own
/// account or for the account of their customers. Similar to MT202 but with key structural
/// differences: field 54a is not present and field 52a is always mandatory. MT205 is used
/// in correspondent banking relationships for institutional transfers and cover payments.
///
/// ## Message Type Specification
/// **Message Type**: `205`  
/// **Category**: Financial Institution Transfers (Category 2)  
/// **Usage**: General Financial Institution Transfer (Enhanced)  
/// **Processing**: Real-time gross settlement (RTGS) and net settlement  
/// **Network**: SWIFT FIN (Financial network)  
///
/// ### Key Differences from MT202
/// - **Field 54a**: Not present in MT205 (completely absent)
/// - **Field 52a**: Always mandatory (no fallback to sender BIC)
/// - **Settlement Logic**: Uses METAFCT003 (simplified scenarios)
/// - **Cover Detection**: No PREC003, based on Sequence B presence
///
/// ### Message Variants
/// ```text
/// MT205        - Standard financial institution transfer
/// MT205.COV    - Cover message for customer credit transfers
/// ```
///
/// ## Message Structure
/// The MT205 message consists of mandatory and optional fields organized in specific sequences:
///
/// ### Mandatory Fields (Core Requirements)
/// - **Field 20**: Transaction Reference Number (sender's unique reference)
/// - **Field 21**: Related Reference (link to previous message or transaction)
/// - **Field 32A**: Value Date/Currency/Amount (settlement details)
/// - **Field 52A**: Ordering Institution (always mandatory in MT205)
/// - **Field 58A**: Beneficiary Institution (final receiving institution)
///
/// ### Optional Fields (Enhanced Processing)
/// ```text
/// Field 13C   - Time Indication (processing timing)
/// Field 53A   - Sender's Correspondent (intermediate bank)
/// Field 56A   - Intermediary Institution (routing bank)
/// Field 57A   - Account With Institution (beneficiary's correspondent)
/// Field 72    - Sender to Receiver Information (processing instructions)
/// ```
///
/// ### MT205.COV Specific Fields (Cover Message)
/// When used as a cover message for customer transfers:
/// ```text
/// Field 50A   - Ordering Customer (ultimate originator)
/// Field 59A   - Beneficiary Customer (ultimate recipient)
/// Field 70    - Remittance Information (payment details)
/// Field 33B   - Currency/Instructed Amount (original amount)
/// ```
///
/// ## Settlement Method Determination (METAFCT003)
/// MT205 uses simplified settlement logic due to absence of field 54a:
/// - **Both 53a and 54a absent**: INDA (54a never exists in MT205)
/// - **53a present and 54a absent**: INDA with account settlement
/// - **Cover scenarios**: Based on Sequence B customer fields presence
#[swift_serde]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[swift_message(mt = "205")]
pub struct MT205 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique sender's reference identifying this specific financial institution transfer.
    /// Used throughout the transfer lifecycle for tracking, reconciliation, and audit.
    /// Must be unique within the sender's system per business day.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory in all MT205 variants  
    /// **Business Rule**: Should follow sender's reference numbering scheme
    #[field("20")]
    pub field_20: Field20,

    /// **Related Reference** - Field 21
    ///
    /// Reference to a related message or transaction that this MT205 is associated with.
    /// Critical for linking cover payments to underlying customer transfers and for
    /// maintaining audit trails across related transactions.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory in all MT205 messages  
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

    /// **Ordering Institution** - Field 52A (MANDATORY in MT205)
    ///
    /// Identifies the financial institution that is ordering the transfer.
    /// Unlike MT202, this field is always mandatory in MT205 with no fallback
    /// to sender BIC from message header.
    ///
    /// **Format**: [/account]BIC code  
    /// **Usage**: MANDATORY in all MT205 messages  
    /// **MT205 Rule**: No fallback to sender BIC (always required)
    /// **Routing Role**: First institution in the transfer chain
    #[field("52A")]
    pub field_52a: Field52A,

    /// **Beneficiary Institution** - Field 58A
    ///
    /// Identifies the financial institution that will receive the funds being transferred.
    /// This is the final destination institution in the transfer chain and must be
    /// clearly identified for successful settlement.
    ///
    /// **Format**: [/account]BIC code  
    /// **Usage**: Mandatory in all MT205 messages  
    /// **Settlement**: Final destination for fund settlement
    #[field("58A")]
    pub field_58a: Field58A,

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

    /// **Sender's Correspondent** - Field 53A (Optional)
    ///
    /// Identifies the correspondent bank of the sending institution.
    /// Used in correspondent banking arrangements where direct settlement
    /// relationships may not exist between sender and receiver.
    ///
    /// **Format**: [/account]BIC code  
    /// **Usage**: Optional, facilitates correspondent banking  
    /// **Routing Role**: Intermediate institution in transfer chain
    /// **MT205 Note**: Uses METAFCT003 for settlement method determination
    #[field("53A")]
    pub field_53a: Option<Field53A>,

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
    pub field_56a: Option<Field56A>,

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
    pub field_57a: Option<Field57A>,

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
    /// Identifies the ultimate ordering customer when MT205 is used as a cover
    /// message for customer credit transfers. This field links the institutional
    /// transfer to the underlying customer transaction.
    ///
    /// **Usage**: Optional, mandatory for MT205.COV  
    /// **Cover Role**: Ultimate originator of underlying transaction  
    /// **Format**: [/account]BIC or name/address
    #[field("50A")]
    pub field_50a: Option<Field50>,

    /// **Beneficiary Customer** - Field 59A (Optional, Cover Messages)
    ///
    /// Identifies the ultimate beneficiary customer when MT205 is used as a cover
    /// message. This field identifies the final recipient of the underlying
    /// customer transfer being covered.
    ///
    /// **Usage**: Optional, used in cover messages  
    /// **Cover Role**: Ultimate beneficiary of underlying transaction  
    /// **Format**: [/account]BIC or name/address
    #[field("59A")]
    pub field_59a: Option<Field59A>,

    /// **Remittance Information** - Field 70 (Optional, Cover Messages)
    ///
    /// Provides details about the purpose and context of the underlying
    /// customer transfer when MT205 is used as a cover message. Contains
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
    pub field_33b: Option<Field33B>,

    /// **Ordering Institution** - Field 52A Sequence B (Optional, Cover Messages)
    ///
    /// Identifies the ordering institution in the underlying customer transaction
    /// when MT205 is used as a cover message. This can be different from the
    /// institutional ordering institution in Sequence A.
    ///
    /// **Usage**: Optional, MT205.COV Sequence B only  
    /// **Cover Role**: Ordering institution for underlying customer transaction  
    /// **Difference**: Distinct from Sequence A Field 52A (institutional context)
    #[field("52A_SEQ_B")]
    pub field_52a_seq_b: Option<Field52A>,

    /// **Intermediary Institution** - Field 56A Sequence B (Optional, Cover Messages)
    ///
    /// Identifies an intermediary institution in the underlying customer transaction
    /// routing chain when MT205 is used as a cover message. This provides the
    /// customer transaction routing context separate from institutional routing.
    ///
    /// **Usage**: Optional, MT205.COV Sequence B only  
    /// **Cover Role**: Intermediary for underlying customer transaction  
    /// **Routing**: Customer transaction routing, not institutional routing
    #[field("56A_SEQ_B")]
    pub field_56a_seq_b: Option<Field56A>,

    /// **Account With Institution** - Field 57A Sequence B (Optional, Cover Messages)
    ///
    /// Identifies the account with institution in the underlying customer transaction
    /// when MT205 is used as a cover message. This specifies where the beneficiary
    /// customer's account is held in the underlying transaction.
    ///
    /// **Usage**: Optional, MT205.COV Sequence B only  
    /// **Cover Role**: Beneficiary's bank for underlying customer transaction  
    /// **Context**: Customer transaction settlement, not institutional settlement
    #[field("57A_SEQ_B")]
    pub field_57a_seq_b: Option<Field57A>,

    /// **Sender to Receiver Information** - Field 72 Sequence B (Optional, Cover Messages)
    ///
    /// Provides additional processing instructions and information specific to
    /// the underlying customer transaction when MT205 is used as a cover message.
    /// This is separate from institutional processing instructions in Sequence A.
    ///
    /// **Usage**: Optional, MT205.COV Sequence B only  
    /// **Cover Role**: Customer transaction processing instructions  
    /// **Content**: Customer-specific instructions, references, operational information
    #[field("72_SEQ_B")]
    pub field_72_seq_b: Option<Field72>,
}

impl MT205 {
    /// Create a new MT205 with required fields only
    pub fn new(
        field_20: Field20,
        field_21: Field21,
        field_32a: Field32A,
        field_52a: Field52A, // Mandatory in MT205
        field_58a: Field58A,
    ) -> Self {
        Self {
            field_20,
            field_21,
            field_32a,
            field_52a,
            field_58a,
            field_13c: None,
            field_53a: None,
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

    /// Create a new MT205 with all fields for complete functionality
    #[allow(clippy::too_many_arguments)]
    pub fn new_complete(
        field_20: Field20,
        field_21: Field21,
        field_32a: Field32A,
        field_52a: Field52A, // Mandatory in MT205
        field_58a: Field58A,
        field_13c: Option<Vec<Field13C>>,
        field_53a: Option<Field53A>,
        field_56a: Option<Field56A>,
        field_57a: Option<Field57A>,
        field_72: Option<Field72>,
        field_50a: Option<Field50>,
        field_59a: Option<Field59A>,
        field_70: Option<Field70>,
        field_33b: Option<Field33B>,
        field_52a_seq_b: Option<Field52A>,
        field_56a_seq_b: Option<Field56A>,
        field_57a_seq_b: Option<Field57A>,
        field_72_seq_b: Option<Field72>,
    ) -> Self {
        Self {
            field_20,
            field_21,
            field_32a,
            field_52a,
            field_58a,
            field_13c,
            field_53a,
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

    /// Get the ordering institution BIC (always present in MT205)
    pub fn ordering_institution_bic(&self) -> &str {
        self.field_52a.bic()
    }

    /// Get the beneficiary institution BIC
    pub fn beneficiary_institution_bic(&self) -> &str {
        self.field_58a.bic()
    }

    /// Get time indications if present
    pub fn time_indications(&self) -> Option<&Vec<Field13C>> {
        self.field_13c.as_ref()
    }

    /// Get ordering institution (always present in MT205)
    pub fn ordering_institution(&self) -> &Field52A {
        &self.field_52a
    }

    /// Get sender's correspondent if present
    pub fn senders_correspondent(&self) -> Option<&Field53A> {
        self.field_53a.as_ref()
    }

    /// Get intermediary institution if present
    pub fn intermediary_institution(&self) -> Option<&Field56A> {
        self.field_56a.as_ref()
    }

    /// Get account with institution if present
    pub fn account_with_institution(&self) -> Option<&Field57A> {
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
    pub fn beneficiary_customer(&self) -> Option<&Field59A> {
        self.field_59a.as_ref()
    }

    /// Get remittance information if present (cover message)
    pub fn remittance_information(&self) -> Option<&Field70> {
        self.field_70.as_ref()
    }

    /// Get instructed amount if present (cover message)
    pub fn instructed_amount(&self) -> Option<&Field33B> {
        self.field_33b.as_ref()
    }

    /// Check if this is a cover message (MT205.COV)
    ///
    /// MT205 cover detection uses METAFCT003 logic with simplified scenarios:
    /// Since field 54a is not present in MT205, cover detection is based on:
    /// - Presence of Sequence B customer fields (50a, 59a, 70)
    /// - OR explicit correspondent banking chain with field 53a
    ///
    /// # Parameters
    /// - `sender_bic`: The BIC of the message sender (from SWIFT message header)
    /// - `receiver_bic`: The BIC of the message receiver (from SWIFT message header)
    pub fn is_cover_message(&self, sender_bic: &str) -> bool {
        // MT205 specific: Since field 54a is never present, we check for:
        // 1. Field 53a present and different from sender BIC
        // 2. OR presence of customer cover fields (50a, 59a, 70)

        let has_correspondent_chain = if let Some(field_53a) = &self.field_53a {
            // Get first 6 characters for BIC comparison (institution identifier)
            let message_sender_prefix = if sender_bic.len() >= 6 {
                &sender_bic[0..6]
            } else {
                sender_bic
            };

            let field_53a_bic = field_53a.bic();
            let field_53a_prefix = if field_53a_bic.len() >= 6 {
                &field_53a_bic[0..6]
            } else {
                field_53a_bic
            };

            // Different correspondent suggests cover scenario
            field_53a_prefix != message_sender_prefix
        } else {
            false
        };

        let has_customer_fields =
            self.field_50a.is_some() || self.field_59a.is_some() || self.field_70.is_some();

        has_correspondent_chain || has_customer_fields
    }

    /// Check if this is a cover message using ordering/beneficiary institutions as fallback
    ///
    /// This is a convenience method when message header BICs are not available.
    /// Uses field_52a (ordering institution) as sender and field_58a (beneficiary institution) as receiver.
    pub fn is_cover_message_from_fields(&self) -> bool {
        // MT205 specific: Uses METAFCT003 logic
        // Check for customer cover fields presence or correspondent banking
        let has_customer_fields =
            self.field_50a.is_some() || self.field_59a.is_some() || self.field_70.is_some();

        let has_correspondent = if let Some(field_53a) = &self.field_53a {
            // Compare 53a with 52a (ordering institution)
            let ordering_bic = self.field_52a.bic();
            let correspondent_bic = field_53a.bic();

            let ordering_prefix = if ordering_bic.len() >= 6 {
                &ordering_bic[0..6]
            } else {
                ordering_bic
            };
            let correspondent_prefix = if correspondent_bic.len() >= 6 {
                &correspondent_bic[0..6]
            } else {
                correspondent_bic
            };

            ordering_prefix != correspondent_prefix
        } else {
            false
        };

        has_customer_fields || has_correspondent
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
            "MT205.COV"
        } else {
            "MT205"
        }
    }

    /// Get all institution fields in routing order
    /// Note: MT205 does not have field 54a (receiver's correspondent)
    pub fn get_routing_chain(&self) -> Vec<(&str, String)> {
        let mut chain = Vec::new();

        // Ordering Institution (always present in MT205)
        chain.push(("Ordering Institution", self.field_52a.bic().to_string()));

        // Sender's Correspondent
        if let Some(field_53a) = &self.field_53a {
            chain.push(("Sender's Correspondent", field_53a.bic().to_string()));
        }

        // Note: Field 54a (Receiver's Correspondent) is not present in MT205

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
    /// MT205 has more mandatory fields than MT202
    pub fn validate_structure(&self) -> bool {
        // All required fields are enforced by the struct
        // Field 52a is mandatory in MT205 (unlike MT202)
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
    // SEQUENCE B FIELD ACCESSORS (MT205.COV Cover Message Support)
    // ================================

    /// Get ordering institution from sequence B if present (cover message)
    pub fn ordering_institution_seq_b(&self) -> Option<&Field52A> {
        self.field_52a_seq_b.as_ref()
    }

    /// Get intermediary institution from sequence B if present (cover message)
    pub fn intermediary_institution_seq_b(&self) -> Option<&Field56A> {
        self.field_56a_seq_b.as_ref()
    }

    /// Get account with institution from sequence B if present (cover message)
    pub fn account_with_institution_seq_b(&self) -> Option<&Field57A> {
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
    fn test_mt205_creation() {
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_52a = Field52A::new(None, None, "CHASUS33XXX").unwrap();
        let field_58a = Field58A::new(None, None, "DEUTDEFFXXX").unwrap();

        let mt205 = MT205::new(field_20, field_21, field_32a, field_52a, field_58a);

        assert_eq!(mt205.transaction_reference(), "FT21234567890");
        assert_eq!(mt205.related_reference(), "REL20241201001");
        assert_eq!(mt205.currency_code(), "USD");
        assert_eq!(mt205.amount_decimal(), 1000000.00);
        assert_eq!(mt205.ordering_institution_bic(), "CHASUS33XXX");
        assert_eq!(mt205.beneficiary_institution_bic(), "DEUTDEFFXXX");
    }

    #[test]
    fn test_mt205_message_type() {
        assert_eq!(MT205::message_type(), "205");
    }

    #[test]
    fn test_mt205_required_fields() {
        let required = MT205::required_fields();
        assert!(required.contains(&"20"));
        assert!(required.contains(&"21"));
        assert!(required.contains(&"32A"));
        assert!(required.contains(&"52A")); // Mandatory in MT205
        assert!(required.contains(&"58A"));
    }

    #[test]
    fn test_mt205_optional_fields() {
        let optional = MT205::optional_fields();
        assert!(optional.contains(&"13C"));
        assert!(optional.contains(&"53A"));
        // Note: 54A is not present in MT205
        assert!(!optional.contains(&"54A"));
        assert!(optional.contains(&"56A"));
        assert!(optional.contains(&"57A"));
        assert!(optional.contains(&"72"));
        assert!(optional.contains(&"50A"));
        assert!(optional.contains(&"59A"));
        assert!(optional.contains(&"70"));
        assert!(optional.contains(&"33B"));
    }

    #[test]
    fn test_mt205_cover_message_detection() {
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_52a = Field52A::new(None, None, "CHASUS33XXX").unwrap();
        let field_58a = Field58A::new(None, None, "DEUTDEFFXXX").unwrap();
        let field_50a = Field50::K(Field50K::new(vec!["JOHN DOE".to_string()]).unwrap());

        // Standard MT205
        let mt205_standard = MT205::new(
            field_20.clone(),
            field_21.clone(),
            field_32a.clone(),
            field_52a.clone(),
            field_58a.clone(),
        );
        assert_eq!(mt205_standard.get_variant(), "MT205");

        // MT205.COV with ordering customer
        let mt205_cover = MT205::new_complete(
            field_20,
            field_21,
            field_32a,
            field_52a,
            field_58a,
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
        assert!(mt205_cover.is_cover_message_from_fields());
        assert_eq!(mt205_cover.get_variant(), "MT205.COV");
    }

    #[test]
    fn test_mt205_routing_chain() {
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_52a = Field52A::new(None, None, "CHASUS33XXX").unwrap();
        let field_58a = Field58A::new(None, None, "DEUTDEFFXXX").unwrap();
        let field_53a = Field53A::new(None, None, "BARCGB22XXX").unwrap();

        let mt205 = MT205::new_complete(
            field_20,
            field_21,
            field_32a,
            field_52a,
            field_58a,
            None,
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
        );

        let routing_chain = mt205.get_routing_chain();
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
    fn test_mt205_mandatory_field_52a() {
        // MT205 always requires field 52a (unlike MT202)
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_52a = Field52A::new(None, None, "CHASUS33XXX").unwrap();
        let field_58a = Field58A::new(None, None, "DEUTDEFFXXX").unwrap();

        let mt205 = MT205::new(field_20, field_21, field_32a, field_52a, field_58a);

        // Ordering institution is always accessible (not optional)
        assert_eq!(mt205.ordering_institution().bic(), "CHASUS33XXX");
        assert_eq!(mt205.ordering_institution_bic(), "CHASUS33XXX");
    }

    #[test]
    fn test_mt205_cross_currency() {
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_52a = Field52A::new(None, None, "CHASUS33XXX").unwrap();
        let field_58a = Field58A::new(None, None, "DEUTDEFFXXX").unwrap();
        let field_33b = Field33B::new("EUR", 850000.00).unwrap();

        let mt205 = MT205::new_complete(
            field_20,
            field_21,
            field_32a,
            field_52a,
            field_58a,
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

        assert!(mt205.is_cross_currency());
        assert!(mt205.instructed_amount().is_some());
    }

    #[test]
    fn test_mt205_correspondent_cover_detection() {
        let field_20 = Field20::new("FT21234567890".to_string());
        let field_21 = Field21::new("REL20241201001".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            "USD".to_string(),
            1000000.00,
        );
        let field_52a = Field52A::new(None, None, "CHASUS33XXX").unwrap();
        let field_58a = Field58A::new(None, None, "DEUTDEFFXXX").unwrap();
        let field_53a = Field53A::new(None, None, "BARCGB22XXX").unwrap(); // Different correspondent

        let mt205 = MT205::new_complete(
            field_20,
            field_21,
            field_32a,
            field_52a,
            field_58a,
            None,
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
        );

        // Should be detected as cover due to different correspondent
        assert!(mt205.is_cover_message_from_fields());
        assert_eq!(mt205.get_variant(), "MT205.COV");
    }
}
