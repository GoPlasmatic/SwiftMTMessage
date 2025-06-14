use crate::{SwiftField, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 52D: Ordering Institution (Option D)
///
/// ## Overview
/// Field 52D identifies the ordering institution in SWIFT payment messages using name
/// and address information rather than a BIC code. This option is used when the ordering
/// institution does not have a BIC or when full name and address details are required
/// for regulatory, compliance, or routing purposes.
///
/// ## Format Specification
/// **Format**: `4*35x`
/// - **4*35x**: Up to 4 lines of name and address information
/// - **Line length**: Maximum 35 characters per line
/// - **Character set**: SWIFT character set (printable ASCII)
/// - **Content**: Institution name, street address, city, country
///
/// ## Structure
/// ```text
/// Line 1: Institution Name
/// Line 2: Street Address
/// Line 3: City, State/Province, Postal Code
/// Line 4: Country (optional)
/// ```
///
/// ## Usage Context
/// Field 52D is used in:
/// - **MT103**: Single Customer Credit Transfer (when 52A not available)
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer  
/// - **MT202COV**: Cover for customer credit transfer
///
/// ### Business Applications
/// - **Non-SWIFT institutions**: Identifying institutions without BIC codes
/// - **Regulatory compliance**: Providing full address for compliance requirements
/// - **Local banks**: Identifying smaller banks or credit unions
/// - **Correspondent banking**: When full address details are required
/// - **Sanctions screening**: Enabling comprehensive name/address screening
///
/// ## Examples
/// ```text
/// :52D:FIRST NATIONAL BANK
/// 123 MAIN STREET
/// ANYTOWN NY 12345
/// UNITED STATES
/// └─── US regional bank with full address
///
/// :52D:BANQUE REGIONALE SARL
/// 45 RUE DE LA PAIX
/// PARIS 75001 FRANCE
/// └─── French regional bank (3 lines)
///
/// :52D:CREDIT UNION COOPERATIVE
/// 789 COMMUNITY DRIVE
/// SMALLTOWN CA 90210
/// └─── Credit union (minimal address)
/// ```
///
/// ## Address Format Guidelines
/// - **Line 1**: Institution legal name (required)
/// - **Line 2**: Street address/building number (recommended)
/// - **Line 3**: City, state/province, postal code (recommended)
/// - **Line 4**: Country name (optional but recommended for international)
///
/// ### Address Standards
/// - Use standard postal abbreviations
/// - Include postal/ZIP codes when available
/// - Spell out country names in full
/// - Avoid special characters and diacritical marks
/// - Use standard address formatting conventions
///
/// ## Validation Rules
/// 1. **Minimum content**: At least 1 line required
/// 2. **Maximum lines**: No more than 4 lines
/// 3. **Line length**: Each line maximum 35 characters
/// 4. **Character validation**: Only printable ASCII characters
/// 5. **Content requirement**: Must contain meaningful institution information
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Minimum 1 line, maximum 4 lines allowed (Error: C54)
/// - Each line cannot exceed 35 characters (Error: T14)
/// - Characters must be from SWIFT character set (Error: T61)
/// - Lines cannot be empty (Error: T11)
/// - Must contain institution name in first line (Error: C55)
/// - Field 52D alternative to 52A (Error: C52)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field52D {
    /// Name and address lines (up to 4 lines of 35 characters each)
    pub name_and_address: Vec<String>,
}

impl Field52D {
    /// Create a new Field52D with validation
    pub fn new(name_and_address: Vec<String>) -> Result<Self, crate::ParseError> {
        if name_and_address.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "52D".to_string(),
                message: "Name and address cannot be empty".to_string(),
            });
        }

        if name_and_address.len() > 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "52D".to_string(),
                message: "Too many name/address lines (max 4)".to_string(),
            });
        }

        for (i, line) in name_and_address.iter().enumerate() {
            if line.len() > 35 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52D".to_string(),
                    message: format!("Line {} too long (max 35 characters)", i + 1),
                });
            }

            // Validate characters (printable ASCII)
            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52D".to_string(),
                    message: format!("Line {} contains invalid characters", i + 1),
                });
            }
        }

        Ok(Field52D { name_and_address })
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
                field_tag: "52D".to_string(),
                message: "Cannot add more lines (max 4)".to_string(),
            });
        }

        if line.len() > 35 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "52D".to_string(),
                message: "Line too long (max 35 characters)".to_string(),
            });
        }

        if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "52D".to_string(),
                message: "Line contains invalid characters".to_string(),
            });
        }

        self.name_and_address.push(line);
        Ok(())
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!("Ordering Institution ({} lines)", self.line_count())
    }
}

