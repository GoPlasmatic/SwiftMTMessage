use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// Sequence B - Transaction details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MT104Transaction {
    /// Transaction reference (Field 21)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Instruction code (Field 23E)
    #[serde(rename = "23E")]
    pub field_23e: Option<Field23E>,

    /// Mandate reference (Field 21C)
    #[serde(rename = "21C")]
    pub field_21c: Option<Field21C>,

    /// Direct debit reference (Field 21D)
    #[serde(rename = "21D")]
    pub field_21d: Option<Field21D>,

    /// Registration reference (Field 21E)
    #[serde(rename = "21E")]
    pub field_21e: Option<Field21E>,

    /// Currency and amount (Field 32B)
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    /// Instructing party (Field 50C/L)
    #[serde(flatten)]
    pub instructing_party_tx: Option<Field50InstructingParty>,

    /// Creditor (Field 50A/K)
    #[serde(flatten)]
    pub creditor_tx: Option<Field50Creditor>,

    /// Creditor's bank (Field 52)
    #[serde(flatten)]
    pub field_52: Option<Field52CreditorBank>,

    /// Debtor's bank (Field 57)
    #[serde(flatten)]
    pub field_57: Option<Field57DebtorBank>,

    /// Debtor (Field 59)
    #[serde(flatten)]
    pub field_59: Field59Debtor,

    /// Remittance information (Field 70)
    #[serde(rename = "70")]
    pub field_70: Option<Field70>,

    /// Transaction type code (Field 26T)
    #[serde(rename = "26T")]
    pub field_26t: Option<Field26T>,

    /// Regulatory reporting (Field 77B)
    #[serde(rename = "77B")]
    pub field_77b: Option<Field77B>,

    /// Original ordered amount (Field 33B)
    #[serde(rename = "33B")]
    pub field_33b: Option<Field33B>,

    /// Details of charges (Field 71A)
    #[serde(rename = "71A")]
    pub field_71a: Option<Field71A>,

    /// Sender's charges (Field 71F)
    #[serde(rename = "71F")]
    pub field_71f: Option<Field71F>,

    /// Receiver's charges (Field 71G)
    #[serde(rename = "71G")]
    pub field_71g: Option<Field71G>,

    /// Exchange rate (Field 36)
    #[serde(rename = "36")]
    pub field_36: Option<Field36>,
}

/// **MT104: Direct Debit and Request for Debit Transfer**
///
/// Creditor instruction to collect funds from one or more debtors via banks.
/// Supports batch direct debit processing with settlement details.
///
/// **Usage:** Direct debits, batch collections, SEPA debits
/// **Category:** Category 1 (Customer Payments)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MT104 {
    /// Sender's reference (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Customer specified reference (Field 21R)
    #[serde(rename = "21R")]
    pub field_21r: Option<Field21R>,

    /// Instruction code (Field 23E)
    #[serde(rename = "23E")]
    pub field_23e: Option<Field23E>,

    /// Registration reference (Field 21E)
    #[serde(rename = "21E")]
    pub field_21e: Option<Field21E>,

    /// Requested execution date (Field 30)
    #[serde(rename = "30")]
    pub field_30: Field30,

    /// Sending institution (Field 51A)
    #[serde(rename = "51A")]
    pub field_51a: Option<Field51A>,

    /// Instructing party (Field 50C/L)
    #[serde(flatten)]
    pub instructing_party: Option<Field50InstructingParty>,

    /// Creditor (Field 50A/K)
    #[serde(flatten)]
    pub creditor: Option<Field50Creditor>,

    /// Creditor's bank (Field 52)
    #[serde(flatten)]
    pub field_52: Option<Field52CreditorBank>,

    /// Transaction type code (Field 26T)
    #[serde(rename = "26T")]
    pub field_26t: Option<Field26T>,

    /// Regulatory reporting (Field 77B)
    #[serde(rename = "77B")]
    pub field_77b: Option<Field77B>,

    /// Details of charges (Field 71A)
    #[serde(rename = "71A")]
    pub field_71a: Option<Field71A>,

    /// Sender to receiver information (Field 72)
    #[serde(rename = "72")]
    pub field_72: Option<Field72>,

    /// Transaction details (Sequence B)
    #[serde(rename = "#")]
    pub transactions: Vec<MT104Transaction>,

    /// Settlement amount (Field 32B, Sequence C)
    #[serde(rename = "32B")]
    pub field_32b: Option<Field32B>,

    /// Sum of amounts (Field 19)
    #[serde(rename = "19")]
    pub field_19: Option<Field19>,

    /// Sum of sender's charges (Field 71F)
    #[serde(rename = "71F")]
    pub field_71f: Option<Field71F>,

    /// Sum of receiver's charges (Field 71G)
    #[serde(rename = "71G")]
    pub field_71g: Option<Field71G>,

    /// Sender's correspondent (Field 53)
    #[serde(flatten)]
    pub field_53: Option<Field53SenderCorrespondent>,
}

