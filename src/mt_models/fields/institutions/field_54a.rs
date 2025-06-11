//! Field 54a: Receiver's Correspondent
//!
//! This field specifies the receiver's correspondent institution.
//! Options: A (BIC), B (account + BIC), D (name and address)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 54A: Receiver's Correspondent (BIC option)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field54A {
    /// BIC code (8 or 11 characters)
    pub bic: String,
}

/// Field 54B: Receiver's Correspondent (account + BIC option)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field54B {
    /// Account number (up to 35 characters)
    pub account: String,
    /// BIC code (8 or 11 characters)
    pub bic: String,
}

/// Field 54D: Receiver's Correspondent (name and address option)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field54D {
    /// Account line indicator (optional, 1 character)
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    pub account_number: Option<String>,
    /// Name and address lines (up to 4 lines, 35 characters each)
    pub name_address: Vec<String>,
}

/// Field 54: Receiver's Correspondent (with options A, B, D)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Field54 {
    A(Field54A),
    B(Field54B),
    D(Field54D),
}

impl Field54A {
    /// Create a new Field54A with validation
    pub fn new(bic: impl Into<String>) -> Result<Self> {
        let bic = bic.into().to_uppercase();
        validate_bic(&bic)?;
        Ok(Field54A {
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

impl Field54B {
    /// Create a new Field54B with validation
    pub fn new(account: impl Into<String>, bic: impl Into<String>) -> Result<Self> {
        let account = account.into();
        let bic = bic.into().to_uppercase();

        validate_bic(&bic)?;
        validate_account_54b(&account)?;

        Ok(Field54B {
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

impl Field54D {
    /// Create a new Field54D with validation
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        name_address: Vec<String>,
    ) -> Result<Self> {
        // Validate name and address lines
        if name_address.is_empty() {
            return Err(
                FieldParseError::missing_data("54D", "Name and address cannot be empty").into(),
            );
        }

        if name_address.len() > 4 {
            return Err(FieldParseError::invalid_format(
                "54D",
                "Name and address cannot exceed 4 lines",
            )
            .into());
        }

        for (i, line) in name_address.iter().enumerate() {
            if line.len() > 35 {
                return Err(FieldParseError::invalid_format(
                    "54D",
                    &format!("Name/address line {} exceeds 35 characters", i + 1),
                )
                .into());
            }

            if line.trim().is_empty() {
                return Err(FieldParseError::invalid_format(
                    "54D",
                    &format!("Name/address line {} cannot be empty", i + 1),
                )
                .into());
            }

            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(FieldParseError::invalid_format(
                    "54D",
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

        Ok(Field54D {
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

impl Field54 {
    pub fn parse(tag: &str, content: &str) -> Result<Self> {
        match tag {
            "54A" => Ok(Field54::A(Field54A::parse(content)?)),
            "54B" => Ok(Field54::B(Field54B::parse(content)?)),
            "54D" => Ok(Field54::D(Field54D::parse(content)?)),
            _ => Err(FieldParseError::InvalidFieldOption {
                field: "54".to_string(),
                option: tag.chars().last().unwrap_or('?').to_string(),
                valid_options: vec!["A".to_string(), "B".to_string(), "D".to_string()],
            }
            .into()),
        }
    }

    pub fn tag(&self) -> &'static str {
        match self {
            Field54::A(_) => "54A",
            Field54::B(_) => "54B",
            Field54::D(_) => "54D",
        }
    }
}

impl SwiftField for Field54A {
    const TAG: &'static str = "54A";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(FieldParseError::missing_data("54A", "BIC code is required").into());
        }
        Field54A::new(content)
    }

    fn to_swift_string(&self) -> String {
        self.bic.clone()
    }

    fn validate(&self, _rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        Ok(())
    }

    fn description() -> &'static str {
        "Receiver's Correspondent - BIC"
    }
}

impl SwiftField for Field54B {
    const TAG: &'static str = "54B";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(
                FieldParseError::missing_data("54B", "Account and BIC are required").into(),
            );
        }

        let lines: Vec<&str> = content.lines().collect();
        if lines.len() != 2 {
            return Err(FieldParseError::invalid_format(
                "54B",
                "Field must contain exactly 2 lines: account and BIC",
            )
            .into());
        }

        let account = lines[0].trim();
        let bic = lines[1].trim();

        if account.is_empty() {
            return Err(FieldParseError::missing_data("54B", "Account number is required").into());
        }

        if bic.is_empty() {
            return Err(FieldParseError::missing_data("54B", "BIC code is required").into());
        }

        Field54B::new(account, bic)
    }

    fn to_swift_string(&self) -> String {
        format!("{}\n{}", self.account, self.bic)
    }

    fn validate(&self, _rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        Ok(())
    }

    fn description() -> &'static str {
        "Receiver's Correspondent - Account and BIC"
    }
}

impl SwiftField for Field54D {
    const TAG: &'static str = "54D";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(
                FieldParseError::missing_data("54D", "Field content cannot be empty").into(),
            );
        }

        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Err(
                FieldParseError::missing_data("54D", "Name and address are required").into(),
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
                        "54D",
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

        Field54D::new(account_line_indicator, account_number, name_address_lines)
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
        "Receiver's Correspondent - Name and Address"
    }
}

impl SwiftField for Field54 {
    const TAG: &'static str = "54";

    fn parse(_content: &str) -> Result<Self> {
        Err(
            FieldParseError::InvalidUsage("Use Field54::parse(tag, content) instead".to_string())
                .into(),
        )
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field54::A(field) => field.to_swift_string(),
            Field54::B(field) => field.to_swift_string(),
            Field54::D(field) => field.to_swift_string(),
        }
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        match self {
            Field54::A(field) => field.validate(rules),
            Field54::B(field) => field.validate(rules),
            Field54::D(field) => field.validate(rules),
        }
    }

    fn options() -> Vec<&'static str> {
        vec!["A", "B", "D"]
    }

    fn description() -> &'static str {
        "Receiver's Correspondent"
    }
}

// Helper functions
fn validate_bic(bic: &str) -> Result<()> {
    if bic.len() != 8 && bic.len() != 11 {
        return Err(FieldParseError::invalid_format("54", "BIC must be 8 or 11 characters").into());
    }

    if !bic.chars().all(|c| c.is_alphanumeric() && c.is_ascii()) {
        return Err(FieldParseError::invalid_format(
            "54",
            "BIC must contain only alphanumeric characters",
        )
        .into());
    }

    let bank_code = &bic[0..4];
    let country_code = &bic[4..6];
    let location_code = &bic[6..8];

    if !bank_code.chars().all(|c| c.is_alphabetic()) {
        return Err(FieldParseError::invalid_format(
            "54",
            "BIC bank code (first 4 characters) must be alphabetic",
        )
        .into());
    }

    if !country_code.chars().all(|c| c.is_alphabetic()) {
        return Err(FieldParseError::invalid_format(
            "54",
            "BIC country code (characters 5-6) must be alphabetic",
        )
        .into());
    }

    if !location_code.chars().all(|c| c.is_alphanumeric()) {
        return Err(FieldParseError::invalid_format(
            "54",
            "BIC location code (characters 7-8) must be alphanumeric",
        )
        .into());
    }

    if bic.len() == 11 {
        let branch_code = &bic[8..11];
        if !branch_code.chars().all(|c| c.is_alphanumeric()) {
            return Err(FieldParseError::invalid_format(
                "54",
                "BIC branch code (characters 9-11) must be alphanumeric",
            )
            .into());
        }
    }

    Ok(())
}

fn validate_account_54b(account: &str) -> Result<()> {
    if account.is_empty() {
        return Err(
            FieldParseError::invalid_format("54B", "Account number cannot be empty").into(),
        );
    }

    if account.len() > 35 {
        return Err(FieldParseError::invalid_format(
            "54B",
            "Account number too long (max 35 characters)",
        )
        .into());
    }

    if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
        return Err(FieldParseError::invalid_format(
            "54B",
            "Account number contains invalid characters",
        )
        .into());
    }

