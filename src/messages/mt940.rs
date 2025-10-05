use crate::errors::SwiftValidationError;
use crate::fields::*;
use crate::parser::utils::*;
use serde::{Deserialize, Serialize};

/// **MT940: Customer Statement**
///
/// Account statement with transaction details for specified period.
/// Sent from account servicing institution to account owner.
///
/// **Usage:** Daily statements, account reconciliation
/// **Category:** Category 9 (Cash Management & Customer Status)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT940 {
    /// Transaction Reference Number (Field 20)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Related Reference (Field 21)
    #[serde(rename = "21", skip_serializing_if = "Option::is_none")]
    pub field_21: Option<Field21NoOption>,

    /// Account Identification (Field 25)
    #[serde(rename = "25")]
    pub field_25: Field25NoOption,

    /// Statement Number/Sequence Number (Field 28C)
    #[serde(rename = "28C")]
    pub field_28c: Field28C,

    /// Opening Balance (Field 60F)
    #[serde(rename = "60F")]
    pub field_60f: Field60F,

    /// Statement lines (1-500 occurrences)
    #[serde(rename = "statement_lines")]
    pub statement_lines: Vec<MT940StatementLine>,

    /// Closing Balance (Field 62F)
    #[serde(rename = "62F")]
    pub field_62f: Field62F,

    /// Available Balance (Field 64)
    #[serde(rename = "64", skip_serializing_if = "Option::is_none")]
    pub field_64: Option<Field64>,

    /// Forward Available Balance (Field 65)
    #[serde(rename = "65", skip_serializing_if = "Option::is_none")]
    pub field_65: Option<Vec<Field65>>,
}

/// Statement line for MT940
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT940StatementLine {
    /// Statement Line (Field 61)
    #[serde(rename = "61")]
    pub field_61: Field61,

    /// Information to Account Owner (Field 86)
    #[serde(rename = "86", skip_serializing_if = "Option::is_none")]
    pub field_86: Option<Field86>,
}

