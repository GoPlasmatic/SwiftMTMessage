//! Field 77B: Regulatory Reporting
//!
//! Free text field for regulatory information.
//! Format: 3*35x (up to 3 lines of 35 characters each)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 77B: Regulatory Reporting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field77B {
    /// Regulatory reporting information lines (up to 3 lines of 35 characters each)
    pub information: Vec<String>,
    pub ordering_country: Option<String>,
    pub beneficiary_country: Option<String>,
}

impl Field77B {
    /// Create a new Field77B with validation
    pub fn new(information: Vec<String>) -> Result<Self> {
        if information.is_empty() {
            return Err(FieldParseError::missing_data(
                "77B",
                "Regulatory reporting information cannot be empty",
            )
            .into());
        }

        if information.len() > 3 {
            return Err(FieldParseError::invalid_format("77B", "Too many lines (max 3)").into());
        }

        for (i, line) in information.iter().enumerate() {
            if line.len() > 35 {
                return Err(FieldParseError::invalid_format(
                    "77B",
                    &format!("Line {} too long (max 35 characters)", i + 1),
                )
                .into());
            }

            // Validate characters (printable ASCII)
            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(FieldParseError::invalid_format(
                    "77B",
                    &format!("Line {} contains invalid characters", i + 1),
                )
                .into());
            }
        }

        // /ORDERRES/DE//REGULATORY INFO
        // SOFTWARE LICENSE COMPLIANCE
        // TRADE RELATED TRANSACTION
        let mut ordering_country = None;
        let mut beneficiary_country = None;
        if let Some(first_line) = information.get(0) {
            if first_line.starts_with("/ORDERRES/") {
                ordering_country = Some(first_line.split("/").nth(2).unwrap_or("").to_string());
            }
            if first_line.starts_with("/BENEFRES/") {
                beneficiary_country = Some(first_line.split("/").nth(2).unwrap_or("").to_string());
            }
        }

        Ok(Field77B { information, ordering_country, beneficiary_country })
    }

    /// Create from a single string, splitting on newlines
    pub fn from_string(content: &str) -> Result<Self> {
        let lines: Vec<String> = content
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        Self::new(lines)
    }

    /// Get all information lines
    pub fn lines(&self) -> &[String] {
        &self.information
    }

    /// Get as a single string with newlines
    pub fn as_string(&self) -> String {
        self.information.join("\n")
    }
}

impl SwiftField for Field77B {
    const TAG: &'static str = "77B";

    fn parse(content: &str) -> Result<Self> {
        Self::from_string(content)
    }

    fn to_swift_string(&self) -> String {
        format!(":77B:{}", self.as_string())
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        rules.validate_field("77B", &self.as_string())
    }

    fn description() -> &'static str {
        "Regulatory Reporting - Free text field for regulatory information"
    }
}

impl std::fmt::Display for Field77B {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_parser::FormatRules;
    use std::collections::HashMap;

    #[test]
    fn test_field77b_creation() {
        let lines = vec!["ORDERRES".to_string(), "BE03200000000XXX".to_string()];
        let field = Field77B::new(lines.clone()).unwrap();
        assert_eq!(field.lines(), &lines);
    }

    #[test]
    fn test_field77b_from_string() {
        let content = "ORDERRES\nBE03200000000XXX\nREGPORT123";
        let field = Field77B::from_string(content).unwrap();
        assert_eq!(field.lines().len(), 3);
        assert_eq!(field.lines()[0], "ORDERRES");
        assert_eq!(field.lines()[1], "BE03200000000XXX");
        assert_eq!(field.lines()[2], "REGPORT123");
    }

    #[test]
    fn test_field77b_too_many_lines() {
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(), // Too many
        ];
        let result = Field77B::new(lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_field77b_line_too_long() {
        let lines = vec!["A".repeat(36)]; // 36 characters, max is 35
        let result = Field77B::new(lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_field77b_parse() {
        let field = Field77B::parse("ORDERRES\nBE123").unwrap();
        assert_eq!(field.lines().len(), 2);
        assert_eq!(field.to_swift_string(), ":77B:ORDERRES\nBE123");
    }

    #[test]
    fn test_field77b_validation() {
        let field = Field77B::from_string("ORDERRES").unwrap();
        let rules = FormatRules {
            fields: HashMap::new(),
        };
        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field77b_display() {
        let field = Field77B::from_string("Line 1\nLine 2").unwrap();
        assert_eq!(format!("{}", field), "Line 1\nLine 2");
    }
}
