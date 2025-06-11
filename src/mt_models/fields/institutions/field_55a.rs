//! Field 55a: Third Reimbursement Institution
//!
//! This field specifies the third reimbursement institution.
//! Options: A (BIC), B (account + BIC), D (name and address)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 55A: Third Reimbursement Institution (BIC option)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field55A {
    /// BIC code (8 or 11 characters)
    pub bic: String,
}

/// Field 55B: Third Reimbursement Institution (account + BIC option)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field55B {
    /// Account number (up to 35 characters)
    pub account: String,
    /// BIC code (8 or 11 characters)
    pub bic: String,
}

/// Field 55D: Third Reimbursement Institution (name and address option)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field55D {
    /// Account line indicator (optional, 1 character)
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    pub account_number: Option<String>,
    /// Name and address lines (up to 4 lines, 35 characters each)
    pub name_address: Vec<String>,
}

/// Field 55: Third Reimbursement Institution (with options A, B, D)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Field55 {
    A(Field55A),
    B(Field55B),
    D(Field55D),
}

impl Field55A {
    /// Create a new Field55A with validation
    pub fn new(bic: impl Into<String>) -> Result<Self> {
        let bic = bic.into().to_uppercase();
        validate_bic(&bic)?;
        Ok(Field55A {
            bic: bic.to_string(),
        })
    }

    pub fn bic(&self) -> &str {
        &self.bic
    }

    pub fn is_full_bic(&self) -> bool {
        self.bic.len() == 11
    }
}

impl Field55B {
    /// Create a new Field55B with validation
    pub fn new(account: impl Into<String>, bic: impl Into<String>) -> Result<Self> {
        let account = account.into();
        let bic = bic.into().to_uppercase();

        validate_bic(&bic)?;
        validate_account_55b(&account)?;

        Ok(Field55B {
            account: account.to_string(),
            bic: bic.to_string(),
        })
    }

    pub fn account(&self) -> &str {
        &self.account
    }

    pub fn bic(&self) -> &str {
        &self.bic
    }

    pub fn is_full_bic(&self) -> bool {
        self.bic.len() == 11
    }
}

impl Field55D {
    /// Create a new Field55D with validation
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        name_address: Vec<String>,
    ) -> Result<Self> {
        // Validate name and address lines
        if name_address.is_empty() {
            return Err(
                FieldParseError::missing_data("55D", "Name and address cannot be empty").into(),
            );
        }

        if name_address.len() > 4 {
            return Err(FieldParseError::invalid_format(
                "55D",
                "Name and address cannot exceed 4 lines",
            )
            .into());
        }

        for (i, line) in name_address.iter().enumerate() {
            if line.len() > 35 {
                return Err(FieldParseError::invalid_format(
                    "55D",
                    &format!("Name/address line {} exceeds 35 characters", i + 1),
                )
                .into());
            }

            if line.trim().is_empty() {
                return Err(FieldParseError::invalid_format(
                    "55D",
                    &format!("Name/address line {} cannot be empty", i + 1),
                )
                .into());
            }

            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(FieldParseError::invalid_format(
                    "55D",
                    &format!("Name/address line {} contains invalid characters", i + 1),
                )
                .into());
            }
        }

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            validate_account_line_indicator(indicator)?;
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            validate_account_number(account)?;
        }

        Ok(Field55D {
            account_line_indicator,
            account_number,
            name_address,
        })
    }

    pub fn account_line_indicator(&self) -> Option<&str> {
        self.account_line_indicator.as_deref()
    }

    pub fn account_number(&self) -> Option<&str> {
        self.account_number.as_deref()
    }

    pub fn name_address(&self) -> &[String] {
        &self.name_address
    }
}

impl Field55 {
    pub fn parse(tag: &str, content: &str) -> Result<Self> {
        match tag {
            "55A" => Ok(Field55::A(Field55A::parse(content)?)),
            "55B" => Ok(Field55::B(Field55B::parse(content)?)),
            "55D" => Ok(Field55::D(Field55D::parse(content)?)),
            _ => Err(FieldParseError::InvalidFieldOption {
                field: "55".to_string(),
                option: tag.chars().last().unwrap_or('?').to_string(),
                valid_options: vec!["A".to_string(), "B".to_string(), "D".to_string()],
            }
            .into()),
        }
    }

