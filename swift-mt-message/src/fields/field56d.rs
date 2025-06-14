use crate::{SwiftField, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 56D: Intermediary Institution (Option D)
///
/// ## Overview
/// Field 56D identifies an intermediary institution in SWIFT payment messages using name and
/// address information. This field provides an alternative identification method when BIC codes
/// or account numbers are not available or suitable. It allows for detailed specification of
/// the intermediary institution through structured name and address data, supporting various
/// international payment routing scenarios where traditional identifiers may not be sufficient.
///
/// ## Format Specification
/// **Format**: `4*35x`
/// - **4*35x**: Up to 4 lines of 35 characters each
/// - **Line structure**: Free-form text for name and address
/// - **Character set**: SWIFT character set (A-Z, 0-9, and limited special characters)
/// - **Line separation**: Each line on separate row
///
/// ## Structure
/// ```text
/// INTERMEDIARY BANK NAME LIMITED
/// 123 FINANCIAL DISTRICT AVENUE
/// NEW YORK NY 10005
/// UNITED STATES OF AMERICA
/// │                              │
/// └──────────────────────────────┘
///        Up to 35 characters per line
///        Maximum 4 lines
/// ```
///
/// ## Field Components
/// - **Institution Name**: Official name of intermediary institution
/// - **Street Address**: Physical address details
/// - **City/State**: Location information
/// - **Country**: Country of domicile
/// - **Additional Info**: Branch details, postal codes, etc.
///
/// ## Usage Context
/// Field 56D is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Non-BIC institutions**: Identifying institutions without BIC codes
/// - **Regional banks**: Supporting local and regional financial institutions
/// - **Correspondent banking**: Detailed intermediary identification
/// - **Cross-border payments**: International payment routing
/// - **Regulatory compliance**: Meeting identification requirements
/// - **Payment transparency**: Providing clear intermediary details
///
/// ## Examples
/// ```text
/// :56D:INTERMEDIARY BANK LIMITED
/// 456 BANKING STREET
/// LONDON EC2V 8RF
/// UNITED KINGDOM
/// └─── UK intermediary bank with full address
///
/// :56D:REGIONAL COOPERATIVE BANK
/// HAUPTSTRASSE 123
/// 60311 FRANKFURT AM MAIN
/// GERMANY
/// └─── German regional bank identification
///
/// :56D:CITY COMMERCIAL BANK
/// FINANCIAL DISTRICT
/// MUMBAI 400001
/// INDIA
/// └─── Indian commercial bank details
///
/// :56D:PROVINCIAL SAVINGS BANK
/// RUE DE LA BANQUE 789
/// 75001 PARIS
/// FRANCE
/// └─── French provincial bank with address
/// ```
///
/// ## Name and Address Guidelines
/// - **Line 1**: Institution name (required)
/// - **Line 2**: Street address or building details
/// - **Line 3**: City, state/province, postal code
/// - **Line 4**: Country name
/// - **Formatting**: Clear, unambiguous identification
/// - **Language**: English preferred for international payments
///
/// ## Validation Rules
/// 1. **Line count**: Minimum 1, maximum 4 lines
/// 2. **Line length**: Maximum 35 characters per line
/// 3. **Character set**: SWIFT character set only
/// 4. **Content**: Each line must contain meaningful information
/// 5. **Empty lines**: Not permitted
/// 6. **Control characters**: Not allowed
/// 7. **Special characters**: Limited to SWIFT-approved set
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Maximum 4 lines allowed (Error: T26)
/// - Each line maximum 35 characters (Error: T50)
/// - Must use SWIFT character set only (Error: T61)
/// - At least one line required (Error: T13)
/// - No empty lines permitted (Error: T40)
/// - Field 56D alternative to 56A/56C (Error: C56)
/// - Institution must be identifiable (Error: T51)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field56D {
    /// Name and address lines (up to 4 lines of 35 characters each)
    pub name_and_address: Vec<String>,
}

