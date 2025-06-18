use crate::{SwiftField, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Generic Name Address Field
///
/// ## Overview
/// A generic field structure for SWIFT name and address fields that follow the
/// `4*35x` pattern (up to 4 lines of 35 characters each). This structure
/// consolidates the common functionality used by Field52D, Field53D, Field54D, and Field55D.
///
/// ## Format Specification
/// **Format**: `4*35x`
/// - **4***: Up to 4 lines
/// - **35x**: Each line up to 35 characters
///
/// ### Component Details
/// 1. **Name and Address Lines**:
///    - Up to 4 lines of text
///    - Each line maximum 35 characters
///    - Must contain at least one line
///    - Typically first line is name, subsequent lines are address
///    - All lines must be printable ASCII characters
///
/// ## Usage Context
/// Used in various SWIFT MT message types for institutional identification:
/// - **Field 52D**: Ordering Institution (Name/Address)
/// - **Field 53D**: Sender's Correspondent (Name/Address)
/// - **Field 54D**: Receiver's Correspondent (Name/Address)
/// - **Field 55D**: Third Reimbursement Institution (Name/Address)
///
/// ## Usage Examples
/// ```text
/// DEUTSCHE BANK AG
/// TAUNUSANLAGE 12
/// 60325 FRANKFURT AM MAIN
/// GERMANY
/// └─── Full 4-line name and address
///
/// JPMORGAN CHASE BANK N.A.
/// NEW YORK
/// └─── 2-line name and address
///
/// BANK OF AMERICA
/// └─── Single line name only
/// ```
///
/// ## Validation Rules
/// 1. **Line count**: Must have at least 1 line, maximum 4 lines
/// 2. **Line length**: Each line maximum 35 characters
/// 3. **Character validation**: All characters must be printable ASCII
/// 4. **Content requirement**: Must contain meaningful institution information
/// 5. **Empty lines**: Lines cannot be empty or whitespace-only
///
/// ## Network Validated Rules (SWIFT Standards)
/// - At least one line must be present (Error: T26)
/// - Each line cannot exceed 35 characters (Error: T50)
/// - Maximum 4 lines allowed (Error: T26)
/// - Characters must be from SWIFT character set (Error: T61)
/// - Lines cannot be empty (Error: T26)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenericNameAddressField {
    /// Name and address lines (up to 4 lines, 35 characters each)
    pub name_and_address: Vec<String>,
}

