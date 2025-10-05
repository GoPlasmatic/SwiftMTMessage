use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// Sequence B - Transaction details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT107Transaction {
    /// Transaction reference (Field 21)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Instruction code (Field 23E)
    #[serde(rename = "23E", skip_serializing_if = "Option::is_none")]
    pub field_23e: Option<Field23E>,

    /// Mandate reference (Field 21C)
    #[serde(rename = "21C", skip_serializing_if = "Option::is_none")]
    pub field_21c: Option<Field21C>,

    /// Direct debit reference (Field 21D)
    #[serde(rename = "21D", skip_serializing_if = "Option::is_none")]
    pub field_21d: Option<Field21D>,

    /// Registration reference (Field 21E)
    #[serde(rename = "21E", skip_serializing_if = "Option::is_none")]
    pub field_21e: Option<Field21E>,

    /// Transaction amount (Field 32B)
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    /// Instructing party (Field 50C/L)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub instructing_party_tx: Option<Field50InstructingParty>,

    /// Creditor (Field 50A/K)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub creditor_tx: Option<Field50Creditor>,

    /// Creditor's bank (Field 52)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52CreditorBank>,

    /// Debtor's bank (Field 57)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_57: Option<Field57DebtorBank>,

    /// Debtor (Field 59)
    #[serde(flatten)]
    pub field_59: Field59,

    /// Remittance information (Field 70)
    #[serde(rename = "70", skip_serializing_if = "Option::is_none")]
    pub field_70: Option<Field70>,

    /// Transaction type code (Field 26T)
    #[serde(rename = "26T", skip_serializing_if = "Option::is_none")]
    pub field_26t: Option<Field26T>,

    /// Regulatory reporting (Field 77B)
    #[serde(rename = "77B", skip_serializing_if = "Option::is_none")]
    pub field_77b: Option<Field77B>,

    /// Original ordered amount (Field 33B)
    #[serde(rename = "33B", skip_serializing_if = "Option::is_none")]
    pub field_33b: Option<Field33B>,

    /// Details of charges (Field 71A)
    #[serde(rename = "71A", skip_serializing_if = "Option::is_none")]
    pub field_71a: Option<Field71A>,

    /// Sender's charges (Field 71F)
    #[serde(rename = "71F", skip_serializing_if = "Option::is_none")]
    pub field_71f: Option<Field71F>,

    /// Receiver's charges (Field 71G)
    #[serde(rename = "71G", skip_serializing_if = "Option::is_none")]
    pub field_71g: Option<Field71G>,

    /// Exchange rate (Field 36)
    #[serde(rename = "36", skip_serializing_if = "Option::is_none")]
    pub field_36: Option<Field36>,
}

/// **MT107: General Direct Debit Message**
///
/// General direct debit instruction with flexible structure and settlement details.
/// Similar to MT104 but with additional flexibility for complex scenarios.
///
/// **Usage:** General direct debits, flexible collections
/// **Category:** Category 1 (Customer Payments)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT107 {
    /// Sender's reference (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Instruction code (Field 23E)
    #[serde(rename = "23E", skip_serializing_if = "Option::is_none")]
    pub field_23e: Option<Field23E>,

    /// Registration reference (Field 21E)
    #[serde(rename = "21E", skip_serializing_if = "Option::is_none")]
    pub field_21e: Option<Field21E>,

    /// Requested execution date (Field 30)
    #[serde(rename = "30")]
    pub field_30: Field30,

    /// Sending institution (Field 51A)
    #[serde(rename = "51A", skip_serializing_if = "Option::is_none")]
    pub field_51a: Option<Field51A>,

    /// Instructing party (Field 50C/L)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub instructing_party: Option<Field50InstructingParty>,

    /// Creditor (Field 50A/K)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub creditor: Option<Field50Creditor>,

    /// Creditor's bank (Field 52)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52CreditorBank>,

    /// Transaction type code (Field 26T)
    #[serde(rename = "26T", skip_serializing_if = "Option::is_none")]
    pub field_26t: Option<Field26T>,

    /// Regulatory reporting (Field 77B)
    #[serde(rename = "77B", skip_serializing_if = "Option::is_none")]
    pub field_77b: Option<Field77B>,

    /// Details of charges (Field 71A)
    #[serde(rename = "71A", skip_serializing_if = "Option::is_none")]
    pub field_71a: Option<Field71A>,

    /// Sender to receiver information (Field 72)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,

    /// Transaction details (Sequence B)
    #[serde(rename = "#")]
    pub transactions: Vec<MT107Transaction>,

    /// Settlement amount (Field 32B, Sequence C)
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    /// Sum of amounts (Field 19)
    #[serde(rename = "19", skip_serializing_if = "Option::is_none")]
    pub field_19: Option<Field19>,

    /// Sum of sender's charges (Field 71F)
    #[serde(rename = "71F", skip_serializing_if = "Option::is_none")]
    pub field_71f: Option<Field71F>,

    /// Sum of receiver's charges (Field 71G)
    #[serde(rename = "71G", skip_serializing_if = "Option::is_none")]
    pub field_71g: Option<Field71G>,

    /// Sender's correspondent (Field 53)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_53: Option<Field53SenderCorrespondent>,
}

