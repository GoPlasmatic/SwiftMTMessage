use crate::{SwiftField, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Field 28 - Statement Number/Sequence Number
///
/// ## Overview
/// Field 28 represents the statement number and optional sequence number for balance
/// reporting messages. It follows the format `5n[/2n]` where the first part is the
/// statement number (up to 5 digits) and the optional second part is the sequence
/// number (up to 2 digits).
///
/// ## Format Specification
/// **Format**: `5n[/2n]`
/// - **5n**: Statement number (1-5 digits, leading zeros allowed)
/// - **[/2n]**: Optional sequence number (1-2 digits, preceded by slash)
///
/// ## Usage Context
/// Used in MT941 (Balance Report) messages to identify:
/// - **Statement Number**: Unique identifier for the statement period
/// - **Sequence Number**: Optional sub-sequence for multi-part statements
///
/// ## Usage Examples
/// ```text
/// 00001
/// └─── Statement 1, no sequence
///
/// 12345/01
/// └─── Statement 12345, sequence 1
///
/// 999/99
/// └─── Statement 999, sequence 99
/// ```
///
/// ## Validation Rules
/// 1. **Statement number**: 1-5 digits, cannot be empty
/// 2. **Sequence number**: If present, 1-2 digits after slash
/// 3. **Format**: Must follow exact pattern `5n[/2n]`
/// 4. **Range**: Statement number 1-99999, sequence 1-99
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Statement number must be numeric (Error: T40)
/// - Statement number cannot exceed 5 digits (Error: T50)
/// - Sequence number cannot exceed 2 digits (Error: T50)
/// - Slash required if sequence number present (Error: T41)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field28 {
    /// Statement number (1-5 digits)
    pub statement_number: u32,
    /// Optional sequence number (1-2 digits)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<u8>,
}

impl Field28 {
    /// Create a new Field28 with validation
    ///
    /// # Arguments
    /// * `statement_number` - Statement number (1-99999)
    /// * `sequence_number` - Optional sequence number (1-99)
    ///
    /// # Returns
    /// Result containing the Field28 instance or validation error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field28;
    /// let field = Field28::new(12345, Some(1)).unwrap();
    /// assert_eq!(field.statement_number(), 12345);
    /// assert_eq!(field.sequence_number(), Some(1));
    /// ```
    pub fn new(statement_number: u32, sequence_number: Option<u8>) -> Result<Self, ParseError> {
        // Validate statement number
        if statement_number == 0 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "28".to_string(),
                message: "Statement number cannot be zero".to_string(),
            });
        }

        if statement_number > 99999 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "28".to_string(),
                message: "Statement number cannot exceed 99999".to_string(),
            });
        }

        // Validate sequence number if present
        if let Some(seq) = sequence_number {
            if seq == 0 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "28".to_string(),
                    message: "Sequence number cannot be zero".to_string(),
                });
            }

            if seq > 99 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "28".to_string(),
                    message: "Sequence number cannot exceed 99".to_string(),
                });
            }
        }

        Ok(Field28 {
            statement_number,
            sequence_number,
        })
    }

    /// Create Field28 with only statement number
    ///
    /// # Arguments
    /// * `statement_number` - Statement number (1-99999)
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field28;
    /// let field = Field28::statement_only(12345).unwrap();
    /// assert_eq!(field.statement_number(), 12345);
    /// assert!(field.sequence_number().is_none());
    /// ```
    pub fn statement_only(statement_number: u32) -> Result<Self, ParseError> {
        Self::new(statement_number, None)
    }

    /// Get the statement number
    pub fn statement_number(&self) -> u32 {
        self.statement_number
    }

    /// Get the sequence number
    pub fn sequence_number(&self) -> Option<u8> {
        self.sequence_number
    }

    /// Check if this is a multi-part statement (has sequence number)
    pub fn is_multi_part(&self) -> bool {
        self.sequence_number.is_some()
    }

    /// Check if this is the first sequence in a multi-part statement
    pub fn is_first_sequence(&self) -> bool {
        self.sequence_number == Some(1)
    }

    /// Get the next sequence number for continuation
    pub fn next_sequence(&self) -> Option<u8> {
        match self.sequence_number {
            Some(seq) if seq < 99 => Some(seq + 1),
            None => Some(1),
            _ => None, // Already at maximum
        }
    }

    /// Format as padded statement number string
    pub fn format_statement_padded(&self) -> String {
        format!("{:05}", self.statement_number)
    }

    /// Get a human-readable description
    pub fn description(&self) -> String {
        match self.sequence_number {
            Some(seq) => format!("Statement {} (Sequence {})", self.statement_number, seq),
            None => format!("Statement {}", self.statement_number),
        }
    }
}

impl SwiftField for Field28 {
    fn parse(content: &str) -> Result<Self, ParseError> {
        let content = content.trim();
        if content.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "28".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Remove field tag prefix if present
        let content = if let Some(stripped) = content.strip_prefix(":28:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("28:") {
            stripped
        } else {
            content
        };

        // Parse statement number and optional sequence number
        if let Some(slash_pos) = content.find('/') {
            // Has sequence number
            let statement_str = &content[..slash_pos];
            let sequence_str = &content[slash_pos + 1..];

            if statement_str.is_empty() {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "28".to_string(),
                    message: "Statement number cannot be empty".to_string(),
                });
            }