impl Field56D {
    /// Create a new Field56D with validation
    pub fn new(name_and_address: Vec<String>) -> Result<Self, crate::ParseError> {
        if name_and_address.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "56D".to_string(),
                message: "Name and address cannot be empty".to_string(),
            });
        }

        if name_and_address.len() > 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "56D".to_string(),
                message: "Too many name/address lines (max 4)".to_string(),
            });
        }

        for (i, line) in name_and_address.iter().enumerate() {
            if line.len() > 35 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "56D".to_string(),
                    message: format!("Line {} too long (max 35 characters)", i + 1),
                });
            }

            // Validate characters (printable ASCII)
            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "56D".to_string(),
                    message: format!("Line {} contains invalid characters", i + 1),
                });
            }
        }

        Ok(Field56D { name_and_address })
    }

    /// Create from a single string, splitting on newlines
    pub fn from_string(content: impl Into<String>) -> Result<Self, crate::ParseError> {
        let content = content.into();
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        Self::new(lines)
    }

    /// Get the name and address lines
    pub fn name_and_address(&self) -> &[String] {
        &self.name_and_address
    }

    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.name_and_address.len()
    }

    /// Get a specific line by index
    pub fn line(&self, index: usize) -> Option<&str> {
        self.name_and_address.get(index).map(|s| s.as_str())
    }

    /// Add a line of name/address information
    pub fn add_line(&mut self, line: String) -> Result<(), crate::ParseError> {
        if self.name_and_address.len() >= 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "56D".to_string(),
                message: "Cannot add more lines (max 4)".to_string(),
            });
        }

        if line.len() > 35 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "56D".to_string(),
                message: "Line too long (max 35 characters)".to_string(),
            });
        }

        if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "56D".to_string(),
                message: "Line contains invalid characters".to_string(),
            });
        }

        self.name_and_address.push(line);
        Ok(())
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!("Intermediary Institution ({} lines)", self.line_count())
    }
}

impl SwiftField for Field56D {
    fn parse(content: &str) -> crate::Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "56D".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let content = if let Some(stripped) = content.strip_prefix(":56D:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("56D:") {
            stripped
        } else {
            content
        };

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        Field56D::new(lines)
    }

    fn to_swift_string(&self) -> String {
        format!(":56D:{}", self.name_and_address.join("\n"))
    }

    fn validate(&self) -> ValidationResult {
        use crate::errors::ValidationError;

        let mut errors = Vec::new();

        if self.name_and_address.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "56D".to_string(),
                message: "Name and address cannot be empty".to_string(),
            });
        }

        if self.name_and_address.len() > 4 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "56D".to_string(),
                expected: "max 4 lines".to_string(),
                actual: self.name_and_address.len(),
            });
        }

        for (i, line) in self.name_and_address.iter().enumerate() {
            if line.len() > 35 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "56D".to_string(),
                    expected: format!("max 35 characters for line {}", i + 1),
                    actual: line.len(),
                });
            }

            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "56D".to_string(),
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
        "4*35x"
    }
}

