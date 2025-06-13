use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Field 72: Sender to Receiver Information
///
/// Format: 6*35x (up to 6 lines of 35 characters each)
///
/// Information from the sender to the receiver.
/// Note: Only coded information is allowed in MT103.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field72 {
    /// Information lines (up to 6 lines of 35 characters each)
    pub information: Vec<String>,
}

impl Field72 {
    /// Create a new Field72 with validation
    pub fn new(information: Vec<String>) -> Result<Self, crate::ParseError> {
        if information.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "72".to_string(),
                message: "Sender to receiver information cannot be empty".to_string(),
            });
        }

        if information.len() > 6 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "72".to_string(),
                message: "Too many lines (max 6)".to_string(),
            });
        }

        for (i, line) in information.iter().enumerate() {
            if line.len() > 35 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "72".to_string(),
                    message: format!("Line {} too long (max 35 characters)", i + 1),
                });
            }

            // Validate characters (printable ASCII)
            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "72".to_string(),
                    message: format!("Line {} contains invalid characters", i + 1),
                });
            }
        }

        Ok(Field72 { information })
    }

    /// Create from a single string, splitting on newlines
    pub fn from_string(content: impl Into<String>) -> Result<Self, crate::ParseError> {
        let content = content.into();
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        Self::new(lines)
    }

    /// Get the information lines
    pub fn information(&self) -> &[String] {
        &self.information
    }

    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.information.len()
    }

    /// Get a specific line by index
    pub fn line(&self, index: usize) -> Option<&str> {
        self.information.get(index).map(|s| s.as_str())
    }

    /// Add a line of information
    pub fn add_line(&mut self, line: String) -> Result<(), crate::ParseError> {
        if self.information.len() >= 6 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "72".to_string(),
                message: "Cannot add more lines (max 6)".to_string(),
            });
        }

        if line.len() > 35 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "72".to_string(),
                message: "Line too long (max 35 characters)".to_string(),
            });
        }

        if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "72".to_string(),
                message: "Line contains invalid characters".to_string(),
            });
        }

        self.information.push(line);
        Ok(())
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!(
            "Sender to Receiver Information ({} lines)",
            self.line_count()
        )
    }
}

impl SwiftField for Field72 {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":72:") {
            stripped // Remove ":72:" prefix
        } else if let Some(stripped) = value.strip_prefix("72:") {
            stripped // Remove "72:" prefix
        } else {
            value
        };

        let content = content.trim();

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "72".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        Self::from_string(content)
    }

    fn to_swift_string(&self) -> String {
        format!(":72:{}", self.information.join("\n"))
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate line count
        if self.information.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "72".to_string(),
                message: "Information cannot be empty".to_string(),
            });
        }

        if self.information.len() > 6 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "72".to_string(),
                expected: "max 6 lines".to_string(),
                actual: self.information.len(),
            });
        }

        // Validate each line
        for (i, line) in self.information.iter().enumerate() {
            if line.len() > 35 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "72".to_string(),
                    expected: format!("max 35 characters for line {}", i + 1),
                    actual: line.len(),
                });
            }

            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "72".to_string(),
                    message: format!("Line {} contains invalid characters", i + 1),
                });
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "6*35x"
    }
}

impl std::fmt::Display for Field72 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.information.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field72_creation() {
        let lines = vec!["/INS/CHQS".to_string(), "/BENEFRES/BE".to_string()];
        let field = Field72::new(lines.clone()).unwrap();
        assert_eq!(field.information(), &lines);
        assert_eq!(field.line_count(), 2);
    }

    #[test]
    fn test_field72_from_string() {
        let content = "/INS/CHQS\n/BENEFRES/BE\n/ORDERRES/DE";
        let field = Field72::from_string(content).unwrap();
        assert_eq!(field.line_count(), 3);
        assert_eq!(field.line(0), Some("/INS/CHQS"));
        assert_eq!(field.line(1), Some("/BENEFRES/BE"));
        assert_eq!(field.line(2), Some("/ORDERRES/DE"));
    }

    #[test]
    fn test_field72_parse() {
        let field = Field72::parse("/INS/CHQS\n/BENEFRES/BE").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("/INS/CHQS"));
        assert_eq!(field.line(1), Some("/BENEFRES/BE"));
    }

    #[test]
    fn test_field72_parse_with_prefix() {
        let field = Field72::parse(":72:/INS/CHQS\n/BENEFRES/BE").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("/INS/CHQS"));
    }

    #[test]
    fn test_field72_to_swift_string() {
        let lines = vec!["/INS/CHQS".to_string(), "/BENEFRES/BE".to_string()];
        let field = Field72::new(lines).unwrap();
        assert_eq!(field.to_swift_string(), ":72:/INS/CHQS\n/BENEFRES/BE");
    }

    #[test]
    fn test_field72_add_line() {
        let mut field = Field72::new(vec!["/INS/CHQS".to_string()]).unwrap();
        field.add_line("/BENEFRES/BE".to_string()).unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(1), Some("/BENEFRES/BE"));
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
    fn test_field72_empty() {
        let result = Field72::new(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_field72_validation() {
        let field = Field72::new(vec!["/INS/CHQS".to_string()]).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field72_display() {
        let field =
            Field72::new(vec!["/INS/CHQS".to_string(), "/BENEFRES/BE".to_string()]).unwrap();
        assert_eq!(format!("{}", field), "/INS/CHQS\n/BENEFRES/BE");
    }

    #[test]
    fn test_field72_description() {
        let field =
            Field72::new(vec!["/INS/CHQS".to_string(), "/BENEFRES/BE".to_string()]).unwrap();
        assert_eq!(
            field.description(),
            "Sender to Receiver Information (2 lines)"
        );
    }

    #[test]
    fn test_field72_line_access() {
        let field = Field72::new(vec!["/INS/CHQS".to_string()]).unwrap();
        assert_eq!(field.line(0), Some("/INS/CHQS"));
        assert_eq!(field.line(1), None);
    }

    #[test]
    fn test_field72_add_line_max_reached() {
        let mut field = Field72::new(vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(),
            "Line 5".to_string(),
            "Line 6".to_string(),
        ])
        .unwrap();

        let result = field.add_line("Line 7".to_string());
        assert!(result.is_err());
    }
}