impl MT107 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "107");

        // Parse Sequence A - General Information
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_23e = parser.parse_optional_field::<Field23E>("23E")?;
        let field_21e = parser.parse_optional_field::<Field21E>("21E")?;
        let field_30 = parser.parse_field::<Field30>("30")?;
        let field_51a = parser.parse_optional_field::<Field51A>("51A")?;

        // Try to parse field 50 - could be instructing party (C/L) or creditor (A/K)
        // We'll need to detect the variant and determine which type
        let (instructing_party, creditor) = Self::parse_field_50(&mut parser)?;

        let field_52 = parser.parse_optional_variant_field::<Field52CreditorBank>("52")?;
        let field_26t = parser.parse_optional_field::<Field26T>("26T")?;
        let field_77b = parser.parse_optional_field::<Field77B>("77B")?;
        let field_71a = parser.parse_optional_field::<Field71A>("71A")?;
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        // Parse Sequence B - Transaction Details (repeating)
        let mut transactions = Vec::new();
        parser = parser.with_duplicates(true);

        while parser.detect_field("21") {
            let txn_field_21 = parser.parse_field::<Field21NoOption>("21")?;
            let txn_field_23e = parser.parse_optional_field::<Field23E>("23E")?;
            let txn_field_21c = parser.parse_optional_field::<Field21C>("21C")?;
            let txn_field_21d = parser.parse_optional_field::<Field21D>("21D")?;
            let txn_field_21e = parser.parse_optional_field::<Field21E>("21E")?;
            let txn_field_32b = parser.parse_field::<Field32B>("32B")?;

            let (instructing_party_tx, creditor_tx) = Self::parse_field_50(&mut parser)?;

            let txn_field_52 = parser.parse_optional_variant_field::<Field52CreditorBank>("52")?;
            let txn_field_57 = parser.parse_optional_variant_field::<Field57DebtorBank>("57")?;
            let txn_field_59 = parser.parse_variant_field::<Field59>("59")?;
            let txn_field_70 = parser.parse_optional_field::<Field70>("70")?;
            let txn_field_26t = parser.parse_optional_field::<Field26T>("26T")?;
            let txn_field_77b = parser.parse_optional_field::<Field77B>("77B")?;
            let txn_field_33b = parser.parse_optional_field::<Field33B>("33B")?;
            let txn_field_71a = parser.parse_optional_field::<Field71A>("71A")?;
            let txn_field_71f = parser.parse_optional_field::<Field71F>("71F")?;
            let txn_field_71g = parser.parse_optional_field::<Field71G>("71G")?;
            let txn_field_36 = parser.parse_optional_field::<Field36>("36")?;

            transactions.push(MT107Transaction {
                field_21: txn_field_21,
                field_23e: txn_field_23e,
                field_21c: txn_field_21c,
                field_21d: txn_field_21d,
                field_21e: txn_field_21e,
                field_32b: txn_field_32b,
                instructing_party_tx,
                creditor_tx,
                field_52: txn_field_52,
                field_57: txn_field_57,
                field_59: txn_field_59,
                field_70: txn_field_70,
                field_26t: txn_field_26t,
                field_77b: txn_field_77b,
                field_33b: txn_field_33b,
                field_71a: txn_field_71a,
                field_71f: txn_field_71f,
                field_71g: txn_field_71g,
                field_36: txn_field_36,
            });
        }

        // Parse Sequence C - Settlement Details
        // Note: duplicates remain enabled to allow parsing field 32B again
        let settlement_field_32b = parser.parse_field::<Field32B>("32B")?;
        let settlement_field_19 = parser.parse_optional_field::<Field19>("19")?;
        let settlement_field_71f = parser.parse_optional_field::<Field71F>("71F")?;
        let settlement_field_71g = parser.parse_optional_field::<Field71G>("71G")?;
        let settlement_field_53 =
            parser.parse_optional_variant_field::<Field53SenderCorrespondent>("53")?;

        // Verify all content is consumed
        if !parser.is_complete() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "Unparsed content remaining in message: {}",
                    parser.remaining()
                ),
            });
        }

        Ok(Self {
            field_20,
            field_23e,
            field_21e,
            field_30,
            field_51a,
            instructing_party,
            creditor,
            field_52,
            field_26t,
            field_77b,
            field_71a,
            field_72,
            transactions,
            field_32b: settlement_field_32b,
            field_19: settlement_field_19,
            field_71f: settlement_field_71f,
            field_71g: settlement_field_71g,
            field_53: settlement_field_53,
        })
    }

    /// Helper to parse field 50 which can be either instructing party (C/L) or creditor (A/K)
    fn parse_field_50(
        parser: &mut crate::parser::MessageParser,
    ) -> Result<(Option<Field50InstructingParty>, Option<Field50Creditor>), crate::errors::ParseError>
    {
        // Detect which variant of field 50 is present
        let remaining = parser.remaining();
        let trimmed = remaining.trim_start_matches(|c: char| c.is_whitespace());

        // Check for instructing party variants (C, L)
        if trimmed.starts_with(":50C:") {
            let instructing_party =
                parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
            return Ok((instructing_party, None));
        }
        if trimmed.starts_with(":50L:") {
            let instructing_party =
                parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
            return Ok((instructing_party, None));
        }

        // Check for creditor variants (A, K)
        if trimmed.starts_with(":50A:") || trimmed.starts_with(":50K:") {
            let creditor = parser.parse_optional_variant_field::<Field50Creditor>("50")?;
            return Ok((None, creditor));
        }

        // No field 50 present
        Ok((None, None))
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT107)
    // ========================================================================

    /// Field 23E valid instruction codes for MT107
    const MT107_VALID_23E_CODES: &'static [&'static str] = &[
        "AUTH", // Pre-authorised direct debit
        "NAUT", // Non pre-authorised
        "OTHR", // Other
        "RTND", // Returned
    ];

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Check if field 23E is present in Sequence A
    fn has_23e_in_seq_a(&self) -> bool {
        self.field_23e.is_some()
    }

    /// Check if field 23E is present in all Sequence B transactions
    fn has_23e_in_all_seq_b(&self) -> bool {
        !self.transactions.is_empty() && self.transactions.iter().all(|tx| tx.field_23e.is_some())
    }

    /// Check if field 23E is present in any Sequence B transaction
    fn has_23e_in_any_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_23e.is_some())
    }

    /// Check if creditor (A/K) is present in Sequence A
    fn has_creditor_in_seq_a(&self) -> bool {
        self.creditor.is_some()
    }

    /// Check if creditor (A/K) is present in all Sequence B transactions
    fn has_creditor_in_all_seq_b(&self) -> bool {
        !self.transactions.is_empty() && self.transactions.iter().all(|tx| tx.creditor_tx.is_some())
    }

    /// Check if creditor (A/K) is present in any Sequence B transaction
    fn has_creditor_in_any_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.creditor_tx.is_some())
    }

    /// Check if instructing party (C/L) is present in Sequence A
    fn has_instructing_party_in_seq_a(&self) -> bool {
        self.instructing_party.is_some()
    }

    /// Check if instructing party (C/L) is present in any Sequence B transaction
    fn has_instructing_party_in_any_seq_b(&self) -> bool {
        self.transactions
            .iter()
            .any(|tx| tx.instructing_party_tx.is_some())
    }

    /// Check if field 21E is present in Sequence A
    fn has_21e_in_seq_a(&self) -> bool {
        self.field_21e.is_some()
    }

    /// Check if field 21E is present in any Sequence B transaction
    fn has_21e_in_any_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_21e.is_some())
    }

    /// Check if field 26T is present in Sequence A
    fn has_26t_in_seq_a(&self) -> bool {
        self.field_26t.is_some()
    }

    /// Check if field 26T is present in any Sequence B transaction
    fn has_26t_in_any_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_26t.is_some())
    }

    /// Check if field 77B is present in Sequence A
    fn has_77b_in_seq_a(&self) -> bool {
        self.field_77b.is_some()
    }

    /// Check if field 77B is present in any Sequence B transaction
    fn has_77b_in_any_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_77b.is_some())
    }

    /// Check if field 71A is present in Sequence A
    fn has_71a_in_seq_a(&self) -> bool {
        self.field_71a.is_some()
    }

    /// Check if field 71A is present in any Sequence B transaction
    fn has_71a_in_any_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_71a.is_some())
    }

    /// Check if field 52a (creditor's bank) is present in Sequence A
    fn has_52a_in_seq_a(&self) -> bool {
        self.field_52.is_some()
    }

    /// Check if field 52a (creditor's bank) is present in any Sequence B transaction
    fn has_52a_in_any_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_52.is_some())
    }

    /// Check if field 71F is present in any Sequence B transaction
    fn has_71f_in_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_71f.is_some())
    }

    /// Check if field 71F is present in Sequence C
    fn has_71f_in_seq_c(&self) -> bool {
        self.field_71f.is_some()
    }

    /// Check if field 71G is present in any Sequence B transaction
    fn has_71g_in_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_71g.is_some())
    }

    /// Check if field 71G is present in Sequence C
    fn has_71g_in_seq_c(&self) -> bool {
        self.field_71g.is_some()
    }

    // ========================================================================
    // VALIDATION RULES (C1-C9)
    // ========================================================================

    /// C1: Field 23E and Field 50a (option A or K) Mutual Exclusivity (Error code: D86)
    /// Field 23E and field 50a (option A or K) must be present either in sequence A
    /// or in each occurrence of sequence B, but not in both
    fn validate_c1_23e_and_creditor_placement(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        // Validate Field 23E placement
        let has_23e_a = self.has_23e_in_seq_a();
        let has_23e_all_b = self.has_23e_in_all_seq_b();
        let has_23e_any_b = self.has_23e_in_any_seq_b();

        if has_23e_a && has_23e_any_b {
            errors.push(SwiftValidationError::content_error(
                "D86",
                "23E",
                "",
                "Field 23E must not be present in both Sequence A and Sequence B",
                "Field 23E must be present either in Sequence A or in each occurrence of Sequence B, but not in both",
            ));
        } else if !has_23e_a && !has_23e_all_b {
            if has_23e_any_b {
                errors.push(SwiftValidationError::content_error(
                    "D86",
                    "23E",
                    "",
                    "Field 23E must be present in every Sequence B transaction when not in Sequence A",
                    "When field 23E is not in Sequence A, it must be present in each occurrence of Sequence B",
                ));
            } else {
                errors.push(SwiftValidationError::content_error(
                    "D86",
                    "23E",
                    "",
                    "Field 23E must be present in either Sequence A or in every Sequence B transaction",
                    "Field 23E must be present either in Sequence A or in each occurrence of Sequence B",
                ));
            }
        }

        // Validate Field 50a (A/K - Creditor) placement
        let has_creditor_a = self.has_creditor_in_seq_a();
        let has_creditor_all_b = self.has_creditor_in_all_seq_b();
        let has_creditor_any_b = self.has_creditor_in_any_seq_b();

        if has_creditor_a && has_creditor_any_b {
            errors.push(SwiftValidationError::content_error(
                "D86",
                "50a",
                "",
                "Field 50a (Creditor A/K) must not be present in both Sequence A and Sequence B",
                "Field 50a (option A or K) must be present either in Sequence A or in each occurrence of Sequence B, but not in both",
            ));
        } else if !has_creditor_a && !has_creditor_all_b {
            if has_creditor_any_b {
                errors.push(SwiftValidationError::content_error(
                    "D86",
                    "50a",
                    "",
                    "Field 50a (Creditor A/K) must be present in every Sequence B transaction when not in Sequence A",
                    "When field 50a (option A or K) is not in Sequence A, it must be present in each occurrence of Sequence B",
                ));
            } else {
                errors.push(SwiftValidationError::content_error(
                    "D86",
                    "50a",
                    "",
                    "Field 50a (Creditor A/K) must be present in either Sequence A or in every Sequence B transaction",
                    "Field 50a (option A or K) must be present either in Sequence A or in each occurrence of Sequence B",
                ));
            }
        }

        errors
    }

    /// C2: Fields in Sequence A vs Sequence B Mutual Exclusivity (Error code: D73)
    /// When present in sequence A, fields 21E, 26T, 77B, 71A, 52a and 50a (option C or L)
    /// must not be present in any occurrence of sequence B, and vice versa
    fn validate_c2_seq_a_b_mutual_exclusivity(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        // Field 21E
        if self.has_21e_in_seq_a() && self.has_21e_in_any_seq_b() {
            errors.push(SwiftValidationError::content_error(
                "D73",
                "21E",
                "",
                "Field 21E must not be present in both Sequence A and Sequence B",
                "When present in Sequence A, field 21E must not be present in any occurrence of Sequence B",
            ));
        }

        // Field 26T
        if self.has_26t_in_seq_a() && self.has_26t_in_any_seq_b() {
            errors.push(SwiftValidationError::content_error(
                "D73",
                "26T",
                "",
                "Field 26T must not be present in both Sequence A and Sequence B",
                "When present in Sequence A, field 26T must not be present in any occurrence of Sequence B",
            ));
        }

        // Field 77B
        if self.has_77b_in_seq_a() && self.has_77b_in_any_seq_b() {
            errors.push(SwiftValidationError::content_error(
                "D73",
                "77B",
                "",
                "Field 77B must not be present in both Sequence A and Sequence B",
                "When present in Sequence A, field 77B must not be present in any occurrence of Sequence B",
            ));
        }

        // Field 71A
        if self.has_71a_in_seq_a() && self.has_71a_in_any_seq_b() {
            errors.push(SwiftValidationError::content_error(
                "D73",
                "71A",
                "",
                "Field 71A must not be present in both Sequence A and Sequence B",
                "When present in Sequence A, field 71A must not be present in any occurrence of Sequence B",
            ));
        }

        // Field 52a
        if self.has_52a_in_seq_a() && self.has_52a_in_any_seq_b() {
            errors.push(SwiftValidationError::content_error(
                "D73",
                "52a",
                "",
                "Field 52a must not be present in both Sequence A and Sequence B",
                "When present in Sequence A, field 52a must not be present in any occurrence of Sequence B",
            ));
        }

        // Field 50a (C/L - Instructing Party)
        if self.has_instructing_party_in_seq_a() && self.has_instructing_party_in_any_seq_b() {
            errors.push(SwiftValidationError::content_error(
                "D73",
                "50a",
                "",
                "Field 50a (Instructing Party C/L) must not be present in both Sequence A and Sequence B",
                "When present in Sequence A, field 50a (option C or L) must not be present in any occurrence of Sequence B",
            ));
        }

        errors
    }

    /// C3: Registration Reference and Creditor Dependency (Error code: D77)
    /// If field 21E is present, then field 50a (option A or K) must also be present
    /// in the same sequence
    fn validate_c3_registration_creditor_dependency(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        // Check Sequence A
        if self.field_21e.is_some() && self.creditor.is_none() {
            errors.push(SwiftValidationError::content_error(
                "D77",
                "50a",
                "",
                "Sequence A: Field 50a (Creditor A/K) is mandatory when field 21E is present",
                "If field 21E is present in Sequence A, then field 50a (option A or K) must also be present in Sequence A",
            ));
        }

        // Check each transaction in Sequence B
        for (idx, transaction) in self.transactions.iter().enumerate() {
            if transaction.field_21e.is_some() && transaction.creditor_tx.is_none() {
                errors.push(SwiftValidationError::content_error(
                    "D77",
                    "50a",
                    "",
                    &format!(
                        "Transaction {}: Field 50a (Creditor A/K) is mandatory when field 21E is present",
                        idx + 1
                    ),
                    "If field 21E is present in Sequence B, then field 50a (option A or K) must also be present in the same occurrence",
                ));
            }
        }

        errors
    }

    /// C4: Field 23E RTND and Field 72 Dependency (Error code: C82)
    /// In sequence A, if field 23E contains RTND then field 72 must be present,
    /// otherwise field 72 is not allowed
    fn validate_c4_rtnd_field_72_dependency(&self) -> Option<SwiftValidationError> {
        if let Some(ref field_23e) = self.field_23e {
            let is_rtnd = field_23e.instruction_code == "RTND";

            if is_rtnd && self.field_72.is_none() {
                return Some(SwiftValidationError::content_error(
                    "C82",
                    "72",
                    "",
                    "Field 72 is mandatory when field 23E contains code RTND",
                    "In Sequence A, if field 23E is present and contains RTND then field 72 must be present",
                ));
            }

            if !is_rtnd && self.field_72.is_some() {
                return Some(SwiftValidationError::content_error(
                    "C82",
                    "72",
                    "",
                    "Field 72 is not allowed when field 23E does not contain code RTND",
                    "Field 72 is only allowed when field 23E contains code RTND",
                ));
            }
        } else {
            // Field 23E not present in Sequence A
            if self.field_72.is_some() {
                return Some(SwiftValidationError::content_error(
                    "C82",
                    "72",
                    "",
                    "Field 72 is not allowed when field 23E is not present in Sequence A",
                    "Field 72 is only allowed when field 23E is present in Sequence A with code RTND",
                ));
            }
        }

        None
    }

    /// C5: Charges Fields in Sequence B and Sequence C (Error code: D79)
    /// If fields 71F and 71G are present in Sequence B, they must also be present in Sequence C,
    /// and vice versa
    fn validate_c5_charges_fields_consistency(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        // Field 71F
        let has_71f_b = self.has_71f_in_seq_b();
        let has_71f_c = self.has_71f_in_seq_c();

        if has_71f_b && !has_71f_c {
            errors.push(SwiftValidationError::content_error(
                "D79",
                "71F",
                "",
                "Field 71F is mandatory in Sequence C when present in Sequence B",
                "If field 71F is present in one or more occurrence of Sequence B, then it must also be present in Sequence C",
            ));
        }

        if has_71f_c && !has_71f_b {
            errors.push(SwiftValidationError::content_error(
                "D79",
                "71F",
                "",
                "Field 71F is not allowed in Sequence C when not present in Sequence B",
                "If field 71F is present in Sequence C, it must also be present in at least one occurrence of Sequence B",
            ));
        }

        // Field 71G
        let has_71g_b = self.has_71g_in_seq_b();
        let has_71g_c = self.has_71g_in_seq_c();

        if has_71g_b && !has_71g_c {
            errors.push(SwiftValidationError::content_error(
                "D79",
                "71G",
                "",
                "Field 71G is mandatory in Sequence C when present in Sequence B",
                "If field 71G is present in one or more occurrence of Sequence B, then it must also be present in Sequence C",
            ));
        }

        if has_71g_c && !has_71g_b {
            errors.push(SwiftValidationError::content_error(
                "D79",
                "71G",
                "",
                "Field 71G is not allowed in Sequence C when not present in Sequence B",
                "If field 71G is present in Sequence C, it must also be present in at least one occurrence of Sequence B",
            ));
        }

        errors
    }

    /// C6: Field 33B and 32B Comparison (Error code: D21)
    /// If field 33B is present, the currency code or amount, or both, must be different
    /// between fields 33B and 32B
    fn validate_c6_field_33b_32b_comparison(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, transaction) in self.transactions.iter().enumerate() {
            if let Some(ref field_33b) = transaction.field_33b {
                let currency_32b = &transaction.field_32b.currency;
                let amount_32b = transaction.field_32b.amount;
                let currency_33b = &field_33b.currency;
                let amount_33b = field_33b.amount;

                // Both currency and amount must not be the same
                if currency_32b == currency_33b && (amount_32b - amount_33b).abs() < 0.01 {
                    errors.push(SwiftValidationError::content_error(
                        "D21",
                        "33B",
                        &format!("{}{}", currency_33b, amount_33b),
                        &format!(
                            "Transaction {}: Field 33B must have different currency code or amount from field 32B. Both are {}{:.2}",
                            idx + 1, currency_32b, amount_32b
                        ),
                        "If field 33B is present, the currency code or the amount, or both, must be different between fields 33B and 32B",
                    ));
                }
            }
        }

        errors
    }

    /// C7: Field 33B, 32B and Exchange Rate (Error code: D75)
    /// If field 33B is present and currency codes are different, field 36 must be present.
    /// Otherwise, field 36 must not be present
    fn validate_c7_exchange_rate_dependency(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, transaction) in self.transactions.iter().enumerate() {
            if let Some(ref field_33b) = transaction.field_33b {
                let currency_32b = &transaction.field_32b.currency;
                let currency_33b = &field_33b.currency;

                if currency_32b != currency_33b {
                    // Different currencies - field 36 is mandatory
                    if transaction.field_36.is_none() {
                        errors.push(SwiftValidationError::content_error(
                            "D75",
                            "36",
                            "",
                            &format!(
                                "Transaction {}: Field 36 (Exchange Rate) is mandatory when field 33B has different currency ({}) from field 32B ({})",
                                idx + 1, currency_33b, currency_32b
                            ),
                            "If field 33B is present and currency codes in fields 32B and 33B are different, field 36 must be present",
                        ));
                    }
                } else {
                    // Same currencies - field 36 must not be present
                    if transaction.field_36.is_some() {
                        errors.push(SwiftValidationError::content_error(
                            "D75",
                            "36",
                            "",
                            &format!(
                                "Transaction {}: Field 36 (Exchange Rate) is not allowed when field 33B has same currency as field 32B ({})",
                                idx + 1, currency_32b
                            ),
                            "If field 33B is present and currency codes in fields 32B and 33B are the same, field 36 must not be present",
                        ));
                    }
                }
            } else {
                // Field 33B not present - field 36 must not be present
                if transaction.field_36.is_some() {
                    errors.push(SwiftValidationError::content_error(
                        "D75",
                        "36",
                        "",
                        &format!(
                            "Transaction {}: Field 36 (Exchange Rate) is not allowed when field 33B is not present",
                            idx + 1
                        ),
                        "Field 36 is only allowed when field 33B is present",
                    ));
                }
            }
        }

        errors
    }

    /// C8: Sum of Amounts and Settlement Amount (Error code: D80, C01)
    /// The sum of amounts in field 32B of Sequence B must be in field 32B of Sequence C
    /// (when no charges) or in field 19 of Sequence C
    fn validate_c8_sum_of_amounts(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        if self.transactions.is_empty() {
            return errors;
        }

        // Calculate sum of all transaction amounts
        let sum_of_amounts: f64 = self.transactions.iter().map(|tx| tx.field_32b.amount).sum();

        // Check if charges are present
        let has_charges = self.has_71f_in_seq_b() || self.has_71g_in_seq_b();

        if has_charges {
            // Field 19 should be present and equal to sum
            if let Some(ref field_19) = self.field_19 {
                let field_19_amount = field_19.amount;
                if (field_19_amount - sum_of_amounts).abs() >= 0.01 {
                    errors.push(SwiftValidationError::content_error(
                        "C01",
                        "19",
                        &field_19_amount.to_string(),
                        &format!(
                            "Field 19 ({:.2}) must equal the sum of amounts in field 32B of Sequence B ({:.2})",
                            field_19_amount, sum_of_amounts
                        ),
                        "Field 19 must equal the sum of the amounts in all occurrences of field 32B in Sequence B",
                    ));
                }
            } else {
                errors.push(SwiftValidationError::content_error(
                    "D80",
                    "19",
                    "",
                    "Field 19 is mandatory when charges are present in Sequence B",
                    "When charges are included (field 71F or 71G in Sequence B), the sum of amounts must be in field 19 of Sequence C",
                ));
            }
        } else {
            // No charges - field 32B of Sequence C should equal sum, field 19 must not be present
            let settlement_amount = self.field_32b.amount;
            if (settlement_amount - sum_of_amounts).abs() >= 0.01 {
                errors.push(SwiftValidationError::content_error(
                    "D80",
                    "32B",
                    &settlement_amount.to_string(),
                    &format!(
                        "Sequence C field 32B amount ({:.2}) must equal the sum of amounts in Sequence B field 32B ({:.2}) when no charges are included",
                        settlement_amount, sum_of_amounts
                    ),
                    "When no charges are included, the sum of amounts in field 32B of Sequence B must be in field 32B of Sequence C",
                ));
            }

            if self.field_19.is_some() {
                errors.push(SwiftValidationError::content_error(
                    "D80",
                    "19",
                    "",
                    "Field 19 must not be present when no charges are included in Sequence B",
                    "Field 19 must not be present when the sum goes directly to field 32B of Sequence C",
                ));
            }
        }

        errors
    }

    /// C9: Currency Consistency Across Message (Error code: C02)
    /// Currency codes in fields 32B and 71G must be the same across all sequences.
    /// Currency codes in field 71F must be the same across all sequences.
    fn validate_c9_currency_consistency(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        if self.transactions.is_empty() {
            return errors;
        }

        // Get reference currencies from Sequence C
        let settlement_currency = &self.field_32b.currency;
        let ref_71f_currency = self.field_71f.as_ref().map(|f| &f.currency);
        let ref_71g_currency = self.field_71g.as_ref().map(|f| &f.currency);

        // Check 32B currency consistency in Sequence B
        for (idx, transaction) in self.transactions.iter().enumerate() {
            if &transaction.field_32b.currency != settlement_currency {
                errors.push(SwiftValidationError::content_error(
                    "C02",
                    "32B",
                    &transaction.field_32b.currency,
                    &format!(
                        "Transaction {}: Currency code in field 32B ({}) must be the same as in Sequence C ({})",
                        idx + 1, transaction.field_32b.currency, settlement_currency
                    ),
                    "The currency code in field 32B must be the same for all occurrences in Sequences B and C",
                ));
            }

            // Check 71F currency consistency
            if let Some(ref tx_71f) = transaction.field_71f
                && let Some(ref_currency) = ref_71f_currency
                && &tx_71f.currency != ref_currency
            {
                errors.push(SwiftValidationError::content_error(
                        "C02",
                        "71F",
                        &tx_71f.currency,
                        &format!(
                            "Transaction {}: Currency code in field 71F ({}) must be the same as in Sequence C ({})",
                            idx + 1, tx_71f.currency, ref_currency
                        ),
                        "The currency code in field 71F must be the same for all occurrences in Sequences B and C",
                    ));
            }

            // Check 71G currency consistency
            if let Some(ref tx_71g) = transaction.field_71g {
                if &tx_71g.currency != settlement_currency {
                    errors.push(SwiftValidationError::content_error(
                        "C02",
                        "71G",
                        &tx_71g.currency,
                        &format!(
                            "Transaction {}: Currency code in field 71G ({}) must be the same as in Sequence C ({})",
                            idx + 1, tx_71g.currency, settlement_currency
                        ),
                        "The currency code in field 71G must be the same for all occurrences in Sequences B and C",
                    ));
                }

                if let Some(ref_currency) = ref_71g_currency
                    && &tx_71g.currency != ref_currency
                {
                    errors.push(SwiftValidationError::content_error(
                            "C02",
                            "71G",
                            &tx_71g.currency,
                            &format!(
                                "Transaction {}: Currency code in field 71G ({}) must be the same as in Sequence C ({})",
                                idx + 1, tx_71g.currency, ref_currency
                            ),
                            "The currency code in field 71G must be the same for all occurrences in Sequences B and C",
                        ));
                }
            }
        }

        errors
    }

    /// Validate Field 23E instruction codes (Error codes: T47, D81)
    /// Validates instruction code values and additional information restrictions
    fn validate_field_23e(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        // Check Sequence A field 23E
        if let Some(ref field_23e) = self.field_23e {
            let code = &field_23e.instruction_code;

            // T47: Validate instruction code is in allowed list
            if !Self::MT107_VALID_23E_CODES.contains(&code.as_str()) {
                errors.push(SwiftValidationError::format_error(
                    "T47",
                    "23E",
                    code,
                    &format!("One of: {}", Self::MT107_VALID_23E_CODES.join(", ")),
                    &format!(
                        "Sequence A: Instruction code '{}' is not valid for MT107. Valid codes: {}",
                        code,
                        Self::MT107_VALID_23E_CODES.join(", ")
                    ),
                ));
            }

            // D81: Additional information only allowed for code OTHR
            if field_23e.additional_info.is_some() && code != "OTHR" {
                errors.push(SwiftValidationError::content_error(
                    "D81",
                    "23E",
                    code,
                    &format!(
                        "Sequence A: Additional information is only allowed for code OTHR. Code '{}' does not allow additional information",
                        code
                    ),
                    "Additional information in field 23E is only allowed for code OTHR",
                ));
            }
        }

        // Check Sequence B field 23E
        for (idx, transaction) in self.transactions.iter().enumerate() {
            if let Some(ref field_23e) = transaction.field_23e {
                let code = &field_23e.instruction_code;

                // T47: Validate instruction code is in allowed list
                if !Self::MT107_VALID_23E_CODES.contains(&code.as_str()) {
                    errors.push(SwiftValidationError::format_error(
                        "T47",
                        "23E",
                        code,
                        &format!("One of: {}", Self::MT107_VALID_23E_CODES.join(", ")),
                        &format!(
                            "Transaction {}: Instruction code '{}' is not valid for MT107. Valid codes: {}",
                            idx + 1,
                            code,
                            Self::MT107_VALID_23E_CODES.join(", ")
                        ),
                    ));
                }

                // D81: Additional information only allowed for code OTHR
                if field_23e.additional_info.is_some() && code != "OTHR" {
                    errors.push(SwiftValidationError::content_error(
                        "D81",
                        "23E",
                        code,
                        &format!(
                            "Transaction {}: Additional information is only allowed for code OTHR. Code '{}' does not allow additional information",
                            idx + 1, code
                        ),
                        "Additional information in field 23E is only allowed for code OTHR",
                    ));
                }
            }
        }

        errors
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Field 23E and Creditor Placement
        let c1_errors = self.validate_c1_23e_and_creditor_placement();
        all_errors.extend(c1_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C2: Sequence A/B Mutual Exclusivity
        let c2_errors = self.validate_c2_seq_a_b_mutual_exclusivity();
        all_errors.extend(c2_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C3: Registration Reference and Creditor Dependency
        let c3_errors = self.validate_c3_registration_creditor_dependency();
        all_errors.extend(c3_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C4: Field 23E RTND and Field 72 Dependency
        if let Some(error) = self.validate_c4_rtnd_field_72_dependency() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C5: Charges Fields Consistency
        let c5_errors = self.validate_c5_charges_fields_consistency();
        all_errors.extend(c5_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C6: Field 33B and 32B Comparison
        let c6_errors = self.validate_c6_field_33b_32b_comparison();
        all_errors.extend(c6_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C7: Exchange Rate Dependency
        let c7_errors = self.validate_c7_exchange_rate_dependency();
        all_errors.extend(c7_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C8: Sum of Amounts
        let c8_errors = self.validate_c8_sum_of_amounts();
        all_errors.extend(c8_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C9: Currency Consistency
        let c9_errors = self.validate_c9_currency_consistency();
        all_errors.extend(c9_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // Field 23E Validation
        let f23e_errors = self.validate_field_23e();
        all_errors.extend(f23e_errors);

        all_errors
    }

    /// Parse from SWIFT MT text format
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        // If input starts with block headers, extract Block 4
        let block4 = if input.starts_with("{") {
            crate::parser::SwiftParser::extract_block(input, 4)?.ok_or_else(|| {
                crate::errors::ParseError::InvalidFormat {
                    message: "Block 4 not found".to_string(),
                }
            })?
        } else {
            // Assume input is already block 4 content
            input.to_string()
        };

        Self::parse_from_block4(&block4)
    }
}

impl crate::traits::SwiftMessageBody for MT107 {
    fn message_type() -> &'static str {
        "107"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT107::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        // Sequence A - General Information
        append_field(&mut result, &self.field_20);
        append_optional_field(&mut result, &self.field_23e);
        append_optional_field(&mut result, &self.field_21e);
        append_field(&mut result, &self.field_30);
        append_optional_field(&mut result, &self.field_51a);
        append_optional_field(&mut result, &self.instructing_party);
        append_optional_field(&mut result, &self.creditor);
        append_optional_field(&mut result, &self.field_52);
        append_optional_field(&mut result, &self.field_26t);
        append_optional_field(&mut result, &self.field_77b);
        append_optional_field(&mut result, &self.field_71a);
        append_optional_field(&mut result, &self.field_72);

        // Sequence B - Transaction Details
        for txn in &self.transactions {
            append_field(&mut result, &txn.field_21);
            append_optional_field(&mut result, &txn.field_23e);
            append_optional_field(&mut result, &txn.field_21c);
            append_optional_field(&mut result, &txn.field_21d);
            append_optional_field(&mut result, &txn.field_21e);
            append_field(&mut result, &txn.field_32b);
            append_optional_field(&mut result, &txn.instructing_party_tx);
            append_optional_field(&mut result, &txn.creditor_tx);
            append_optional_field(&mut result, &txn.field_52);
            append_optional_field(&mut result, &txn.field_57);
            append_field(&mut result, &txn.field_59);
            append_optional_field(&mut result, &txn.field_70);
            append_optional_field(&mut result, &txn.field_26t);
            append_optional_field(&mut result, &txn.field_77b);
            append_optional_field(&mut result, &txn.field_33b);
            append_optional_field(&mut result, &txn.field_71a);
            append_optional_field(&mut result, &txn.field_71f);
            append_optional_field(&mut result, &txn.field_71g);
            append_optional_field(&mut result, &txn.field_36);
        }

        // Sequence C - Settlement Details
        append_field(&mut result, &self.field_32b);
        append_optional_field(&mut result, &self.field_19);
        append_optional_field(&mut result, &self.field_71f);
        append_optional_field(&mut result, &self.field_71g);
        append_optional_field(&mut result, &self.field_53);

        finalize_mt_string(result, false)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT107::validate_network_rules(self, stop_on_first_error)
    }
}
