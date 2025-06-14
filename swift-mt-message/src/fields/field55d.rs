use crate::{SwiftField, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 55D: Third Reimbursement Institution (Option D)
///
/// ## Overview
/// Field 55D identifies the third reimbursement institution using name and address
/// information rather than a BIC code or party identifier. This option provides the
/// most detailed identification method for the third institution in the reimbursement
/// chain and is used when full institutional details are required for regulatory
/// compliance, routing, or when other identification methods are not available or sufficient.
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
/// Field 55D is used in:
/// - **MT202**: General Financial Institution Transfer (when 55A/55B not applicable)
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
/// - **MT103**: Single Customer Credit Transfer (in complex routing)
/// - **MT200**: Financial Institution Transfer
///
/// ### Business Applications
/// - **Non-SWIFT institutions**: Identifying institutions without BIC codes
/// - **Regulatory compliance**: Providing complete address for compliance screening
/// - **Small institutions**: Local banks, credit unions, or regional institutions
/// - **Enhanced due diligence**: Meeting KYC requirements for reimbursement details
/// - **Sanctions screening**: Enabling comprehensive name/address verification
/// - **Audit trails**: Maintaining detailed third institution records
/// - **Complex routing**: Facilitating multi-hop reimbursement arrangements
/// - **Regional networks**: Supporting regional correspondent relationships
///
/// ## Examples
/// ```text
/// :55D:THIRD REIMBURSEMENT BANK
/// 456 REIMBURSEMENT STREET
/// LONDON EC1A 1BB
/// UNITED KINGDOM
/// └─── UK third reimbursement bank with full address
///
/// :55D:BANQUE REMBOURSEMENT TIERS
/// 789 RUE DU COMMERCE
/// PARIS 75002 FRANCE
/// └─── French third reimbursement bank (3 lines)
///
/// :55D:COMMUNITY REIMBURSEMENT BANK
/// 321 SETTLEMENT AVENUE
/// CHICAGO IL 60601
/// └─── US community bank (minimal address)
///
/// :55D:BANCO REEMBOLSO TERCERO
/// OFICINA PRINCIPAL
/// CALLE MAYOR 456
/// BARCELONA 08001 SPAIN
/// └─── Spanish third reimbursement bank with detailed address
/// ```
///
/// ## Address Format Guidelines
/// ### Line 1: Institution Name (Required)
/// - Full legal name of the third reimbursement institution
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
/// - Required for international reimbursement relationships
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
/// ## Reimbursement Chain Context
/// In multi-institution reimbursement chains:
/// - **Field 53A/B/D**: Sender's correspondent (first institution)
/// - **Field 54A/B/D**: Receiver's correspondent (second institution)
/// - **Field 55A/B/D**: Third reimbursement institution (third institution)
/// - **Field 56A/C/D**: Intermediary institution (fourth institution)
/// - **Field 57A/B/C/D**: Account with institution (final institution)
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
/// - Minimum 1 line, maximum 4 lines allowed (Error: C55)
/// - Each line cannot exceed 35 characters (Error: T14)
/// - Characters must be from SWIFT character set (Error: T61)
/// - Lines cannot be empty (Error: T11)
/// - Must contain institution name in first line (Error: C56)
/// - Field 55D alternative to 55A/55B (Error: C55)
/// - Address should be verifiable institution address (Error: C57)
/// - Institution must be in reimbursement chain (Error: C58)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field55D {
    /// Name and address lines (up to 4 lines of 35 characters each)
    pub name_and_address: Vec<String>,
}

impl Field55D {
    /// Create a new Field55D with validation
    pub fn new(name_and_address: Vec<String>) -> Result<Self, crate::ParseError> {
        if name_and_address.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "55D".to_string(),
                message: "Name and address cannot be empty".to_string(),
            });
        }

        if name_and_address.len() > 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "55D".to_string(),
                message: "Too many name/address lines (max 4)".to_string(),
            });
        }

        for (i, line) in name_and_address.iter().enumerate() {
            if line.len() > 35 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "55D".to_string(),
                    message: format!("Line {} too long (max 35 characters)", i + 1),
                });
            }

            // Validate characters (printable ASCII)
            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "55D".to_string(),
                    message: format!("Line {} contains invalid characters", i + 1),
                });
            }
        }

        Ok(Field55D { name_and_address })
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
                field_tag: "55D".to_string(),
                message: "Cannot add more lines (max 4)".to_string(),
            });
        }

        if line.len() > 35 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "55D".to_string(),
                message: "Line too long (max 35 characters)".to_string(),
            });
        }

        if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "55D".to_string(),
                message: "Line contains invalid characters".to_string(),
            });
        }

        self.name_and_address.push(line);
        Ok(())
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!(
            "Third Reimbursement Institution ({} lines)",
            self.line_count()
        )
    }
}

impl SwiftField for Field55D {
    fn parse(content: &str) -> crate::Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "55D".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let content = if let Some(stripped) = content.strip_prefix(":55D:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("55D:") {
            stripped
        } else {
            content
        };

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        Field55D::new(lines)
    }

    fn to_swift_string(&self) -> String {
        format!(":55D:{}", self.name_and_address.join("\n"))
    }

    fn validate(&self) -> ValidationResult {
        use crate::errors::ValidationError;

        let mut errors = Vec::new();

        if self.name_and_address.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "55D".to_string(),
                message: "Name and address cannot be empty".to_string(),
            });
        }

        if self.name_and_address.len() > 4 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "55D".to_string(),
                expected: "max 4 lines".to_string(),
                actual: self.name_and_address.len(),
            });
        }

        for (i, line) in self.name_and_address.iter().enumerate() {
            if line.len() > 35 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "55D".to_string(),
                    expected: format!("max 35 characters for line {}", i + 1),
                    actual: line.len(),
                });
            }

            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "55D".to_string(),
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

