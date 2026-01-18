use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Sequence B - Transaction details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MT101Transaction {
    /// Transaction reference (Field 21)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// F/X deal reference (Field 21F)
    #[serde(rename = "21F")]
    pub field_21f: Option<Field21F>,

    /// Instruction codes (Field 23E)
    #[serde(rename = "23E")]
    pub field_23e: Option<Vec<Field23E>>,

    /// Currency and amount (Field 32B)
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    /// Instructing party (Field 50C/L)
    #[serde(flatten)]
    pub instructing_party_tx: Option<Field50InstructingParty>,

    /// Ordering customer (Field 50F/G/H)
    #[serde(flatten)]
    pub ordering_customer_tx: Option<Field50OrderingCustomerFGH>,

    /// Account servicing institution (Field 52)
    #[serde(flatten)]
    pub field_52: Option<Field52AccountServicingInstitution>,

    /// Intermediary (Field 56)
    #[serde(flatten)]
    pub field_56: Option<Field56Intermediary>,

    /// Account with institution (Field 57)
    #[serde(flatten)]
    pub field_57: Option<Field57AccountWithInstitution>,

    /// Beneficiary customer (Field 59)
    #[serde(flatten)]
    pub field_59: Field59,

    /// Remittance information (Field 70)
    #[serde(rename = "70")]
    pub field_70: Option<Field70>,

    /// Regulatory reporting (Field 77B)
    #[serde(rename = "77B")]
    pub field_77b: Option<Field77B>,

    /// Original amount (Field 33B)
    #[serde(rename = "33B")]
    pub field_33b: Option<Field33B>,

    /// Details of charges (Field 71A)
    #[serde(rename = "71A")]
    pub field_71a: Field71A,

    /// Charges account (Field 25A)
    #[serde(rename = "25A")]
    pub field_25a: Option<Field25A>,

    /// Exchange rate (Field 36)
    #[serde(rename = "36")]
    pub field_36: Option<Field36>,
}

/// **MT101: Request for Transfer**
///
/// Batch payment instruction from ordering customer to account servicing institution.
/// Contains one or more transfer instructions for beneficiary payments.
///
/// **Usage:** Batch payments, salary payments, vendor payments
/// **Category:** Category 1 (Customer Payments)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MT101 {
    /// Sender's reference (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Customer specified reference (Field 21R)
    #[serde(rename = "21R")]
    pub field_21r: Option<Field21R>,

    /// Message index/total (Field 28D)
    #[serde(rename = "28D")]
    pub field_28d: Field28D,

    /// Instructing party (Field 50C/L)
    #[serde(flatten)]
    pub instructing_party: Option<Field50InstructingParty>,

    /// Ordering customer (Field 50F/G/H)
    #[serde(flatten)]
    pub ordering_customer: Option<Field50OrderingCustomerFGH>,

    /// Account servicing institution (Field 52)
    #[serde(flatten)]
    pub field_52a: Option<Field52AccountServicingInstitution>,

    /// Sending institution (Field 51A)
    #[serde(rename = "51A")]
    pub field_51a: Option<Field51A>,

    /// Requested execution date (Field 30)
    #[serde(rename = "30")]
    pub field_30: Field30,

    /// Account identification (Field 25)
    #[serde(rename = "25")]
    pub field_25: Option<Field25NoOption>,

    /// Transaction details (Sequence B)
    #[serde(rename = "#")]
    pub transactions: Vec<MT101Transaction>,
}