impl GenericNameAddressField {
    /// Create a new GenericNameAddressField with validation
    ///
    /// # Arguments
    /// * `name_and_address` - Vector of name and address lines (1-4 lines, max 35 chars each)
    ///
    /// # Returns
    /// Result containing the GenericNameAddressField instance or validation error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::GenericNameAddressField;
    /// let field = GenericNameAddressField::new(vec![
    ///     "DEUTSCHE BANK AG".to_string(),
    ///     "TAUNUSANLAGE 12".to_string(),
    ///     "60325 FRANKFURT AM MAIN".to_string(),
    ///     "GERMANY".to_string(),
    /// ]).unwrap();
    /// ```
    pub fn new(name_and_address: Vec<String>) -> Result<Self, ParseError> {
        if name_and_address.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericNameAddressField".to_string(),
                message: "Name and address cannot be empty".to_string(),
            });
        }

        if name_and_address.len() > 4 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericNameAddressField".to_string(),
                message: "Too many name/address lines (max 4)".to_string(),
            });
        }

        for (i, line) in name_and_address.iter().enumerate() {
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "GenericNameAddressField".to_string(),
                    message: format!("Line {} cannot be empty or whitespace-only", i + 1),
                });
            }

            if line.len() > 35 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "GenericNameAddressField".to_string(),
                    message: format!("Line {} too long (max 35 characters)", i + 1),
                });
            }

            // Validate characters (printable ASCII)
            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "GenericNameAddressField".to_string(),
                    message: format!("Line {} contains invalid characters", i + 1),
                });
            }
        }

        // Trim all lines but preserve the original structure
        let trimmed_lines: Vec<String> = name_and_address
            .iter()
            .map(|line| line.trim().to_string())
            .collect();

        Ok(GenericNameAddressField {
            name_and_address: trimmed_lines,
        })
    }

    /// Create from a single string, splitting on newlines
    ///
    /// # Arguments
    /// * `content` - Multi-line string to split into name and address lines
    ///
    /// # Returns
    /// Result containing the GenericNameAddressField instance or validation error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::GenericNameAddressField;
    /// let field = GenericNameAddressField::from_string(
    ///     "DEUTSCHE BANK AG\nTAUNUSANLAGE 12\n60325 FRANKFURT AM MAIN\nGERMANY"
    /// ).unwrap();
    /// ```
    pub fn from_string(content: impl Into<String>) -> Result<Self, ParseError> {
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

    /// Get a specific line by index (0-based)
    pub fn line(&self, index: usize) -> Option<&str> {
        self.name_and_address.get(index).map(|s| s.as_str())
    }

    /// Get the first line (typically the institution name)
    pub fn name(&self) -> Option<&str> {
        self.line(0)
    }

    /// Get address lines (all lines except the first)
    pub fn address_lines(&self) -> &[String] {
        if self.name_and_address.len() > 1 {
            &self.name_and_address[1..]
        } else {
            &[]
        }
    }

    /// Parse content with custom field tag for error messages
    pub fn parse_with_tag(content: &str, field_tag: &str) -> Result<Self, ParseError> {
        let content = content.trim();
        if content.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: field_tag.to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Remove field tag prefix if present
        let content = if let Some(stripped) = content.strip_prefix(&format!(":{}:", field_tag)) {
            stripped
        } else if let Some(stripped) = content.strip_prefix(&format!("{}:", field_tag)) {
            stripped
        } else {
            content
        };

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        // Create with validation but update error field tags
        match Self::new(lines) {
            Ok(field) => Ok(field),
            Err(ParseError::InvalidFieldFormat {
                field_tag: _,
                message,
            }) => Err(ParseError::InvalidFieldFormat {
                field_tag: field_tag.to_string(),
                message,
            }),
            Err(e) => Err(e),
        }
    }

    /// Convert to SWIFT string format with custom field tag
    pub fn to_swift_string_with_tag(&self, field_tag: &str) -> String {
        let content = self.name_and_address.join("\n");
        format!(":{}:{}", field_tag, content)
    }

    /// Get human-readable description with custom context
    pub fn description(&self, context: &str) -> String {
        let name = self.name().unwrap_or("Unknown");
        let line_count = self.line_count();
        format!(
            "{} ({}, {} line{})",
            context,
            name,
            line_count,
            if line_count == 1 { "" } else { "s" }
        )
    }

    /// Check if this appears to be a valid institution name
    pub fn is_valid_institution_name(&self) -> bool {
        if let Some(name) = self.name() {
            // Basic heuristics for institution names
            name.len() >= 3
                && name.chars().any(|c| c.is_alphabetic())
                && !name.chars().all(|c| c.is_numeric())
        } else {
            false
        }
    }

    /// Get a single-line representation
    pub fn to_single_line(&self) -> String {
        self.name_and_address.join(", ")
    }
}

impl SwiftField for GenericNameAddressField {
    fn parse(content: &str) -> Result<Self, ParseError> {
        Self::parse_with_tag(content, "GenericNameAddressField")
    }

