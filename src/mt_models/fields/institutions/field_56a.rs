//! Field 56a: Intermediary Institution
//!
//! This field specifies the intermediary institution.
//! Options: A (BIC), C (code/country/location), D (name and address)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 56A: Intermediary Institution (BIC option)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field56A {
    /// BIC code (8 or 11 characters)
    pub bic: String,
}

/// Field 56C: Intermediary Institution (code/country/location option)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field56C {
    /// Code (up to 35 characters)
    pub code: String,
}

/// Field 56D: Intermediary Institution (name and address option)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field56D {
    /// Account line indicator (optional, 1 character)
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    pub account_number: Option<String>,
    /// Name and address lines (up to 4 lines, 35 characters each)
    pub name_address: Vec<String>,
}

/// Field 56: Intermediary Institution (with options A, C, D)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Field56 {
    A(Field56A),
    C(Field56C),
    D(Field56D),
}

impl Field56A {
    /// Create a new Field56A with validation
    pub fn new(bic: impl Into<String>) -> Result<Self> {
        let bic = bic.into().to_uppercase();
        validate_bic(&bic)?;
        Ok(Field56A {
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

impl Field56C {
    /// Create a new Field56C with validation
    pub fn new(code: impl Into<String>) -> Result<Self> {
        let code = code.into();

        if code.trim().is_empty() {
            return Err(FieldParseError::missing_data("56C", "Code cannot be empty").into());
        }

        if code.len() > 35 {
            return Err(
                FieldParseError::invalid_format("56C", "Code cannot exceed 35 characters").into(),
            );
        }

        if !code.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(
                FieldParseError::invalid_format("56C", "Code contains invalid characters").into(),
            );
        }

        Ok(Field56C {
            code: code.to_string(),
        })
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}

impl Field56D {
    /// Create a new Field56D with validation
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        name_address: Vec<String>,
    ) -> Result<Self> {
        // Validate name and address lines
        if name_address.is_empty() {
            return Err(
                FieldParseError::missing_data("56D", "Name and address cannot be empty").into(),
            );
        }

        if name_address.len() > 4 {
            return Err(FieldParseError::invalid_format(
                "56D",
                "Name and address cannot exceed 4 lines",
            )
            .into());
        }

        for (i, line) in name_address.iter().enumerate() {
            if line.len() > 35 {
                return Err(FieldParseError::invalid_format(
                    "56D",
                    &format!("Name/address line {} exceeds 35 characters", i + 1),
                )
                .into());
            }

            if line.trim().is_empty() {
                return Err(FieldParseError::invalid_format(
                    "56D",
                    &format!("Name/address line {} cannot be empty", i + 1),
                )
                .into());
            }

            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(FieldParseError::invalid_format(
                    "56D",
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

        Ok(Field56D {
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

impl Field56 {
    pub fn parse(tag: &str, content: &str) -> Result<Self> {
        match tag {
            "56A" => Ok(Field56::A(Field56A::parse(content)?)),
            "56C" => Ok(Field56::C(Field56C::parse(content)?)),
            "56D" => Ok(Field56::D(Field56D::parse(content)?)),
            _ => Err(FieldParseError::InvalidFieldOption {
                field: "56".to_string(),
                option: tag.chars().last().unwrap_or('?').to_string(),
                valid_options: vec!["A".to_string(), "C".to_string(), "D".to_string()],
            }
            .into()),
        }
    }

    pub fn tag(&self) -> &'static str {
        match self {
            Field56::A(_) => "56A",
            Field56::C(_) => "56C",
            Field56::D(_) => "56D",
        }
    }
}

impl SwiftField for Field56A {
    const TAG: &'static str = "56A";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(FieldParseError::missing_data("56A", "BIC code is required").into());
        }
        Field56A::new(content)
    }

    fn to_swift_string(&self) -> String {
        format!(":56A:{}", self.bic)
    }

    fn validate(&self, _rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        Ok(())
    }

    fn description() -> &'static str {
        "Intermediary Institution - BIC"
    }
}

impl SwiftField for Field56C {
    const TAG: &'static str = "56C";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(FieldParseError::missing_data("56C", "Code is required").into());
        }
        Field56C::new(content)
    }

    fn to_swift_string(&self) -> String {
        format!(":56C:{}", self.code)
    }

    fn validate(&self, _rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        Ok(())
    }

    fn description() -> &'static str {
        "Intermediary Institution - Code/Country/Location"
    }
}

impl SwiftField for Field56D {
    const TAG: &'static str = "56D";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(
                FieldParseError::missing_data("56D", "Field content cannot be empty").into(),
            );
        }

        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Err(
                FieldParseError::missing_data("56D", "Name and address are required").into(),
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
                        "56D",
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

        Field56D::new(account_line_indicator, account_number, name_address_lines)
    }

    fn to_swift_string(&self) -> String {
        let mut result = ":56D:".to_string();

        if let Some(ref indicator) = self.account_line_indicator {
            result.push('/');
            result.push_str(indicator);
        }

        if let Some(ref account) = self.account_number {
            result.push('/');
            result.push_str(account);
        }

        if self.account_line_indicator.is_some() || self.account_number.is_some() {
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
        "Intermediary Institution - Name and Address"
    }
}

impl SwiftField for Field56 {
    const TAG: &'static str = "56";

    fn parse(_content: &str) -> Result<Self> {
        Err(
            FieldParseError::InvalidUsage("Use Field56::parse(tag, content) instead".to_string())
                .into(),
        )
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field56::A(field) => field.to_swift_string(),
            Field56::C(field) => field.to_swift_string(),
            Field56::D(field) => field.to_swift_string(),
        }
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        match self {
            Field56::A(field) => field.validate(rules),
            Field56::C(field) => field.validate(rules),
            Field56::D(field) => field.validate(rules),
        }
    }

    fn options() -> Vec<&'static str> {
        vec!["A", "C", "D"]
    }

