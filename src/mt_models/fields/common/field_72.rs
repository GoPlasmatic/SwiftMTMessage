//! Field 72: Sender to Receiver Information
//!
//! Information from the sender to the receiver.
//! Format: 6*35x (up to 6 lines of 35 characters each)
//! Note: Only coded information is allowed in MT103

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 72: Sender to Receiver Information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field72 {
    /// Information lines (up to 6 lines of 35 characters each)
    pub information: Vec<String>,
}

impl Field72 {
    /// Create a new Field72 with validation
    pub fn new(information: Vec<String>) -> Result<Self> {
        if information.is_empty() {
            return Err(FieldParseError::missing_data(
                "72",
                "Sender to receiver information cannot be empty",
            )
            .into());
        }

        if information.len() > 6 {
            return Err(FieldParseError::invalid_format("72", "Too many lines (max 6)").into());
        }

        for (i, line) in information.iter().enumerate() {
            if line.len() > 35 {
                return Err(FieldParseError::invalid_format(
                    "72",
                    &format!("Line {} too long (max 35 characters)", i + 1),
                )
                .into());
            }

            // Validate characters (printable ASCII)
            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(FieldParseError::invalid_format(
                    "72",
                    &format!("Line {} contains invalid characters", i + 1),
                )
                .into());
            }
        }

        Ok(Field72 { information })
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

impl SwiftField for Field72 {
    const TAG: &'static str = "72";

    fn parse(content: &str) -> Result<Self> {
        Self::from_string(content)
    }

    fn to_swift_string(&self) -> String {
        format!(":72:{}", self.as_string())
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        rules.validate_field("72", &self.as_string())
    }

    fn description() -> &'static str {
        "Sender to Receiver Information - Information from the sender to the receiver (coded information only)"
    }
}

impl std::fmt::Display for Field72 {
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
    fn test_field72_creation() {
        let lines = vec!["/INS/CHGS".to_string(), "/REC/BENEFICIARY BANK".to_string()];
        let field = Field72::new(lines.clone()).unwrap();
        assert_eq!(field.lines(), &lines);
    }

    #[test]
    fn test_field72_from_string() {
        let content = "/INS/CHGS\n/REC/BENEFICIARY BANK\n/RETN/RETURN REASON";
        let field = Field72::from_string(content).unwrap();
        assert_eq!(field.lines().len(), 3);
        assert_eq!(field.lines()[0], "/INS/CHGS");
        assert_eq!(field.lines()[1], "/REC/BENEFICIARY BANK");
        assert_eq!(field.lines()[2], "/RETN/RETURN REASON");
    }

    #[test]
    fn test_field72_too_many_lines() {
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(),
            "Line 5".to_string(),
            "Line 6".to_string(),
            "Line 7".to_string(), // Too many
        ];
        let result = Field72::new(lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_field72_line_too_long() {
        let lines = vec!["A".repeat(36)]; // 36 characters, max is 35
        let result = Field72::new(lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_field72_parse() {
        let field = Field72::parse("/INS/CHGS\n/REC/TEST").unwrap();
        assert_eq!(field.lines().len(), 2);
        assert_eq!(field.to_swift_string(), ":72:/INS/CHGS\n/REC/TEST");
    }

    #[test]
    fn test_field72_validation() {
        let field = Field72::from_string("/INS/CHGS").unwrap();
        let rules = FormatRules {
            fields: HashMap::new(),
        };
        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field72_display() {
        let field = Field72::from_string("/INS/CHGS\n/REC/TEST").unwrap();
        assert_eq!(format!("{}", field), "/INS/CHGS\n/REC/TEST");
    }
}