    pub fn tag(&self) -> &'static str {
        match self {
            Field55::A(_) => "55A",
            Field55::B(_) => "55B",
            Field55::D(_) => "55D",
        }
    }
}

impl SwiftField for Field55A {
    const TAG: &'static str = "55A";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(FieldParseError::missing_data("55A", "BIC code is required").into());
        }
        Field55A::new(content)
    }

    fn to_swift_string(&self) -> String {
        self.bic.clone()
    }

    fn validate(&self, _rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        Ok(())
    }

    fn description() -> &'static str {
        "Third Reimbursement Institution - BIC"
    }
}

impl SwiftField for Field55B {
    const TAG: &'static str = "55B";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(
                FieldParseError::missing_data("55B", "Account and BIC are required").into(),
            );
        }

        let lines: Vec<&str> = content.lines().collect();
        if lines.len() != 2 {
            return Err(FieldParseError::invalid_format(
                "55B",
                "Field must contain exactly 2 lines: account and BIC",
            )
            .into());
        }

        let account = lines[0].trim();
        let bic = lines[1].trim();

        if account.is_empty() {
            return Err(FieldParseError::missing_data("55B", "Account number is required").into());
        }

        if bic.is_empty() {
            return Err(FieldParseError::missing_data("55B", "BIC code is required").into());
        }

        Field55B::new(account, bic)
    }

    fn to_swift_string(&self) -> String {
        format!("{}\n{}", self.account, self.bic)
    }

    fn validate(&self, _rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        Ok(())
    }

    fn description() -> &'static str {
        "Third Reimbursement Institution - Account and BIC"
    }
}

impl SwiftField for Field55D {
    const TAG: &'static str = "55D";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(
                FieldParseError::missing_data("55D", "Field content cannot be empty").into(),
            );
        }

        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Err(
                FieldParseError::missing_data("55D", "Name and address are required").into(),
            );
        }

        let mut account_line_indicator = None;
        let mut account_number = None;
        let mut name_address_lines = Vec::new();
        let mut start_idx = 0;

        // Check for optional account information
        if lines[0].starts_with('/') {
            let account_line = lines[0];
            let slash_positions: Vec<usize> =
                account_line.match_indices('/').map(|(i, _)| i).collect();

            match slash_positions.len() {
                1 => {
                    account_number = Some(account_line[1..].to_string());
                    start_idx = 1;
                }
                2 => {
                    let indicator_end = slash_positions[1];
                    account_line_indicator = Some(account_line[1..indicator_end].to_string());
                    account_number = Some(account_line[indicator_end + 1..].to_string());
                    start_idx = 1;
                }
                _ => {
                    return Err(FieldParseError::invalid_format(
                        "55D",
                        "Invalid account line format",
                    )
                    .into());
                }
            }
        }

        // Collect name and address lines
        for line in &lines[start_idx..] {
            if !line.trim().is_empty() {
                name_address_lines.push(line.to_string());
            }
        }

        Field55D::new(account_line_indicator, account_number, name_address_lines)
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::new();

        if let Some(ref indicator) = self.account_line_indicator {
            result.push('/');
            result.push_str(indicator);
        }

        if let Some(ref account) = self.account_number {
            result.push('/');
            result.push_str(account);
        }

        if !result.is_empty() {
            result.push('\n');
        }

        for (i, line) in self.name_address.iter().enumerate() {
            if i > 0 {
                result.push('\n');
            }
            result.push_str(line);
        }

        result
    }

    fn validate(&self, _rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        Ok(())
    }

    fn description() -> &'static str {
        "Third Reimbursement Institution - Name and Address"
    }
}

impl SwiftField for Field55 {
    const TAG: &'static str = "55";

    fn parse(_content: &str) -> Result<Self> {
        Err(
            FieldParseError::InvalidUsage("Use Field55::parse(tag, content) instead".to_string())
                .into(),
        )
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field55::A(field) => field.to_swift_string(),
            Field55::B(field) => field.to_swift_string(),
            Field55::D(field) => field.to_swift_string(),
        }
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        match self {
            Field55::A(field) => field.validate(rules),
            Field55::B(field) => field.validate(rules),
            Field55::D(field) => field.validate(rules),
        }
    }

    fn options() -> Vec<&'static str> {
        vec!["A", "B", "D"]
    }

    fn description() -> &'static str {
        "Third Reimbursement Institution"
    }
}