            if sequence_str.is_empty() {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "28".to_string(),
                    message: "Sequence number cannot be empty after slash".to_string(),
                });
            }

            let statement_number =
                statement_str
                    .parse::<u32>()
                    .map_err(|_| ParseError::InvalidFieldFormat {
                        field_tag: "28".to_string(),
                        message: "Invalid statement number format".to_string(),
                    })?;

            let sequence_number =
                sequence_str
                    .parse::<u8>()
                    .map_err(|_| ParseError::InvalidFieldFormat {
                        field_tag: "28".to_string(),
                        message: "Invalid sequence number format".to_string(),
                    })?;

            Self::new(statement_number, Some(sequence_number))
        } else {
            // Statement number only
            let statement_number =
                content
                    .parse::<u32>()
                    .map_err(|_| ParseError::InvalidFieldFormat {
                        field_tag: "28".to_string(),
                        message: "Invalid statement number format".to_string(),
                    })?;

            Self::new(statement_number, None)
        }
    }

    fn to_swift_string(&self) -> String {
        match self.sequence_number {
            Some(seq) => format!(":28:{:05}/{:02}", self.statement_number, seq),
            None => format!(":28:{:05}", self.statement_number),
        }
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
        "5n[/2n]"
    }
}

impl std::fmt::Display for Field28 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.sequence_number {
            Some(seq) => write!(f, "{:05}/{:02}", self.statement_number, seq),
            None => write!(f, "{:05}", self.statement_number),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field28_creation_statement_only() {
        let field = Field28::statement_only(12345).unwrap();
        assert_eq!(field.statement_number(), 12345);
        assert!(field.sequence_number().is_none());
        assert!(!field.is_multi_part());
    }

    #[test]
    fn test_field28_creation_with_sequence() {
        let field = Field28::new(12345, Some(1)).unwrap();
        assert_eq!(field.statement_number(), 12345);
        assert_eq!(field.sequence_number(), Some(1));
        assert!(field.is_multi_part());
        assert!(field.is_first_sequence());
    }

    #[test]
    fn test_field28_parse_statement_only() {
        let field = Field28::parse("12345").unwrap();
        assert_eq!(field.statement_number(), 12345);
        assert!(field.sequence_number().is_none());
    }

    #[test]
    fn test_field28_parse_with_sequence() {
        let field = Field28::parse("12345/01").unwrap();
        assert_eq!(field.statement_number(), 12345);
        assert_eq!(field.sequence_number(), Some(1));
    }

    #[test]
    fn test_field28_parse_with_field_tag() {
        let field = Field28::parse(":28:12345/01").unwrap();
        assert_eq!(field.statement_number(), 12345);
        assert_eq!(field.sequence_number(), Some(1));
    }

    #[test]
    fn test_field28_to_swift_string() {
        let field1 = Field28::statement_only(12345).unwrap();
        assert_eq!(field1.to_swift_string(), ":28:12345");

        let field2 = Field28::new(12345, Some(1)).unwrap();
        assert_eq!(field2.to_swift_string(), ":28:12345/01");
    }

    #[test]
    fn test_field28_display() {
        let field1 = Field28::statement_only(123).unwrap();
        assert_eq!(format!("{}", field1), "00123");

        let field2 = Field28::new(123, Some(5)).unwrap();
        assert_eq!(format!("{}", field2), "00123/05");
    }

    #[test]
    fn test_field28_validation_errors() {
        // Zero statement number
        let result = Field28::new(0, None);
        assert!(result.is_err());

        // Statement number too large
        let result = Field28::new(100000, None);
        assert!(result.is_err());

        // Zero sequence number
        let result = Field28::new(123, Some(0));
        assert!(result.is_err());

        // Sequence number too large
        let result = Field28::new(123, Some(100));
        assert!(result.is_err());
    }

    #[test]
    fn test_field28_next_sequence() {
        let field1 = Field28::statement_only(123).unwrap();
        assert_eq!(field1.next_sequence(), Some(1));

        let field2 = Field28::new(123, Some(5)).unwrap();
        assert_eq!(field2.next_sequence(), Some(6));

        let field3 = Field28::new(123, Some(99)).unwrap();
        assert!(field3.next_sequence().is_none());
    }

    #[test]
    fn test_field28_description() {
        let field1 = Field28::statement_only(123).unwrap();
        assert_eq!(field1.description(), "Statement 123");

        let field2 = Field28::new(123, Some(5)).unwrap();
        assert_eq!(field2.description(), "Statement 123 (Sequence 5)");
    }

    #[test]
    fn test_field28_format_statement_padded() {
        let field = Field28::statement_only(123).unwrap();
        assert_eq!(field.format_statement_padded(), "00123");
    }

    #[test]
    fn test_field28_parse_errors() {
        // Empty content
        let result = Field28::parse("");
        assert!(result.is_err());

        // Invalid statement number
        let result = Field28::parse("abc");
        assert!(result.is_err());

        // Invalid sequence number
        let result = Field28::parse("123/abc");
        assert!(result.is_err());

        // Empty sequence after slash
        let result = Field28::parse("123/");
        assert!(result.is_err());
    }
}