    fn to_swift_string(&self) -> String {
        self.to_swift_string_with_tag("GenericNameAddressField")
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

impl std::fmt::Display for GenericNameAddressField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name_and_address.join(" / "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_name_address_field_creation_single_line() {
        let field = GenericNameAddressField::new(vec!["DEUTSCHE BANK AG".to_string()]).unwrap();
        assert_eq!(field.line_count(), 1);
        assert_eq!(field.name(), Some("DEUTSCHE BANK AG"));
        assert_eq!(field.address_lines().len(), 0);
    }

    #[test]
    fn test_generic_name_address_field_creation_multiple_lines() {
        let field = GenericNameAddressField::new(vec![
            "DEUTSCHE BANK AG".to_string(),
            "TAUNUSANLAGE 12".to_string(),
            "60325 FRANKFURT AM MAIN".to_string(),
            "GERMANY".to_string(),
        ])
        .unwrap();
        assert_eq!(field.line_count(), 4);
        assert_eq!(field.name(), Some("DEUTSCHE BANK AG"));
        assert_eq!(field.address_lines().len(), 3);
        assert_eq!(field.line(1), Some("TAUNUSANLAGE 12"));
    }

    #[test]
    fn test_generic_name_address_field_from_string() {
        let field = GenericNameAddressField::from_string(
            "DEUTSCHE BANK AG\nTAUNUSANLAGE 12\n60325 FRANKFURT AM MAIN\nGERMANY",
        )
        .unwrap();
        assert_eq!(field.line_count(), 4);
        assert_eq!(field.name(), Some("DEUTSCHE BANK AG"));
    }

    #[test]
    fn test_generic_name_address_field_parse_with_tag() {
        let field = GenericNameAddressField::parse_with_tag(
            ":52D:DEUTSCHE BANK AG\nTAUNUSANLAGE 12",
            "52D",
        )
        .unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.name(), Some("DEUTSCHE BANK AG"));
    }

    #[test]
    fn test_generic_name_address_field_to_swift_string_with_tag() {
        let field = GenericNameAddressField::new(vec![
            "DEUTSCHE BANK AG".to_string(),
            "TAUNUSANLAGE 12".to_string(),
        ])
        .unwrap();
        assert_eq!(
            field.to_swift_string_with_tag("52D"),
            ":52D:DEUTSCHE BANK AG\nTAUNUSANLAGE 12"
        );
    }

    #[test]
    fn test_generic_name_address_field_validation_errors() {
        // Empty lines
        let result = GenericNameAddressField::new(vec![]);
        assert!(result.is_err());

        // Too many lines
        let result = GenericNameAddressField::new(vec![
            "LINE1".to_string(),
            "LINE2".to_string(),
            "LINE3".to_string(),
            "LINE4".to_string(),
            "LINE5".to_string(), // Too many
        ]);
        assert!(result.is_err());

        // Line too long
        let result = GenericNameAddressField::new(vec!["A".repeat(36)]);
        assert!(result.is_err());

        // Empty line
        let result = GenericNameAddressField::new(vec!["".to_string()]);
        assert!(result.is_err());

        // Whitespace-only line
        let result = GenericNameAddressField::new(vec!["   ".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_generic_name_address_field_display() {
        let field = GenericNameAddressField::new(vec![
            "DEUTSCHE BANK AG".to_string(),
            "TAUNUSANLAGE 12".to_string(),
        ])
        .unwrap();
        assert_eq!(format!("{}", field), "DEUTSCHE BANK AG / TAUNUSANLAGE 12");
    }

    #[test]
    fn test_generic_name_address_field_description() {
        let field = GenericNameAddressField::new(vec!["DEUTSCHE BANK AG".to_string()]).unwrap();
        assert_eq!(
            field.description("Test Institution"),
            "Test Institution (DEUTSCHE BANK AG, 1 line)"
        );
    }

    #[test]
    fn test_generic_name_address_field_is_valid_institution_name() {
        let valid_field =
            GenericNameAddressField::new(vec!["DEUTSCHE BANK AG".to_string()]).unwrap();
        assert!(valid_field.is_valid_institution_name());

        let invalid_field = GenericNameAddressField::new(vec!["123456789".to_string()]).unwrap();
        assert!(!invalid_field.is_valid_institution_name());
    }

    #[test]
    fn test_generic_name_address_field_to_single_line() {
        let field = GenericNameAddressField::new(vec![
            "DEUTSCHE BANK AG".to_string(),
            "TAUNUSANLAGE 12".to_string(),
            "FRANKFURT".to_string(),
        ])
        .unwrap();
        assert_eq!(
            field.to_single_line(),
            "DEUTSCHE BANK AG, TAUNUSANLAGE 12, FRANKFURT"
        );
    }
}