// Helper functions
fn validate_bic(bic: &str) -> Result<()> {
    if bic.len() != 8 && bic.len() != 11 {
        return Err(FieldParseError::invalid_format("55", "BIC must be 8 or 11 characters").into());
    }

    if !bic.chars().all(|c| c.is_alphanumeric() && c.is_ascii()) {
        return Err(FieldParseError::invalid_format(
            "55",
            "BIC must contain only alphanumeric characters",
        )
        .into());
    }

    let bank_code = &bic[0..4];
    let country_code = &bic[4..6];
    let location_code = &bic[6..8];

    if !bank_code.chars().all(|c| c.is_alphabetic()) {
        return Err(FieldParseError::invalid_format(
            "55",
            "BIC bank code (first 4 characters) must be alphabetic",
        )
        .into());
    }

    if !country_code.chars().all(|c| c.is_alphabetic()) {
        return Err(FieldParseError::invalid_format(
            "55",
            "BIC country code (characters 5-6) must be alphabetic",
        )
        .into());
    }

    if !location_code.chars().all(|c| c.is_alphanumeric()) {
        return Err(FieldParseError::invalid_format(
            "55",
            "BIC location code (characters 7-8) must be alphanumeric",
        )
        .into());
    }

    if bic.len() == 11 {
        let branch_code = &bic[8..11];
        if !branch_code.chars().all(|c| c.is_alphanumeric()) {
            return Err(FieldParseError::invalid_format(
                "55",
                "BIC branch code (characters 9-11) must be alphanumeric",
            )
            .into());
        }
    }

    Ok(())
}

fn validate_account_55b(account: &str) -> Result<()> {
    if account.is_empty() {
        return Err(
            FieldParseError::invalid_format("55B", "Account number cannot be empty").into(),
        );
    }

    if account.len() > 35 {
        return Err(FieldParseError::invalid_format(
            "55B",
            "Account number too long (max 35 characters)",
        )
        .into());
    }

    if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
        return Err(FieldParseError::invalid_format(
            "55B",
            "Account number contains invalid characters",
        )
        .into());
    }

    Ok(())
}

fn validate_account_line_indicator(indicator: &str) -> Result<()> {
    if indicator.len() != 1 {
        return Err(FieldParseError::invalid_format(
            "55D",
            "Account line indicator must be exactly 1 character",
        )
        .into());
    }

    if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
        return Err(FieldParseError::invalid_format(
            "55D",
            "Account line indicator contains invalid characters",
        )
        .into());
    }

    Ok(())
}

fn validate_account_number(account: &str) -> Result<()> {
    if account.is_empty() {
        return Err(FieldParseError::invalid_format(
            "55D",
            "Account number cannot be empty if specified",
        )
        .into());
    }

    if account.len() > 34 {
        return Err(FieldParseError::invalid_format(
            "55D",
            "Account number too long (max 34 characters)",
        )
        .into());
    }

    if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
        return Err(FieldParseError::invalid_format(
            "55D",
            "Account number contains invalid characters",
        )
        .into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field55a_basic() {
        let field = Field55A::new("CHASUS33").unwrap();
        assert_eq!(field.bic(), "CHASUS33");
        assert!(!field.is_full_bic());
    }

    #[test]
    fn test_field55b_basic() {
        let field = Field55B::new("123456789", "CHASUS33").unwrap();
        assert_eq!(field.account(), "123456789");
        assert_eq!(field.bic(), "CHASUS33");
    }

    #[test]
    fn test_field55d_basic() {
        let name_address = vec!["BANK NAME".to_string(), "ADDRESS".to_string()];
        let field = Field55D::new(None, None, name_address.clone()).unwrap();
        assert_eq!(field.name_address(), &name_address);
    }

    #[test]
    fn test_field55_parse() {
        let field_a = Field55::parse("55A", "CHASUS33").unwrap();
        assert!(matches!(field_a, Field55::A(_)));

        let field_b = Field55::parse("55B", "123456\nCHASUS33").unwrap();
        assert!(matches!(field_b, Field55::B(_)));

        let field_d = Field55::parse("55D", "BANK NAME\nADDRESS").unwrap();
        assert!(matches!(field_d, Field55::D(_)));
    }

    #[test]
    fn test_field55_invalid_option() {
        let result = Field55::parse("55X", "content");
        assert!(result.is_err());
    }
}