impl std::fmt::Display for Field55D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name_and_address.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field55d_creation() {
        let lines = vec![
            "THIRD REIMBURSEMENT BANK".to_string(),
            "456 REIMBURSEMENT STREET".to_string(),
            "LONDON EC1A 1BB".to_string(),
            "UNITED KINGDOM".to_string(),
        ];
        let field = Field55D::new(lines.clone()).unwrap();
        assert_eq!(field.name_and_address(), &lines);
        assert_eq!(field.line_count(), 4);
        assert_eq!(field.line(0), Some("THIRD REIMBURSEMENT BANK"));
        assert_eq!(field.line(1), Some("456 REIMBURSEMENT STREET"));
        assert_eq!(field.line(2), Some("LONDON EC1A 1BB"));
        assert_eq!(field.line(3), Some("UNITED KINGDOM"));
        assert_eq!(field.line(4), None);
    }

    #[test]
    fn test_field55d_creation_single_line() {
        let lines = vec!["THIRD REIMBURSEMENT BANK".to_string()];
        let field = Field55D::new(lines.clone()).unwrap();
        assert_eq!(field.name_and_address(), &lines);
        assert_eq!(field.line_count(), 1);
    }

    #[test]
    fn test_field55d_from_string() {
        let content =
            "THIRD REIMBURSEMENT BANK\n456 REIMBURSEMENT STREET\nLONDON EC1A 1BB\nUNITED KINGDOM";
        let field = Field55D::from_string(content).unwrap();
        assert_eq!(field.line_count(), 4);
        assert_eq!(field.line(0), Some("THIRD REIMBURSEMENT BANK"));
        assert_eq!(field.line(1), Some("456 REIMBURSEMENT STREET"));
        assert_eq!(field.line(2), Some("LONDON EC1A 1BB"));
        assert_eq!(field.line(3), Some("UNITED KINGDOM"));
    }

    #[test]
    fn test_field55d_parse() {
        let field = Field55D::parse("THIRD REIMBURSEMENT BANK\n456 REIMBURSEMENT STREET").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("THIRD REIMBURSEMENT BANK"));
        assert_eq!(field.line(1), Some("456 REIMBURSEMENT STREET"));
    }

    #[test]
    fn test_field55d_parse_with_tag() {
        let field =
            Field55D::parse(":55D:THIRD REIMBURSEMENT BANK\n456 REIMBURSEMENT STREET").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("THIRD REIMBURSEMENT BANK"));
        assert_eq!(field.line(1), Some("456 REIMBURSEMENT STREET"));
    }

    #[test]
    fn test_field55d_to_swift_string() {
        let lines = vec![
            "THIRD REIMBURSEMENT BANK".to_string(),
            "456 REIMBURSEMENT STREET".to_string(),
        ];
        let field = Field55D::new(lines).unwrap();
        assert_eq!(
            field.to_swift_string(),
            ":55D:THIRD REIMBURSEMENT BANK\n456 REIMBURSEMENT STREET"
        );
    }

    #[test]
    fn test_field55d_display() {
        let lines = vec![
            "THIRD REIMBURSEMENT BANK".to_string(),
            "456 REIMBURSEMENT STREET".to_string(),
        ];
        let field = Field55D::new(lines).unwrap();
        assert_eq!(
            format!("{}", field),
            "THIRD REIMBURSEMENT BANK\n456 REIMBURSEMENT STREET"
        );
    }

    #[test]
    fn test_field55d_description() {
        let lines = vec![
            "THIRD REIMBURSEMENT BANK".to_string(),
            "456 REIMBURSEMENT STREET".to_string(),
        ];
        let field = Field55D::new(lines).unwrap();
        assert_eq!(
            field.description(),
            "Third Reimbursement Institution (2 lines)"
        );
    }

    #[test]
    fn test_field55d_add_line() {
        let lines = vec!["THIRD REIMBURSEMENT BANK".to_string()];
        let mut field = Field55D::new(lines).unwrap();

        field
            .add_line("456 REIMBURSEMENT STREET".to_string())
            .unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(1), Some("456 REIMBURSEMENT STREET"));

        field.add_line("LONDON EC1A 1BB".to_string()).unwrap();
        assert_eq!(field.line_count(), 3);

        field.add_line("UNITED KINGDOM".to_string()).unwrap();
        assert_eq!(field.line_count(), 4);

        // Should fail when trying to add 5th line
        let result = field.add_line("TOO MANY LINES".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_field55d_validation_empty() {
        let result = Field55D::new(vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_field55d_validation_too_many_lines() {
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(),
            "Line 5".to_string(), // Too many
        ];
        let result = Field55D::new(lines);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("max 4"));
    }

    #[test]
    fn test_field55d_validation_line_too_long() {
        let lines = vec!["A".repeat(36)]; // 36 characters, max is 35
        let result = Field55D::new(lines);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too long"));
    }

    #[test]
    fn test_field55d_validation_invalid_characters() {
        let lines = vec!["THIRD REIMBURSEMENT BANK\x00".to_string()]; // Contains null character
        let result = Field55D::new(lines);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid characters")
        );
    }

    #[test]
    fn test_field55d_validate() {
        let lines = vec![
            "THIRD REIMBURSEMENT BANK".to_string(),
            "456 REIMBURSEMENT STREET".to_string(),
        ];
        let field = Field55D::new(lines).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field55d_validate_errors() {
        let lines = vec!["A".repeat(36)]; // Line too long
        let field = Field55D {
            name_and_address: lines,
        };
        let validation = field.validate();
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
    }
}
