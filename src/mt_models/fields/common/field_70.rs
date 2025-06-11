//! Field 70: Remittance Information
//!
//! Free text information for the beneficiary.
//! Format: 4*35x (up to 4 lines of 35 characters each)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 70: Remittance Information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field70 {
    /// Remittance information lines (up to 4 lines of 35 characters each)
    pub information: Vec<String>,
}

impl Field70 {
    /// Create a new Field70 with validation
    pub fn new(information: Vec<String>) -> Result<Self> {
        if information.is_empty() {
            return Err(FieldParseError::missing_data(
                "70",
                "Remittance information cannot be empty",
            )
            .into());
        }

        if information.len() > 4 {
            return Err(FieldParseError::invalid_format("70", "Too many lines (max 4)").into());
        }

        for (i, line) in information.iter().enumerate() {
            if line.len() > 35 {
                return Err(FieldParseError::invalid_format(
                    "70",
                    &format!("Line {} too long (max 35 characters)", i + 1),
                )
                .into());
            }

            // Validate characters (printable ASCII)
            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(FieldParseError::invalid_format(
                    "70",
                    &format!("Line {} contains invalid characters", i + 1),
                )
                .into());
            }
        }

        Ok(Field70 { information })
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

impl SwiftField for Field70 {
    const TAG: &'static str = "70";

    fn parse(content: &str) -> Result<Self> {
        Self::from_string(content)
    }

    fn to_swift_string(&self) -> String {
        format!(":70:{}", self.as_string())
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        rules.validate_field("70", &self.as_string())
    }

    fn description() -> &'static str {
        "Remittance Information - Free text information for the beneficiary"
    }
}

impl std::fmt::Display for Field70 {
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
    fn test_field70_creation() {
        let lines = vec![
            "Invoice payment".to_string(),
            "Reference: INV-2024-001".to_string(),
        ];
        let field = Field70::new(lines.clone()).unwrap();
        assert_eq!(field.lines(), &lines);
    }

    #[test]
    fn test_field70_from_string() {
        let content = "Invoice payment\nReference: INV-2024-001\nDue date: 2024-03-15";
        let field = Field70::from_string(content).unwrap();
        assert_eq!(field.lines().len(), 3);
        assert_eq!(field.lines()[0], "Invoice payment");
        assert_eq!(field.lines()[1], "Reference: INV-2024-001");
        assert_eq!(field.lines()[2], "Due date: 2024-03-15");
    }

    #[test]
    fn test_field70_too_many_lines() {
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(),
            "Line 5".to_string(), // Too many
        ];
        let result = Field70::new(lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_field70_line_too_long() {
        let lines = vec!["A".repeat(36)]; // 36 characters, max is 35
        let result = Field70::new(lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_field70_parse() {
        let field = Field70::parse("Invoice payment\nRef: 123").unwrap();
        assert_eq!(field.lines().len(), 2);
        assert_eq!(field.to_swift_string(), ":70:Invoice payment\nRef: 123");
    }

    #[test]
    fn test_field70_validation() {
        let field = Field70::from_string("Test payment").unwrap();
        let rules = FormatRules {
            fields: HashMap::new(),
        };
        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field70_display() {
        let field = Field70::from_string("Line 1\nLine 2").unwrap();
        assert_eq!(format!("{}", field), "Line 1\nLine 2");
    }
}
