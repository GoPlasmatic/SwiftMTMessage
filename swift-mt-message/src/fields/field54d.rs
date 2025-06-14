use crate::{SwiftField, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 54D: Receiver's Correspondent (Option D)
///
/// ## Overview
/// Field 54D identifies the receiver's correspondent institution using name and address
/// information rather than a BIC code or party identifier. This option provides the
/// most detailed identification method for receiver's correspondents and is used when
/// full institutional details are required for regulatory compliance, routing, or when
/// other identification methods are not available or sufficient.
///
/// ## Format Specification
/// **Format**: `4*35x`
/// - **4*35x**: Up to 4 lines of name and address information
/// - **Line length**: Maximum 35 characters per line
/// - **Character set**: SWIFT character set (printable ASCII)
/// - **Content**: Institution name, street address, city, postal code, country
///
/// ## Structure
/// ```text
/// Line 1: Institution Name (required)
/// Line 2: Street Address/Building Number
/// Line 3: City, State/Province, Postal Code
/// Line 4: Country (recommended for international)
/// ```
///
/// ## Usage Context
/// Field 54D is used in:
/// - **MT103**: Single Customer Credit Transfer (when 54A/54B not applicable)
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Non-SWIFT institutions**: Identifying institutions without BIC codes
/// - **Regulatory compliance**: Providing complete address for compliance screening
/// - **Small institutions**: Local banks, credit unions, or regional institutions
/// - **Enhanced due diligence**: Meeting KYC requirements for correspondent details
/// - **Sanctions screening**: Enabling comprehensive name/address verification
/// - **Audit trails**: Maintaining detailed receiver's correspondent records
/// - **Cross-border routing**: Facilitating international payment routing
///
/// ## Examples
/// ```text
/// :54D:RECEIVER CORRESPONDENT BANK
/// 789 BANKING BOULEVARD
/// SINGAPORE 049910
/// SINGAPORE
/// └─── Singapore correspondent bank with full address
///
/// :54D:BANQUE CORRESPONDANTE LOCALE
/// 123 RUE DES FINANCES
/// PARIS 75001 FRANCE
/// └─── French correspondent bank (3 lines)
///
/// :54D:COMMUNITY CORRESPONDENT BANK
/// 456 CORRESPONDENT AVENUE
/// MIDDLETOWN CA 90210
/// └─── US community bank (minimal address)
///
/// :54D:BANCO CORRESPONSAL REGIONAL
/// SUCURSAL PRINCIPAL
/// AVENIDA LIBERTAD 789
/// MADRID 28001 SPAIN
/// └─── Spanish correspondent with detailed address
/// ```
///
/// ## Address Format Guidelines
/// ### Line 1: Institution Name (Required)
/// - Full legal name of the receiver's correspondent institution
/// - Include organizational form (Bank, Credit Union, Trust, etc.)
/// - Avoid abbreviations when possible
/// - Maximum 35 characters
///
/// ### Line 2: Street Address (Recommended)
/// - Building number and street name
/// - Suite/floor information if applicable
/// - PO Box if street address not available
/// - Maximum 35 characters
///
/// ### Line 3: City and Postal Information (Recommended)
/// - City name, state/province abbreviation
/// - Postal code or ZIP code
/// - Administrative district if required
/// - Maximum 35 characters
///
/// ### Line 4: Country (Optional but Recommended)
/// - Full country name (preferred) or ISO code
/// - Required for international correspondent relationships
/// - Helps with routing and compliance screening
/// - Maximum 35 characters
///
/// ## Address Standards
/// - Use standard postal abbreviations for states/provinces
/// - Include postal/ZIP codes when available
/// - Spell out country names in full when possible
/// - Avoid special characters and diacritical marks
/// - Follow local address formatting conventions
/// - Ensure consistency with official institution records
///
/// ## Validation Rules
/// 1. **Minimum content**: At least 1 line required
/// 2. **Maximum lines**: No more than 4 lines allowed
/// 3. **Line length**: Each line maximum 35 characters
/// 4. **Character validation**: Only printable ASCII characters
/// 5. **Content requirement**: Must contain meaningful institution information
/// 6. **Line ordering**: Institution name should be in first line
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Minimum 1 line, maximum 4 lines allowed (Error: C54)
/// - Each line cannot exceed 35 characters (Error: T14)
/// - Characters must be from SWIFT character set (Error: T61)
/// - Lines cannot be empty (Error: T11)
/// - Must contain institution name in first line (Error: C55)
/// - Field 54D alternative to 54A/54B (Error: C54)
/// - Address should be verifiable institution address (Error: C56)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field54D {
    /// Name and address lines (up to 4 lines of 35 characters each)
    pub name_and_address: Vec<String>,
}

