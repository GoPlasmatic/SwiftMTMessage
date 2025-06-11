//! Field 52a: Ordering Institution
//!
//! This field identifies the ordering institution.
//! Options: A (BIC), D (name and address)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use crate::utils::{account, bic, multiline};
use serde::{Deserialize, Serialize};

/// Field 52A: Ordering Institution (BIC option)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field52A {
    /// Account line indicator (optional, 1 character)  
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    pub account_number: Option<String>,
    /// BIC code (8 or 11 characters)
    pub bic: String,
}

/// Field 52D: Ordering Institution (name and address option)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field52D {
    /// Account line indicator (optional, 1 character)
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    pub account_number: Option<String>,
    /// Name and address lines (up to 4 lines, 35 characters each)
    pub name_address: Vec<String>,
}

/// Field 52: Ordering Institution (with options A, D)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Field52 {
    A(Field52A),
    D(Field52D),
}

impl Field52A {
    /// Create a new Field52A with validation
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        bic: impl Into<String>,
    ) -> Result<Self> {
        let bic = bic.into().to_uppercase();

        // Validate BIC
        bic::validate_bic(&bic)?;

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            account::validate_account_line_indicator(indicator)?;
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            account::validate_account_number(account)?;
        }

        Ok(Field52A {
            account_line_indicator,
            account_number,
            bic: bic.to_string(),
        })
    }

    pub fn account_line_indicator(&self) -> Option<&str> {
        self.account_line_indicator.as_deref()
    }

    pub fn account_number(&self) -> Option<&str> {
        self.account_number.as_deref()
    }

    pub fn bic(&self) -> &str {
        &self.bic
    }

    pub fn is_full_bic(&self) -> bool {
        self.bic.len() == 11
    }
}

impl Field52D {
    /// Create a new Field52D with validation
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        name_address: Vec<String>,
    ) -> Result<Self> {
        // Validate name and address lines using shared utility
        multiline::validate_multiline_field("52D", &name_address, 4, 35)?;

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            account::validate_account_line_indicator(indicator)?;
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            account::validate_account_number(account)?;
        }

        Ok(Field52D {
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

impl Field52 {
    pub fn parse(tag: &str, content: &str) -> Result<Self> {
        match tag {
            "52A" => Ok(Field52::A(Field52A::parse(content)?)),
            "52D" => Ok(Field52::D(Field52D::parse(content)?)),
            _ => Err(FieldParseError::InvalidFieldOption {
                field: "52".to_string(),
                option: tag.chars().last().unwrap_or('?').to_string(),
                valid_options: vec!["A".to_string(), "D".to_string()],
            }
            .into()),
        }
    }

    pub fn tag(&self) -> &'static str {
        match self {
            Field52::A(_) => "52A",
            Field52::D(_) => "52D",
        }
    }
}

impl SwiftField for Field52A {
    const TAG: &'static str = "52A";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(
                FieldParseError::missing_data("52A", "Field content cannot be empty").into(),
            );
        }

        let (account_line_indicator, account_number, bic_content) =
            account::parse_account_line_and_content(content)?;

        // BIC should be on its own line or the last part
        let bic = bic_content.trim().to_string();
        if bic.is_empty() {
            return Err(FieldParseError::missing_data("52A", "BIC code is required").into());
        }

        Field52A::new(account_line_indicator, account_number, bic)
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
        result.push_str(&self.bic);

        result
    }

    fn validate(&self, _rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        // BIC validation is done in constructor
        Ok(())
    }

    fn description() -> &'static str {
        "Ordering Institution - BIC"
    }
}

impl SwiftField for Field52D {
    const TAG: &'static str = "52D";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(
                FieldParseError::missing_data("52D", "Field content cannot be empty").into(),
            );
        }

        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Err(
                FieldParseError::missing_data("52D", "Name and address are required").into(),
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
                    // Only account number: /account
                    account_number = Some(account_line[1..].to_string());
                    start_idx = 1;
                }
                2 => {
                    // Indicator and account: /indicator/account
                    let indicator_end = slash_positions[1];
                    account_line_indicator = Some(account_line[1..indicator_end].to_string());
                    account_number = Some(account_line[indicator_end + 1..].to_string());
                    start_idx = 1;
                }
                _ => {
                    return Err(FieldParseError::invalid_format(
                        "52D",
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

        Field52D::new(account_line_indicator, account_number, name_address_lines)
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
        // Validation is done in constructor
        Ok(())
    }

    fn description() -> &'static str {
        "Ordering Institution - Name and Address"
    }
}