impl MT104 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "104");

        // Parse Sequence A fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21r = parser.parse_optional_field::<Field21R>("21R")?;
        let field_23e = parser.parse_optional_field::<Field23E>("23E")?;
        let field_21e = parser.parse_optional_field::<Field21E>("21E")?;
        let field_30 = parser.parse_field::<Field30>("30")?;
        let field_51a = parser.parse_optional_field::<Field51A>("51A")?;

        // Parse optional ordering parties - check variant to determine field type
        let mut instructing_party = None;
        let mut creditor = None;

        // Check if field 50 exists and determine its type based on the variant
        if let Some(variant) = parser.peek_field_variant("50") {
            match variant.as_str() {
                "C" | "L" => {
                    // These variants are for Field50InstructingParty
                    instructing_party =
                        parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
                }
                "A" | "K" => {
                    // These variants are for Field50Creditor
                    creditor = parser.parse_optional_variant_field::<Field50Creditor>("50")?;
                }
                _ => {
                    // Unknown variant, try both
                    if let Ok(ip) =
                        parser.parse_optional_variant_field::<Field50InstructingParty>("50")
                    {
                        instructing_party = ip;
                    } else {
                        creditor = parser.parse_optional_variant_field::<Field50Creditor>("50")?;
                    }
                }
            }
        }

        let field_52 = parser.parse_optional_variant_field::<Field52CreditorBank>("52")?;
        let field_26t = parser.parse_optional_field::<Field26T>("26T")?;
        let field_77b = parser.parse_optional_field::<Field77B>("77B")?;
        let field_71a = parser.parse_optional_field::<Field71A>("71A")?;
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        // Parse Sequence B (transactions) - enable duplicates
        let mut transactions = Vec::new();
        parser = parser.with_duplicates(true);

        while parser.detect_field("21") {
            let field_21 = parser.parse_field::<Field21NoOption>("21")?;
            let field_23e_tx = parser.parse_optional_field::<Field23E>("23E")?;
            let field_21c = parser.parse_optional_field::<Field21C>("21C")?;
            let field_21d = parser.parse_optional_field::<Field21D>("21D")?;
            let field_21e_tx = parser.parse_optional_field::<Field21E>("21E")?;
            let field_32b = parser.parse_field::<Field32B>("32B")?;

            // Transaction-level optional parties - check variant to determine field type
            let mut instructing_party_tx = None;
            let mut creditor_tx = None;

            // Check if field 50 exists and determine its type based on the variant
            if let Some(variant) = parser.peek_field_variant("50") {
                match variant.as_str() {
                    "C" | "L" => {
                        // These variants are for Field50InstructingParty
                        instructing_party_tx =
                            parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
                    }
                    "A" | "K" => {
                        // These variants are for Field50Creditor
                        creditor_tx =
                            parser.parse_optional_variant_field::<Field50Creditor>("50")?;
                    }
                    _ => {
                        // Unknown variant, try both
                        if let Ok(ip) =
                            parser.parse_optional_variant_field::<Field50InstructingParty>("50")
                        {
                            instructing_party_tx = ip;
                        } else {
                            creditor_tx =
                                parser.parse_optional_variant_field::<Field50Creditor>("50")?;
                        }
                    }
                }
            }

            let field_52_tx = parser.parse_optional_variant_field::<Field52CreditorBank>("52")?;
            let field_57 = parser.parse_optional_variant_field::<Field57DebtorBank>("57")?;
            let field_59 = parser.parse_variant_field::<Field59Debtor>("59")?;
            let field_70 = parser.parse_optional_field::<Field70>("70")?;
            let field_26t_tx = parser.parse_optional_field::<Field26T>("26T")?;
            let field_77b_tx = parser.parse_optional_field::<Field77B>("77B")?;
            let field_33b = parser.parse_optional_field::<Field33B>("33B")?;
            let field_71a_tx = parser.parse_optional_field::<Field71A>("71A")?;
            let field_71f = parser.parse_optional_field::<Field71F>("71F")?;
            let field_71g = parser.parse_optional_field::<Field71G>("71G")?;
            let field_36 = parser.parse_optional_field::<Field36>("36")?;

            transactions.push(MT104Transaction {
                field_21,
                field_23e: field_23e_tx,
                field_21c,
                field_21d,
                field_21e: field_21e_tx,
                field_32b,
                instructing_party_tx,
                creditor_tx,
                field_52: field_52_tx,
                field_57,
                field_59,
                field_70,
                field_26t: field_26t_tx,
                field_77b: field_77b_tx,
                field_33b,
                field_71a: field_71a_tx,
                field_71f,
                field_71g,
                field_36,
            });
        }

        // Parse Sequence C (optional settlement details)
        let field_32b = parser.parse_optional_field::<Field32B>("32B")?;
        let field_19 = parser.parse_optional_field::<Field19>("19")?;
        let field_71f = parser.parse_optional_field::<Field71F>("71F")?;
        let field_71g = parser.parse_optional_field::<Field71G>("71G")?;
        let field_53 = parser.parse_optional_variant_field::<Field53SenderCorrespondent>("53")?;

        // Verify all content is consumed
        verify_parser_complete(&parser)?;

        Ok(Self {
            field_20,
            field_21r,
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
            field_32b,
            field_19,
            field_71f,
            field_71g,
            field_53,
        })
    }

    /// Parse from generic SWIFT input
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        let block4 = extract_block4(input)?;
        Self::parse_from_block4(&block4)
    }

    /// Convert to SWIFT MT text format
    pub fn to_mt_string(&self) -> String {
        let mut result = String::new();

        // Sequence A fields
        append_field(&mut result, &self.field_20);
        append_optional_field(&mut result, &self.field_21r);
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

        // Sequence B (transactions)
        for transaction in &self.transactions {
            append_field(&mut result, &transaction.field_21);
            append_optional_field(&mut result, &transaction.field_23e);
            append_optional_field(&mut result, &transaction.field_21c);
            append_optional_field(&mut result, &transaction.field_21d);
            append_optional_field(&mut result, &transaction.field_21e);
            append_field(&mut result, &transaction.field_32b);
            append_optional_field(&mut result, &transaction.instructing_party_tx);
            append_optional_field(&mut result, &transaction.creditor_tx);
            append_optional_field(&mut result, &transaction.field_52);
            append_optional_field(&mut result, &transaction.field_57);
            append_field(&mut result, &transaction.field_59);
            append_optional_field(&mut result, &transaction.field_70);
            append_optional_field(&mut result, &transaction.field_26t);
            append_optional_field(&mut result, &transaction.field_77b);
            append_optional_field(&mut result, &transaction.field_33b);
            append_optional_field(&mut result, &transaction.field_71a);
            append_optional_field(&mut result, &transaction.field_71f);
            append_optional_field(&mut result, &transaction.field_71g);
            append_optional_field(&mut result, &transaction.field_36);
        }

        // Sequence C (optional settlement)
        append_optional_field(&mut result, &self.field_32b);
        append_optional_field(&mut result, &self.field_19);
        append_optional_field(&mut result, &self.field_71f);
        append_optional_field(&mut result, &self.field_71g);
        append_optional_field(&mut result, &self.field_53);

        finalize_mt_string(result, false)
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT104)
    // ========================================================================

    /// Field 23E valid instruction codes for MT104 Sequence A
    const MT104_VALID_23E_CODES_SEQ_A: &'static [&'static str] =
        &["AUTH", "NAUT", "OTHR", "RFDD", "RTND"];

    /// Field 23E valid instruction codes for MT104 Sequence B
    const MT104_VALID_23E_CODES_SEQ_B: &'static [&'static str] = &["AUTH", "NAUT", "OTHR"];

    /// Field 23E code that allows additional information
    const CODE_WITH_ADDITIONAL_INFO: &'static str = "OTHR";

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Check if Sequence C is present (has at least field 32B)
    fn has_sequence_c(&self) -> bool {
        self.field_32b.is_some()
    }

    /// Check if field 23E in Sequence A contains RFDD
    fn has_rfdd_in_seq_a(&self) -> bool {
        self.field_23e
            .as_ref()
            .is_some_and(|f| f.instruction_code == "RFDD")
    }

    /// Check if field 23E in Sequence A contains RTND
    fn has_rtnd_in_seq_a(&self) -> bool {
        self.field_23e
            .as_ref()
            .is_some_and(|f| f.instruction_code == "RTND")
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

    /// Check if field 52a is present in Sequence A
    fn has_52a_in_seq_a(&self) -> bool {
        self.field_52.is_some()
    }

    /// Check if field 52a is present in any Sequence B transaction
    fn has_52a_in_any_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_52.is_some())
    }

    /// Check if field 71A is present in Sequence A
    fn has_71a_in_seq_a(&self) -> bool {
        self.field_71a.is_some()
    }

    /// Check if field 71A is present in any Sequence B transaction
    fn has_71a_in_any_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_71a.is_some())
    }

    /// Check if field 77B is present in Sequence A
    fn has_77b_in_seq_a(&self) -> bool {
        self.field_77b.is_some()
    }

    /// Check if field 77B is present in any Sequence B transaction
    fn has_77b_in_any_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_77b.is_some())
    }

    /// Check if field 71F is present in any Sequence B transaction
    fn has_71f_in_any_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_71f.is_some())
    }

    /// Check if field 71F is present in Sequence C
    fn has_71f_in_seq_c(&self) -> bool {
        self.field_71f.is_some()
    }

    /// Check if field 71G is present in any Sequence B transaction
    fn has_71g_in_any_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_71g.is_some())
    }

    /// Check if field 71G is present in Sequence C
    fn has_71g_in_seq_c(&self) -> bool {
        self.field_71g.is_some()
    }

    // ========================================================================
    // VALIDATION RULES (C1-C13)
    // ========================================================================

    /// C1: Field 23E in Sequence A and Sequence B Dependencies (Error code: C75)
    fn validate_c1_field_23e_dependencies(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        if let Some(ref field_23e_a) = self.field_23e {
            if field_23e_a.instruction_code == "RFDD" {
                // Present and equal to RFDD → Mandatory in all occurrences of Seq B
                for (idx, transaction) in self.transactions.iter().enumerate() {
                    if transaction.field_23e.is_none() {
                        errors.push(SwiftValidationError::content_error(
                            "C75",
                            "23E",
                            "",
                            &format!(
                                "Transaction {}: Field 23E is mandatory in Sequence B when field 23E in Sequence A contains RFDD",
                                idx + 1
                            ),
                            "If field 23E is present in sequence A and contains RFDD then field 23E must be present in all occurrences of sequence B",
                        ));
                    }
                }
            } else {
                // Present and not equal to RFDD → Not allowed in Seq B
                for (idx, transaction) in self.transactions.iter().enumerate() {
                    if transaction.field_23e.is_some() {
                        errors.push(SwiftValidationError::content_error(
                            "C75",
                            "23E",
                            "",
                            &format!(
                                "Transaction {}: Field 23E must not be present in Sequence B when field 23E in Sequence A does not contain RFDD",
                                idx + 1
                            ),
                            "If field 23E is present in sequence A and does not contain RFDD then field 23E must not be present in any occurrence of sequence B",
                        ));
                    }
                }
            }
        } else {
            // Not present in Seq A → Mandatory in all occurrences of Seq B
            for (idx, transaction) in self.transactions.iter().enumerate() {
                if transaction.field_23e.is_none() {
                    errors.push(SwiftValidationError::content_error(
                        "C75",
                        "23E",
                        "",
                        &format!(
                            "Transaction {}: Field 23E is mandatory in Sequence B when field 23E is not present in Sequence A",
                            idx + 1
                        ),
                        "If field 23E is not present in sequence A then field 23E must be present in all occurrences of sequence B",
                    ));
                }
            }
        }

        errors
    }

    /// C2: Creditor Field (Field 50a options A or K) (Error code: C76)
    fn validate_c2_creditor_field(&self) -> Option<SwiftValidationError> {
        let in_seq_a = self.has_creditor_in_seq_a();
        let in_all_seq_b = self.has_creditor_in_all_seq_b();
        let in_any_seq_b = self.has_creditor_in_any_seq_b();

        if in_seq_a && in_any_seq_b {
            // Present in both sequences - NOT ALLOWED
            return Some(SwiftValidationError::content_error(
                "C76",
                "50a",
                "",
                "Field 50a (Creditor A/K) must not be present in both Sequence A and Sequence B",
                "Field 50a (option A or K), must be present in either sequence A or in each occurrence of sequence B, but must never be present in both sequences",
            ));
        }

        if !in_seq_a && !in_all_seq_b {
            if in_any_seq_b {
                // Present in some but not all Seq B transactions
                return Some(SwiftValidationError::content_error(
                    "C76",
                    "50a",
                    "",
                    "Field 50a (Creditor A/K) must be present in every Sequence B transaction when not in Sequence A",
                    "Field 50a (option A or K), must be present in each occurrence of sequence B when not present in sequence A",
                ));
            } else {
                // Not present anywhere - NOT ALLOWED
                return Some(SwiftValidationError::content_error(
                    "C76",
                    "50a",
                    "",
                    "Field 50a (Creditor A/K) must be present in either Sequence A or in every Sequence B transaction",
                    "Field 50a (option A or K), must be present in either sequence A or in each occurrence of sequence B, but must never be absent from both sequences",
                ));
            }
        }

        None
    }

    /// C3: Mutual Exclusivity of Fields 21E, 26T, 52a, 71A, 77B, and 50a (C or L) (Error code: D73)
    fn validate_c3_mutual_exclusivity(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        // Check field 21E
        if self.has_21e_in_seq_a() && self.has_21e_in_any_seq_b() {
            errors.push(SwiftValidationError::content_error(
                "D73",
                "21E",
                "",
                "Field 21E must not be present in both Sequence A and Sequence B",
                "When present in sequence A, field 21E must not be present in any occurrence of sequence B. When present in one or more occurrences of sequence B, field 21E must not be present in sequence A",
            ));
        }

        // Check field 26T
        if self.has_26t_in_seq_a() && self.has_26t_in_any_seq_b() {
            errors.push(SwiftValidationError::content_error(
                "D73",
                "26T",
                "",
                "Field 26T must not be present in both Sequence A and Sequence B",
                "When present in sequence A, field 26T must not be present in any occurrence of sequence B. When present in one or more occurrences of sequence B, field 26T must not be present in sequence A",
            ));
        }

        // Check field 52a
        if self.has_52a_in_seq_a() && self.has_52a_in_any_seq_b() {
            errors.push(SwiftValidationError::content_error(
                "D73",
                "52a",
                "",
                "Field 52a must not be present in both Sequence A and Sequence B",
                "When present in sequence A, field 52a must not be present in any occurrence of sequence B. When present in one or more occurrences of sequence B, field 52a must not be present in sequence A",
            ));
        }

        // Check field 71A
        if self.has_71a_in_seq_a() && self.has_71a_in_any_seq_b() {
            errors.push(SwiftValidationError::content_error(
                "D73",
                "71A",
                "",
                "Field 71A must not be present in both Sequence A and Sequence B",
                "When present in sequence A, field 71A must not be present in any occurrence of sequence B. When present in one or more occurrences of sequence B, field 71A must not be present in sequence A",
            ));
        }

        // Check field 77B
        if self.has_77b_in_seq_a() && self.has_77b_in_any_seq_b() {
            errors.push(SwiftValidationError::content_error(
                "D73",
                "77B",
                "",
                "Field 77B must not be present in both Sequence A and Sequence B",
                "When present in sequence A, field 77B must not be present in any occurrence of sequence B. When present in one or more occurrences of sequence B, field 77B must not be present in sequence A",
            ));
        }

        // Check field 50a (C or L) - Instructing Party
        if self.has_instructing_party_in_seq_a() && self.has_instructing_party_in_any_seq_b() {
            errors.push(SwiftValidationError::content_error(
                "D73",
                "50a",
                "",
                "Field 50a (Instructing Party C/L) must not be present in both Sequence A and Sequence B",
                "When present in sequence A, field 50a (option C or L) must not be present in any occurrence of sequence B. When present in one or more occurrences of sequence B, field 50a (option C or L) must not be present in sequence A",
            ));
        }

        errors
    }

    /// C4: Registration Reference and Creditor Dependencies (Error code: D77)
    fn validate_c4_registration_reference(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        // Check Sequence A
        if self.field_21e.is_some() && self.creditor.is_none() {
            errors.push(SwiftValidationError::content_error(
                "D77",
                "50a",
                "",
                "Field 50a (Creditor A/K) is mandatory in Sequence A when field 21E is present",
                "If field 21E is present in sequence A, field 50a (option A or K), must also be present in sequence A",
            ));
        }

        // Check each occurrence of Sequence B
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
                    "In each occurrence of sequence B, if field 21E is present, then field 50a (option A or K), must also be present in the same occurrence",
                ));
            }
        }

        errors
    }

    /// C5: Field 72 and RTND Code Dependency (Error code: C82)
    fn validate_c5_field_72_rtnd(&self) -> Option<SwiftValidationError> {
        let has_rtnd = self.has_rtnd_in_seq_a();
        let has_field_72 = self.field_72.is_some();

        if has_rtnd && !has_field_72 {
            return Some(SwiftValidationError::content_error(
                "C82",
                "72",
                "",
                "Field 72 is mandatory when field 23E in Sequence A contains RTND",
                "In sequence A, if field 23E is present and contains RTND then field 72 must be present",
            ));
        }

        if !has_rtnd && has_field_72 {
            return Some(SwiftValidationError::content_error(
                "C82",
                "72",
                "",
                "Field 72 is not allowed when field 23E in Sequence A does not contain RTND or is not present",
                "In sequence A, if field 23E not present, or field 23E does not contain RTND - field 72 is not allowed",
            ));
        }

        None
    }

    /// C6: Charges Fields Dependencies Between Sequences B and C (Error code: D79)
    fn validate_c6_charges_dependencies(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        let has_71f_b = self.has_71f_in_any_seq_b();
        let has_71f_c = self.has_71f_in_seq_c();
        let has_71g_b = self.has_71g_in_any_seq_b();
        let has_71g_c = self.has_71g_in_seq_c();

        // Field 71F validation
        if has_71f_b && !has_71f_c {
            errors.push(SwiftValidationError::content_error(
                "D79",
                "71F",
                "",
                "Field 71F is mandatory in Sequence C when present in Sequence B",
                "If field 71F is present in one or more occurrence of sequence B, then it must also be present in sequence C",
            ));
        }

        if !has_71f_b && has_71f_c {
            errors.push(SwiftValidationError::content_error(
                "D79",
                "71F",
                "",
                "Field 71F is not allowed in Sequence C when not present in Sequence B",
                "If field 71F is not present in sequence B, then it must not be present in sequence C",
            ));
        }

        // Field 71G validation
        if has_71g_b && !has_71g_c {
            errors.push(SwiftValidationError::content_error(
                "D79",
                "71G",
                "",
                "Field 71G is mandatory in Sequence C when present in Sequence B",
                "If field 71G is present in one or more occurrence of sequence B, then it must also be present in sequence C",
            ));
        }

        if !has_71g_b && has_71g_c {
            errors.push(SwiftValidationError::content_error(
                "D79",
                "71G",
                "",
                "Field 71G is not allowed in Sequence C when not present in Sequence B",
                "If field 71G is not present in sequence B, then it must not be present in sequence C",
            ));
        }

        errors
    }

    /// C7: Fields 33B and 32B Currency/Amount Difference (Error code: D21)
    fn validate_c7_currency_amount_difference(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, transaction) in self.transactions.iter().enumerate() {
            if let Some(ref field_33b) = transaction.field_33b {
                let currency_32b = &transaction.field_32b.currency;
                let currency_33b = &field_33b.currency;
                let amount_32b = transaction.field_32b.amount;
                let amount_33b = field_33b.amount;

                // Check if both currency and amount are the same
                if currency_32b == currency_33b && (amount_32b - amount_33b).abs() < 0.01 {
                    errors.push(SwiftValidationError::content_error(
                        "D21",
                        "33B",
                        &format!("{}{}", currency_33b, amount_33b),
                        &format!(
                            "Transaction {}: Currency code or amount, or both, must be different between fields 33B and 32B",
                            idx + 1
                        ),
                        "In each occurrence of sequence B, if field 33B is present then the currency code or the amount, or both, must be different between fields 33B and 32B",
                    ));
                }
            }
        }

        errors
    }

    /// C8: Exchange Rate and Currency Dependency (Error code: D75)
    fn validate_c8_exchange_rate(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, transaction) in self.transactions.iter().enumerate() {
            if let Some(ref field_33b) = transaction.field_33b {
                let currency_32b = &transaction.field_32b.currency;
                let currency_33b = &field_33b.currency;

                if currency_32b != currency_33b {
                    // Different currencies → field 36 must be present
                    if transaction.field_36.is_none() {
                        errors.push(SwiftValidationError::content_error(
                            "D75",
                            "36",
                            "",
                            &format!(
                                "Transaction {}: Field 36 (Exchange Rate) is mandatory when currencies in fields 32B and 33B are different",
                                idx + 1
                            ),
                            "In any occurrence of sequence B, if field 33B is present and the currency codes in fields 32B and 33B are different, then field 36 must be present",
                        ));
                    }
                } else {
                    // Same currencies → field 36 must not be present
                    if transaction.field_36.is_some() {
                        errors.push(SwiftValidationError::content_error(
                            "D75",
                            "36",
                            "",
                            &format!(
                                "Transaction {}: Field 36 (Exchange Rate) is not allowed when currencies in fields 32B and 33B are the same",
                                idx + 1
                            ),
                            "If field 33B is present and the currency codes in fields 32B and 33B are the same, then field 36 must not be present",
                        ));
                    }
                }
            } else {
                // Field 33B not present → field 36 must not be present
                if transaction.field_36.is_some() {
                    errors.push(SwiftValidationError::content_error(
                        "D75",
                        "36",
                        "",
                        &format!(
                            "Transaction {}: Field 36 (Exchange Rate) is not allowed when field 33B is not present",
                            idx + 1
                        ),
                        "Field 36 must not be present when field 33B is not present",
                    ));
                }
            }
        }

        errors
    }

    /// C9: Field 19 and Settlement Amount Calculation (Error code: D80)
    fn validate_c9_field_19(&self) -> Option<SwiftValidationError> {
        if !self.has_sequence_c() {
            return None; // Rule doesn't apply if Sequence C is not present
        }

        let field_32b_c = self.field_32b.as_ref()?;
        let settlement_amount = field_32b_c.amount;

        // Calculate sum of amounts in Sequence B
        let sum_of_amounts: f64 = self.transactions.iter().map(|tx| tx.field_32b.amount).sum();

        let amounts_equal = (settlement_amount - sum_of_amounts).abs() < 0.01;

        if amounts_equal && self.field_19.is_some() {
            return Some(SwiftValidationError::content_error(
                "D80",
                "19",
                "",
                "Field 19 must not be present when amount in field 32B of Sequence C equals the sum of amounts in field 32B of Sequence B",
                "If sequence C is present and if the amount in field 32B of sequence C is equal to the sum of the amounts of the fields 32B of sequence B, then field 19 must not be present",
            ));
        }

        if !amounts_equal && self.field_19.is_none() {
            return Some(SwiftValidationError::content_error(
                "D80",
                "19",
                "",
                "Field 19 must be present when amount in field 32B of Sequence C is not equal to the sum of amounts in field 32B of Sequence B",
                "If the amount in field 32B of sequence C is not equal to the sum of the amounts of the fields 32B of sequence B, then field 19 must be present",
            ));
        }

        None
    }

    /// C10: Field 19 Amount Validation (Error code: C01)
    fn validate_c10_field_19_amount(&self) -> Option<SwiftValidationError> {
        if let Some(ref field_19) = self.field_19 {
            // Calculate sum of amounts in Sequence B
            let sum_of_amounts: f64 = self.transactions.iter().map(|tx| tx.field_32b.amount).sum();

            if (field_19.amount - sum_of_amounts).abs() > 0.01 {
                return Some(SwiftValidationError::content_error(
                    "C01",
                    "19",
                    &field_19.amount.to_string(),
                    &format!(
                        "Field 19 amount ({}) must equal the sum of amounts in all occurrences of field 32B in Sequence B ({})",
                        field_19.amount, sum_of_amounts
                    ),
                    "If field 19 is present in sequence C then it must be equal to the sum of the amounts in all occurrences of field 32B in sequence B",
                ));
            }
        }

        None
    }

    /// C11: Currency Code Consistency (Error code: C02)
    fn validate_c11_currency_consistency(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        // Collect all 32B currencies from Sequence B
        let mut currencies_32b: Vec<&String> = self
            .transactions
            .iter()
            .map(|tx| &tx.field_32b.currency)
            .collect();

        // Add 32B currency from Sequence C if present
        if let Some(ref field_32b_c) = self.field_32b {
            currencies_32b.push(&field_32b_c.currency);
        }

        // Check if all 32B currencies are the same
        if !currencies_32b.is_empty() {
            let first_currency_32b = currencies_32b[0];
            for currency in currencies_32b.iter().skip(1) {
                if *currency != first_currency_32b {
                    errors.push(SwiftValidationError::content_error(
                        "C02",
                        "32B",
                        currency,
                        &format!(
                            "Currency code in field 32B ({}) must be the same for all occurrences in the message (expected: {})",
                            currency, first_currency_32b
                        ),
                        "The currency code in fields 32B must be the same for all occurrences of these fields in the message",
                    ));
                    break; // Report only once
                }
            }
        }

        // Collect all 71G currencies from Sequence B
        let mut currencies_71g: Vec<&String> = self
            .transactions
            .iter()
            .filter_map(|tx| tx.field_71g.as_ref().map(|f| &f.currency))
            .collect();

        // Add 71G currency from Sequence C if present
        if let Some(ref field_71g_c) = self.field_71g {
            currencies_71g.push(&field_71g_c.currency);
        }

        // Check if all 71G currencies are the same
        if !currencies_71g.is_empty() {
            let first_currency_71g = currencies_71g[0];
            for currency in currencies_71g.iter().skip(1) {
                if *currency != first_currency_71g {
                    errors.push(SwiftValidationError::content_error(
                        "C02",
                        "71G",
                        currency,
                        &format!(
                            "Currency code in field 71G ({}) must be the same for all occurrences in the message (expected: {})",
                            currency, first_currency_71g
                        ),
                        "The currency code in field 71G in sequences B and C must be the same for all occurrences of these fields in the message",
                    ));
                    break; // Report only once
                }
            }
        }

        // Collect all 71F currencies from Sequence B
        let mut currencies_71f: Vec<&String> = self
            .transactions
            .iter()
            .filter_map(|tx| tx.field_71f.as_ref().map(|f| &f.currency))
            .collect();

        // Add 71F currency from Sequence C if present
        if let Some(ref field_71f_c) = self.field_71f {
            currencies_71f.push(&field_71f_c.currency);
        }

        // Check if all 71F currencies are the same
        if !currencies_71f.is_empty() {
            let first_currency_71f = currencies_71f[0];
            for currency in currencies_71f.iter().skip(1) {
                if *currency != first_currency_71f {
                    errors.push(SwiftValidationError::content_error(
                        "C02",
                        "71F",
                        currency,
                        &format!(
                            "Currency code in field 71F ({}) must be the same for all occurrences in the message (expected: {})",
                            currency, first_currency_71f
                        ),
                        "The currency code in the charges fields 71F (in sequences B and C) must be the same for all occurrences of these fields in the message",
                    ));
                    break; // Report only once
                }
            }
        }

        errors
    }

    /// C12: Request for Direct Debit (RFDD) Comprehensive Rules (Error code: C96)
    fn validate_c12_rfdd_comprehensive(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();
        let has_rfdd = self.has_rfdd_in_seq_a();

        if has_rfdd {
            // Field 23E = RFDD in Sequence A
            // In Sequence B: 21E, 50a (A/K), 52a, 71F, 71G must NOT be present
            for (idx, transaction) in self.transactions.iter().enumerate() {
                if transaction.field_21e.is_some() {
                    errors.push(SwiftValidationError::content_error(
                        "C96",
                        "21E",
                        "",
                        &format!(
                            "Transaction {}: Field 21E is not allowed in Sequence B when field 23E in Sequence A contains RFDD",
                            idx + 1
                        ),
                        "In sequence A, if field 23E is present and contains RFDD, then in sequence B the fields 21E, 50a (option A or K), 52a, 71F, 71G must not be present",
                    ));
                }

                if transaction.creditor_tx.is_some() {
                    errors.push(SwiftValidationError::content_error(
                        "C96",
                        "50a",
                        "",
                        &format!(
                            "Transaction {}: Field 50a (Creditor A/K) is not allowed in Sequence B when field 23E in Sequence A contains RFDD",
                            idx + 1
                        ),
                        "In sequence A, if field 23E is present and contains RFDD, then in sequence B the fields 21E, 50a (option A or K), 52a, 71F, 71G must not be present",
                    ));
                }

                if transaction.field_52.is_some() {
                    errors.push(SwiftValidationError::content_error(
                        "C96",
                        "52a",
                        "",
                        &format!(
                            "Transaction {}: Field 52a is not allowed in Sequence B when field 23E in Sequence A contains RFDD",
                            idx + 1
                        ),
                        "In sequence A, if field 23E is present and contains RFDD, then in sequence B the fields 21E, 50a (option A or K), 52a, 71F, 71G must not be present",
                    ));
                }

                if transaction.field_71f.is_some() {
                    errors.push(SwiftValidationError::content_error(
                        "C96",
                        "71F",
                        "",
                        &format!(
                            "Transaction {}: Field 71F is not allowed in Sequence B when field 23E in Sequence A contains RFDD",
                            idx + 1
                        ),
                        "In sequence A, if field 23E is present and contains RFDD, then in sequence B the fields 21E, 50a (option A or K), 52a, 71F, 71G must not be present",
                    ));
                }

                if transaction.field_71g.is_some() {
                    errors.push(SwiftValidationError::content_error(
                        "C96",
                        "71G",
                        "",
                        &format!(
                            "Transaction {}: Field 71G is not allowed in Sequence B when field 23E in Sequence A contains RFDD",
                            idx + 1
                        ),
                        "In sequence A, if field 23E is present and contains RFDD, then in sequence B the fields 21E, 50a (option A or K), 52a, 71F, 71G must not be present",
                    ));
                }
            }

            // Sequence C must NOT be present
            if self.has_sequence_c() {
                errors.push(SwiftValidationError::content_error(
                    "C96",
                    "32B",
                    "",
                    "Sequence C is not allowed when field 23E in Sequence A contains RFDD",
                    "In sequence A, if field 23E is present and contains RFDD, then sequence C must not be present",
                ));
            }
        } else {
            // Field 23E does NOT contain RFDD or is not present
            // Field 21R must NOT be present in Sequence A
            if self.field_21r.is_some() {
                errors.push(SwiftValidationError::content_error(
                    "C96",
                    "21R",
                    "",
                    "Field 21R is not allowed in Sequence A when field 23E does not contain RFDD or is not present",
                    "In sequence A field 23E does not contain RFDD or field 23E is not present, in sequence A field 21R must not be present",
                ));
            }

            // Sequence C must be present
            if !self.has_sequence_c() {
                errors.push(SwiftValidationError::content_error(
                    "C96",
                    "32B",
                    "",
                    "Sequence C is mandatory when field 23E in Sequence A does not contain RFDD or is not present",
                    "In sequence A field 23E does not contain RFDD or field 23E is not present, sequence C must be present",
                ));
            }
        }

        errors
    }

    /// Validate Field 23E instruction codes in Sequence A (Error codes: T47, D81)
    fn validate_field_23e_seq_a(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        if let Some(ref field_23e) = self.field_23e {
            let code = &field_23e.instruction_code;

            // T47: Validate instruction code is in allowed list for Sequence A
            if !Self::MT104_VALID_23E_CODES_SEQ_A.contains(&code.as_str()) {
                errors.push(SwiftValidationError::format_error(
                    "T47",
                    "23E",
                    code,
                    &format!("One of: {}", Self::MT104_VALID_23E_CODES_SEQ_A.join(", ")),
                    &format!(
                        "Sequence A: Instruction code '{}' is not valid for MT104. Valid codes: {}",
                        code,
                        Self::MT104_VALID_23E_CODES_SEQ_A.join(", ")
                    ),
                ));
            }

            // D81: Additional information only allowed for OTHR
            if field_23e.additional_info.is_some() && code != Self::CODE_WITH_ADDITIONAL_INFO {
                errors.push(SwiftValidationError::content_error(
                    "D81",
                    "23E",
                    code,
                    &format!(
                        "Sequence A: Additional information is only allowed for code OTHR. Code '{}' does not allow additional information",
                        code
                    ),
                    "The narrative second subfield can only be used in combination with OTHR",
                ));
            }
        }

        errors
    }

    /// Validate Field 23E instruction codes in Sequence B (Error codes: T47, D81)
    fn validate_field_23e_seq_b(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, transaction) in self.transactions.iter().enumerate() {
            if let Some(ref field_23e) = transaction.field_23e {
                let code = &field_23e.instruction_code;

                // T47: Validate instruction code is in allowed list for Sequence B
                if !Self::MT104_VALID_23E_CODES_SEQ_B.contains(&code.as_str()) {
                    errors.push(SwiftValidationError::format_error(
                        "T47",
                        "23E",
                        code,
                        &format!("One of: {}", Self::MT104_VALID_23E_CODES_SEQ_B.join(", ")),
                        &format!(
                            "Transaction {}: Instruction code '{}' is not valid for MT104 Sequence B. Valid codes: {}",
                            idx + 1,
                            code,
                            Self::MT104_VALID_23E_CODES_SEQ_B.join(", ")
                        ),
                    ));
                }

                // D81: Additional information only allowed for OTHR
                if field_23e.additional_info.is_some() && code != Self::CODE_WITH_ADDITIONAL_INFO {
                    errors.push(SwiftValidationError::content_error(
                        "D81",
                        "23E",
                        code,
                        &format!(
                            "Transaction {}: Additional information is only allowed for code OTHR. Code '{}' does not allow additional information",
                            idx + 1,
                            code
                        ),
                        "The narrative second subfield can only be used in combination with OTHR",
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

        // C1: Field 23E Dependencies
        let c1_errors = self.validate_c1_field_23e_dependencies();
        all_errors.extend(c1_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C2: Creditor Field
        if let Some(error) = self.validate_c2_creditor_field() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C3: Mutual Exclusivity
        let c3_errors = self.validate_c3_mutual_exclusivity();
        all_errors.extend(c3_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C4: Registration Reference
        let c4_errors = self.validate_c4_registration_reference();
        all_errors.extend(c4_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C5: Field 72 and RTND
        if let Some(error) = self.validate_c5_field_72_rtnd() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C6: Charges Dependencies
        let c6_errors = self.validate_c6_charges_dependencies();
        all_errors.extend(c6_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C7: Currency/Amount Difference
        let c7_errors = self.validate_c7_currency_amount_difference();
        all_errors.extend(c7_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C8: Exchange Rate
        let c8_errors = self.validate_c8_exchange_rate();
        all_errors.extend(c8_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C9: Field 19
        if let Some(error) = self.validate_c9_field_19() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C10: Field 19 Amount
        if let Some(error) = self.validate_c10_field_19_amount() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C11: Currency Consistency
        let c11_errors = self.validate_c11_currency_consistency();
        all_errors.extend(c11_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C12: RFDD Comprehensive Rules
        let c12_errors = self.validate_c12_rfdd_comprehensive();
        all_errors.extend(c12_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // Field 23E Validation - Sequence A
        let f23e_a_errors = self.validate_field_23e_seq_a();
        all_errors.extend(f23e_a_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // Field 23E Validation - Sequence B
        let f23e_b_errors = self.validate_field_23e_seq_b();
        all_errors.extend(f23e_b_errors);

        all_errors
    }
}

impl crate::traits::SwiftMessageBody for MT104 {
    fn message_type() -> &'static str {
        "104"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT104::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT104::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT104::validate_network_rules(self, stop_on_first_error)
    }
}
