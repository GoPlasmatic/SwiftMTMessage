use crate::fields::common::MultiLineField;
use crate::{SwiftField, ValidationError, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Field 86 - Information to Account Owner
///
/// ## Overview
/// Field 86 provides additional information to the account owner regarding
/// transactions or account status. It supports multiple lines of text and
/// is commonly used for transaction descriptions, references, and supplementary
/// details in statement messages.
///
/// ## Format Specification
/// **Format**: `6*65x`
/// - **6***: Up to 6 lines of text
/// - **65x**: Each line can contain up to 65 characters
///
/// ## Usage Context
/// Used in MT940 (Customer Statement Message) and MT942 (Interim Transaction Report) for:
/// - **Transaction descriptions**: Detailed information about transactions
/// - **Reference information**: Additional reference codes or identifiers
/// - **Supplementary details**: Extra context for account holders
/// - **Structured information**: Formatted data using sub-fields
///
/// ## Usage Examples
/// ```text
/// TRANSFER FROM SAVINGS ACCOUNT
/// REF: TXN123456789
/// BENEFICIARY: JOHN DOE
/// ```
///
/// ## Validation Rules
/// 1. **Line count**: Maximum 6 lines
/// 2. **Line length**: Each line maximum 65 characters
/// 3. **Character set**: SWIFT character set (A-Z, 0-9, space, and specific symbols)
/// 4. **Empty lines**: Not allowed between content lines
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Maximum 6 lines allowed (Error: T26)
/// - Each line maximum 65 characters (Error: T26)
/// - Valid SWIFT character set only (Error: T27)
/// - No empty lines between content (Error: T26)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field86 {
    /// Lines of information text
    pub information: Vec<String>,
}

impl MultiLineField for Field86 {
    const MAX_LINES: usize = 6;
    const FIELD_TAG: &'static str = "86";

    fn lines(&self) -> &[String] {
        &self.information
    }

    fn lines_mut(&mut self) -> &mut Vec<String> {
        &mut self.information
    }

    fn new_with_lines(lines: Vec<String>) -> Result<Self, ParseError> {
        Ok(Field86 { information: lines })
    }
}

impl Field86 {
    /// Create a new Field86 with validation
    ///
    /// # Arguments
    /// * `lines` - Vector of text lines (maximum 6 lines, 65 characters each)
    ///
    /// # Returns
    /// Result containing the Field86 instance or validation error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field86;
    /// # use swift_mt_message::fields::common::MultiLineField;
    /// let lines = vec![
    ///     "TRANSFER FROM SAVINGS ACCOUNT".to_string(),
    ///     "REF: TXN123456789".to_string(),
    ///     "BENEFICIARY: JOHN DOE".to_string(),
    /// ];
    /// let field = Field86::new(lines).unwrap();
    /// assert_eq!(field.line_count(), 3);
    /// ```
    pub fn new(lines: Vec<String>) -> Result<Self, ParseError> {
        // Validate with custom line length (65 instead of 35)
        Self::validate_lines_custom(&lines)?;
        Ok(Field86 { information: lines })
    }

