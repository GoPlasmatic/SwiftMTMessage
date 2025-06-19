use crate::{ValidationResult, errors::ParseError};

/// # Multi-Line Field Trait
///
/// ## Overview
/// A trait that provides multi-line field functionality for SWIFT fields that follow
/// the `N*35x` pattern (multiple lines of 35 characters each). This trait allows
/// different field structures to implement their own line limits and additional
/// fields while sharing common multi-line validation and processing logic.
///
/// ## Design Benefits
/// - **Flexible line limits**: Each implementing struct defines its own `MAX_LINES`
/// - **Additional fields**: Structs can have other fields beyond just the lines
/// - **Consistent validation**: Shared validation logic across all multi-line fields
/// - **Type safety**: Compile-time enforcement of line limits
/// - **Extensibility**: Easy to add new multi-line field types
///
/// ## Supported Patterns
/// - **3*35x**: Up to 3 lines (Field 77B - Regulatory Reporting)
/// - **4*35x**: Up to 4 lines (Field 70, Field 50K, Field 59 Basic)
/// - **6*35x**: Up to 6 lines (Field 72 - Sender to Receiver Info)
///
/// ## Implementation Example
/// ```rust
/// use swift_mt_message::{fields::MultiLineField, ParseError};
///
/// #[derive(Debug, Clone)]
/// pub struct Field70 {
///     pub information: Vec<String>,
/// }
///
/// impl MultiLineField for Field70 {
///     const MAX_LINES: usize = 4;
///     const FIELD_TAG: &'static str = "70";
///     
///     fn lines(&self) -> &[String] {
///         &self.information
///     }
///     
///     fn lines_mut(&mut self) -> &mut Vec<String> {
///         &mut self.information
///     }
///     
///     fn new_with_lines(lines: Vec<String>) -> Result<Self, ParseError> {
///         Ok(Field70 { information: lines })
///     }
/// }
/// ```
pub trait MultiLineField: Sized {
    /// Maximum number of lines allowed for this field type
    const MAX_LINES: usize;

    /// Field tag for error messages and identification
    const FIELD_TAG: &'static str;

    /// Get reference to the lines
    fn lines(&self) -> &[String];

    /// Get mutable reference to the lines
    fn lines_mut(&mut self) -> &mut Vec<String>;

    /// Create a new instance with the given lines (after validation)
    fn new_with_lines(lines: Vec<String>) -> Result<Self, ParseError>;

    /// Create a new instance with validation
    fn new(lines: Vec<String>) -> Result<Self, ParseError> {
        Self::validate_lines(&lines)?;
        Self::new_with_lines(lines)
    }

