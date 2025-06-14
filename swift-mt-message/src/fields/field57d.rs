use crate::{SwiftField, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 57D: Account With Institution (Option D)
///
/// ## Overview
/// Field 57D identifies the account with institution in SWIFT payment messages using name and
/// address information. This field provides an alternative identification method when BIC codes
/// or account numbers are not available or suitable for the beneficiary's bank. It allows for
/// detailed specification of the account with institution through structured name and address
/// data, supporting various international payment scenarios where traditional identifiers may
/// not be sufficient for proper payment delivery.
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
/// BENEFICIARY BANK NAME LIMITED
/// 789 FINANCIAL CENTER BOULEVARD
/// SINGAPORE 018956
/// REPUBLIC OF SINGAPORE
/// │                              │
/// └──────────────────────────────┘
///        Up to 35 characters per line
///        Maximum 4 lines
/// ```
///
/// ## Field Components
/// - **Institution Name**: Official name of beneficiary's bank
/// - **Street Address**: Physical address details
/// - **City/State**: Location information with postal code
/// - **Country**: Country of domicile
/// - **Additional Info**: Branch details, building information, etc.
///
/// ## Usage Context
/// Field 57D is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Non-BIC institutions**: Identifying beneficiary banks without BIC codes
/// - **Regional banks**: Supporting local and regional financial institutions
/// - **Correspondent banking**: Detailed beneficiary bank identification
/// - **Cross-border payments**: International payment routing and delivery
/// - **Regulatory compliance**: Meeting beneficiary bank identification requirements
/// - **Payment transparency**: Providing clear destination bank details
///
/// ## Examples
/// ```text
/// :57D:BENEFICIARY BANK LIMITED
/// 456 BANKING STREET
/// LONDON EC2V 8RF
/// UNITED KINGDOM
/// └─── UK beneficiary bank with full address
///
/// :57D:REGIONAL SAVINGS BANK
/// HAUPTSTRASSE 789
/// 60311 FRANKFURT AM MAIN
/// GERMANY
/// └─── German regional bank identification
///
/// :57D:CITY COMMERCIAL BANK
/// FINANCIAL DISTRICT
/// MUMBAI 400001
/// INDIA
/// └─── Indian commercial bank details
///
/// :57D:PROVINCIAL CREDIT UNION
/// RUE DE LA BANQUE 456
/// 75001 PARIS
/// FRANCE
/// └─── French credit union with address
/// ```
///
/// ## Name and Address Guidelines
/// - **Line 1**: Institution name (required)
/// - **Line 2**: Street address or building details
/// - **Line 3**: City, state/province, postal code
/// - **Line 4**: Country name
/// - **Formatting**: Clear, unambiguous identification
/// - **Language**: English preferred for international payments
/// - **Accuracy**: Must match official institution records
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
/// - Field 57D alternative to 57A/57B/57C (Error: C57)
/// - Institution must be identifiable (Error: T51)
/// - Address must be complete and valid (Error: T52)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field57D {
    /// Name and address lines (up to 4 lines of 35 characters each)
    pub lines: Vec<String>,
}

impl Field57D {
    /// Create a new Field57D with validation
    pub fn new(lines: Vec<String>) -> Result<Self, crate::ParseError> {
        if lines.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57D".to_string(),
                message: "At least one line is required".to_string(),
            });
        }

        if lines.len() > 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57D".to_string(),
                message: "Cannot exceed 4 lines".to_string(),
            });
        }

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "57D".to_string(),
                    message: format!("Line {} cannot be empty", i + 1),
                });
            }

            if trimmed.len() > 35 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "57D".to_string(),
                    message: format!("Line {} cannot exceed 35 characters", i + 1),
                });
            }

            if !trimmed.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "57D".to_string(),
                    message: format!("Line {} contains invalid characters", i + 1),
                });
            }
        }

        let trimmed_lines: Vec<String> = lines.iter().map(|line| line.trim().to_string()).collect();

        Ok(Field57D {
            lines: trimmed_lines,
        })
    }

    /// Create a new Field57D from a single line
    pub fn from_string(content: impl Into<String>) -> Result<Self, crate::ParseError> {
        let content = content.into().trim().to_string();
        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57D".to_string(),
                message: "Content cannot be empty".to_string(),
            });
        }
        Self::new(vec![content])
    }

    /// Add a line to the field
    pub fn add_line(&mut self, line: impl Into<String>) -> Result<(), crate::ParseError> {
        if self.lines.len() >= 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57D".to_string(),
                message: "Cannot exceed 4 lines".to_string(),
            });
        }

        let line = line.into().trim().to_string();
        if line.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57D".to_string(),
                message: "Line cannot be empty".to_string(),
            });
        }

        if line.len() > 35 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57D".to_string(),
                message: "Line cannot exceed 35 characters".to_string(),
            });
        }

        if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57D".to_string(),
                message: "Line contains invalid characters".to_string(),
            });
        }

        self.lines.push(line);
        Ok(())
    }

    /// Get a specific line by index (0-based)
    pub fn line(&self, index: usize) -> Option<&str> {
        self.lines.get(index).map(|s| s.as_str())
    }

    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get all lines
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!(
            "Account With Institution (Name/Address: {})",
            self.lines.join(", ")
        )
    }
}

impl SwiftField for Field57D {
    fn parse(content: &str) -> crate::Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57D".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let content = if let Some(stripped) = content.strip_prefix(":57D:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("57D:") {
            stripped
        } else {
            content
        };