impl SwiftField for Field52 {
    const TAG: &'static str = "52";

    fn parse(_content: &str) -> Result<Self> {
        // This shouldn't be called directly; use Field52::parse(tag, content) instead
        Err(
            FieldParseError::InvalidUsage("Use Field52::parse(tag, content) instead".to_string())
                .into(),
        )
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field52::A(field) => field.to_swift_string(),
            Field52::D(field) => field.to_swift_string(),
        }
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        match self {
            Field52::A(field) => field.validate(rules),
            Field52::D(field) => field.validate(rules),
        }
    }

    fn options() -> Vec<&'static str> {
        vec!["A", "D"]
    }

    fn description() -> &'static str {
        "Ordering Institution"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field52a_bic_only() {
        let field = Field52A::new(None, None, "CHASUS33").unwrap();
        assert_eq!(field.bic(), "CHASUS33");
        assert!(field.account_line_indicator().is_none());
        assert!(field.account_number().is_none());
        assert!(!field.is_full_bic());
    }

    #[test]
    fn test_field52a_with_account() {
        let field = Field52A::new(None, Some("12345678".to_string()), "CHASUS33XXX").unwrap();
        assert_eq!(field.bic(), "CHASUS33XXX");
        assert_eq!(field.account_number(), Some("12345678"));
        assert!(field.is_full_bic());
    }

    #[test]
    fn test_field52a_parse() {
        let content = "/A/12345678\nCHASUS33";
        let field = Field52A::parse(content).unwrap();
        assert_eq!(field.account_line_indicator(), Some("A"));
        assert_eq!(field.account_number(), Some("12345678"));
        assert_eq!(field.bic(), "CHASUS33");
    }

    #[test]
    fn test_field52d_basic() {
        let name_address = vec!["JPMORGAN CHASE BANK".to_string(), "NEW YORK NY".to_string()];
        let field = Field52D::new(None, None, name_address.clone()).unwrap();
        assert_eq!(field.name_address(), &name_address);
    }

    #[test]
    fn test_field52d_with_account() {
        let name_address = vec!["BANK NAME".to_string(), "ADDRESS".to_string()];
        let field = Field52D::new(
            Some("A".to_string()),
            Some("123456".to_string()),
            name_address,
        )
        .unwrap();
        assert_eq!(field.account_line_indicator(), Some("A"));
        assert_eq!(field.account_number(), Some("123456"));
    }

    #[test]
    fn test_field52d_parse() {
        let content = "/A/12345\nBANK NAME\nADDRESS LINE";
        let field = Field52D::parse(content).unwrap();
        assert_eq!(field.account_line_indicator(), Some("A"));
        assert_eq!(field.account_number(), Some("12345"));
        assert_eq!(field.name_address().len(), 2);
    }

    #[test]
    fn test_field52_parse() {
        let field_a = Field52::parse("52A", "CHASUS33").unwrap();
        assert!(matches!(field_a, Field52::A(_)));

        let field_d = Field52::parse("52D", "BANK NAME\nADDRESS").unwrap();
        assert!(matches!(field_d, Field52::D(_)));
    }

    #[test]
    fn test_field52_invalid_option() {
        let result = Field52::parse("52X", "content");
        assert!(result.is_err());
    }

    #[test]
    fn test_bic_validation() {
        use crate::utils::bic;
        assert!(bic::validate_bic("CHASUS33").is_ok());
        assert!(bic::validate_bic("CHASUS33XXX").is_ok());
        assert!(bic::validate_bic("INVALID").is_err()); // too short
        assert!(bic::validate_bic("TOOSHORT").is_err()); // invalid length
        assert!(bic::validate_bic("123US33").is_err()); // numbers in bank code
    }
}