impl Field54D {
    /// Create a new Field54D with validation
    pub fn new(name_and_address: Vec<String>) -> Result<Self, crate::ParseError> {
        if name_and_address.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54D".to_string(),
                message: "Name and address cannot be empty".to_string(),
            });
        }

        if name_and_address.len() > 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54D".to_string(),
                message: "Too many name/address lines (max 4)".to_string(),
            });
        }

        for (i, line) in name_and_address.iter().enumerate() {
            if line.len() > 35 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54D".to_string(),
                    message: format!("Line {} too long (max 35 characters)", i + 1),
                });
            }

            // Validate characters (printable ASCII)
            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54D".to_string(),
                    message: format!("Line {} contains invalid characters", i + 1),
                });
            }
        }

        Ok(Field54D { name_and_address })
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
                field_tag: "54D".to_string(),
                message: "Cannot add more lines (max 4)".to_string(),
            });
        }

        if line.len() > 35 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54D".to_string(),
                message: "Line too long (max 35 characters)".to_string(),
            });
        }

        if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54D".to_string(),
                message: "Line contains invalid characters".to_string(),
            });
        }

        self.name_and_address.push(line);
        Ok(())
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!("Receiver's Correspondent ({} lines)", self.line_count())
    }
}

impl SwiftField for Field54D {
    fn parse(content: &str) -> crate::Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54D".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let content = if let Some(stripped) = content.strip_prefix(":54D:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("54D:") {
            stripped
        } else {
            content
        };

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        Field54D::new(lines)
    }

    fn to_swift_string(&self) -> String {
        format!(":54D:{}", self.name_and_address.join("\n"))
    }

    fn validate(&self) -> ValidationResult {
        use crate::errors::ValidationError;

        let mut errors = Vec::new();

        if self.name_and_address.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "54D".to_string(),
                message: "Name and address cannot be empty".to_string(),
            });
        }

        if self.name_and_address.len() > 4 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "54D".to_string(),
                expected: "max 4 lines".to_string(),
                actual: self.name_and_address.len(),
            });
        }

        for (i, line) in self.name_and_address.iter().enumerate() {
            if line.len() > 35 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "54D".to_string(),
                    expected: format!("max 35 characters for line {}", i + 1),
                    actual: line.len(),
                });
            }

            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "54D".to_string(),
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