impl MT940 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::parser::MessageParser::new(block4, "940");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_optional_field::<Field21NoOption>("21")?;
        let field_25 = parser.parse_field::<Field25NoOption>("25")?;
        let field_28c = parser.parse_field::<Field28C>("28C")?;
        let field_60f = parser.parse_field::<Field60F>("60F")?;

        // Enable duplicate field handling for statement lines
        parser = parser.with_duplicates(true);

        // Parse statement lines (1-500)
        let mut statement_lines = Vec::new();

        while parser.detect_field("61") && statement_lines.len() < 500 {
            let field_61 = parser.parse_field::<Field61>("61")?;
            let field_86 = parser.parse_optional_field::<Field86>("86")?;

            statement_lines.push(MT940StatementLine { field_61, field_86 });
        }

        // Disable duplicates mode after parsing statement lines
        parser = parser.with_duplicates(false);

        // Must have at least one statement line
        if statement_lines.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT940: At least one statement line (field 61) is required".to_string(),
            });
        }

        // Parse mandatory closing balance
        let field_62f = parser.parse_field::<Field62F>("62F")?;

        // Parse optional fields
        let field_64 = parser.parse_optional_field::<Field64>("64")?;

        // Parse optional repetitive Field 65 (Forward Available Balance)
        parser = parser.with_duplicates(true);
        let mut forward_balances = Vec::new();
        while let Ok(field_65) = parser.parse_field::<Field65>("65") {
            forward_balances.push(field_65);
        }

        let field_65 = if forward_balances.is_empty() {
            None
        } else {
            Some(forward_balances)
        };

        Ok(MT940 {
            field_20,
            field_21,
            field_25,
            field_28c,
            field_60f,
            statement_lines,
            field_62f,
            field_64,
            field_65,
        })
    }

    /// Validate the message instance according to MT940 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // C1: Statement lines must occur 1-500 times
        if self.statement_lines.is_empty() || self.statement_lines.len() > 500 {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "MT940: Statement lines must occur 1-500 times, found {}",
                    self.statement_lines.len()
                ),
            });
        }

        // C2 is automatically satisfied as fields 60F and 62F are mandatory

        Ok(())
    }

    // ========================================================================
    // NETWORK VALIDATION RULES (SR 2025 MT940)
    // ========================================================================

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Extract currency code from Field60F (Opening Balance)
    fn get_field_60f_currency(&self) -> &str {
        &self.field_60f.currency
    }

    /// Extract currency code from Field62F (Closing Balance)
    fn get_field_62f_currency(&self) -> &str {
        &self.field_62f.currency
    }

    /// Extract currency code from Field64 (Closing Available Balance)
    fn get_field_64_currency(&self) -> Option<&str> {
        self.field_64.as_ref().map(|f| f.currency.as_str())
    }

    /// Get first two characters of a currency code
    fn get_currency_prefix(currency: &str) -> &str {
        if currency.len() >= 2 {
            &currency[0..2]
        } else {
            currency
        }
    }

    // ========================================================================
    // VALIDATION RULES (C1-C2)
    // ========================================================================

    /// C1: Field 86 Must Follow Field 61 (Error code: C24)
    /// If field 86 is present in any occurrence of the repetitive sequence, it must be
    /// preceded by a field 61. In addition, if field 86 is present, it must be present
    /// on the same page (message) of the statement as the related field 61
    ///
    /// Note: This validation is structural and enforced during parsing. The parser ensures
    /// that field 86 can only appear after field 61 within a statement line. The message
    /// structure (MT940StatementLine) guarantees this relationship - field 86 can only exist
    /// within a statement line, which always has a mandatory field 61. Therefore, this rule
    /// is automatically satisfied by the structure and no additional validation is needed.
    fn validate_c1_field_86_follows_61(&self) -> Vec<SwiftValidationError> {
        // This rule is enforced by the message structure itself (MT940StatementLine)
        // Field 86 can only exist within a statement line, which always has field 61
        // The parser ensures correct ordering and pairing during message parsing
        // No additional validation needed - return empty errors
        Vec::new()
    }

    /// C2: Currency Code Consistency (Error code: C27)
    /// The first two characters of the three character currency code in fields 60a,
    /// 62a, 64 and 65 must be the same for all occurrences of these fields
    fn validate_c2_currency_consistency(&self) -> Vec<SwiftValidationError> {
        let mut errors = Vec::new();

        // Get the reference currency prefix from field 60F (Opening Balance)
        let reference_currency = self.get_field_60f_currency();
        let reference_prefix = Self::get_currency_prefix(reference_currency);

        // Check field 62F (Closing Balance)
        let field_62f_currency = self.get_field_62f_currency();
        let field_62f_prefix = Self::get_currency_prefix(field_62f_currency);

        if field_62f_prefix != reference_prefix {
            errors.push(SwiftValidationError::content_error(
                "C27",
                "62F",
                field_62f_currency,
                &format!(
                    "Currency prefix in field 62F ('{}') must match the prefix in field 60F ('{}') - found '{}' vs '{}'",
                    field_62f_prefix, reference_prefix, field_62f_currency, reference_currency
                ),
                "The first two characters of the currency code must be the same in fields 60a, 62a, 64 and 65",
            ));
        }

        // Check field 64 (Closing Available Balance) if present
        if let Some(field_64_currency) = self.get_field_64_currency() {
            let field_64_prefix = Self::get_currency_prefix(field_64_currency);

            if field_64_prefix != reference_prefix {
                errors.push(SwiftValidationError::content_error(
                    "C27",
                    "64",
                    field_64_currency,
                    &format!(
                        "Currency prefix in field 64 ('{}') must match the prefix in field 60F ('{}') - found '{}' vs '{}'",
                        field_64_prefix, reference_prefix, field_64_currency, reference_currency
                    ),
                    "The first two characters of the currency code must be the same in fields 60a, 62a, 64 and 65",
                ));
            }
        }

        // Check field 65 (Forward Available Balance) if present
        if let Some(field_65_vec) = &self.field_65 {
            for (idx, field_65) in field_65_vec.iter().enumerate() {
                let field_65_currency = &field_65.currency;
                let field_65_prefix = Self::get_currency_prefix(field_65_currency);

                if field_65_prefix != reference_prefix {
                    errors.push(SwiftValidationError::content_error(
                        "C27",
                        "65",
                        field_65_currency,
                        &format!(
                            "Currency prefix in field 65 occurrence {} ('{}') must match the prefix in field 60F ('{}') - found '{}' vs '{}'",
                            idx + 1, field_65_prefix, reference_prefix, field_65_currency, reference_currency
                        ),
                        "The first two characters of the currency code must be the same in fields 60a, 62a, 64 and 65",
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

        // C1: Field 86 Must Follow Field 61
        let c1_errors = self.validate_c1_field_86_follows_61();
        all_errors.extend(c1_errors);
        if stop_on_first_error && !all_errors.is_empty() {
            return all_errors;
        }

        // C2: Currency Code Consistency
        let c2_errors = self.validate_c2_currency_consistency();
        all_errors.extend(c2_errors);

        all_errors
    }
}

impl crate::traits::SwiftMessageBody for MT940 {
    fn message_type() -> &'static str {
        "940"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT940::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT940::to_mt_string(self)
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // Call the existing public method implementation
        MT940::validate_network_rules(self, stop_on_first_error)
    }
}

impl MT940 {
    /// Convert to SWIFT MT text format
    pub fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_optional_field(&mut result, &self.field_21);
        append_field(&mut result, &self.field_25);
        append_field(&mut result, &self.field_28c);
        append_field(&mut result, &self.field_60f);

        // Statement lines
        for statement_line in &self.statement_lines {
            append_field(&mut result, &statement_line.field_61);
            append_optional_field(&mut result, &statement_line.field_86);
        }

        append_field(&mut result, &self.field_62f);
        append_optional_field(&mut result, &self.field_64);
        append_vec_field(&mut result, &self.field_65);

        finalize_mt_string(result, false)
    }
}