    Ok(())
}

fn validate_account_line_indicator(indicator: &str) -> Result<()> {
    if indicator.len() != 1 {
        return Err(FieldParseError::invalid_format(
            "54D",
            "Account line indicator must be exactly 1 character",
        )
        .into());
    }

    if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
        return Err(FieldParseError::invalid_format(
            "54D",
            "Account line indicator contains invalid characters",
        )
        .into());
    }

    Ok(())
}

fn validate_account_number(account: &str) -> Result<()> {
    if account.is_empty() {
        return Err(FieldParseError::invalid_format(
            "54D",
            "Account number cannot be empty if specified",
        )
        .into());
    }

    if account.len() > 34 {
        return Err(FieldParseError::invalid_format(
            "54D",
            "Account number too long (max 34 characters)",
        )
        .into());
    }

    if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
        return Err(FieldParseError::invalid_format(
            "54D",
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
    fn test_field54a_basic() {
        let field = Field54A::new("CHASUS33").unwrap();
        assert_eq!(field.bic(), "CHASUS33");
        assert!(!field.is_full_bic());
    }

    #[test]
    fn test_field54b_basic() {
        let field = Field54B::new("123456789", "CHASUS33").unwrap();
        assert_eq!(field.account(), "123456789");
        assert_eq!(field.bic(), "CHASUS33");
    }

    #[test]
    fn test_field54d_basic() {
        let name_address = vec!["BANK NAME".to_string(), "ADDRESS".to_string()];
        let field = Field54D::new(None, None, name_address.clone()).unwrap();
        assert_eq!(field.name_address(), &name_address);
    }

    #[test]
    fn test_field54_parse() {
        let field_a = Field54::parse("54A", "CHASUS33").unwrap();
        assert!(matches!(field_a, Field54::A(_)));

        let field_b = Field54::parse("54B", "123456\nCHASUS33").unwrap();
        assert!(matches!(field_b, Field54::B(_)));

        let field_d = Field54::parse("54D", "BANK NAME\nADDRESS").unwrap();
        assert!(matches!(field_d, Field54::D(_)));
    }

    #[test]
    fn test_field54_invalid_option() {
        let result = Field54::parse("54X", "content");
        assert!(result.is_err());
    }
}