impl std::fmt::Display for Field56D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name_and_address.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field56d_creation() {
        let lines = vec![
            "INTERMEDIARY BANK".to_string(),
            "123 INTERMEDIARY AVENUE".to_string(),
            "NEW YORK NY 10001".to_string(),
            "UNITED STATES".to_string(),
        ];
        let field = Field56D::new(lines.clone()).unwrap();
        assert_eq!(field.name_and_address(), &lines);
        assert_eq!(field.line_count(), 4);
        assert_eq!(field.line(0), Some("INTERMEDIARY BANK"));
        assert_eq!(field.line(1), Some("123 INTERMEDIARY AVENUE"));
        assert_eq!(field.line(2), Some("NEW YORK NY 10001"));
        assert_eq!(field.line(3), Some("UNITED STATES"));
        assert_eq!(field.line(4), None);
    }

    #[test]
    fn test_field56d_creation_single_line() {
        let lines = vec!["INTERMEDIARY BANK".to_string()];
        let field = Field56D::new(lines.clone()).unwrap();
        assert_eq!(field.name_and_address(), &lines);
        assert_eq!(field.line_count(), 1);
    }

    #[test]
    fn test_field56d_from_string() {
        let content =
            "INTERMEDIARY BANK\n123 INTERMEDIARY AVENUE\nNEW YORK NY 10001\nUNITED STATES";
        let field = Field56D::from_string(content).unwrap();
        assert_eq!(field.line_count(), 4);
        assert_eq!(field.line(0), Some("INTERMEDIARY BANK"));
        assert_eq!(field.line(1), Some("123 INTERMEDIARY AVENUE"));
        assert_eq!(field.line(2), Some("NEW YORK NY 10001"));
        assert_eq!(field.line(3), Some("UNITED STATES"));
    }

    #[test]
    fn test_field56d_parse() {
        let field = Field56D::parse("INTERMEDIARY BANK\n123 INTERMEDIARY AVENUE").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("INTERMEDIARY BANK"));
        assert_eq!(field.line(1), Some("123 INTERMEDIARY AVENUE"));
    }

    #[test]
    fn test_field56d_parse_with_tag() {
        let field = Field56D::parse(":56D:INTERMEDIARY BANK\n123 INTERMEDIARY AVENUE").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("INTERMEDIARY BANK"));
        assert_eq!(field.line(1), Some("123 INTERMEDIARY AVENUE"));
    }

    #[test]
    fn test_field56d_to_swift_string() {
        let lines = vec![
            "INTERMEDIARY BANK".to_string(),
            "123 INTERMEDIARY AVENUE".to_string(),
        ];
        let field = Field56D::new(lines).unwrap();
        assert_eq!(
            field.to_swift_string(),
            ":56D:INTERMEDIARY BANK\n123 INTERMEDIARY AVENUE"
        );
    }

    #[test]
    fn test_field56d_display() {
        let lines = vec![
            "INTERMEDIARY BANK".to_string(),
            "123 INTERMEDIARY AVENUE".to_string(),
        ];
        let field = Field56D::new(lines).unwrap();
        assert_eq!(
            format!("{}", field),
            "INTERMEDIARY BANK\n123 INTERMEDIARY AVENUE"
        );
    }

    #[test]
    fn test_field56d_description() {
        let lines = vec![
            "INTERMEDIARY BANK".to_string(),
            "123 INTERMEDIARY AVENUE".to_string(),
        ];
        let field = Field56D::new(lines).unwrap();
        assert_eq!(field.description(), "Intermediary Institution (2 lines)");
    }

    #[test]
    fn test_field56d_add_line() {
        let lines = vec!["INTERMEDIARY BANK".to_string()];
        let mut field = Field56D::new(lines).unwrap();

        field
            .add_line("123 INTERMEDIARY AVENUE".to_string())
            .unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(1), Some("123 INTERMEDIARY AVENUE"));

        field.add_line("NEW YORK NY 10001".to_string()).unwrap();
        assert_eq!(field.line_count(), 3);

        field.add_line("UNITED STATES".to_string()).unwrap();
        assert_eq!(field.line_count(), 4);

        // Should fail when trying to add 5th line
        let result = field.add_line("TOO MANY LINES".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_field56d_validation_empty() {
        let result = Field56D::new(vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_field56d_validation_too_many_lines() {
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(),
            "Line 5".to_string(), // Too many
        ];
        let result = Field56D::new(lines);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("max 4"));
    }

    #[test]
    fn test_field56d_validation_line_too_long() {
        let lines = vec!["A".repeat(36)]; // 36 characters, max is 35
        let result = Field56D::new(lines);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too long"));
    }

    #[test]
    fn test_field56d_validation_invalid_characters() {
        let lines = vec!["INTERMEDIARY BANK\x00".to_string()]; // Contains null character
        let result = Field56D::new(lines);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid characters")
        );
    }

    #[test]
    fn test_field56d_validate() {
        let lines = vec![
            "INTERMEDIARY BANK".to_string(),
            "123 INTERMEDIARY AVENUE".to_string(),
        ];
        let field = Field56D::new(lines).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field56d_validate_errors() {
        let lines = vec!["A".repeat(36)]; // Line too long
        let field = Field56D {
            name_and_address: lines,
        };
        let validation = field.validate();
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
    }
}