impl SwiftField for Field52D {
    fn parse(content: &str) -> crate::Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "52D".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let content = if let Some(stripped) = content.strip_prefix(":52D:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("52D:") {
            stripped
        } else {
            content
        };

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        Field52D::new(lines)
    }

    fn to_swift_string(&self) -> String {
        format!(":52D:{}", self.name_and_address.join("\n"))
    }

    fn validate(&self) -> ValidationResult {
        use crate::errors::ValidationError;

        let mut errors = Vec::new();

        if self.name_and_address.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "52D".to_string(),
                message: "Name and address cannot be empty".to_string(),
            });
        }

        if self.name_and_address.len() > 4 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "52D".to_string(),
                expected: "max 4 lines".to_string(),
                actual: self.name_and_address.len(),
            });
        }

        for (i, line) in self.name_and_address.iter().enumerate() {
            if line.len() > 35 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "52D".to_string(),
                    expected: format!("max 35 characters for line {}", i + 1),
                    actual: line.len(),
                });
            }

            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "52D".to_string(),
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

impl std::fmt::Display for Field52D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name_and_address.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field52d_creation() {
        let lines = vec![
            "ABC BANK".to_string(),
            "123 MAIN STREET".to_string(),
            "NEW YORK NY 10001".to_string(),
        ];
        let field = Field52D::new(lines.clone()).unwrap();
        assert_eq!(field.name_and_address(), &lines);
        assert_eq!(field.line_count(), 3);
        assert_eq!(field.line(0), Some("ABC BANK"));
        assert_eq!(field.line(1), Some("123 MAIN STREET"));
        assert_eq!(field.line(2), Some("NEW YORK NY 10001"));
        assert_eq!(field.line(3), None);
    }

    #[test]
    fn test_field52d_creation_single_line() {
        let lines = vec!["ABC BANK".to_string()];
        let field = Field52D::new(lines.clone()).unwrap();
        assert_eq!(field.name_and_address(), &lines);
        assert_eq!(field.line_count(), 1);
    }

    #[test]
    fn test_field52d_from_string() {
        let content = "ABC BANK\n123 MAIN STREET\nNEW YORK NY 10001";
        let field = Field52D::from_string(content).unwrap();
        assert_eq!(field.line_count(), 3);
        assert_eq!(field.line(0), Some("ABC BANK"));
        assert_eq!(field.line(1), Some("123 MAIN STREET"));
        assert_eq!(field.line(2), Some("NEW YORK NY 10001"));
    }

    #[test]
    fn test_field52d_parse() {
        let field = Field52D::parse("ABC BANK\n123 MAIN STREET").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("ABC BANK"));
        assert_eq!(field.line(1), Some("123 MAIN STREET"));
    }

    #[test]
    fn test_field52d_parse_with_tag() {
        let field = Field52D::parse(":52D:ABC BANK\n123 MAIN STREET").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("ABC BANK"));
        assert_eq!(field.line(1), Some("123 MAIN STREET"));
    }

    #[test]
    fn test_field52d_to_swift_string() {
        let lines = vec!["ABC BANK".to_string(), "123 MAIN STREET".to_string()];
        let field = Field52D::new(lines).unwrap();
        assert_eq!(field.to_swift_string(), ":52D:ABC BANK\n123 MAIN STREET");
    }

    #[test]
    fn test_field52d_display() {
        let lines = vec!["ABC BANK".to_string(), "123 MAIN STREET".to_string()];
        let field = Field52D::new(lines).unwrap();
        assert_eq!(format!("{}", field), "ABC BANK\n123 MAIN STREET");
    }

    #[test]
    fn test_field52d_description() {
        let lines = vec!["ABC BANK".to_string(), "123 MAIN STREET".to_string()];
        let field = Field52D::new(lines).unwrap();
        assert_eq!(field.description(), "Ordering Institution (2 lines)");
    }

    #[test]
    fn test_field52d_add_line() {
        let lines = vec!["ABC BANK".to_string()];
        let mut field = Field52D::new(lines).unwrap();

        field.add_line("123 MAIN STREET".to_string()).unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(1), Some("123 MAIN STREET"));

        field.add_line("NEW YORK NY 10001".to_string()).unwrap();
        assert_eq!(field.line_count(), 3);

        field.add_line("USA".to_string()).unwrap();
        assert_eq!(field.line_count(), 4);

        // Should fail when trying to add 5th line
        let result = field.add_line("TOO MANY LINES".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_field52d_validation_empty() {
        let result = Field52D::new(vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_field52d_validation_too_many_lines() {
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(),
            "Line 5".to_string(), // Too many
        ];
        let result = Field52D::new(lines);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("max 4"));
    }

    #[test]
    fn test_field52d_validation_line_too_long() {
        let lines = vec!["A".repeat(36)]; // 36 characters, max is 35
        let result = Field52D::new(lines);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too long"));
    }

    #[test]
    fn test_field52d_validation_invalid_characters() {
        let lines = vec!["ABC BANK\x00".to_string()]; // Contains null character
        let result = Field52D::new(lines);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid characters")
        );
    }

    #[test]
    fn test_field52d_validate() {
        let lines = vec!["ABC BANK".to_string(), "123 MAIN STREET".to_string()];
        let field = Field52D::new(lines).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field52d_validate_errors() {
        let lines = vec!["A".repeat(36)]; // Line too long
        let field = Field52D {
            name_and_address: lines,
        };
        let validation = field.validate();
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
    }
}
