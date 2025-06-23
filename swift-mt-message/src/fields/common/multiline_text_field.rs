use crate::{Result, SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Generic MultiLine Text Field
///
/// Parameterized multiline field for different line and character constraints.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenericMultiLineTextField<const MAX_LINES: usize, const MAX_CHARS: usize> {
    /// Text lines
    pub lines: Vec<String>,
}

impl<const MAX_LINES: usize, const MAX_CHARS: usize>
    GenericMultiLineTextField<MAX_LINES, MAX_CHARS>
{
    /// Remove field tag prefix using generic regex pattern
    /// Handles patterns like ":50K:", "50K:", ":20:", "32A:", etc.
    fn remove_field_tag_prefix(value: &str) -> &str {
        // Use lazy_static for regex compilation performance
        use std::sync::OnceLock;
        static FIELD_TAG_REGEX: OnceLock<regex::Regex> = OnceLock::new();

        let regex = FIELD_TAG_REGEX.get_or_init(|| {
            // Pattern matches: optional colon + field identifier + mandatory colon
            // Field identifier: 1-3 digits optionally followed by 1-2 letters
            regex::Regex::new(r"^:?([0-9]{1,3}[A-Z]{0,2}):").unwrap()
        });

        if let Some(captures) = regex.find(value) {
            &value[captures.end()..]
        } else {
            value
        }
    }
}

impl<const MAX_LINES: usize, const MAX_CHARS: usize> SwiftField
    for GenericMultiLineTextField<MAX_LINES, MAX_CHARS>
{
    fn parse(value: &str) -> Result<Self> {
        let content = value.trim();

        // Remove field tag prefix using generic regex pattern
        let content = Self::remove_field_tag_prefix(content);

        let lines = content.lines().map(|line| line.to_string()).collect();
        Ok(Self { lines })
    }

    fn to_swift_string(&self) -> String {
        self.lines.join("\n")
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        let warnings = Vec::new();

        if self.lines.len() > MAX_LINES {
            errors.push(ValidationError::ValueValidation {
                field_tag: "multiline".to_string(),
                message: format!("Too many lines: {} (max {})", self.lines.len(), MAX_LINES),
            });
        }

        for (i, line) in self.lines.iter().enumerate() {
            if line.len() > MAX_CHARS {
                errors.push(ValidationError::ValueValidation {
                    field_tag: "multiline".to_string(),
                    message: format!(
                        "Line {} too long: {} chars (max {})",
                        i + 1,
                        line.len(),
                        MAX_CHARS
                    ),
                });
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    fn format_spec() -> &'static str {
        "lines"
    }
}

/// Type aliases for common sizes
pub type GenericMultiLine3x35 = GenericMultiLineTextField<3, 35>;
pub type GenericMultiLine4x35 = GenericMultiLineTextField<4, 35>;
pub type GenericMultiLine6x35 = GenericMultiLineTextField<6, 35>;
pub type GenericMultiLine6x65 = GenericMultiLineTextField<6, 65>;
pub type GenericMultiLine20x35 = GenericMultiLineTextField<20, 35>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_field_tag_removal() {
        // Test various field tag patterns
        let test_cases = vec![
            (":50K:/1234567890", "/1234567890"),
            ("50K:/1234567890", "/1234567890"),
            (":20:FT123456789", "FT123456789"),
            ("20:FT123456789", "FT123456789"),
            (":32A:241231USD1000000,", "241231USD1000000,"),
            ("32A:241231USD1000000,", "241231USD1000000,"),
            (":71F:USD50,00", "USD50,00"),
            ("71F:USD50,00", "USD50,00"),
            (":13C:/123045+0/+0100/-0500", "/123045+0/+0100/-0500"),
            ("13C:/123045+0/+0100/-0500", "/123045+0/+0100/-0500"),
            // Edge cases
            ("plain_text", "plain_text"), // No field tag
            ("", ""),                     // Empty string
            (":", ":"),                   // Just colon
            ("::", "::"),                 // Two colons - no valid field tag pattern
        ];

        for (input, expected) in test_cases {
            let result = GenericMultiLineTextField::<4, 35>::remove_field_tag_prefix(input);
            assert_eq!(result, expected, "Failed for input: '{}'", input);
        }

        println!("✅ Generic field tag removal works for all patterns!");
    }

    #[test]
    fn test_multiline_field_parsing() {
        // Test multiline field with field tag
        let input = "50K:/1234567890\nACME CORPORATION\n123 MAIN STREET\nNEW YORK NY 10001 US";

        let result = GenericMultiLineTextField::<4, 35>::parse(input).unwrap();

        assert_eq!(result.lines.len(), 4);
        assert_eq!(result.lines[0], "/1234567890");
        assert_eq!(result.lines[1], "ACME CORPORATION");
        assert_eq!(result.lines[2], "123 MAIN STREET");
        assert_eq!(result.lines[3], "NEW YORK NY 10001 US");

        println!("✅ Multiline field parsing with field tag removal works correctly!");
    }
}
