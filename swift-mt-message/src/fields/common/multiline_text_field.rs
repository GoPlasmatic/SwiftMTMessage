use crate::{Result, SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Generic MultiLine Text Field
/// Parameterized multiline field for different line/character constraints.
/// Used for fields like Field70 (4*35x), Field72 (6*35x), Field77B (3*35x), etc.
/// Format: {MAX_LINES}*{MAX_CHARS}x
/// Validation: line_count, line_length
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenericMultiLineTextField<const MAX_LINES: usize, const MAX_CHARS: usize> {
    /// Text lines (up to MAX_LINES lines, MAX_CHARS characters each)
    pub lines: Vec<String>,
}

impl<const MAX_LINES: usize, const MAX_CHARS: usize> SwiftField
    for GenericMultiLineTextField<MAX_LINES, MAX_CHARS>
{
    fn parse(value: &str) -> Result<Self> {
        let content = value.trim();
        let content = if let Some(colon_pos) = content.find(':') {
            &content[colon_pos + 1..]
        } else {
            content
        };
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

// Type aliases for common sizes
pub type GenericMultiLine3x35 = GenericMultiLineTextField<3, 35>;
pub type GenericMultiLine4x35 = GenericMultiLineTextField<4, 35>;
pub type GenericMultiLine6x35 = GenericMultiLineTextField<6, 35>;
pub type GenericMultiLine6x65 = GenericMultiLineTextField<6, 65>;