        let lines: Vec<String> = content
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        Field57D::new(lines)
    }

    fn to_swift_string(&self) -> String {
        format!(":57D:{}", self.lines.join("\n"))
    }

    fn validate(&self) -> ValidationResult {
        // Validation is done in constructor
        ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "4*35x"
    }
}

impl std::fmt::Display for Field57D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lines.join(" / "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field57d_creation_single_line() {
        let field = Field57D::new(vec!["ACCOUNT WITH BANK NAME".to_string()]).unwrap();
        assert_eq!(field.line_count(), 1);
        assert_eq!(field.line(0), Some("ACCOUNT WITH BANK NAME"));
    }

    #[test]
    fn test_field57d_creation_multiple_lines() {
        let field = Field57D::new(vec![
            "ACCOUNT WITH BANK NAME".to_string(),
            "123 MAIN STREET".to_string(),
            "NEW YORK, NY 10001".to_string(),
        ])
        .unwrap();
        assert_eq!(field.line_count(), 3);
        assert_eq!(field.line(0), Some("ACCOUNT WITH BANK NAME"));
        assert_eq!(field.line(1), Some("123 MAIN STREET"));
        assert_eq!(field.line(2), Some("NEW YORK, NY 10001"));
    }

    #[test]
    fn test_field57d_from_string() {
        let field = Field57D::from_string("ACCOUNT WITH BANK NAME").unwrap();
        assert_eq!(field.line_count(), 1);
        assert_eq!(field.line(0), Some("ACCOUNT WITH BANK NAME"));
    }

    #[test]
    fn test_field57d_add_line() {
        let mut field = Field57D::from_string("ACCOUNT WITH BANK NAME").unwrap();
        field.add_line("123 MAIN STREET").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(1), Some("123 MAIN STREET"));
    }

    #[test]
    fn test_field57d_parse_single_line() {
        let field = Field57D::parse("ACCOUNT WITH BANK NAME").unwrap();
        assert_eq!(field.line_count(), 1);
        assert_eq!(field.line(0), Some("ACCOUNT WITH BANK NAME"));
    }

    #[test]
    fn test_field57d_parse_multiple_lines() {
        let content = "ACCOUNT WITH BANK NAME\n123 MAIN STREET\nNEW YORK, NY 10001";
        let field = Field57D::parse(content).unwrap();
        assert_eq!(field.line_count(), 3);
        assert_eq!(field.line(0), Some("ACCOUNT WITH BANK NAME"));
        assert_eq!(field.line(1), Some("123 MAIN STREET"));
        assert_eq!(field.line(2), Some("NEW YORK, NY 10001"));
    }

    #[test]
    fn test_field57d_parse_with_tag() {
        let field = Field57D::parse(":57D:ACCOUNT WITH BANK NAME").unwrap();
        assert_eq!(field.line_count(), 1);
        assert_eq!(field.line(0), Some("ACCOUNT WITH BANK NAME"));
    }

    #[test]
    fn test_field57d_to_swift_string() {
        let field = Field57D::new(vec![
            "ACCOUNT WITH BANK NAME".to_string(),
            "123 MAIN STREET".to_string(),
        ])
        .unwrap();
        assert_eq!(
            field.to_swift_string(),
            ":57D:ACCOUNT WITH BANK NAME\n123 MAIN STREET"
        );
    }

    #[test]
    fn test_field57d_display() {
        let field = Field57D::new(vec![
            "ACCOUNT WITH BANK NAME".to_string(),
            "123 MAIN STREET".to_string(),
        ])
        .unwrap();
        assert_eq!(
            format!("{}", field),
            "ACCOUNT WITH BANK NAME / 123 MAIN STREET"
        );
    }

    #[test]
    fn test_field57d_description() {
        let field = Field57D::from_string("ACCOUNT WITH BANK NAME").unwrap();
        assert_eq!(
            field.description(),
            "Account With Institution (Name/Address: ACCOUNT WITH BANK NAME)"
        );
    }

    #[test]
    fn test_field57d_validation_empty_lines() {
        let result = Field57D::new(vec![]);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("At least one line is required")
        );
    }

    #[test]
    fn test_field57d_validation_too_many_lines() {
        let result = Field57D::new(vec![
            "LINE1".to_string(),
            "LINE2".to_string(),
            "LINE3".to_string(),
            "LINE4".to_string(),
            "LINE5".to_string(),
        ]);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Cannot exceed 4 lines")
        );
    }

    #[test]
    fn test_field57d_validation_line_too_long() {
        let long_line = "A".repeat(36); // 36 characters, max is 35
        let result = Field57D::new(vec![long_line]);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("cannot exceed 35 characters")
        );
    }

    #[test]
    fn test_field57d_validation_empty_line() {
        let result = Field57D::new(vec!["".to_string()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_field57d_validation_invalid_characters() {
        let result = Field57D::new(vec!["BANK\x00NAME".to_string()]); // Contains null character
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid characters")
        );
    }

    #[test]
    fn test_field57d_add_line_too_many() {
        let mut field = Field57D::new(vec![
            "LINE1".to_string(),
            "LINE2".to_string(),
            "LINE3".to_string(),
            "LINE4".to_string(),
        ])
        .unwrap();
        let result = field.add_line("LINE5");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Cannot exceed 4 lines")
        );
    }

    #[test]
    fn test_field57d_validate() {
        let field = Field57D::from_string("ACCOUNT WITH BANK NAME").unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }
}
