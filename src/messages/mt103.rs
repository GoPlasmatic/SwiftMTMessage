use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// **MT103: Single Customer Credit Transfer**
///
/// Customer payment instruction from ordering to beneficiary customer via financial institutions.
/// Most common SWIFT payment message for cross-border transfers.
///
/// **Usage:** Customer credit transfers, STP payments
/// **Category:** Category 1 (Customer Payments)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT103 {
    /// Transaction reference (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Bank operation code (Field 23B)
    #[serde(rename = "23B")]
    pub field_23b: Field23B,

    /// Value date, currency, amount (Field 32A)
    #[serde(rename = "32A")]
    pub field_32a: Field32A,

    /// Ordering customer (Field 50)
    #[serde(flatten)]
    pub field_50: Field50OrderingCustomerAFK,

    /// Beneficiary customer (Field 59)
    #[serde(flatten)]
    pub field_59: Field59,

    /// Details of charges (Field 71A)
    #[serde(rename = "71A")]
    pub field_71a: Field71A,

    /// Time indication (Field 13C)
    #[serde(rename = "13C")]
    pub field_13c: Option<Vec<Field13C>>,

    /// Instruction codes (Field 23E)
    #[serde(rename = "23E")]
    pub field_23e: Option<Vec<Field23E>>,

    /// Transaction type code (Field 26T)
    #[serde(rename = "26T")]
    pub field_26t: Option<Field26T>,

    /// Instructed amount (Field 33B)
    #[serde(rename = "33B")]
    pub field_33b: Option<Field33B>,

    /// Exchange rate (Field 36)
    #[serde(rename = "36")]
    pub field_36: Option<Field36>,

    /// Instructing institution (Field 51A)
    #[serde(rename = "51A")]
    pub field_51a: Option<Field51A>,

    /// Ordering institution (Field 52)
    #[serde(flatten)]
    pub field_52: Option<Field52OrderingInstitution>,

    /// Sender's correspondent (Field 53)
    #[serde(flatten)]
    pub field_53: Option<Field53SenderCorrespondent>,

    /// Receiver's correspondent (Field 54)
    #[serde(flatten)]
    pub field_54: Option<Field54ReceiverCorrespondent>,

    /// Third reimbursement institution (Field 55)
    #[serde(flatten)]
    pub field_55: Option<Field55ThirdReimbursementInstitution>,

    /// Intermediary institution (Field 56)
    #[serde(flatten)]
    pub field_56: Option<Field56Intermediary>,

    /// Account with institution (Field 57)
    #[serde(flatten)]
    pub field_57: Option<Field57AccountWithInstitution>,

    /// Remittance information (Field 70)
    #[serde(rename = "70")]
    pub field_70: Option<Field70>,

    /// Sender's charges (Field 71F)
    #[serde(rename = "71F")]
    pub field_71f: Option<Vec<Field71F>>,

    /// Receiver's charges (Field 71G)
    #[serde(rename = "71G")]
    pub field_71g: Option<Field71G>,

    /// Sender to receiver information (Field 72)
    #[serde(rename = "72")]
    pub field_72: Option<Field72>,

    /// Regulatory reporting (Field 77B)
    #[serde(rename = "77B")]
    pub field_77b: Option<Field77B>,

    /// Envelope contents (Field 77T)
    #[serde(rename = "77T")]
    pub field_77t: Option<Field77T>,
}