    /// Validate lines according to SWIFT multi-line field rules
    fn validate_lines(lines: &[String]) -> Result<(), ParseError> {
        if lines.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: Self::FIELD_TAG.to_string(),
                message: "Lines cannot be empty".to_string(),
            });
        }

        if lines.len() > Self::MAX_LINES {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: Self::FIELD_TAG.to_string(),
                message: format!("Too many lines (max {})", Self::MAX_LINES),
            });
        }

        for (i, line) in lines.iter().enumerate() {
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: Self::FIELD_TAG.to_string(),
                    message: format!("Line {} cannot be empty or whitespace-only", i + 1),
                });
            }

            if line.len() > 35 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: Self::FIELD_TAG.to_string(),
                    message: format!("Line {} too long (max 35 characters)", i + 1),
                });
            }

            // Validate characters (printable ASCII)
            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: Self::FIELD_TAG.to_string(),
                    message: format!("Line {} contains invalid characters", i + 1),
                });
            }
        }

        Ok(())
    }

    /// Parse content with field tag removal
    fn parse_content(content: &str) -> Result<Self, ParseError> {
        // Remove field tag prefix if present
        let content = if let Some(tag_end) = content.find(':') {
            let after_first_colon = &content[tag_end + 1..];
            if let Some(second_colon) = after_first_colon.find(':') {
                &after_first_colon[second_colon + 1..]
            } else {
                after_first_colon
            }
        } else {
            content
        };

        let lines: Vec<String> = content
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: Self::FIELD_TAG.to_string(),
                message: "Content cannot be empty".to_string(),
            });
        }

        Self::new(lines)
    }

    /// Convert to SWIFT string format
    fn to_swift_format(&self) -> String {
        format!(":{}:{}", Self::FIELD_TAG, self.lines().join("\n"))
    }

    /// Get the number of lines
    fn line_count(&self) -> usize {
        self.lines().len()
    }

    /// Get a specific line by index
    fn line(&self, index: usize) -> Option<&str> {
        self.lines().get(index).map(|s| s.as_str())
    }

    /// Get the first line (often used as primary content)
    fn first_line(&self) -> Option<&str> {
        self.lines().first().map(|s| s.as_str())
    }

    /// Get a single-line representation
    fn to_single_line(&self) -> String {
        self.lines().join(" / ")
    }

    /// Check if the field is at maximum capacity
    fn is_full(&self) -> bool {
        self.lines().len() >= Self::MAX_LINES
    }

    /// Add a line if there's space
    fn add_line(&mut self, line: String) -> Result<(), ParseError> {
        if self.lines().len() >= Self::MAX_LINES {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: Self::FIELD_TAG.to_string(),
                message: format!("Cannot add more lines (max {})", Self::MAX_LINES),
            });
        }

        if line.len() > 35 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: Self::FIELD_TAG.to_string(),
                message: "Line too long (max 35 characters)".to_string(),
            });
        }

        if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: Self::FIELD_TAG.to_string(),
                message: "Line contains invalid characters".to_string(),
            });
        }

        let trimmed_line = line.trim().to_string();
        if trimmed_line.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: Self::FIELD_TAG.to_string(),
                message: "Line cannot be empty or whitespace-only".to_string(),
            });
        }

        self.lines_mut().push(trimmed_line);
        Ok(())
    }

    /// Validate the current state
    fn validate_multiline(&self) -> ValidationResult {
        match Self::validate_lines(self.lines()) {
            Ok(()) => ValidationResult {
                is_valid: true,
                errors: Vec::new(),
                warnings: Vec::new(),
            },
            Err(e) => ValidationResult {
                is_valid: false,
                errors: vec![crate::ValidationError::FormatValidation {
                    field_tag: Self::FIELD_TAG.to_string(),
                    message: e.to_string(),
                }],
                warnings: Vec::new(),
            },
        }
    }

    /// Get the format specification for this multi-line field
    fn multiline_format_spec() -> String {
        format!("{}*35x", Self::MAX_LINES)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftField;
    use serde::{Deserialize, Serialize};

    // Example implementations showing how to use the trait
    /// Example: Field70-style implementation using the MultiLineField trait
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct ExampleRemittanceField {
        pub information: Vec<String>,
    }

    impl MultiLineField for ExampleRemittanceField {
        const MAX_LINES: usize = 4;
        const FIELD_TAG: &'static str = "70";

        fn lines(&self) -> &[String] {
            &self.information
        }

        fn lines_mut(&mut self) -> &mut Vec<String> {
            &mut self.information
        }

        fn new_with_lines(lines: Vec<String>) -> Result<Self, ParseError> {
            Ok(ExampleRemittanceField { information: lines })
        }
    }

    impl SwiftField for ExampleRemittanceField {
        fn parse(content: &str) -> Result<Self, ParseError> {
            Self::parse_content(content)
        }

        fn to_swift_string(&self) -> String {
            self.to_swift_format()
        }

        fn validate(&self) -> ValidationResult {
            self.validate_multiline()
        }

        fn format_spec() -> &'static str {
            "4*35x"
        }
    }

    /// Example: Field77B-style implementation with additional fields
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct ExampleRegulatoryField {
        pub information: Vec<String>,
        pub ordering_country: Option<String>,
        pub beneficiary_country: Option<String>,
    }

    impl MultiLineField for ExampleRegulatoryField {
        const MAX_LINES: usize = 3;
        const FIELD_TAG: &'static str = "77B";

        fn lines(&self) -> &[String] {
            &self.information
        }

        fn lines_mut(&mut self) -> &mut Vec<String> {
            &mut self.information
        }

        fn new_with_lines(lines: Vec<String>) -> Result<Self, ParseError> {
            // Extract country codes from the information lines
            let mut ordering_country = None;
            let mut beneficiary_country = None;

            for line in &lines {
                if line.starts_with("/ORDERRES/") {
                    if let Some(country_start) = line.find("/ORDERRES/").map(|i| i + 10) {
                        if let Some(country_end) = line[country_start..].find('/') {
                            ordering_country =
                                Some(line[country_start..country_start + country_end].to_string());
                        }
                    }
                }
                if line.starts_with("/BENEFRES/") {
                    if let Some(country_start) = line.find("/BENEFRES/").map(|i| i + 10) {
                        if let Some(country_end) = line[country_start..].find('/') {
                            beneficiary_country =
                                Some(line[country_start..country_start + country_end].to_string());
                        }
                    }
                }
            }

            Ok(ExampleRegulatoryField {
                information: lines,
                ordering_country,
                beneficiary_country,
            })
        }
    }

    impl SwiftField for ExampleRegulatoryField {
        fn parse(content: &str) -> Result<Self, ParseError> {
            Self::parse_content(content)
        }

        fn to_swift_string(&self) -> String {
            self.to_swift_format()
        }

        fn validate(&self) -> ValidationResult {
            self.validate_multiline()
        }

        fn format_spec() -> &'static str {
            "3*35x"
        }
    }

    #[test]
    fn test_multiline_trait_remittance_field() {
        let lines = vec![
            "PAYMENT FOR INVOICE 12345".to_string(),
            "CONTRACT REF: SUPPLY-2024".to_string(),
        ];

        let field = ExampleRemittanceField::new(lines.clone()).unwrap();
        assert_eq!(field.lines(), &lines);
        assert_eq!(field.line_count(), 2);
        assert_eq!(ExampleRemittanceField::MAX_LINES, 4);
        assert_eq!(ExampleRemittanceField::FIELD_TAG, "70");
    }

    #[test]
    fn test_multiline_trait_regulatory_field() {
        let lines = vec![
            "/ORDERRES/DE//REGULATORY INFO".to_string(),
            "/BENEFRES/US//COMPLIANCE DATA".to_string(),
            "TRADE RELATED TRANSACTION".to_string(),
        ];

        let field = ExampleRegulatoryField::new(lines.clone()).unwrap();
        assert_eq!(field.lines(), &lines);
        assert_eq!(field.line_count(), 3);
        assert_eq!(field.ordering_country, Some("DE".to_string()));
        assert_eq!(field.beneficiary_country, Some("US".to_string()));
        assert_eq!(ExampleRegulatoryField::MAX_LINES, 3);
        assert_eq!(ExampleRegulatoryField::FIELD_TAG, "77B");
    }

    #[test]
    fn test_multiline_trait_validation() {
        let field = ExampleRemittanceField::new(vec!["Valid line".to_string()]).unwrap();
        let validation = field.validate_multiline();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_multiline_trait_too_many_lines() {
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(),
            "Line 5".to_string(), // Too many for 4-line field
        ];
        let result = ExampleRemittanceField::new(lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiline_trait_line_too_long() {
        let lines = vec!["A".repeat(36)]; // 36 characters, max is 35
        let result = ExampleRemittanceField::new(lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiline_trait_parse_content() {
        let content = ":70:PAYMENT FOR INVOICE 12345\nCONTRACT REF: SUPPLY-2024";
        let field = ExampleRemittanceField::parse_content(content).unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("PAYMENT FOR INVOICE 12345"));
        assert_eq!(field.line(1), Some("CONTRACT REF: SUPPLY-2024"));
    }

    #[test]
    fn test_multiline_trait_to_swift_format() {
        let lines = vec![
            "PAYMENT FOR INVOICE 12345".to_string(),
            "CONTRACT REF: SUPPLY-2024".to_string(),
        ];
        let field = ExampleRemittanceField::new(lines).unwrap();
        let swift_string = field.to_swift_format();
        assert_eq!(
            swift_string,
            ":70:PAYMENT FOR INVOICE 12345\nCONTRACT REF: SUPPLY-2024"
        );
    }

    #[test]
    fn test_multiline_trait_add_line() {
        let lines = vec!["First line".to_string()];
        let mut field = ExampleRemittanceField::new(lines).unwrap();

        field.add_line("Second line".to_string()).unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(1), Some("Second line"));
    }

    #[test]
    fn test_multiline_trait_add_line_when_full() {
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
        ];
        let mut field = ExampleRegulatoryField::new(lines).unwrap();

        let result = field.add_line("Line 4".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_multiline_trait_format_spec() {
        assert_eq!(ExampleRemittanceField::multiline_format_spec(), "4*35x");
        assert_eq!(ExampleRegulatoryField::multiline_format_spec(), "3*35x");
    }

    #[test]
    fn test_multiline_trait_single_line_representation() {
        let lines = vec![
            "PAYMENT FOR INVOICE 12345".to_string(),
            "CONTRACT REF: SUPPLY-2024".to_string(),
        ];
        let field = ExampleRemittanceField::new(lines).unwrap();
        assert_eq!(
            field.to_single_line(),
            "PAYMENT FOR INVOICE 12345 / CONTRACT REF: SUPPLY-2024"
        );
    }
}