impl std::fmt::Display for Field54D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name_and_address.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field54d_creation() {
        let lines = vec![
            "RECEIVER CORRESPONDENT BANK".to_string(),
            "789 BANKING BOULEVARD".to_string(),
            "SINGAPORE 049910".to_string(),
            "SINGAPORE".to_string(),
        ];
        let field = Field54D::new(lines.clone()).unwrap();
        assert_eq!(field.name_and_address(), &lines);
        assert_eq!(field.line_count(), 4);
        assert_eq!(field.line(0), Some("RECEIVER CORRESPONDENT BANK"));
        assert_eq!(field.line(1), Some("789 BANKING BOULEVARD"));
        assert_eq!(field.line(2), Some("SINGAPORE 049910"));
        assert_eq!(field.line(3), Some("SINGAPORE"));
        assert_eq!(field.line(4), None);
    }

    #[test]
    fn test_field54d_creation_single_line() {
        let lines = vec!["RECEIVER CORRESPONDENT BANK".to_string()];
        let field = Field54D::new(lines.clone()).unwrap();
        assert_eq!(field.name_and_address(), &lines);
        assert_eq!(field.line_count(), 1);
    }

    #[test]
    fn test_field54d_from_string() {
        let content =
            "RECEIVER CORRESPONDENT BANK\n789 BANKING BOULEVARD\nSINGAPORE 049910\nSINGAPORE";
        let field = Field54D::from_string(content).unwrap();
        assert_eq!(field.line_count(), 4);
        assert_eq!(field.line(0), Some("RECEIVER CORRESPONDENT BANK"));
        assert_eq!(field.line(1), Some("789 BANKING BOULEVARD"));
        assert_eq!(field.line(2), Some("SINGAPORE 049910"));
        assert_eq!(field.line(3), Some("SINGAPORE"));
    }

    #[test]
    fn test_field54d_parse() {
        let field = Field54D::parse("RECEIVER CORRESPONDENT BANK\n789 BANKING BOULEVARD").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("RECEIVER CORRESPONDENT BANK"));
        assert_eq!(field.line(1), Some("789 BANKING BOULEVARD"));
    }

    #[test]
    fn test_field54d_parse_with_tag() {
        let field =
            Field54D::parse(":54D:RECEIVER CORRESPONDENT BANK\n789 BANKING BOULEVARD").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("RECEIVER CORRESPONDENT BANK"));
        assert_eq!(field.line(1), Some("789 BANKING BOULEVARD"));
    }

    #[test]
    fn test_field54d_to_swift_string() {
        let lines = vec![
            "RECEIVER CORRESPONDENT BANK".to_string(),
            "789 BANKING BOULEVARD".to_string(),
        ];
        let field = Field54D::new(lines).unwrap();
        assert_eq!(
            field.to_swift_string(),
            ":54D:RECEIVER CORRESPONDENT BANK\n789 BANKING BOULEVARD"
        );
    }

    #[test]
    fn test_field54d_display() {
        let lines = vec![
            "RECEIVER CORRESPONDENT BANK".to_string(),
            "789 BANKING BOULEVARD".to_string(),
        ];
        let field = Field54D::new(lines).unwrap();
        assert_eq!(
            format!("{}", field),
            "RECEIVER CORRESPONDENT BANK\n789 BANKING BOULEVARD"
        );
    }

    #[test]
    fn test_field54d_description() {
        let lines = vec![
            "RECEIVER CORRESPONDENT BANK".to_string(),
            "789 BANKING BOULEVARD".to_string(),
        ];
        let field = Field54D::new(lines).unwrap();
        assert_eq!(field.description(), "Receiver's Correspondent (2 lines)");
    }

    #[test]
    fn test_field54d_add_line() {
        let lines = vec!["RECEIVER CORRESPONDENT BANK".to_string()];
        let mut field = Field54D::new(lines).unwrap();

        field.add_line("789 BANKING BOULEVARD".to_string()).unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(1), Some("789 BANKING BOULEVARD"));

        field.add_line("SINGAPORE 049910".to_string()).unwrap();
        assert_eq!(field.line_count(), 3);

        field.add_line("SINGAPORE".to_string()).unwrap();
        assert_eq!(field.line_count(), 4);

        // Should fail when trying to add 5th line
        let result = field.add_line("TOO MANY LINES".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_field54d_validation_empty() {
        let result = Field54D::new(vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_field54d_validation_too_many_lines() {
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(),
            "Line 5".to_string(), // Too many
        ];
        let result = Field54D::new(lines);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("max 4"));
    }

    #[test]
    fn test_field54d_validation_line_too_long() {
        let lines = vec!["A".repeat(36)]; // 36 characters, max is 35
        let result = Field54D::new(lines);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too long"));
    }

    #[test]
    fn test_field54d_validation_invalid_characters() {
        let lines = vec!["RECEIVER CORRESPONDENT BANK\x00".to_string()]; // Contains null character
        let result = Field54D::new(lines);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid characters")
        );
    }

    #[test]
    fn test_field54d_validate() {
        let lines = vec![
            "RECEIVER CORRESPONDENT BANK".to_string(),
            "789 BANKING BOULEVARD".to_string(),
        ];
        let field = Field54D::new(lines).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field54d_validate_errors() {
        let lines = vec!["A".repeat(36)]; // Line too long
        let field = Field54D {
            name_and_address: lines,
        };
        let validation = field.validate();
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
    }
}