// Additional methods for MT103
impl MT103 {
    /// Parse from SWIFT MT text format
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        let block4 = extract_block4(input)?;
        <Self as crate::traits::SwiftMessageBody>::parse_from_block4(&block4)
    }

    /// Convert to SWIFT MT text format
    pub fn to_mt_string(&self) -> String {
        let mut result = String::new();

        // Add mandatory fields in order
        append_field(&mut result, &self.field_20);
        append_vec_field(&mut result, &self.field_13c);
        append_field(&mut result, &self.field_23b);
        append_vec_field(&mut result, &self.field_23e);
        append_optional_field(&mut result, &self.field_26t);
        append_field(&mut result, &self.field_32a);
        append_optional_field(&mut result, &self.field_33b);
        append_optional_field(&mut result, &self.field_36);
        append_field(&mut result, &self.field_50);
        append_optional_field(&mut result, &self.field_51a);
        append_optional_field(&mut result, &self.field_52);
        append_optional_field(&mut result, &self.field_53);
        append_optional_field(&mut result, &self.field_54);
        append_optional_field(&mut result, &self.field_55);
        append_optional_field(&mut result, &self.field_56);
        append_optional_field(&mut result, &self.field_57);
        append_field(&mut result, &self.field_59);
        append_optional_field(&mut result, &self.field_70);
        append_field(&mut result, &self.field_71a);
        append_vec_field(&mut result, &self.field_71f);
        append_optional_field(&mut result, &self.field_71g);
        append_optional_field(&mut result, &self.field_72);
        append_optional_field(&mut result, &self.field_77b);
        append_optional_field(&mut result, &self.field_77t);

        result.push('-');
        result
    }

    /// Check if this MT103 message contains reject codes
    pub fn has_reject_codes(&self) -> bool {
        // Check field 72 for reject codes like /REJT/
        if let Some(ref field_72) = self.field_72 {
            for line in &field_72.information {
                if line.contains("/REJT/") || line.contains("/RETN/") {
                    return true;
                }
            }
        }
        false
    }

    /// Check if this MT103 message contains return codes
    pub fn has_return_codes(&self) -> bool {
        // Check field 72 for return codes
        if let Some(ref field_72) = self.field_72 {
            for line in &field_72.information {
                if line.contains("/RETN/") {
                    return true;
                }
            }
        }
        false
    }

    /// Check if this MT103 message is STP compliant
    pub fn is_stp_compliant(&self) -> bool {
        // Check if this is an STP message (SPRI, SSTD, or SPAY)
        let bank_op_code = &self.field_23b.instruction_code;
        if !["SPRI", "SSTD", "SPAY"].contains(&bank_op_code.as_str()) {
            // Not an STP message type, so it's compliant by default
            return true;
        }

        // C3: If 23B is SPRI, field 23E may contain only SDVA, TELB, PHOB, INTC
        // If 23B is SSTD or SPAY, field 23E must not be used
        if bank_op_code == "SPRI" {
            if let Some(ref field_23e_vec) = self.field_23e {
                let allowed_codes = ["SDVA", "TELB", "PHOB", "INTC"];
                for field_23e in field_23e_vec {
                    if !allowed_codes.contains(&field_23e.instruction_code.as_str()) {
                        return false;
                    }
                }
            }
        } else if ["SSTD", "SPAY"].contains(&bank_op_code.as_str()) && self.field_23e.is_some() {
            return false;
        }

        // C10: If 23B is SPRI, field 56 is not allowed
        // If 23B is SSTD or SPAY, field 56 may be present but only option A or C
        if bank_op_code == "SPRI" && self.field_56.is_some() {
            return false;
        }

        // Additional STP validation rules could be added here
        // For now, return true if basic checks pass
        true
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT103 STP & REMIT)
    // ========================================================================

    /// Field 23B valid bank operation codes for MT103
    const MT103_VALID_23B_CODES: &'static [&'static str] =
        &["CRED", "CRTS", "SPAY", "SPRI", "SSTD"];

    /// Field 23E valid instruction codes for MT103 (combined STP and REMIT)
    const MT103_VALID_23E_CODES: &'static [&'static str] = &[
        "CHQB", "CORT", "HOLD", "INTC", "PHOB", "PHOI", "PHON", "REPA", "SDVA", "TELB", "TELE",
        "TELI",
    ];

    /// Field 23E codes that allow additional information
    const CODES_WITH_ADDITIONAL_INFO: &'static [&'static str] = &[
        "PHON", "PHOB", "PHOI", "TELE", "TELB", "TELI", "HOLD", "REPA",
    ];

    /// Field 23E valid codes for REMIT when 23B is SPRI
    const REMIT_SPRI_ALLOWED_23E: &'static [&'static str] = &["SDVA", "TELB", "PHOB", "INTC"];

    /// Field 23E invalid code combinations
    const INVALID_23E_COMBINATIONS: &'static [(&'static str, &'static [&'static str])] = &[
        ("SDVA", &["HOLD", "CHQB"]),
        ("INTC", &["HOLD", "CHQB"]),
        ("REPA", &["HOLD", "CHQB", "CORT"]),
        ("CORT", &["HOLD", "CHQB"]),
        ("HOLD", &["CHQB"]),
        ("PHOB", &["TELB"]),
        ("PHON", &["TELE"]),
        ("PHOI", &["TELI"]),
    ];

    /// Field 23E code ordering for validation (D98)
    const FIELD_23E_CODE_ORDER: &'static [&'static str] = &[
        "SDVA", "INTC", "REPA", "CORT", "HOLD", "CHQB", "PHOB", "TELB", "PHON", "TELE", "PHOI",
        "TELI",
    ];

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Check if field 56a is present
    fn has_field_56(&self) -> bool {
        self.field_56.is_some()
    }

    /// Check if field 57a is present
    fn has_field_57(&self) -> bool {
        self.field_57.is_some()
    }

    /// Check if field 53a is present
    fn has_field_53(&self) -> bool {
        self.field_53.is_some()
    }

    /// Check if field 54a is present
    fn has_field_54(&self) -> bool {
        self.field_54.is_some()
    }

    /// Check if field 55a is present
    fn has_field_55(&self) -> bool {
        self.field_55.is_some()
    }

    /// Check if field 71F is present (any occurrence)
    fn has_field_71f(&self) -> bool {
        self.field_71f.is_some() && !self.field_71f.as_ref().unwrap().is_empty()
    }

    /// Check if field 71G is present
    fn has_field_71g(&self) -> bool {
        self.field_71g.is_some()
    }

    // ========================================================================
    // VALIDATION RULES (C1-C18, T36, T48, etc.)
    // ========================================================================

    /// C1: Currency/Instructed Amount and Exchange Rate (Error code: D75)
    /// If field 33B is present and currency differs from 32A, field 36 must be present
    fn validate_c1_currency_exchange(&self) -> Option<SwiftValidationError> {
        if let Some(ref field_33b) = self.field_33b {
            let currency_32a = &self.field_32a.currency;
            let currency_33b = &field_33b.currency;

            if currency_32a != currency_33b {
                // Currencies differ - field 36 is mandatory
                if self.field_36.is_none() {
                    return Some(SwiftValidationError::content_error(
                        "D75",
                        "36",
                        "",
                        "Field 36 (Exchange Rate) is mandatory when field 33B is present and currency code differs from field 32A",
                        "If field 33B is present and the currency code is different from the currency code in field 32A, field 36 must be present",
                    ));
                }
            } else {
                // Currencies are the same - field 36 is not allowed
                if self.field_36.is_some() {
                    return Some(SwiftValidationError::content_error(
                        "D75",
                        "36",
                        "",
                        "Field 36 (Exchange Rate) is not allowed when field 33B currency code is the same as field 32A",
                        "If field 33B is present and the currency code is equal to the currency code in field 32A, field 36 must not be present",
                    ));
                }
            }
        } else {
            // Field 33B not present - field 36 is not allowed
            if self.field_36.is_some() {
                return Some(SwiftValidationError::content_error(
                    "D75",
                    "36",
                    "",
                    "Field 36 (Exchange Rate) is not allowed when field 33B is not present",
                    "Field 36 is only allowed when field 33B is present",
                ));
            }
        }

        None
    }

    /// C3: Field 23B and 23E Code Dependencies (Error codes: E01, E02)
    /// Restricts field 23E codes based on field 23B value
    fn validate_c3_bank_op_instruction_codes(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();
        let bank_op_code = &self.field_23b.instruction_code;

        if bank_op_code == "SPRI" {
            // For SPRI: field 23E may contain only SDVA or INTC (STP) or SDVA, TELB, PHOB, INTC (REMIT)
            // We'll allow the broader REMIT set to cover both cases
            if let Some(ref field_23e_vec) = self.field_23e {
                for field_23e in field_23e_vec {
                    let code = &field_23e.instruction_code;
                    if !Self::REMIT_SPRI_ALLOWED_23E.contains(&code.as_str()) {
                        errors.push(SwiftValidationError::content_error(
                            "E01",
                            "23E",
                            code,
                            &format!(
                                "When field 23B is SPRI, field 23E may only contain codes: {}. Code '{}' is not allowed",
                                Self::REMIT_SPRI_ALLOWED_23E.join(", "),
                                code
                            ),
                            "If field 23B contains SPRI, field 23E may contain only SDVA, TELB, PHOB, or INTC",
                        ));
                    }
                }
            }
        } else if bank_op_code == "SSTD" || bank_op_code == "SPAY" {
            // For SSTD or SPAY: field 23E must not be used
            if self.field_23e.is_some() {
                errors.push(SwiftValidationError::content_error(
                    "E02",
                    "23E",
                    "",
                    &format!(
                        "When field 23B is {} or {}, field 23E must not be used",
                        "SSTD", "SPAY"
                    ),
                    "If field 23B contains one of the codes SSTD or SPAY, field 23E must not be used",
                ));
            }
        }

        errors
    }

    /// C4: Third Reimbursement Institution Dependencies (Error code: E06)
    /// If field 55a is present, both fields 53a and 54a must also be present
    fn validate_c4_third_reimbursement(&self) -> Option<SwiftValidationError> {
        if self.has_field_55() && (!self.has_field_53() || !self.has_field_54()) {
            return Some(SwiftValidationError::content_error(
                "E06",
                "55a",
                "",
                "Fields 53a (Sender's Correspondent) and 54a (Receiver's Correspondent) are mandatory when field 55a (Third Reimbursement Institution) is present",
                "If field 55a is present, both fields 53a and 54a must also be present",
            ));
        }

        None
    }

    /// C5 (C9): Intermediary and Account With Institution (Error code: C81)
    /// If field 56a is present, field 57a must also be present
    fn validate_c5_intermediary(&self) -> Option<SwiftValidationError> {
        if self.has_field_56() && !self.has_field_57() {
            return Some(SwiftValidationError::content_error(
                "C81",
                "57a",
                "",
                "Field 57a (Account With Institution) is mandatory when field 56a (Intermediary) is present",
                "If field 56a is present, field 57a must also be present",
            ));
        }

        None
    }

    /// C6 (C10): Field 23B SPRI and Field 56A (Error codes: E16, E17)
    /// If field 23B is SPRI, field 56a must not be present
    /// If field 23B is SSTD or SPAY, field 56a may be used with option A or C only
    fn validate_c6_field_56_restrictions(&self) -> Option<SwiftValidationError> {
        let bank_op_code = &self.field_23b.instruction_code;

        if bank_op_code == "SPRI" && self.has_field_56() {
            return Some(SwiftValidationError::content_error(
                "E16",
                "56a",
                "",
                "Field 56a (Intermediary Institution) must not be present when field 23B is SPRI",
                "If field 23B contains the code SPRI, field 56a must not be present",
            ));
        }

        None
    }

    /// C7 (C14): Details of Charges and Sender's/Receiver's Charges (Error codes: E13, D50, E15)
    /// Complex rules for fields 71A, 71F, and 71G
    fn validate_c7_charges(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();
        let charges_code = &self.field_71a.code;

        match charges_code.as_str() {
            "OUR" => {
                // If 71A is OUR, field 71F is not allowed, field 71G is optional
                if self.has_field_71f() {
                    errors.push(SwiftValidationError::content_error(
                        "E13",
                        "71F",
                        "",
                        "Field 71F (Sender's Charges) is not allowed when field 71A is OUR",
                        "If field 71A contains OUR, then field 71F is not allowed",
                    ));
                }
            }
            "SHA" => {
                // If 71A is SHA, field 71F is optional, field 71G is not allowed
                if self.has_field_71g() {
                    errors.push(SwiftValidationError::content_error(
                        "D50",
                        "71G",
                        "",
                        "Field 71G (Receiver's Charges) is not allowed when field 71A is SHA",
                        "If field 71A contains SHA, then field 71G is not allowed",
                    ));
                }
            }
            "BEN" => {
                // If 71A is BEN, at least one occurrence of 71F is mandatory, 71G is not allowed
                if !self.has_field_71f() {
                    errors.push(SwiftValidationError::content_error(
                        "E15",
                        "71F",
                        "",
                        "At least one occurrence of field 71F (Sender's Charges) is mandatory when field 71A is BEN",
                        "If field 71A contains BEN, then at least one occurrence of field 71F is mandatory",
                    ));
                }
                if self.has_field_71g() {
                    errors.push(SwiftValidationError::content_error(
                        "E15",
                        "71G",
                        "",
                        "Field 71G (Receiver's Charges) is not allowed when field 71A is BEN",
                        "If field 71A contains BEN, then field 71G is not allowed",
                    ));
                }
            }
            _ => {}
        }

        errors
    }

    /// C8 (C15): Sender's/Receiver's Charges and Field 33B (Error code: D51)
    /// If either field 71F or field 71G is present, then field 33B is mandatory
    fn validate_c8_charges_instructed_amount(&self) -> Option<SwiftValidationError> {
        if (self.has_field_71f() || self.has_field_71g()) && self.field_33b.is_none() {
            return Some(SwiftValidationError::content_error(
                "D51",
                "33B",
                "",
                "Field 33B (Currency/Instructed Amount) is mandatory when field 71F or 71G is present",
                "If either field 71F (at least one occurrence) or field 71G is present, then field 33B is mandatory",
            ));
        }

        None
    }

    /// C9 (C18): Currency Codes in Fields 71G and 32A (Error code: C02)
    /// The currency code in fields 71G and 32A must be the same
    fn validate_c9_receiver_charges_currency(&self) -> Option<SwiftValidationError> {
        if let Some(ref field_71g) = self.field_71g {
            let currency_32a = &self.field_32a.currency;
            let currency_71g = &field_71g.currency;

            if currency_32a != currency_71g {
                return Some(SwiftValidationError::content_error(
                    "C02",
                    "71G",
                    currency_71g,
                    &format!(
                        "Currency code in field 71G ({}) must be the same as in field 32A ({})",
                        currency_71g, currency_32a
                    ),
                    "The currency code in fields 71G and 32A must be the same",
                ));
            }
        }

        None
    }

    /// C13: Field 59a Account Restriction for Cheque (Error code: E18)
    /// If any field 23E contains CHQB, subfield 1 (Account) in field 59a is not allowed
    fn validate_c13_chqb_beneficiary_account(&self) -> Option<SwiftValidationError> {
        if let Some(ref field_23e_vec) = self.field_23e {
            let has_chqb = field_23e_vec.iter().any(|f| f.instruction_code == "CHQB");

            if has_chqb {
                // Check if field 59 has account - this depends on the variant
                // Field59F has party_identifier instead of account, and account is not restricted for F variant
                let has_account = match &self.field_59 {
                    Field59::NoOption(f) => f.account.is_some(),
                    Field59::A(f) => f.account.is_some(),
                    Field59::F(_) => false, // Option F uses party_identifier, not account
                };

                if has_account {
                    return Some(SwiftValidationError::content_error(
                        "E18",
                        "59a",
                        "",
                        "Subfield 1 (Account) in field 59a (Beneficiary Customer) is not allowed when field 23E contains code CHQB",
                        "If any field 23E contains the code CHQB, subfield 1 (Account) in field 59a Beneficiary Customer is not allowed",
                    ));
                }
            }
        }

        None
    }

    /// C16: Field 23E TELI/PHOI Restriction (Error code: E44)
    /// If field 56a is not present, no field 23E may contain TELI or PHOI
    fn validate_c16_teli_phoi_restriction(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        if !self.has_field_56()
            && let Some(ref field_23e_vec) = self.field_23e
        {
            for field_23e in field_23e_vec {
                let code = &field_23e.instruction_code;
                if code == "TELI" || code == "PHOI" {
                    errors.push(SwiftValidationError::content_error(
                        "E44",
                        "23E",
                        code,
                        &format!(
                            "Field 23E code '{}' is not allowed when field 56a is not present",
                            code
                        ),
                        "If field 56a is not present, no field 23E may contain TELI or PHOI",
                    ));
                }
            }
        }

        errors
    }

    /// C17: Field 23E TELE/PHON Restriction (Error code: E45)
    /// If field 57a is not present, no field 23E may contain TELE or PHON
    fn validate_c17_tele_phon_restriction(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        if !self.has_field_57()
            && let Some(ref field_23e_vec) = self.field_23e
        {
            for field_23e in field_23e_vec {
                let code = &field_23e.instruction_code;
                if code == "TELE" || code == "PHON" {
                    errors.push(SwiftValidationError::content_error(
                        "E45",
                        "23E",
                        code,
                        &format!(
                            "Field 23E code '{}' is not allowed when field 57a is not present",
                            code
                        ),
                        "If field 57a is not present, no field 23E may contain TELE or PHON",
                    ));
                }
            }
        }

        errors
    }

    /// Validate Field 23B bank operation code (Error code: T36)
    fn validate_field_23b(&self) -> Option<SwiftValidationError> {
        let code = &self.field_23b.instruction_code;

        if !Self::MT103_VALID_23B_CODES.contains(&code.as_str()) {
            return Some(SwiftValidationError::format_error(
                "T36",
                "23B",
                code,
                &format!("One of: {}", Self::MT103_VALID_23B_CODES.join(", ")),
                &format!(
                    "Bank operation code '{}' is not valid for MT103. Valid codes: {}",
                    code,
                    Self::MT103_VALID_23B_CODES.join(", ")
                ),
            ));
        }

        None
    }

    /// Validate Field 23E instruction codes (Error codes: T48, D97, D98, D67, E46)
    /// Complex validation for instruction code combinations and restrictions
    fn validate_field_23e(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        if let Some(ref field_23e_vec) = self.field_23e {
            let mut seen_codes = HashSet::new();
            let mut code_positions: Vec<(String, usize)> = Vec::new();

            for field_23e in field_23e_vec {
                let code = &field_23e.instruction_code;

                // T48: Validate instruction code is in allowed list
                if !Self::MT103_VALID_23E_CODES.contains(&code.as_str()) {
                    errors.push(SwiftValidationError::format_error(
                        "T48",
                        "23E",
                        code,
                        &format!("One of: {}", Self::MT103_VALID_23E_CODES.join(", ")),
                        &format!(
                            "Instruction code '{}' is not valid for MT103. Valid codes: {}",
                            code,
                            Self::MT103_VALID_23E_CODES.join(", ")
                        ),
                    ));
                }

                // D97: Additional information only allowed for specific codes
                if field_23e.additional_info.is_some()
                    && !Self::CODES_WITH_ADDITIONAL_INFO.contains(&code.as_str())
                {
                    errors.push(SwiftValidationError::content_error(
                        "D97",
                        "23E",
                        code,
                        &format!(
                            "Additional information is only allowed for codes: {}. Code '{}' does not allow additional information",
                            Self::CODES_WITH_ADDITIONAL_INFO.join(", "),
                            code
                        ),
                        "Additional information in field 23E is only allowed for codes: PHON, PHOB, PHOI, TELE, TELB, TELI, HOLD, REPA",
                    ));
                }

                // E46: Same code must not be present more than once
                if seen_codes.contains(code) {
                    errors.push(SwiftValidationError::relation_error(
                        "E46",
                        "23E",
                        vec![],
                        &format!(
                            "Instruction code '{}' appears more than once. Same code must not be repeated",
                            code
                        ),
                        "When field 23E is repeated, the same code must not be present more than once",
                    ));
                }
                seen_codes.insert(code.clone());

                // Track position for ordering check
                if let Some(pos) = Self::FIELD_23E_CODE_ORDER.iter().position(|&c| c == code) {
                    code_positions.push((code.clone(), pos));
                }
            }

            // D98: Check code ordering
            for i in 1..code_positions.len() {
                if code_positions[i].1 < code_positions[i - 1].1 {
                    errors.push(SwiftValidationError::content_error(
                        "D98",
                        "23E",
                        &code_positions[i].0,
                        &format!(
                            "Instruction codes must appear in the following order: {}. Code '{}' appears out of order",
                            Self::FIELD_23E_CODE_ORDER.join(", "),
                            code_positions[i].0
                        ),
                        "When field 23E is repeated, codes must appear in specified order",
                    ));
                    break;
                }
            }

            // D67: Check for invalid combinations
            for field_23e in field_23e_vec {
                let code = &field_23e.instruction_code;

                for &(base_code, forbidden_codes) in Self::INVALID_23E_COMBINATIONS {
                    if code == base_code {
                        for other_field in field_23e_vec {
                            let other_code = &other_field.instruction_code;
                            if forbidden_codes.contains(&other_code.as_str()) {
                                errors.push(SwiftValidationError::content_error(
                                    "D67",
                                    "23E",
                                    code,
                                    &format!(
                                        "Instruction code '{}' cannot be combined with code '{}'. Invalid combination",
                                        code, other_code
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

        errors
    }

    /// Main validation method - validates all network rules
    /// Returns array of validation errors, respects stop_on_first_error flag
    pub fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        let mut all_errors = Vec::new();

        // Field 23B Validation
        if let Some(error) = self.validate_field_23b() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // Field 23E Validation
        let f23e_errors = self.validate_field_23e();
        all_errors.extend(f23e_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C1: Currency/Instructed Amount and Exchange Rate
        if let Some(error) = self.validate_c1_currency_exchange() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C3: Field 23B and 23E Code Dependencies
        let c3_errors = self.validate_c3_bank_op_instruction_codes();
        all_errors.extend(c3_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C4: Third Reimbursement Institution Dependencies
        if let Some(error) = self.validate_c4_third_reimbursement() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C5 (C9): Intermediary and Account With Institution
        if let Some(error) = self.validate_c5_intermediary() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C6 (C10): Field 23B SPRI and Field 56A
        if let Some(error) = self.validate_c6_field_56_restrictions() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C7 (C14): Details of Charges
        let c7_errors = self.validate_c7_charges();
        all_errors.extend(c7_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C8 (C15): Charges and Instructed Amount
        if let Some(error) = self.validate_c8_charges_instructed_amount() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C9 (C18): Receiver's Charges Currency
        if let Some(error) = self.validate_c9_receiver_charges_currency() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C13: CHQB Beneficiary Account Restriction
        if let Some(error) = self.validate_c13_chqb_beneficiary_account() {
            all_errors.push(error);
            if stop_on_first_error {
                return all_errors;
            }
        }

        // C16: TELI/PHOI Restriction
        let c16_errors = self.validate_c16_teli_phoi_restriction();
        all_errors.extend(c16_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C17: TELE/PHON Restriction
        let c17_errors = self.validate_c17_tele_phon_restriction();
        all_errors.extend(c17_errors);

        all_errors
    }
}

impl crate::traits::SwiftMessageBody for MT103 {
    fn message_type() -> &'static str {
        "103"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "103");

        // Parse mandatory field 20
        let field_20 = parser.parse_field::<Field20>("20")?;

        // Parse optional repeating Field13C
        parser = parser.with_duplicates(true);
        let mut field_13c = Vec::new();
        while let Ok(field) = parser.parse_field::<Field13C>("13C") {
            field_13c.push(field);
        }
        parser = parser.with_duplicates(false);

        // Parse mandatory field 23B
        let field_23b = parser.parse_field::<Field23B>("23B")?;

        // Parse optional repeating Field23E
        parser = parser.with_duplicates(true);
        let mut field_23e = Vec::new();
        while let Ok(field) = parser.parse_field::<Field23E>("23E") {
            field_23e.push(field);
        }
        parser = parser.with_duplicates(false);

        // Parse optional field 26T
        let field_26t = parser.parse_optional_field::<Field26T>("26T")?;

        // Parse mandatory field 32A
        let field_32a = parser.parse_field::<Field32A>("32A")?;

        // Parse optional fields 33B and 36
        let field_33b = parser.parse_optional_field::<Field33B>("33B")?;
        let field_36 = parser.parse_optional_field::<Field36>("36")?;

        // Parse mandatory field 50
        let field_50 = parser.parse_variant_field::<Field50OrderingCustomerAFK>("50")?;

        // Parse optional fields that come before field 59
        let field_51a = parser.parse_optional_field::<Field51A>("51A")?;
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_53 = parser.parse_optional_variant_field::<Field53SenderCorrespondent>("53")?;
        let field_54 = parser.parse_optional_variant_field::<Field54ReceiverCorrespondent>("54")?;
        let field_55 =
            parser.parse_optional_variant_field::<Field55ThirdReimbursementInstitution>("55")?;
        let field_56 = parser.parse_optional_variant_field::<Field56Intermediary>("56")?;
        let field_57 =
            parser.parse_optional_variant_field::<Field57AccountWithInstitution>("57")?;

        // Parse mandatory field 59 (after optional routing fields)
        let field_59 = parser.parse_variant_field::<Field59>("59")?;

        // Parse optional field 70
        let field_70 = parser.parse_optional_field::<Field70>("70")?;

        // Parse mandatory field 71A
        let field_71a = parser.parse_field::<Field71A>("71A")?;

        // Parse optional repeating Field71F
        parser = parser.with_duplicates(true);
        let mut field_71f = Vec::new();
        while let Ok(field) = parser.parse_field::<Field71F>("71F") {
            field_71f.push(field);
        }
        parser = parser.with_duplicates(false);

        // Parse remaining optional fields
        let field_71g = parser.parse_optional_field::<Field71G>("71G")?;
        let field_72 = parser.parse_optional_field::<Field72>("72")?;
        let field_77b = parser.parse_optional_field::<Field77B>("77B")?;
        let field_77t = parser.parse_optional_field::<Field77T>("77T")?;

        // Verify all content is consumed
        verify_parser_complete(&parser)?;

        Ok(Self {
            field_20,
            field_23b,
            field_32a,
            field_50,
            field_59,
            field_71a,
            field_13c: if field_13c.is_empty() {
                None
            } else {
                Some(field_13c)
            },
            field_23e: if field_23e.is_empty() {
                None
            } else {
                Some(field_23e)
            },
            field_26t,
            field_33b,
            field_36,
            field_51a,
            field_52,
            field_53,
            field_54,
            field_55,
            field_56,
            field_57,
            field_70,
            field_71f: if field_71f.is_empty() {
                None
            } else {
                Some(field_71f)
            },
            field_71g,
            field_72,
            field_77b,
            field_77t,
        })
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT103::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT103::validate_network_rules(self, stop_on_first_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt103_parse() {
        let mt103_text = r#":20:123456789012345
:23B:CRED
:32A:241201USD1000000,00
:50K:/12345678901234567890
JOHN DOE
123 MAIN STREET
NEW YORK, NY 10001
:59:/98765432109876543210
JANE SMITH
456 OAK AVENUE
LOS ANGELES, CA 90001
:71A:OUR
-"#;
        let result = <MT103 as crate::traits::SwiftMessageBody>::parse_from_block4(mt103_text);
        assert!(result.is_ok());
        let mt103 = result.unwrap();
        assert_eq!(mt103.field_20.reference, "123456789012345");
        assert_eq!(mt103.field_23b.instruction_code, "CRED");
        assert_eq!(mt103.field_71a.code, "OUR");
    }

    #[test]
    fn test_mt103_stp_compliance() {
        let mt103_text = r#":20:123456789012345
:23B:SPRI
:32A:241201USD1000000,00
:50K:/12345678901234567890
JOHN DOE
123 MAIN STREET
NEW YORK, NY 10001
:59:/98765432109876543210
JANE SMITH
456 OAK AVENUE
LOS ANGELES, CA 90001
:71A:OUR
-"#;
        let result = <MT103 as crate::traits::SwiftMessageBody>::parse_from_block4(mt103_text);
        assert!(result.is_ok());
        let mt103 = result.unwrap();

        // SPRI message without field 56 should be STP compliant
        assert!(mt103.is_stp_compliant());
    }
}