    fn description() -> &'static str {
        "Intermediary Institution"
    }
}

// Helper functions
fn validate_bic(bic: &str) -> Result<()> {
    if bic.is_empty() {
        return Err(FieldParseError::missing_data("BIC", "BIC cannot be empty").into());
    }

    if bic.len() != 8 && bic.len() != 11 {
        return Err(
            FieldParseError::invalid_format("BIC", "BIC must be 8 or 11 characters long").into(),
        );
    }

    // Basic validation: first 4 chars should be letters (bank code)
    let bank_code = &bic[0..4];
    if !bank_code.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(FieldParseError::invalid_format(
            "BIC",
            "First 4 characters must be letters (bank code)",
        )
        .into());
    }

    // Next 2 chars should be letters (country code)
    let country_code = &bic[4..6];
    if !country_code.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(FieldParseError::invalid_format(
            "BIC",
            "Characters 5-6 must be letters (country code)",
        )
        .into());
    }

    // Next 2 chars should be alphanumeric (location code)
    let location_code = &bic[6..8];
    if !location_code.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(FieldParseError::invalid_format(
            "BIC",
            "Characters 7-8 must be alphanumeric (location code)",
        )
        .into());
    }

    // If 11 chars, last 3 should be alphanumeric (branch code)
    if bic.len() == 11 {
        let branch_code = &bic[8..11];
        if !branch_code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(FieldParseError::invalid_format(
                "BIC",
                "Characters 9-11 must be alphanumeric (branch code)",
            )
            .into());
        }
    }

    Ok(())
}

fn validate_account_line_indicator(indicator: &str) -> Result<()> {
    if indicator.is_empty() {
        return Err(FieldParseError::invalid_format(
            "Account Line Indicator",
            "Cannot be empty if specified",
        )
        .into());
    }

    if indicator.len() != 1 {
        return Err(FieldParseError::invalid_format(
            "Account Line Indicator",
            "Must be exactly 1 character",
        )
        .into());
    }

    if !indicator.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(FieldParseError::invalid_format(
            "Account Line Indicator",
            "Must be alphanumeric",
        )
        .into());
    }

    Ok(())
}

fn validate_account_number(account: &str) -> Result<()> {
    if account.is_empty() {
        return Err(FieldParseError::invalid_format(
            "Account Number",
            "Cannot be empty if specified",
        )
        .into());
    }

    if account.len() > 34 {
        return Err(FieldParseError::invalid_format(
            "Account Number",
            "Cannot exceed 34 characters",
        )
        .into());
    }

    if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
        return Err(FieldParseError::invalid_format(
            "Account Number",
            "Contains invalid characters",
        )
        .into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field56a_basic() {
        let field = Field56A::new("CHASUS33").unwrap();
        assert_eq!(field.bic(), "CHASUS33");
        assert!(!field.is_full_bic());
    }

    #[test]
    fn test_field56c_basic() {
        let field = Field56C::new("US/NEW YORK/BIC CODE").unwrap();
        assert_eq!(field.code(), "US/NEW YORK/BIC CODE");
    }

    #[test]
    fn test_field56d_basic() {
        let name_address = vec!["BANK OF AMERICA".to_string(), "NEW YORK NY".to_string()];
        let field = Field56D::new(None, None, name_address).unwrap();
        assert_eq!(field.name_address().len(), 2);
    }

    #[test]
    fn test_field56_parse() {
        let field = Field56::parse("56A", "CHASUS33").unwrap();
        assert!(matches!(field, Field56::A(_)));

        let field = Field56::parse("56C", "US/NEW YORK").unwrap();
        assert!(matches!(field, Field56::C(_)));

        let field = Field56::parse("56D", "BANK NAME\nADDRESS").unwrap();
        assert!(matches!(field, Field56::D(_)));
    }

    #[test]
    fn test_field56_invalid_option() {
        let result = Field56::parse("56B", "INVALID");
        assert!(result.is_err());
    }
}