    /// Create from single text with automatic line breaking
    ///
    /// # Arguments
    /// * `text` - Single text string to be broken into lines
    ///
    /// # Returns
    /// Result containing the Field86 instance or validation error
    pub fn from_text(text: impl Into<String>) -> Result<Self, ParseError> {
        let text = text.into();
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.len() + word.len() < 65 {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
                }

                if word.len() <= 65 {
                    current_line = word.to_string();
                } else {
                    // Split long words
                    let mut remaining = word;
                    while remaining.len() > 65 {
                        lines.push(remaining[..65].to_string());
                        remaining = &remaining[65..];
                    }
                    if !remaining.is_empty() {
                        current_line = remaining.to_string();
                    }
                }
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "86".to_string(),
                message: "Text cannot be empty".to_string(),
            });
        }

        Self::new(lines)
    }

    /// Validate lines with custom length limit (65 characters)
    fn validate_lines_custom(lines: &[String]) -> Result<(), ParseError> {
        if lines.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "86".to_string(),
                message: "Lines cannot be empty".to_string(),
            });
        }

        if lines.len() > 6 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "86".to_string(),
                message: "Too many lines (max 6)".to_string(),
            });
        }

        for (i, line) in lines.iter().enumerate() {
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "86".to_string(),
                    message: format!("Line {} cannot be empty or whitespace-only", i + 1),
                });
            }

            if line.len() > 65 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "86".to_string(),
                    message: format!("Line {} too long (max 65 characters)", i + 1),
                });
            }

            // Validate characters (printable ASCII)
            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "86".to_string(),
                    message: format!("Line {} contains invalid characters", i + 1),
                });
            }
        }

        Ok(())
    }

    /// Get the full text as a single string
    pub fn full_text(&self) -> String {
        self.information.join(" ")
    }

    /// Get the full text with custom separator
    pub fn full_text_with_separator(&self, separator: &str) -> String {
        self.information.join(separator)
    }

    /// Check if the field is empty
    pub fn is_empty(&self) -> bool {
        self.information.is_empty() || self.information.iter().all(|line| line.trim().is_empty())
    }

    /// Get total character count across all lines
    pub fn total_character_count(&self) -> usize {
        self.information.iter().map(|line| line.len()).sum()
    }

    /// Check if this contains structured information (sub-fields)
    pub fn has_structured_info(&self) -> bool {
        self.information
            .iter()
            .any(|line| line.contains("//") || line.contains("?") || line.contains("/"))
    }

    /// Extract structured information if present
    pub fn extract_structured_info(&self) -> Vec<(String, String)> {
        let mut structured_info = Vec::new();

        for line in &self.information {
            // Look for patterns like /TAG/VALUE or ?TAG?VALUE
            if line.contains("//") {
                let parts: Vec<&str> = line.split("//").collect();
                for part in parts {
                    if let Some(slash_pos) = part.find('/') {
                        let tag = &part[..slash_pos];
                        let value = &part[slash_pos + 1..];
                        if !tag.is_empty() && !value.is_empty() {
                            structured_info.push((tag.to_string(), value.to_string()));
                        }
                    }
                }
            } else if line.contains('?') {
                let parts: Vec<&str> = line.split('?').collect();
                for i in (0..parts.len()).step_by(2) {
                    if i + 1 < parts.len() {
                        let tag = parts[i];
                        let value = parts[i + 1];
                        if !tag.is_empty() && !value.is_empty() {
                            structured_info.push((tag.to_string(), value.to_string()));
                        }
                    }
                }
            }
        }

        structured_info
    }

    /// Check if this is a transaction description
    pub fn is_transaction_description(&self) -> bool {
        let text = self.full_text().to_uppercase();
        text.contains("TRANSFER")
            || text.contains("PAYMENT")
            || text.contains("DEPOSIT")
            || text.contains("WITHDRAWAL")
            || text.contains("TXN")
            || text.contains("TRANSACTION")
    }

    /// Check if this contains reference information
    pub fn has_reference_info(&self) -> bool {
        let text = self.full_text().to_uppercase();
        text.contains("REF:")
            || text.contains("REFERENCE")
            || text.contains("ID:")
            || text.contains("NO:")
    }

    /// Extract reference numbers from the text
    pub fn extract_references(&self) -> Vec<String> {
        let mut references = Vec::new();
        let text = self.full_text();

        // Look for patterns like REF: followed by alphanumeric
        if let Some(ref_pos) = text.find("REF:") {
            let after_ref = &text[ref_pos + 4..];
            if let Some(end_pos) = after_ref.find(|c: char| c.is_whitespace() || c == '\n') {
                let ref_value = after_ref[..end_pos].trim();
                if !ref_value.is_empty() {
                    references.push(ref_value.to_string());
                }
            }
        }

        references
    }

    /// Add a new line to the field
    pub fn add_line(&mut self, line: impl Into<String>) -> Result<(), ParseError> {
        let line = line.into();

        if self.information.len() >= 6 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "86".to_string(),
                message: "Cannot add more lines (max 6)".to_string(),
            });
        }

        if line.len() > 65 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "86".to_string(),
                message: "Line too long (max 65 characters)".to_string(),
            });
        }

        if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "86".to_string(),
                message: "Line contains invalid characters".to_string(),
            });
        }

        let trimmed_line = line.trim().to_string();
        if trimmed_line.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "86".to_string(),
                message: "Line cannot be empty or whitespace-only".to_string(),
            });
        }

        self.information.push(trimmed_line);
        Ok(())
    }

    /// Remove a line by index
    pub fn remove_line(&mut self, index: usize) -> Result<(), ParseError> {
        if index >= self.information.len() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "86".to_string(),
                message: "Line index out of bounds".to_string(),
            });
        }

        self.information.remove(index);

        if self.information.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "86".to_string(),
                message: "Cannot remove last line - field would be empty".to_string(),
            });
        }

        Ok(())
    }
}

impl SwiftField for Field86 {
    fn parse(content: &str) -> Result<Self, ParseError> {
        let content = content.trim();
        if content.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "86".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Remove field tag prefix if present
        let content = if let Some(stripped) = content.strip_prefix(":86:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("86:") {
            stripped
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
                field_tag: "86".to_string(),
                message: "Content cannot be empty".to_string(),
            });
        }

        Self::new(lines)
    }

    fn to_swift_string(&self) -> String {
        format!(":86:{}", self.information.join("\n"))
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Validate using custom validation
        if let Err(e) = Self::validate_lines_custom(&self.information) {
            errors.push(ValidationError::FormatValidation {
                field_tag: "86".to_string(),
                message: e.to_string(),
            });
        }

        // Add specific business validations for information field
        if self.is_empty() {
            warnings
                .push("Empty information field - consider adding transaction details".to_string());
        }

        if self.has_structured_info() {
            warnings.push("Structured information detected - ensure proper formatting".to_string());
        }

        if self.total_character_count() > 300 {
            warnings
                .push("Large information field - consider condensing for readability".to_string());
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    fn format_spec() -> &'static str {
        "6*65x"
    }
}

impl std::fmt::Display for Field86 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.line_count() == 1 {
            write!(f, "Info: {}", self.line(0).unwrap_or(""))
        } else {
            write!(
                f,
                "Info ({} lines): {}",
                self.line_count(),
                self.line(0)
                    .unwrap_or("")
                    .chars()
                    .take(50)
                    .collect::<String>()
            )
        }
    }
}