impl MT101 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "101");

        // Parse mandatory and optional fields in sequence A
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21r = parser.parse_optional_field::<Field21R>("21R")?;
        let field_28d = parser.parse_field::<Field28D>("28D")?;

        // Parse optional ordering customer and instructing party (can appear in either order)
        // Field 50 can be either instructing party (C/L) or ordering customer (F/G/H)
        // Check which variant is present and parse accordingly
        let (instructing_party, ordering_customer) = {
            let mut instructing = None;
            let mut ordering = None;

            // Detect which Field 50 variant is present
            if let Some(variant) = parser.detect_variant_optional("50") {
                match variant.as_str() {
                    "C" | "L" => {
                        // Instructing party variants
                        instructing =
                            parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
                    }
                    "F" | "G" | "H" => {
                        // Ordering customer variants
                        ordering = parser
                            .parse_optional_variant_field::<Field50OrderingCustomerFGH>("50")?;
                    }
                    _ => {
                        // Unknown variant - try instructing party first, then ordering customer
                        if let Ok(Some(field)) =
                            parser.parse_optional_variant_field::<Field50InstructingParty>("50")
                        {
                            instructing = Some(field);
                        } else {
                            ordering = parser
                                .parse_optional_variant_field::<Field50OrderingCustomerFGH>("50")?;
                        }
                    }
                }
            }

            (instructing, ordering)
        };

        let field_52a =
            parser.parse_optional_variant_field::<Field52AccountServicingInstitution>("52")?;
        let field_51a = parser.parse_optional_field::<Field51A>("51A")?;
        let field_30 = parser.parse_field::<Field30>("30")?;
        let field_25 = parser.parse_optional_field::<Field25NoOption>("25")?;

        // Parse transactions - this is a repeating sequence B
        let mut transactions = Vec::new();

        // Enable duplicates for repeating fields
        parser = parser.with_duplicates(true);

        // Parse each transaction - they start with field 21
        while parser.detect_field("21") {
            let field_21 = parser.parse_field::<Field21NoOption>("21")?;
            let field_21f = parser.parse_optional_field::<Field21F>("21F")?;

            // Field 23E can appear multiple times within a transaction
            // Only parse consecutive 23E fields (stop when we hit any other field)
            let field_23e = if parser.detect_field("23E") {
                let mut codes = Vec::new();
                while parser.detect_field("23E") {
                    if let Ok(field) = parser.parse_field::<Field23E>("23E") {
                        codes.push(field);
                    } else {
                        break;
                    }
                }
                if !codes.is_empty() { Some(codes) } else { None }
            } else {
                None
            };

            let field_32b = parser.parse_field::<Field32B>("32B")?;

            // Transaction-level optional ordering parties
            let instructing_party_tx =
                parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
            let ordering_customer_tx =
                parser.parse_optional_variant_field::<Field50OrderingCustomerFGH>("50")?;

            let field_52 =
                parser.parse_optional_variant_field::<Field52AccountServicingInstitution>("52")?;
            let field_56 = parser.parse_optional_variant_field::<Field56Intermediary>("56")?;
            let field_57 =
                parser.parse_optional_variant_field::<Field57AccountWithInstitution>("57")?;
            let field_59 = parser.parse_variant_field::<Field59>("59")?;
            let field_70 = parser.parse_optional_field::<Field70>("70")?;
            let field_77b = parser.parse_optional_field::<Field77B>("77B")?;
            let field_33b = parser.parse_optional_field::<Field33B>("33B")?;
            let field_71a = parser.parse_field::<Field71A>("71A")?; // Mandatory
            let field_25a = parser.parse_optional_field::<Field25A>("25A")?;
            let field_36 = parser.parse_optional_field::<Field36>("36")?;

            transactions.push(MT101Transaction {
                field_21,
                field_21f,
                field_23e,
                field_32b,
                instructing_party_tx,
                ordering_customer_tx,
                field_52,
                field_56,
                field_57,
                field_59,
                field_70,
                field_77b,
                field_33b,
                field_71a,
                field_25a,
                field_36,
            });
        }

        // Verify all content is consumed
        verify_parser_complete(&parser)?;

        Ok(Self {
            field_20,
            field_21r,
            field_28d,
            instructing_party,
            ordering_customer,
            field_52a,
            field_51a,
            field_30,
            field_25,
            transactions,
        })
    }

    /// Parse from generic SWIFT input (tries to detect blocks)
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        let block4 = extract_block4(input)?;
        Self::parse_from_block4(&block4)
    }

    /// Convert to SWIFT MT text format
    pub fn to_mt_string(&self) -> String {
        let mut result = String::new();

        // Add mandatory fields in sequence A
        append_field(&mut result, &self.field_20);
        append_optional_field(&mut result, &self.field_21r);
        append_field(&mut result, &self.field_28d);
        append_optional_field(&mut result, &self.instructing_party);
        append_optional_field(&mut result, &self.ordering_customer);
        append_optional_field(&mut result, &self.field_52a);
        append_optional_field(&mut result, &self.field_51a);
        append_field(&mut result, &self.field_30);
        append_optional_field(&mut result, &self.field_25);

        // Add transactions (sequence B)
        for transaction in &self.transactions {
            append_field(&mut result, &transaction.field_21);
            append_optional_field(&mut result, &transaction.field_21f);
            append_vec_field(&mut result, &transaction.field_23e);
            append_field(&mut result, &transaction.field_32b);
            append_optional_field(&mut result, &transaction.instructing_party_tx);
            append_optional_field(&mut result, &transaction.ordering_customer_tx);
            append_optional_field(&mut result, &transaction.field_52);
            append_optional_field(&mut result, &transaction.field_56);
            append_optional_field(&mut result, &transaction.field_57);
            append_field(&mut result, &transaction.field_59);
            append_optional_field(&mut result, &transaction.field_70);
            append_optional_field(&mut result, &transaction.field_77b);
            append_optional_field(&mut result, &transaction.field_33b);
            append_field(&mut result, &transaction.field_71a);
            append_optional_field(&mut result, &transaction.field_25a);
            append_optional_field(&mut result, &transaction.field_36);
        }

        finalize_mt_string(result, false)
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT101)
    // ========================================================================

    /// Field 23E valid instruction codes for MT101
    const MT101_VALID_23E_CODES: &'static [&'static str] = &[
        "CHQB", "CMSW", "CMTO", "CMZB", "CORT", "EQUI", "INTC", "NETS", "OTHR", "PHON", "REPA",
        "RTGS", "URGP",
    ];

    /// Field 23E codes that allow additional information
    const CODES_WITH_ADDITIONAL_INFO: &'static [&'static str] = &["CMTO", "PHON", "OTHR", "REPA"];

    /// Field 23E invalid code combinations
    const INVALID_23E_COMBINATIONS: &'static [(&'static str, &'static [&'static str])] = &[
        (
            "CHQB",
            &[
                "CMSW", "CMTO", "CMZB", "CORT", "NETS", "PHON", "REPA", "RTGS", "URGP",
            ],
        ),
        ("CMSW", &["CMTO", "CMZB"]),
        ("CMTO", &["CMZB"]),
        ("CORT", &["CMSW", "CMTO", "CMZB", "REPA"]),
        ("EQUI", &["CMSW", "CMTO", "CMZB"]),
        ("NETS", &["RTGS"]),
    ];

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Check if ordering customer (F/G/H) is present in Sequence A
    fn has_ordering_customer_in_seq_a(&self) -> bool {
        self.ordering_customer.is_some()
    }

    /// Check if ordering customer (F/G/H) is present in all Sequence B transactions
    fn has_ordering_customer_in_all_seq_b(&self) -> bool {
        !self.transactions.is_empty()
            && self
                .transactions
                .iter()
                .all(|tx| tx.ordering_customer_tx.is_some())
    }

    /// Check if ordering customer (F/G/H) is present in any Sequence B transaction
    fn has_ordering_customer_in_any_seq_b(&self) -> bool {
        self.transactions
            .iter()
            .any(|tx| tx.ordering_customer_tx.is_some())
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

    /// Check if account servicing institution is present in Sequence A
    fn has_account_servicing_in_seq_a(&self) -> bool {
        self.field_52a.is_some()
    }

    /// Check if account servicing institution is present in any Sequence B transaction
    fn has_account_servicing_in_any_seq_b(&self) -> bool {
        self.transactions.iter().any(|tx| tx.field_52.is_some())
    }

    // ========================================================================
    // VALIDATION RULES (C1-C9)
    // ========================================================================

    /// C1: Exchange Rate and F/X Deal Reference (Error code: D54)
    /// If field 36 is present, field 21F must be present
    fn validate_c1_fx_deal_reference(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, transaction) in self.transactions.iter().enumerate() {
            if transaction.field_36.is_some() && transaction.field_21f.is_none() {
                errors.push(SwiftValidationError::content_error(
                    "D54",
                    "21F",
                    "",
                    &format!(
                        "Transaction {}: Field 21F (F/X Deal Reference) is mandatory when field 36 (Exchange Rate) is present",
                        idx + 1
                    ),
                    "If an exchange rate is given in field 36, the corresponding forex deal must be referenced in field 21F",
                ));
            }
        }

        errors
    }

    /// C2: Field 33B, 32B Amount, and Field 36 (Error code: D60)
    /// Dependencies between fields 33B, 32B amount, and 36
    fn validate_c2_amount_exchange(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, transaction) in self.transactions.iter().enumerate() {
            if let Some(ref _field_33b) = transaction.field_33b {
                // Check if amount in field_32b is zero
                let amount_is_zero = transaction.field_32b.amount.abs() < 0.01;

                if amount_is_zero {
                    // Field 33B present AND 32B amount = 0 → field 36 NOT allowed
                    if transaction.field_36.is_some() {
                        errors.push(SwiftValidationError::content_error(
                            "D60",
                            "36",
                            "",
                            &format!(
                                "Transaction {}: Field 36 (Exchange Rate) is not allowed when field 33B is present and amount in field 32B is zero",
                                idx + 1
                            ),
                            "When field 33B is present and amount in field 32B equals zero, field 36 must not be present",
                        ));
                    }
                } else {
                    // Field 33B present AND 32B amount ≠ 0 → field 36 MANDATORY
                    if transaction.field_36.is_none() {
                        errors.push(SwiftValidationError::content_error(
                            "D60",
                            "36",
                            "",
                            &format!(
                                "Transaction {}: Field 36 (Exchange Rate) is mandatory when field 33B is present and amount in field 32B is not zero",
                                idx + 1
                            ),
                            "When field 33B is present and amount in field 32B is not equal to zero, field 36 must be present",
                        ));
                    }
                }
            } else {
                // Field 33B NOT present → field 36 NOT allowed
                if transaction.field_36.is_some() {
                    errors.push(SwiftValidationError::content_error(
                        "D60",
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

    /// C3: Ordering Customer - Single vs Multiple Debit Accounts (Error code: D61)
    /// Field 50a (F/G/H) must be in EITHER Seq A OR every Seq B, never both, never neither
    fn validate_c3_ordering_customer(&self) -> Option<SwiftValidationError> {
        let in_seq_a = self.has_ordering_customer_in_seq_a();
        let in_all_seq_b = self.has_ordering_customer_in_all_seq_b();
        let in_any_seq_b = self.has_ordering_customer_in_any_seq_b();

        if in_seq_a && in_any_seq_b {
            // Present in both sequences - NOT ALLOWED
            return Some(SwiftValidationError::content_error(
                "D61",
                "50a",
                "",
                "Field 50a (Ordering Customer F/G/H) must not be present in both Sequence A and Sequence B",
                "If single debit account, field 50a (F/G/H) must be in Sequence A only. If multiple debit accounts, field 50a (F/G/H) must be in every Sequence B transaction only",
            ));
        }

        if !in_seq_a && !in_all_seq_b {
            // Not present in Seq A, and not in all Seq B transactions
            if in_any_seq_b {
                // Present in some but not all Seq B transactions
                return Some(SwiftValidationError::content_error(
                    "D61",
                    "50a",
                    "",
                    "Field 50a (Ordering Customer F/G/H) must be present in every Sequence B transaction when using multiple debit accounts",
                    "When field 50a (F/G/H) is not in Sequence A, it must be present in every occurrence of Sequence B",
                ));
            } else {
                // Not present anywhere - NOT ALLOWED
                return Some(SwiftValidationError::content_error(
                    "D61",
                    "50a",
                    "",
                    "Field 50a (Ordering Customer F/G/H) must be present in either Sequence A or in every Sequence B transaction",
                    "Field 50a (F/G/H) must be present in either Sequence A (single debit account) or in each occurrence of Sequence B (multiple debit accounts)",
                ));
            }
        }

        None
    }

    /// C4: Instructing Party Field (Error code: D62)
    /// Field 50a (C/L) may be in Seq A OR Seq B, but not both
    fn validate_c4_instructing_party(&self) -> Option<SwiftValidationError> {
        let in_seq_a = self.has_instructing_party_in_seq_a();
        let in_any_seq_b = self.has_instructing_party_in_any_seq_b();

        if in_seq_a && in_any_seq_b {
            return Some(SwiftValidationError::content_error(
                "D62",
                "50a",
                "",
                "Field 50a (Instructing Party C/L) must not be present in both Sequence A and Sequence B",
                "Field 50a (C/L) may be present in either Sequence A or in one or more occurrences of Sequence B, but must not be present in both sequences",
            ));
        }

        None
    }

    /// C5: Currency Codes in Fields 33B and 32B (Error code: D68)
    /// If field 33B is present, its currency code must differ from field 32B currency code
    fn validate_c5_currency_codes(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, transaction) in self.transactions.iter().enumerate() {
            if let Some(ref field_33b) = transaction.field_33b {
                let currency_32b = &transaction.field_32b.currency;
                let currency_33b = &field_33b.currency;

                if currency_32b == currency_33b {
                    errors.push(SwiftValidationError::content_error(
                        "D68",
                        "33B",
                        currency_33b,
                        &format!(
                            "Transaction {}: Currency code in field 33B ({}) must be different from currency code in field 32B ({})",
                            idx + 1, currency_33b, currency_32b
                        ),
                        "When field 33B is present, its currency code must differ from the currency code in field 32B within the same transaction",
                    ));
                }
            }
        }

        errors
    }

    /// C6: Account Servicing Institution Field (Error code: D64)
    /// Field 52a may be in Seq A OR Seq B, but not both
    fn validate_c6_account_servicing(&self) -> Option<SwiftValidationError> {
        let in_seq_a = self.has_account_servicing_in_seq_a();
        let in_any_seq_b = self.has_account_servicing_in_any_seq_b();

        if in_seq_a && in_any_seq_b {
            return Some(SwiftValidationError::content_error(
                "D64",
                "52a",
                "",
                "Field 52a (Account Servicing Institution) must not be present in both Sequence A and Sequence B",
                "Field 52a may be present in either Sequence A or in one or more occurrences of Sequence B, but must not be present in both sequences",
            ));
        }

        None
    }

    /// C7: Intermediary and Account With Institution (Error code: D65)
    /// If field 56a is present, field 57a must also be present
    fn validate_c7_intermediary(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, transaction) in self.transactions.iter().enumerate() {
            if transaction.field_56.is_some() && transaction.field_57.is_none() {
                errors.push(SwiftValidationError::content_error(
                    "D65",
                    "57a",
                    "",
                    &format!(
                        "Transaction {}: Field 57a (Account With Institution) is mandatory when field 56a (Intermediary) is present",
                        idx + 1
                    ),
                    "If field 56a is present, field 57a must also be present",
                ));
            }
        }

        errors
    }

    /// C8: Customer Specified Reference and Currency Consistency (Error code: D98)
    /// If field 21R is present, all transactions must have the same currency in field 32B
    fn validate_c8_currency_consistency(&self) -> Option<SwiftValidationError> {
        self.field_21r.as_ref()?;

        if self.transactions.is_empty() {
            return None;
        }

        // Get the currency from the first transaction
        let first_currency = &self.transactions[0].field_32b.currency;

        // Check if all transactions have the same currency
        for (idx, transaction) in self.transactions.iter().enumerate().skip(1) {
            if &transaction.field_32b.currency != first_currency {
                return Some(SwiftValidationError::content_error(
                    "D98",
                    "32B",
                    &transaction.field_32b.currency,
                    &format!(
                        "Transaction {}: Currency code in field 32B ({}) must be the same as in other transactions ({}) when field 21R is present",
                        idx + 1,
                        transaction.field_32b.currency,
                        first_currency
                    ),
                    "When field 21R is present in Sequence A, the currency code in field 32B must be the same in all occurrences of Sequence B",
                ));
            }
        }

        None
    }

    /// C9: Fields 33B, 21F, 32B and 23E Dependencies (Error code: E54)
    /// Complex dependencies for zero-amount transactions
    fn validate_c9_zero_amount(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, transaction) in self.transactions.iter().enumerate() {
            // Check if amount in field_32b is zero
            let amount_is_zero = transaction.field_32b.amount.abs() < 0.01;

            if amount_is_zero {
                // Check if field 23E has EQUI code
                let has_equi = transaction
                    .field_23e
                    .as_ref()
                    .is_some_and(|codes| codes.iter().any(|code| code.instruction_code == "EQUI"));

                if has_equi {
                    // Amount = 0 AND 23E = EQUI → 33B MANDATORY, 21F OPTIONAL
                    if transaction.field_33b.is_none() {
                        errors.push(SwiftValidationError::relation_error(
                            "E54",
                            "33B",
                            vec!["32B".to_string(), "23E".to_string()],
                            &format!(
                                "Transaction {}: Field 33B is mandatory when amount in field 32B is zero and field 23E contains code EQUI",
                                idx + 1
                            ),
                            "When amount in field 32B equals zero and field 23E is present with code EQUI, field 33B is mandatory",
                        ));
                    }
                } else {
                    // Amount = 0 AND (23E ≠ EQUI OR no 23E) → 33B and 21F NOT ALLOWED
                    if transaction.field_33b.is_some() {
                        errors.push(SwiftValidationError::relation_error(
                            "E54",
                            "33B",
                            vec!["32B".to_string(), "23E".to_string()],
                            &format!(
                                "Transaction {}: Field 33B is not allowed when amount in field 32B is zero and field 23E does not contain code EQUI",
                                idx + 1
                            ),
                            "When amount in field 32B equals zero and field 23E is not present or does not contain code EQUI, field 33B must not be present",
                        ));
                    }
                    if transaction.field_21f.is_some() {
                        errors.push(SwiftValidationError::relation_error(
                            "E54",
                            "21F",
                            vec!["32B".to_string(), "23E".to_string()],
                            &format!(
                                "Transaction {}: Field 21F is not allowed when amount in field 32B is zero and field 23E does not contain code EQUI",
                                idx + 1
                            ),
                            "When amount in field 32B equals zero and field 23E is not present or does not contain code EQUI, field 21F must not be present",
                        ));
                    }
                }
            }
        }

        errors
    }

    /// Validate Field 23E instruction codes (Error codes: T47, D66, D67, E46)
    /// Complex validation for instruction code combinations and restrictions
    fn validate_field_23e(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        for (idx, transaction) in self.transactions.iter().enumerate() {
            if let Some(ref field_23e_vec) = transaction.field_23e {
                let mut seen_codes = HashSet::new();

                for field_23e in field_23e_vec {
                    let code = &field_23e.instruction_code;

                    // T47: Validate instruction code is in allowed list
                    if !Self::MT101_VALID_23E_CODES.contains(&code.as_str()) {
                        errors.push(SwiftValidationError::format_error(
                            "T47",
                            "23E",
                            code,
                            &format!("One of: {}", Self::MT101_VALID_23E_CODES.join(", ")),
                            &format!(
                                "Transaction {}: Instruction code '{}' is not valid for MT101. Valid codes: {}",
                                idx + 1,
                                code,
                                Self::MT101_VALID_23E_CODES.join(", ")
                            ),
                        ));
                    }

                    // D66: Additional information only allowed for specific codes
                    if field_23e.additional_info.is_some()
                        && !Self::CODES_WITH_ADDITIONAL_INFO.contains(&code.as_str())
                    {
                        errors.push(SwiftValidationError::content_error(
                            "D66",
                            "23E",
                            code,
                            &format!(
                                "Transaction {}: Additional information is only allowed for codes: {}. Code '{}' does not allow additional information",
                                idx + 1,
                                Self::CODES_WITH_ADDITIONAL_INFO.join(", "),
                                code
                            ),
                            "Additional information in field 23E is only allowed for codes: CMTO, PHON, OTHR, REPA",
                        ));
                    }

                    // E46: Same code must not be present more than once (except OTHR)
                    if code != "OTHR" {
                        if seen_codes.contains(code) {
                            errors.push(SwiftValidationError::relation_error(
                                "E46",
                                "23E",
                                vec![],
                                &format!(
                                    "Transaction {}: Instruction code '{}' appears more than once. Same code must not be repeated except OTHR",
                                    idx + 1, code
                                ),
                                "When field 23E is repeated in Sequence B, the same code must not be present more than once, except for code OTHR which may be repeated",
                            ));
                        }
                        seen_codes.insert(code.clone());
                    }
                }

                // D67: Check for invalid combinations
                for field_23e in field_23e_vec {
                    let code = &field_23e.instruction_code;

                    for &(base_code, forbidden_codes) in Self::INVALID_23E_COMBINATIONS {
                        if code == base_code {
                            // Check if any forbidden code is present
                            for other_field in field_23e_vec {
                                let other_code = &other_field.instruction_code;
                                if forbidden_codes.contains(&other_code.as_str()) {
                                    errors.push(SwiftValidationError::content_error(
                                        "D67",
                                        "23E",
                                        code,
                                        &format!(
                                            "Transaction {}: Instruction code '{}' cannot be combined with code '{}'. Invalid combination",
                                            idx + 1, code, other_code
                                        ),
                                        &format!(
                                            "Code '{}' cannot be combined with: {}",
                                            base_code,
                                            forbidden_codes.join(", ")
                                        ),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        errors
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // C1: Exchange Rate and F/X Deal Reference
        let c1_errors = self.validate_c1_fx_deal_reference();
        all_errors.extend(c1_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C2: Field 33B, 32B, and 36 Dependencies
        let c2_errors = self.validate_c2_amount_exchange();
        all_errors.extend(c2_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C3: Ordering Customer Placement
        if let Some(error) = self.validate_c3_ordering_customer() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C4: Instructing Party Placement
        if let Some(error) = self.validate_c4_instructing_party() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C5: Currency Code Mismatch
        let c5_errors = self.validate_c5_currency_codes();
        all_errors.extend(c5_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C6: Account Servicing Institution
        if let Some(error) = self.validate_c6_account_servicing() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C7: Intermediary & Account With
        let c7_errors = self.validate_c7_intermediary();
        all_errors.extend(c7_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C8: Currency Consistency
        if let Some(error) = self.validate_c8_currency_consistency() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C9: Zero Amount Dependencies
        let c9_errors = self.validate_c9_zero_amount();
        all_errors.extend(c9_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // Field 23E Validation
        let f23e_errors = self.validate_field_23e();
        all_errors.extend(f23e_errors);

        all_errors
    }
}

impl crate::traits::SwiftMessageBody for MT101 {
    fn message_type() -> &'static str {
        "101"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT101::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT101::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT101::validate_network_rules(self, stop_on_first_error)
    }
}
